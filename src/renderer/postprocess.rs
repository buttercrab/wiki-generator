use crate::util::{path, string};
use chrono::{Datelike, Timelike};
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

pub fn fix_header<S: AsRef<str>>(html: S) -> String {
    let mut id_counter = HashMap::new();
    let mut counter = vec![0usize];
    let mut toc = String::new();
    let mut toc_level = 1usize;

    let s = Regex::new(r"<h(\d)>(.*?)</h\d>")
        .unwrap()
        .replace_all(html.as_ref(), |caps: &Captures<'_>| {
            let level = caps[1].parse().unwrap();
            let content = &caps[2];

            let raw_id = string::make_id(content);

            let id_count = id_counter.entry(raw_id.clone()).or_insert(0);

            let id = match *id_count {
                0 => raw_id,
                other => format!("{}-{}", raw_id, other),
            };

            *id_count += 1;

            while counter.len() < level - 1 {
                counter.push(0);
            }

            while counter.len() > level - 1 {
                counter.pop();
            }

            if counter.len() > 0 {
                *counter.last_mut().unwrap() += 1;
            }

            let mut number = String::new();

            for x in counter.iter() {
                number.push_str(&format!("{}.", x));
            }

            let typing = string::typing_effect(string::typing_process(content));

            if level == 1 {
                format!(
                    r##"
<script>
function sleep(ms) {{
  return new Promise(resolve => setTimeout(resolve, ms));
}}

async function writeTitle() {{
    let s = {:?};
    let title = document.getElementsByClassName('title')[0];
    for(let i in s) {{
        title.innerHTML = s[i];
        await sleep(100);
    }}
}}

addOnload(writeTitle);
</script>
<!-- :title={title}: -->
<h{level} class="header title">_</h{level}>"##,
                    typing,
                    level = level,
                    title = content,
                )
            } else {
                if toc_level < level {
                    while toc_level < level {
                        toc.push_str(r"<ul><li>");
                        toc_level += 1;
                    }
                } else {
                    while toc_level > level {
                        toc.push_str(r"</li></ul>");
                        toc_level -= 1;
                    }
                    toc.push_str(r"</li><li>");
                }

                toc.push_str(&*format!(
                    r##"<a href="#{id}">{number} {text}</a>"##,
                    id = id,
                    number = number,
                    text = content
                ));

                format!(
                    r##"<h{level}><a class="header" href="#{id}" id="{id}">{number} {text}</a></h{level}>"##,
                    level = level,
                    id = id,
                    number = number,
                    text = content
                )
            }
        })
        .into_owned();

    while toc_level > 1 {
        toc.push_str(r"</li></ul>");
        toc_level -= 1;
    }

    s.replace(r"<!-- :toc: -->", &*toc)
}

pub fn add_github_things<P: AsRef<Path>, S: AsRef<str>>(
    html: S,
    github_url: &Option<String>,
    path: P,
) -> String {
    let html = html.as_ref();
    let path = path.as_ref();

    if let Some(github_url) = github_url {
        let path = path::os_to_str(path);

        let regex = Regex::new(r##"<!-- :title=(.*?): -->"##).unwrap();
        let title = &regex.captures_iter(html.as_ref()).collect::<Vec<_>>()[0][1];

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

        let mut cont_html =
            String::from(r##"<div class="description"><span>기여자:&nbsp;</span></div>"##);
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
        html.to_string()
    }
}
