use crate::config::config::Config;
use crate::renderer::context::Context;
use crate::renderer::render_markdown::render_markdown;
use crate::themes::INDEX;
use crate::util::path;

use crate::util::path::make_dir_above;
use handlebars::Handlebars;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str;

static IMG_LIST: &[&str] = &[
    "apng", "bmp", "gif", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "svg", "tif",
    "tiff", "webp",
];

pub fn render(config: &Config) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index.hbs", str::from_utf8(INDEX).unwrap())
        .unwrap();
    handlebars.register_escape_fn(|s| s.into());
    let mut img_map = HashMap::new();
    let file_list = path::get_files_all("src");
    fs::remove_dir_all("public").expect("failed to erase public/");

    for file in file_list.iter() {
        match file.extension() {
            Some(ext) => {
                let ext = path::os_to_str(ext);
                if IMG_LIST.contains(&&*ext) {
                    let mut hasher = Sha256::new();
                    hasher.update(fs::read(file).unwrap());
                    let res = &hasher.finalize()[..];
                    let mut hash = String::new();
                    for i in 0..8 {
                        hash.push_str(&*format!("{:02x}", res[i]));
                    }

                    let path_to = format!(
                        "img/{name}-{hash}.{ext}",
                        name = path::os_to_str(file.file_stem().unwrap()),
                        hash = hash,
                        ext = path::os_to_str(ext)
                    );

                    img_map.insert(
                        path::path_to_str(file.strip_prefix("src").unwrap()),
                        path_to.clone(),
                    );
                    let path_to = Path::new("public").join(path_to);
                    make_dir_above(&path_to);

                    fs::copy(file, path_to).expect(&*format!("file {:?} copy failed", file));
                } else if ext != "md" {
                    path::move_file(file);
                }
            }

            None => {
                path::move_file(file);
            }
        }
    }

    for file in file_list.iter() {
        match file.extension() {
            Some(ext) => {
                if ext.to_os_string().into_string().unwrap() == "md" {
                    print!("processing {:?}... ", file);

                    let to = &*path::target_path(file);
                    let to = &*to.with_extension("html");
                    path::make_dir_above(to);

                    render_markdown(
                        file,
                        to,
                        Context {
                            img_map: &img_map,
                            handlebars: &handlebars,
                            config,
                        },
                    );

                    println!("done!");
                }
            }

            None => {}
        }
    }
}
