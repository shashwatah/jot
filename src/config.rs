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

    pub fn update_current_vault(&mut self, vault: Option<&str>) {
        match vault {
            Some(name) => {
                if self.vaults.contains_key(name) {
                    self.current_vault = Some(name.to_string());
                    confy::store("jot", self).unwrap();
                    println!("switched to {}", name)
                } else {
                    panic!("Vault doesn't exist")
                }
            }
            None => {
                self.current_vault = None;
                confy::store("jot", self).unwrap()
            }
        }
    }

    pub fn add_vault(&mut self, name: String, path: String) {
        self.vaults.entry(name).or_insert(path);
        confy::store("jot", self).unwrap()
    }

    pub fn delete_vault(&mut self, name: &str) {
        if let Some(vault) = self.get_current_vault() {
            if name == vault {
                self.update_current_vault(None)
            }
        }

        if self.vaults.contains_key(name) {
            self.vaults.remove(name);
            confy::store("jot", self).unwrap();

            println!("{} deleted", name)
        } else {
            panic!("vault doesn't exist")
        }
    }
}
