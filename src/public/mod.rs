use std::fs;
use std::path::Path;

pub static INDEX_HBS: &str = include_str!("index.hbs");
pub static REDIRECT_HBS: &str = include_str!("redirect.hbs");

pub static STYLE_CSS: &str = include_str!("css/style.css");
pub static VARIABLE_CSS: &str = include_str!("css/variable.css");

pub static LINK_DARK_SVG: &str = include_str!("img/link-dark.svg");
pub static LINK_LIGHT_SVG: &str = include_str!("img/link-light.svg");

pub static WIKI_JS: &str = include_str!("js/wiki.js");
pub static HIGHLIGHT_JS: &str = include_str!("js/highlight.min.js");

pub async fn get_min(ext: &str, content: &str) -> String {
    let url = match ext {
        "css" => "https://cssminifier.com/raw",
        "js" => "https://javascript-minifier.com/raw",
        "html" => "https://html-minifier.com/raw",
        _ => {
            return String::from(content);
        }
    };

    let mut body = Vec::from("input=");
    body.append(&mut Vec::from(urlencoding::encode(content).as_bytes()));

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("charset", "utf-8")
        .body(body)
        .send()
        .await
        .unwrap();
    String::from_utf8(res.bytes().await.unwrap().to_vec()).unwrap()
}

pub async fn init<P: AsRef<Path>>(out_dir: P) {
    let out_dir = out_dir.as_ref();
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).expect(&*format!("failed to remove {:?}", out_dir));
    }
    fs::create_dir(&out_dir).expect("failed to make public/");

    let style_css_path = Path::new("css/style.css");
    let variable_css_path = Path::new("css/variable.css");
    let link_dark_svg_path = Path::new("img/link-dark.svg");
    let link_light_svg_path = Path::new("img/link-light.svg");
    let wiki_js_path = Path::new("js/wiki.js");
    let highlight_js_path = Path::new("js/highlight.min.js");

    fs::create_dir_all(out_dir.join("r/css"))
        .expect(&*format!("failed to make {:?}", out_dir.join("r/css")));
    fs::create_dir_all(out_dir.join("r/img"))
        .expect(&*format!("failed to make {:?}", out_dir.join("r/css")));
    fs::create_dir_all(out_dir.join("r/js"))
        .expect(&*format!("failed to make {:?}", out_dir.join("r/css")));

    futures::future::join_all(
        vec![
            (style_css_path, STYLE_CSS),
            (variable_css_path, VARIABLE_CSS),
            (link_dark_svg_path, LINK_DARK_SVG),
            (link_light_svg_path, LINK_LIGHT_SVG),
            (wiki_js_path, WIKI_JS),
            (highlight_js_path, HIGHLIGHT_JS),
        ]
        .iter()
        .map(|(path, content)| async move {
            let content = get_min(path.extension().unwrap().to_str().unwrap(), content).await;
            fs::write(out_dir.join("r").join(path), content).expect(&*format!(
                "failed to copy {:?}",
                out_dir.join("r").join(path)
            ))
        }),
    )
    .await;
}
