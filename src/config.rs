//! Helper functions for reading the config file.

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, fs};

use serde::Deserialize;

use crate::category::Category;

/// Find and deserialize the config file.
pub fn read_config() -> Result<Config, ConfigReadError> {
    let config_file = fs::File::open(config_file_path())?;
    let config = serde_yaml::from_reader(config_file)?;
    Ok(config)
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

#[derive(Debug)]
pub enum ConfigReadError {
    IOError(std::io::Error),
    ParseError(serde_yaml::Error),
}

impl From<std::io::Error> for ConfigReadError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<serde_yaml::Error> for ConfigReadError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::ParseError(e)
    }
}

impl fmt::Display for ConfigReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use ConfigReadError::*;
        match self {
            IOError(e) => write!(f, "config error: {}", e),
            ParseError(e) => write!(f, "config error: {}", e),
        }
    }
}

impl std::error::Error for ConfigReadError {}
