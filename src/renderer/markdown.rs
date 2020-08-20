use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

pub fn cmark_to_html<S: AsRef<str>>(content: S) -> String {
    let content = content.as_ref();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut res = String::with_capacity(content.len() * 2);
    html::push_html(&mut res, parser);

    res
}

pub fn get_title<S: AsRef<str>>(content: S) -> Option<String> {
    let content = content.as_ref();

    let title = Regex::new(r##"<h1.*>(.*)</h1>"##)
        .unwrap()
        .captures_iter(content)
        .collect::<Vec<_>>();

    if title.len() != 1 {
        None
    } else {
        Some(title[0][1].to_string())
    }
}

pub fn fix_markdown<S: AsRef<str>>(content: S) -> String {
    let content = content.as_ref();
    content.to_string()
}
