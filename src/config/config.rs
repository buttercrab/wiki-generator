use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) wiki: Wiki,
    pub(crate) html: Option<Html>,
}

#[derive(Deserialize)]
pub struct Wiki {
    pub(crate) title: String,
    pub(crate) author: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) src: Option<String>,
    pub(crate) out: Option<String>,
    pub(crate) preserve: Option<Vec<String>>,
    pub(crate) main: Option<String>,
}

#[derive(Deserialize)]
pub struct Html {
    pub(crate) github: Option<String>,
    pub(crate) ga: Option<String>,
    pub(crate) logo: Option<String>,
}

pub fn get_config() -> Config {
    toml::from_str(&*fs::read_to_string("wiki.toml").expect("failed to locate wiki.toml"))
        .expect("toml parsing failed")
}
