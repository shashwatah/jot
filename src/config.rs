use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
enum FileFormat {
    Text,
    Markdown,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    format: FileFormat,
    vaults: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
            format: FileFormat::Markdown,
            vaults: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Self, confy::ConfyError> {
        let config: Config = confy::load("jot")?;
        Ok(config)
    }
}
