//! Helper functions for reading the config file.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::TResult;

/// Find and deserialize the config file.
pub fn read_config() -> TResult<Config> {
    let config_path = config_file_path();
    let yaml = fs::read_to_string(config_path)?;
    Ok(serde_yaml::from_str(&yaml)?)
}

/// Find the path to the config file.
fn config_file_path() -> PathBuf {
    // TODO: Add alternative paths in case `~/.config/o/config.yml` is not
    // available.

    let mut path =
        dirs::config_dir().expect("could not find config directory!");
    path.push("o");
    path.push("config.yml");
    path
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub categories: HashMap<String, Category>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub dirs:      Vec<String>,
    pub filetypes: Vec<String>,
    pub command:   Option<String>,
    pub wait:      bool,
}
