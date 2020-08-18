use crate::renderer::context::Context;

use crate::renderer::post_render::add_github_things;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::fs;
use std::path::Path;

fn cmark_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&*content, options);
    let mut res = String::new();
    html::push_html(&mut res, parser);

    res
}

fn get_title(content: &str) -> String {
    Regex::new(r##"<h1[.*]>(.*)</h1>"##)
        .unwrap()
        .captures_iter(content)
        .collect::<Vec<_>>()[0][1]
        .to_owned()
}

pub fn render_markdown(from: &Path, to: &Path, context: Context) {
    let content = fs::read_to_string(from).expect(&*format!("reading from {:?} failed", from));
    let content = cmark_to_html(content);

    let mut data = serde_json::Map::new();
    data.insert("content".to_owned(), json!(content));
    data.insert("title".to_owned(), json!(get_title(&*content)));

    let html = context
        .handlebars
        .render("index.hbs", &data)
        .expect(&*format!("render {:?} failed", from));

    let github_url = match &context.config.html {
        Some(s) => &s.github,
        None => &None,
    };

    let html = add_github_things(html, github_url, &from.to_path_buf());

    fs::write(to, html).expect("file write failed");
}
