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
    vaults: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
            format: FileFormat::Markdown,
            vaults: HashMap::new(),
            current_vault: None,
        }
    }
}

impl Config {
    pub fn load_config() -> Self {
        let config: Config = confy::load("jot").unwrap();
        config
    }

    pub fn get_current_vault(&self) -> &Option<String> {
        &self.current_vault
    }

    pub fn get_vaults(&self) -> &HashMap<String, String> {
        &self.vaults
    }

    pub fn get_vault_path(&self, name: &str) -> Option<&String> {
        self.vaults.get(name)
    }

    pub fn check_vault(&self, name: &str) -> bool {
        self.vaults.contains_key(name)
    }

    pub fn update_current_vault(&mut self, vault: Option<String>) {
        self.current_vault = vault;
        self.update_config_file();
    }

    pub fn add_vault(&mut self, name: String, path: String) {
        self.vaults.insert(name, path);
        self.update_config_file();
    }

    pub fn delete_vault(&mut self, name: &str) {
        self.vaults.remove(name);
        self.update_config_file();
    }

    pub fn rename_vault(&mut self, name: &str, new_name: &str) {
        let value = self.vaults.remove(name);
        self.vaults.insert(new_name.to_string(), value.unwrap());
        self.update_config_file();
    }

    pub fn update_vault_path(&mut self, name: &str, new_path: &str) {
        self.vaults.insert(name.to_string(), new_path.to_string());
        self.update_config_file();
    }

    fn update_config_file(&self) {
        confy::store("jot", self).unwrap();
    }
}
