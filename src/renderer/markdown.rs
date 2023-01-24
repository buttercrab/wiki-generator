use crate::util::{path, string};
use chrono::{Datelike, Timelike};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::path::Path;
use std::process::Command;

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

    let title = Regex::new(r##"<h1.*>(.*?)</h1>"##)
        .unwrap()
        .captures_iter(content)
        .collect::<Vec<_>>();

    if title.len() != 1 {
        None
    } else {
        Some(string::unescape_html(&title[0][1]))
    }
}

pub fn get_time<P: AsRef<Path>>(path: P) -> String {
    let path = path::path_to_str(path.as_ref());

    let time = String::from_utf8(
        Command::new("/bin/bash")
            .arg("-c")
            .arg(format!("git log -1 --format=%cd {path}"))
            .output()
            .expect("git command failed")
            .stdout,
    )
    .expect("parse error");

    let time = chrono::DateTime::parse_from_str(&time, "%a %b %-d %H:%M:%S %Y %z%n")
        .expect("date parse fail");

    format!(
        r##"{}-{:02}-{:02} {:02}:{:02}:{:02}"##,
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second()
    )
}

pub fn get_github_history<P: AsRef<Path>, S: AsRef<str>>(path: P, github_url: S) -> String {
    let path = path::path_to_str(path.as_ref());
    let github_url = github_url.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/commits/master/{path}"><div class="button">역사</div></a>"##
    )
}

pub fn get_github_edit<P: AsRef<Path>, S: AsRef<str>>(path: P, github_url: S) -> String {
    let path = path::path_to_str(path.as_ref());
    let github_url = github_url.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/edit/master/{path}"><div class="button">편집</div></a>"##
    )
}

pub fn get_github_view_issue<S: AsRef<str>, T: AsRef<str>>(github_url: S, title: T) -> String {
    let github_url = github_url.as_ref();
    let title = title.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+%5B{title}%5D"><div class="button">진행중인 토론</div></a>"##
    )
}

pub fn get_github_make_issue<S: AsRef<str>, T: AsRef<str>>(github_url: S, title: T) -> String {
    let github_url = github_url.as_ref();
    let title = title.as_ref();

    format!(
        r##"<a target="_blank" href={github_url}/issues/new?title=%5B{title}%5D+"><div class="button">토론 생성하기</div></a>"##
    )
}

pub fn get_view_in_github<P: AsRef<Path>, S: AsRef<str>>(path: P, github_url: S) -> String {
    let path = path::path_to_str(path.as_ref());
    let github_url = github_url.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/blob/master/{path}"><div class="button">Github에서 보기</div></a>"##
    )
}

pub fn get_view_in_github_mobile<P: AsRef<Path>, S: AsRef<str>>(path: P, github_url: S) -> String {
    let path = path::path_to_str(path.as_ref());
    let github_url = github_url.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/blob/master/{path}"><div class="button hs">Github에서 보기</div></a>"##
    )
}

pub fn get_github_make_issue_mobile<S: AsRef<str>, T: AsRef<str>>(
    github_url: S,
    title: T,
) -> String {
    let github_url = github_url.as_ref();
    let title = title.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/issues/new?title=%5B{title}%5D+"><div class="button hs">토론 생성하기</div><div class="button hm">새 토론</div></a>"##
    )
}

pub fn get_github_view_issue_mobile<S: AsRef<str>, T: AsRef<str>>(
    github_url: S,
    title: T,
) -> String {
    let github_url = github_url.as_ref();
    let title = title.as_ref();

    format!(
        r##"<a target="_blank" href="{github_url}/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+%5B{title}%5D"><div class="button hs">진행중인 토론</div><div class="button hm">토론 보기</div></a>"##
    )
}
