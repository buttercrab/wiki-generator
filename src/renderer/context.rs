use crate::config::config::Config;
use handlebars::Handlebars;
use std::collections::HashMap;

pub struct Context<'a> {
    pub(crate) img_map: &'a HashMap<String, String>,
    pub(crate) handlebars: &'a Handlebars<'a>,
    pub(crate) config: &'a Config,
}
