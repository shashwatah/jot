use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum FileFormat {
    Text,
    Markdown,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    format: FileFormat,
    current_vault: Option<String>,
    vaults: HashMap<String, String>
    
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
            format: FileFormat::Markdown,
            vaults: HashMap::new(),
            current_vault: None 
        }
    }
}

impl Config {
    pub fn load_config() -> Self {
        let config: Config = confy::load("jot").unwrap();
        config
    }

    pub fn get_vaults(&self) -> &HashMap<String, String> {
        &self.vaults
    }

    pub fn get_current_vault(&self) -> &Option<String> {
        &self.current_vault
    }

    pub fn update_editor(&mut self, editor: String) {
        self.editor = editor;
        confy::store("jot", self).unwrap()
    }
    pub fn update_format(&mut self, format: FileFormat) {
        self.format = format;
        confy::store("jot", self).unwrap()
    }

    pub fn add_vault(&mut self, name: String, path: String) {
        self.vaults.entry(name).or_insert(path);
        confy::store("jot", self).unwrap()
    }
    
    pub fn del_vault(&mut self, name: &str) {
        self.vaults.remove(name);
        confy::store("jot", self).unwrap()
    }

    pub fn update_current_vault(&mut self, name: String) {
        if self.vaults.contains_key(&name) {
            self.current_vault = Some(name);
            confy::store("jot", self).unwrap()
        }
    }

    pub fn exit_current_vault(&mut self) {
        self.current_vault = None;
        confy::store("jot", self).unwrap()
    }
}
