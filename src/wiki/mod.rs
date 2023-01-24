pub mod file;
pub mod page;

use crate::config::Config;
use crate::public;
use crate::util::{path, string};
use crate::wiki::file::File;
use crate::wiki::page::Page;
use handlebars::Handlebars;
use regex::Regex;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub struct Wiki {
    config: Config,
    pages: Vec<Page>,
    files: Vec<File>,
    file_map: HashMap<String, String>,
    contrib_data: HashMap<String, String>,
}

impl Wiki {
    pub async fn new(config: Config) -> Wiki {
        let src_dir = PathBuf::from(config.wiki.src.clone().unwrap_or_else(|| "src".to_string()));
        let out_dir = PathBuf::from(
            config
                .wiki
                .out
                .clone()
                .unwrap_or_else(|| "public".to_string()),
        );
        let file_list = path::get_files_all(&src_dir);
        let mut pages = Vec::new();
        let mut files = Vec::new();
        let mut file_map = HashMap::new();
        let mut contrib_urls = Vec::new();
        let mut github_url = String::new();

        if let Some(h) = &config.html {
            if let Some(gu) = &h.github {
                github_url = gu.clone();
            }
        }

        for file in file_list.iter() {
            if path::os_to_str(file.extension().unwrap_or_else(|| OsStr::new(""))) == "md" {
                pages.push(Page::new(file, &src_dir, &out_dir, &config.wiki.preserve));
                if !github_url.is_empty() {
                    let from = path::path_to_str(file);
                    contrib_urls.push((
                        from.clone(),
                        format!("{github_url}/contributors-list/master/{from}"),
                    ));
                }
            } else {
                files.push(File::new(file, &src_dir, &out_dir, &config.wiki.preserve));
                file_map.insert(
                    path::path_to_str(&files.last().unwrap().from),
                    path::path_to_str(files.last().unwrap().to.strip_prefix(&out_dir).unwrap()),
                );
            }
        }

        let client = Client::new();

        let bodies = futures::future::join_all(contrib_urls.into_iter().map(|(from, url)| {
            let client = &client;
            async move {
                let resp = client.get(&url).send().await?;
                resp.bytes().await.map(|b| (from, b))
            }
        }))
        .await;

        let mut contrib_data = HashMap::new();

        for b in bodies.iter() {
            let (from, data) = b.as_ref().unwrap_or_else(|_| {
                panic!("Error on fetching contributors");
            });
            let contributors = String::from_utf8(data.to_vec()).unwrap();

            let cont_id = Regex::new(r##"href="/(.*?)""##)
                .unwrap()
                .captures_iter(&contributors)
                .map(|c| String::from(&c[1]))
                .collect::<Vec<_>>();

            let mut cont_html =
                String::from(r##"<div class="description"><span>기여자:&nbsp;</span></div>"##);
            for id in cont_id.iter() {
                cont_html.push_str(
                    &format!(
                        r##"<a href="https://github.com/{id}" target="_blank"><span title="{id}"><img src="https://github.com/{id}.png?size=32" width="24" height="24" alt="@{id}"/></span></a>"##,
                    )
                );
            }

            contrib_data.insert(from.clone(), cont_html);
        }

        public::init(&out_dir).await;

        Wiki {
            config,
            pages,
            files,
            file_map,
            contrib_data,
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
                panic!("title is same: {}", page.title);
            }
        }

        let mut title_data = serde_json::Map::new();

        for i in titles.iter() {
            let v = string::typing_process(i);
            for j in v.iter() {
                let j = j.to_ascii_lowercase();
                if title_data.contains_key(&j) {
                    if let serde_json::Value::Array(a) = title_data.get_mut(&j).unwrap() {
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
        let out_dir = PathBuf::from(
            &self
                .config
                .wiki
                .out
                .clone()
                .unwrap_or_else(|| "public".to_string()),
        );
        fs::write(out_dir.join("r").join("search.json"), title_data)
            .expect("failed to write search.json");

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("index.hbs", public::INDEX_HBS)
            .unwrap();
        handlebars
            .register_template_string("redirect.hbs", public::REDIRECT_HBS)
            .unwrap();

        let mut data = serde_json::Map::new();
        if let Some(h) = &self.config.html {
            if let Some(g) = &h.ga {
                data.insert("google_analytics".to_string(), json!(g));
            }

            if let Some(l) = &h.logo {
                data.insert("logo".to_string(), json!(l));
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
            data.insert("url".to_string(), json!(format!("/w/{m}")));
            let html = handlebars.render("redirect.hbs", &data).unwrap();
            let out_dir = PathBuf::from(
                &self
                    .config
                    .wiki
                    .out
                    .clone()
                    .unwrap_or_else(|| "public".to_string()),
            );
            fs::write(out_dir.join("index.html"), html).expect("failed to write index.html");
        }

        for page in self.pages.iter() {
            page.render(
                &self.config,
                &handlebars,
                data.clone(),
                &self.file_map,
                &titles,
                &self.contrib_data,
            );
        }

        let out_dir = PathBuf::from(
            self.config
                .wiki
                .out
                .clone()
                .unwrap_or_else(|| "public".to_string()),
        );
        fs::remove_dir_all(out_dir.join("t"))
            .unwrap_or_else(|_| panic!("failed to remove {:?}", out_dir));
    }
}
