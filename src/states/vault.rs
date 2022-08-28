use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::fs::join_paths;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<PathBuf>,
    folder: PathBuf,
    history: Vec<(String, PathBuf)>,
}

impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            folder: PathBuf::new(),
            history: vec![],
        }
    }
}

// using confy right now but will eventually use standard toml parsing
impl Vault {
    pub fn load(name: &String, location: &PathBuf) -> Self {
        let data_path = join_paths(vec![location.to_str().unwrap(), name, ".jot/data"]);

        let mut vault: Vault = confy::load_path(data_path).unwrap();

        // if name or location = None = new data file -> set name and location
        if vault.name.is_none() {
            vault.name = Some(name.to_owned());
            vault.location = Some(location.to_owned());
            vault.store()
        }

        vault
    }

    pub fn get_name(&self) -> &String {
        self.name.as_ref().unwrap()
    }

    pub fn get_location(&self) -> &PathBuf {
        self.location.as_ref().unwrap()
    }

    pub fn get_folder(&self) -> &PathBuf {
        &self.folder
    }

    pub fn get_path_data(&self) -> (&String, &PathBuf, &PathBuf) {
        (self.get_name(), self.get_location(), self.get_folder())
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
        self.store()
    }

    pub fn set_location(&mut self, location: PathBuf) {
        self.location = Some(location);
        self.store()
    }

    pub fn set_folder(&mut self, folder: PathBuf) {
        self.folder = folder;
        self.store()
    }

    fn store(&self) {
        let data_path = join_paths(vec![
            self.location.as_ref().unwrap().to_str().unwrap(),
            self.name.as_ref().unwrap(),
            ".jot/data",
        ]);
        confy::store_path(data_path, self).unwrap()
    }
}
