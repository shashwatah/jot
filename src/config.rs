use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    editor_conflict: bool,
    current_vault: Option<String>,
    vaults: HashMap<String, PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
            editor_conflict: true,
            current_vault: None,
            vaults: HashMap::new(),
        }
    }
}

// will eventually use toml parsing with dirs crate
impl Config {
    pub fn load() -> Self {
        let config: Config = confy::load("jot").unwrap();
        config
    }

    pub fn get_editor(&self) -> &str {
        &self.editor
    }

    pub fn editor_conflict(&self) -> bool {
        self.editor_conflict
    }

    pub fn get_current_vault(&self) -> Option<&String> {
        match &self.current_vault {
            Some(current_vault_name) => Some(current_vault_name),
            None => None,
        }
    }

    pub fn get_vaults(&self) -> &HashMap<String, PathBuf> {
        &self.vaults
    }

    pub fn vault_exists(&self, name: &str) -> bool {
        self.vaults.contains_key(name)
    }

    pub fn get_vault_location(&self, name: &str) -> Option<&PathBuf> {
        self.vaults.get(name)
    }

    pub fn set_current_vault(&mut self, vault: Option<String>) {
        self.current_vault = vault;
        self.store()
    }

    pub fn add_vault(&mut self, name: String, location: PathBuf) {
        self.vaults.insert(name, location);
        self.store()
    }

    pub fn delete_vault(&mut self, name: &str) {
        self.vaults.remove(name);
        self.store()
    }

    pub fn rename_vault(&mut self, name: &str, new_name: String) {
        let value = self.vaults.remove(name);
        self.vaults.insert(new_name, value.unwrap());
        self.store()
    }

    pub fn set_vault_location(&mut self, name: &String, new_location: PathBuf) {
        self.vaults.insert(name.to_owned(), new_location);
        self.store()
    }

    fn store(&self) {
        confy::store("jot", self).unwrap();
    }
}
