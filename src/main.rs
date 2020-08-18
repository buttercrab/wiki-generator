use wiki_generator::config::config::get_config;
use wiki_generator::renderer::render::render;
use wiki_generator::themes::init;

fn main() {
    init();
    render(&get_config());
}
