use wiki_generator::config::config;
use wiki_generator::wiki::wiki::Wiki;

fn main() {
    let mut wiki = Wiki::new(config::get_config());
    wiki.render();
}
