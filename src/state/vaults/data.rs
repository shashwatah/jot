use crate::traits::FileIO;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    current: Option<String>,
    vaults: HashMap<String, PathBuf>,
}

impl FileIO for Data {
    fn path(&self) -> PathBuf {
        if let Some(dirs) = ProjectDirs::from("com", "", "jot") {
            let mut path = dirs.data_dir().to_path_buf();
            path.push("vaults");
            path
        } else {
            panic!("current path couldn't be generated")
        }
    }
}

impl Data {
    pub fn get_current_vault(&self) -> Option<&String> {
        self.current.as_ref()
    }

    pub fn set_current_vault(&mut self, vault: Option<String>) {
        self.current = vault;
        self.store()
    }

    pub fn get_vaults(&self) -> &HashMap<String, PathBuf> {
        &self.vaults
    }

    pub fn get_vault_location(&self, name: &str) -> Option<&PathBuf> {
        self.vaults.get(name)
    }

    pub fn vault_exists(&self, name: &str) -> bool {
        self.vaults.contains_key(name)
    }

    pub fn add_vault(&mut self, name: String, location: PathBuf) {
        self.vaults.insert(name, location);
        self.store()
    }

    pub fn remove_vault(&mut self, name: &str) {
        self.vaults.remove(name);
        self.store()
    }

    pub fn rename_vault(&mut self, name: &str, new_name: String) {
        let value = self.vaults.remove(name);
        self.vaults.insert(new_name, value.unwrap());
        self.store()
    }

    pub fn set_vault_location(&mut self, name: &str, new_location: PathBuf) {
        self.vaults.insert(name.to_owned(), new_location);
        self.store()
    }
}
