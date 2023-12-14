pub(crate) mod connection;
pub(crate) mod web;

use crate::Result;

use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub web: Option<web::WebConfig>,
    pub connections: HashMap<String, connection::ConnectionConfig>,
}

impl Config {
    pub(crate) fn try_parse(file: &Path) -> Result<Self> {
        let contents = fs::read_to_string(file)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }
}