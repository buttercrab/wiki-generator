use std::fs;
use std::path::Path;

pub static INDEX_HBS: &[u8] = include_bytes!("index.hbs");
pub static REDIRECT_HBS: &[u8] = include_bytes!("redirect.hbs");

pub static STYLE_CSS: &[u8] = include_bytes!("css/style.css");
pub static VARIABLE_CSS: &[u8] = include_bytes!("css/variable.css");

pub static LINK_DARK_SVG: &[u8] = include_bytes!("img/link-dark.svg");
pub static LINK_LIGHT_SVG: &[u8] = include_bytes!("img/link-light.svg");

pub static WIKI_JS: &[u8] = include_bytes!("js/wiki.js");
pub static HIGHLIGHT_JS: &[u8] = include_bytes!("js/highlight.min.js");

pub fn init<P: AsRef<Path>>(out_dir: P) {
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

    fs::write(out_dir.join("r").join(style_css_path), STYLE_CSS).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(style_css_path)
    ));
    fs::write(out_dir.join("r").join(variable_css_path), VARIABLE_CSS).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(variable_css_path)
    ));
    fs::write(out_dir.join("r").join(link_dark_svg_path), LINK_DARK_SVG).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(link_dark_svg_path)
    ));
    fs::write(out_dir.join("r").join(link_light_svg_path), LINK_LIGHT_SVG).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(link_light_svg_path)
    ));
    fs::write(out_dir.join("r").join(wiki_js_path), WIKI_JS).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(wiki_js_path)
    ));
    fs::write(out_dir.join("r").join(highlight_js_path), HIGHLIGHT_JS).expect(&*format!(
        "failed to copy {:?}",
        out_dir.join("r").join(highlight_js_path)
    ));
}
