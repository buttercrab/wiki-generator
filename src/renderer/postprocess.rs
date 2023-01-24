use crate::renderer::markdown;
use crate::util::path;
use crate::util::string;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use url::Url;

pub fn fix_header<S: AsRef<str>>(html: S) -> String {
    let mut counter = vec![0usize];
    let mut toc = String::new();
    let mut toc_level = 1usize;

    let s = Regex::new(r"<h(\d)>(.*?)</h\d>")
        .unwrap()
        .replace_all(html.as_ref(), |caps: &Captures<'_>| {
            let level = caps[1].parse().unwrap();
            let content = &caps[2];

            while counter.len() < level - 1 {
                counter.push(0);
            }

            while counter.len() > level - 1 {
                counter.pop();
            }

            if !counter.is_empty() {
                *counter.last_mut().unwrap() += 1;
            }

            let mut number = String::new();
            let mut id = String::from("s-");

            for x in counter.iter() {
                number.push_str(&format!("{x}."));
                id.push_str(&format!("{x}-"));
            }

            let typing = string::typing_effect(string::typing_process(string::unescape_html(content)))
                .iter()
                .map( string::escape_html)
                .collect::<Vec<_>>();

            if level == 1 {
                format!(
                    r##"
<script>
function sleep(ms) {{
  return new Promise(resolve => setTimeout(resolve, ms));
}}

async function writeTitle() {{
    let s = {typing:?};
    let title = document.getElementsByClassName('title')[0];
    for (let i in s) {{
        title.innerHTML = s[i];
        await sleep(100);
    }}
}}

addOnload(writeTitle);
</script>
<h{level} class="heading title">_</h{level}>"##
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

                let regex = Regex::new(r#"<a.+>(.+?)</a>"#).unwrap();
                let text = regex.replace_all(content, |cap: &Captures<'_>| cap[1].to_string()).into_owned();

                toc.push_str(&format!(
                    r##"<a href="#{id}">{number} {text}</a>"##,
                ));

                format!(
                    r##"<h{level} class="heading"><a href="#{id}" id="{id}">{number}</a> {content}</h{level}>"##
                )
            }
        })
        .into_owned();

    while toc_level > 1 {
        toc.push_str(r"</li></ul>");
        toc_level -= 1;
    }

    s.replace(r"<!-- :toc: -->", &toc)
}

pub fn fix_link<S: AsRef<str>, P: AsRef<Path>>(
    html: S,
    path: P,
    file_map: &HashMap<String, String>,
    titles: &HashSet<String>,
) -> String {
    let html = html.as_ref();
    let path = path.as_ref();

    let html = Regex::new(r#"(<img [^>]*?src=")([^"]+?)""#)
        .unwrap()
        .replace_all(html, |caps: &Captures<'_>| {
            let img_link = &caps[2];

            if Regex::new(r"^[a-z][a-z0-9+.-]*:")
                .unwrap()
                .is_match(img_link)
            {
                panic!(
                    "Image link {} from {:?} that is using outer link",
                    img_link, path
                );
            }

            if img_link.starts_with('/') {
                panic!(
                    "Image link {} from {:?} that is using absolute path",
                    img_link, path
                );
            }

            let img_path = path::path_to_str(path::simplify(path.parent().unwrap().join(img_link)));
            let to = file_map
                .get(&*img_path)
                .unwrap_or_else(|| panic!("No image {:?} from {:?}", img_link, path));

            format!("{}/{}\"", &caps[1], to)
        })
        .to_string();

    let html = Regex::new(r#"<a (.*?)>"#)
        .unwrap()
        .replace_all(&html, |caps: &Captures<'_>| {
            let attrs = &caps[1];
            let href = &Regex::new(r#"href="(.*?)""#)
                .unwrap()
                .captures(attrs)
                .unwrap()[1];

            if Regex::new(r"^[a-z][a-z0-9+.-]*:").unwrap().is_match(href) {
                if Regex::new(r#"class="(.*?)""#).unwrap().is_match(attrs) {
                    let attrs = Regex::new(r#"class="(.*?)""#)
                        .unwrap()
                        .replace_all(attrs, |caps: &Captures<'_>| {
                            let mut class = caps[1].to_string();
                            class.push_str(" outer");
                            format!(r#"class="{class}""#)
                        })
                        .to_string();

                    format!(r#"<a {attrs}>"#)
                } else {
                    format!(r#"<a {attrs} class="outer" target="_blank">"#)
                }
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    let rep = |html: String, s: usize, e: usize| -> String {
        if s != e {
            let tmp = Regex::new(r"\[\[ +(.+?) +]]")
                .unwrap()
                .replace_all(&html[s..e], |caps: &Captures<'_>| {
                    let title = caps[1].to_string();

                    let mut s = String::new();
                    let mut t = String::new();
                    let mut esc = false;
                    for i in title.chars() {
                        match i {
                            '\\' => {
                                if esc {
                                    s.push('\\');
                                    esc = false;
                                } else {
                                    esc = true;
                                }
                            }
                            '[' => {
                                if esc {
                                    s.push('[');
                                    esc = false;
                                }
                            }
                            ']' => {
                                if esc {
                                    s.push(']');
                                    esc = false;
                                }
                            }
                            '|' => {
                                if esc {
                                    s.push('|');
                                    esc = false;
                                } else {
                                    t = s;
                                    s = String::new();
                                }
                            }
                            c => {
                                s.push(c);
                                esc = false;
                            }
                        }
                    }
                    let title = s;

                    let url: String = Url::parse(&format!("https://example.com/w/{title}"))
                        .unwrap()
                        .into();
                    let href = url.trim_start_matches("https://example.com");
                    let l = title.find('#');
                    let title_without_loc = if let Some(l) = l {
                        title[..l].to_string()
                    } else {
                        title.clone()
                    };
                    if t.is_empty() {
                        t = title;
                    }
                    let title_without_loc = string::unescape_html(title_without_loc);

                    if titles.contains(&title_without_loc) {
                        format!(r##"<a href="{href}">{t}</a>"##)
                    } else {
                        format!(r##"<a class="no-link" href="{href}">{t}</a>"##)
                    }
                })
                .to_string();

            format!("{}{}{}", &html[..s], tmp, &html[e..])
        } else {
            html
        }
    };

    let mut idx = 0usize;
    let mut range = Vec::new();

    for m in Regex::new(r#"(?:<h1>[\s\S]*?</h1>|<code>[\s\S]*?</code>)"#)
        .unwrap()
        .find_iter(&html)
    {
        range.push((idx, m.start()));
        idx = m.end();
    }

    range.push((idx, html.len()));
    let mut html = html;

    for (s, e) in range.iter().rev() {
        html = rep(html, *s, *e);
    }

    html
}

pub fn add_github_info<P: AsRef<Path>, S: AsRef<str>, T: AsRef<str>>(
    data: &mut serde_json::Map<String, serde_json::Value>,
    from: P,
    title: S,
    github_url: T,
    contrib_html: String,
) {
    let from = from.as_ref();
    let title = title.as_ref();
    let github_url = github_url.as_ref();

    data.insert(
        "time".to_string(),
        json!(format!("최근 수정 시각: {}", markdown::get_time(from))),
    );
    data.insert("github_contributors".to_string(), json!(contrib_html));
    data.insert(
        "github_history".to_string(),
        json!(markdown::get_github_history(from, github_url)),
    );
    data.insert(
        "github_edit".to_string(),
        json!(markdown::get_github_edit(from, github_url)),
    );
    data.insert(
        "github_view_issue".to_string(),
        json!(markdown::get_github_view_issue(github_url, title)),
    );
    data.insert(
        "github_make_issue".to_string(),
        json!(markdown::get_github_make_issue(github_url, title)),
    );
    data.insert(
        "view_in_github".to_string(),
        json!(markdown::get_view_in_github(from, github_url)),
    );
    data.insert(
        "view_in_github_mobile".to_string(),
        json!(markdown::get_view_in_github_mobile(from, github_url)),
    );
    data.insert(
        "github_make_issue_mobile".to_string(),
        json!(markdown::get_github_make_issue_mobile(github_url, title)),
    );
    data.insert(
        "github_view_issue_mobile".to_string(),
        json!(markdown::get_github_view_issue_mobile(github_url, title)),
    );
}

pub fn fix_footnotes<S: AsRef<str>>(html: S) -> String {
    let html = html.as_ref();

    let html = Regex::new(
        r##"(<div class="footnote-definition" id=")(.*?)("><sup class="footnote-definition-label">)(.*?)</sup>([\s\S]*?)</div>"##,
    )
        .unwrap()
        .replace_all(html, |caps: &Captures<'_>| {
            format!(r##"{}f-{id}{}<a href="#b-{id}">{}</a></sup>{}</div>"##, &caps[1], &caps[3], &caps[4], &caps[5], id = &caps[2])
        })
        .to_string();

    let html =
        Regex::new(r##"(<sup class="footnote-reference"><a) href="#(.*?)">([\s\S]*?)</a></sup>"##)
            .unwrap()
            .replace_all(&html, |caps: &Captures<'_>| {
                format!(
                    r##"{} id="b-{id}">{}</a></sup>"##,
                    &caps[1],
                    &caps[3],
                    id = &caps[2]
                )
            })
            .to_string();

    html
}
