use crate::config::config::Config;
use crate::renderer::markdown;
use crate::renderer::postprocess::{add_github_info, fix_footnotes, fix_header, fix_link};
use crate::util::path;
use handlebars::Handlebars;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Page {
    from: PathBuf,
    temp: PathBuf,
    to: PathBuf,
    preserve: bool,
    pub(crate) title: String,
}

impl Page {
    /// Creates new Page
    /// makes `temp` and `to`
    ///
    /// assertion: extension of `from` is `md`
    /// example:
    ///   "src/hello.md" -> "public/w/hello/index.html"
    ///   "src/world/index.md" -> "public/w/world/index.html"
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
        from: P,
        src: Q,
        out: R,
        preserve: &Option<Vec<String>>,
    ) -> Page {
        let from = from.as_ref();
        let src = src.as_ref();
        let out = out.as_ref();

        debug_assert_eq!(
            path::os_to_str(from.extension().unwrap_or(OsStr::new(""))),
            "md"
        );

        let preserve = match preserve {
            Some(preserve) => preserve.contains(
                &path::path_to_str(from.strip_prefix(&src).unwrap()).to_ascii_lowercase(),
            ),
            None => false,
        };

        let mut file_name = from.strip_prefix(src).unwrap().with_extension("");
        if path::os_to_str(file_name.file_stem().unwrap_or(OsStr::new(""))) == "index" {
            file_name = file_name.parent().unwrap().to_path_buf();
        }
        let file_name = file_name.join("index.html");

        let temp = Path::new(out).join("t").join(&file_name);
        let to = if preserve {
            Path::new(out).join(from.strip_prefix(src).unwrap())
        } else {
            Path::new(out).join("w")
        };

        Page {
            from: from.to_path_buf(),
            temp,
            to,
            preserve,
            title: "".to_string(),
        }
    }

    /// Pre-render markdown to html
    pub fn pre_render(&mut self) {
        let content =
            fs::read_to_string(&self.from).expect(&*format!("reading from {:?} failed", self.from));
        let content = markdown::cmark_to_html(content);

        self.title =
            markdown::get_title(&content).expect(&*format!("Title not found in {:?}", self.from));
        if !self.preserve {
            self.to = self.to.join(&self.title).join("index.html");
        }

        path::make_dir_above(&self.temp);
        fs::write(&self.temp, &content).expect(&*format!("writing to {:?} failed", self.temp));
    }

    /// Render main html
    pub fn render(
        &self,
        config: &Config,
        handlebars: &Handlebars,
        mut data: serde_json::Map<String, serde_json::Value>,
        file_map: &HashMap<String, String>,
        titles: &HashSet<String>,
        contrib_data: &HashMap<String, String>,
    ) {
        print!("Rendering {:?} ...", self.from);
        io::stdout().flush().unwrap();

        let content =
            fs::read_to_string(&self.temp).expect(&*format!("reading from {:?} failed", self.temp));

        let content = fix_link(content, &self.from, file_map, titles);
        let content = fix_footnotes(content);

        data.insert("content".to_string(), json!(content));
        data.insert(
            "title".to_string(),
            json!(format!("{} - {}", self.title, config.wiki.title)),
        );

        if let Some(h) = &config.html {
            if let Some(gu) = &h.github {
                add_github_info(
                    &mut data,
                    &self.from,
                    &self.title,
                    gu,
                    contrib_data
                        .get(&path::path_to_str(&self.from))
                        .unwrap()
                        .clone(),
                );
            }
        }

        let html = handlebars
            .render("index.hbs", &data)
            .expect(&*format!("render {:?} failed", self.from));

        let html = fix_header(html);

        path::make_dir_above(&self.to);
        fs::write(&self.to, html).expect(&*format!("writing to {:?} failed", self.to));

        println!(" done");
    }
}

#[cfg(test)]
mod tests {
    use crate::util::path;
    use crate::wiki::page::Page;

    #[test]
    fn page_new_path_test() {
        let page = Page::new("src/index.md", "src", "public", &None);
        assert_eq!(path::path_to_str(page.temp), "public/t/index.html");

        let page = Page::new("src/index/index.md", "src", "public", &None);
        assert_eq!(path::path_to_str(page.temp), "public/t/index/index.html");

        let page = Page::new("src/hello/index.md", "src", "public", &None);
        assert_eq!(path::path_to_str(page.temp), "public/t/hello/index.html");

        let page = Page::new("src/hello.md", "src", "public", &None);
        assert_eq!(path::path_to_str(page.temp), "public/t/hello/index.html");

        let page = Page::new("src/hello/world.md", "src", "public", &None);
        assert_eq!(
            path::path_to_str(page.temp),
            "public/t/hello/world/index.html"
        );
    }
}
