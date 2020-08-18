use crate::config::config::Config;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Context<'a> {
    pub(crate) img_map: &'a HashMap<PathBuf, PathBuf>,
    pub(crate) handlebars: &'a Handlebars<'a>,
    pub(crate) config: &'a Config,
}
