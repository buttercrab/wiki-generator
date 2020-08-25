use crate::config::config::Config;
use crate::public;
use crate::util::{path, string};
use crate::wiki::file::File;
use crate::wiki::page::Page;
use handlebars::Handlebars;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, str};

pub struct Wiki {
    config: Config,
    pages: Vec<Page>,
    files: Vec<File>,
    file_map: HashMap<String, String>,
}

impl Wiki {
    pub fn new(config: Config) -> Wiki {
        let src_dir = PathBuf::from(config.wiki.src.clone().unwrap_or("src".to_string()));
        let out_dir = PathBuf::from(config.wiki.out.clone().unwrap_or("public".to_string()));
        let file_list = path::get_files_all(&src_dir);
        let mut pages = Vec::new();
        let mut files = Vec::new();
        let mut file_map = HashMap::new();

        for file in file_list.iter() {
            if path::os_to_str(file.extension().unwrap_or(OsStr::new(""))) == "md" {
                pages.push(Page::new(file, &src_dir, &out_dir, &config.wiki.preserve));
            } else {
                files.push(File::new(file, &src_dir, &out_dir, &config.wiki.preserve));
                file_map.insert(
                    path::path_to_str(&files.last().unwrap().from),
                    path::path_to_str(&files.last().unwrap().to.strip_prefix(&out_dir).unwrap()),
                );
            }
        }

        public::init(&out_dir);

        Wiki {
            config,
            pages,
            files,
            file_map,
        }
    }

    pub fn render(&mut self) {
        for file in self.files.iter() {
            file.copy();
        }

        let mut titles = HashSet::new();

        for page in self.pages.iter_mut() {
            page.pre_render();
            if !titles.insert(page.title.clone()) {
                panic!(format!("title is same: {}", page.title));
            }
        }

        let mut title_data = serde_json::Map::new();

        for i in titles.iter() {
            let i = i.to_ascii_lowercase();
            let v = string::typing_process(&i);
            for j in v.iter() {
                if title_data.contains_key(j) {
                    if let serde_json::Value::Array(a) = title_data.get_mut(j).unwrap() {
                        a.push(serde_json::Value::String(i.clone()));
                    }
                } else {
                    title_data.insert(
                        j.clone(),
                        serde_json::Value::Array(vec![serde_json::Value::String(i.clone())]),
                    );
                }
            }
        }

        let title_data = serde_json::Value::Object(title_data);
        let title_data = title_data.to_string();
        let out_dir = PathBuf::from(&self.config.wiki.out.clone().unwrap_or("public".to_string()));
        fs::write(out_dir.join("r").join("search.json"), title_data)
            .expect("failed to write search.json");

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("index.hbs", str::from_utf8(public::INDEX_HBS).unwrap())
            .unwrap();
        handlebars
            .register_template_string(
                "redirect.hbs",
                str::from_utf8(public::REDIRECT_HBS).unwrap(),
            )
            .unwrap();

        let mut data = serde_json::Map::new();
        if let Some(h) = &self.config.html {
            if let Some(g) = &h.ga {
                data.insert("google_analytics".to_string(), json!(g));
            }
        }

        if let Some(a) = &self.config.wiki.author {
            data.insert("author".to_string(), json!(a));
        }

        if let Some(d) = &self.config.wiki.description {
            data.insert("description".to_string(), json!(d));
        }

        if let Some(m) = &self.config.wiki.main {
            let mut data = serde_json::Map::new();
            data.insert("url".to_string(), json!(format!("/w/{}", m)));
            let html = handlebars.render("redirect.hbs", &data).unwrap();
            let out_dir =
                PathBuf::from(&self.config.wiki.out.clone().unwrap_or("public".to_string()));
            fs::write(out_dir.join("index.html"), html).expect("failed to write index.html");
        }

        for page in self.pages.iter() {
            page.render(
                &self.config,
                &handlebars,
                data.clone(),
                &self.file_map,
                &titles,
            );
        }

        let out_dir = PathBuf::from(self.config.wiki.out.clone().unwrap_or("public".to_string()));
        fs::remove_dir_all(out_dir.join("t")).expect(&*format!("failed to remove {:?}", out_dir));
    }
}
