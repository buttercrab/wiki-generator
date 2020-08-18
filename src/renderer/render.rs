use crate::config::config::Config;
use crate::renderer::render_markdown::render_markdown;
use crate::themes::INDEX;

use crate::renderer::context::Context;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use walkdir::WalkDir;

fn make_dir_above(path: &Path) {
    fs::create_dir_all(path.parent().unwrap())
        .expect(&*format!("make dir above {:?} failed", path));
}

fn target_path(path: &Path) -> PathBuf {
    Path::new("public").join(path.strip_prefix("src").unwrap())
}

fn move_file(path: &Path) {
    let to = &*target_path(path);
    make_dir_above(to);
    fs::copy(path, to).expect(&*format!("file {:?} copy failed", path));
}

pub fn render(config: &Config) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index.hbs", str::from_utf8(INDEX).unwrap())
        .unwrap();
    handlebars.register_escape_fn(|s| s.into());

    let img_map = HashMap::new();

    for item in WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|i| i.metadata().unwrap().is_file())
    {
        print!("processing {:?}... ", item.path());

        match item.path().extension() {
            Some(ext) => {
                if ext.to_os_string().into_string().unwrap() == "md" {
                    let to = &*target_path(item.path());
                    let to = &*to.with_extension("html");
                    make_dir_above(to);
                    render_markdown(
                        item.path(),
                        to,
                        Context {
                            img_map: &img_map,
                            handlebars: &handlebars,
                            config,
                        },
                    );
                } else {
                    move_file(item.path());
                }
            }

            None => {
                move_file(item.path());
            }
        }

        println!("done!");
    }
}
