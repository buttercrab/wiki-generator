use std::path::PathBuf;
use std::process::Command;

use chrono::{Datelike, Timelike};
use regex::Regex;

pub fn add_github_things(html: String, github_url: &Option<String>, path: &PathBuf) -> String {
    if let Some(github_url) = github_url {
        let path = path.clone().into_os_string().into_string().unwrap();

        let regex = Regex::new(r##"<!-- :title=(.*?): -->"##).unwrap();
        let title = &regex.captures_iter(&*html).collect::<Vec<_>>()[0][1];

        let time = String::from_utf8(
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("git log -1 --format=%cd src/{}", path))
                .output()
                .expect("git command failed")
                .stdout,
        )
        .expect("parse error");

        let time = chrono::DateTime::parse_from_str(&*time, "%a %b %-d %H:%M:%S %Y %z%n")
            .expect("date parse fail");

        let contributors = reqwest::blocking::get(&*format!(
            "{github_url}/contributors-list/master/src/{path}",
            github_url = github_url,
            path = path
        ))
        .expect("fail to fetch contributors")
        .text()
        .unwrap();

        let cont_id = Regex::new(r##"href="/(.*)""##)
            .unwrap()
            .captures_iter(&*contributors)
            .map(|c| String::from(&c[1]))
            .collect::<Vec<_>>();

        let mut cont_html = String::new();
        for id in cont_id.iter() {
            cont_html.push_str(
                &*format!(
                    r##"<a href="https://github.com/{id}" target="_blank"><span title="{id}"><img src="https://github.com/{id}.png?size=32" width="24" height="24" alt="@{id}"/></span></a>"##,
                    id = id,
                )
            )
        }

        let html = html.replace(
            "<!-- :github-history: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/commits/master/src/{path}"><div class="link-button">역사</div></a>"##,
                github_url = github_url,
                path = path,
            ),
        ).replace(
            "<!-- :github-edit: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/edit/master/src/{path}"><div class="link-button">편집</div></a>"##,
                github_url = github_url,
                path = path,
            ),
        ).replace(
            "<!-- :github-view-issue: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+%5B{title}%5D"><div class="link-button">진행중인 토론</div></a>"##,
                github_url = github_url,
                title = title,
            ),
        ).replace(
            "<!-- :github-make-issue: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/issues/new?title=%5B{title}%5D+"><div class="link-button">토론 생성하기</div></a>"##,
                github_url = github_url,
                title = title,
            ),
        ).replace(
            "<!-- :view-in-github: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/blob/master/src/{path}"><div class="link-button">Github에서 보기</div></a>"##,
                github_url = github_url,
                path = path,
            ),
        ).replace(
            "<!-- :time: -->",
            &*format!(
                r##"최근 수정 시각: {}-{:02}-{:02} {:02}:{:02}:{:02}"##,
                time.year(),
                time.month(),
                time.day(),
                time.hour(),
                time.minute(),
                time.second()
            ),
        ).replace(
            "<!-- :view-in-github-mobile: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/blob/master/src/{path}"><div class="link-button mobile-hide">Github에서 보기</div></a>"##,
                github_url = github_url,
                path = path,
            ),
        ).replace(
            "<!-- :github-make-issue-mobile: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/issues/new?title=%5B{title}%5D+"><div class="link-button mobile-hide">토론 생성하기</div><div class="link-button mobile-show">새 토론</div></a>"##,
                github_url = github_url,
                title = title,
            ),
        ).replace(
            "<!-- :github-view-issue-mobile: -->",
            &*format!(
                r##"<a target="_blank" href="{github_url}/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+%5B{title}%5D"><div class="link-button mobile-hide">진행중인 토론</div><div class="link-button mobile-show">토론 보기</div></a>"##,
                github_url = github_url,
                title = title,
            ),
        ).replace(
            "<!-- :github-contributors: -->",
            &*cont_html,
        );

        html
    } else {
        html
    }
}
