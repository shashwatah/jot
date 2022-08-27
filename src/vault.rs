use crate::config::Config;
use crate::fs::{
    create_folder, delete_folder, join_paths, move_item, process_path, rename_item, valid_name,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

pub fn create_vault(name: &String, location: &PathBuf, config: &mut Config) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    if config.vault_exists(name) {
        panic!("vault named {} already exists", name)
    }

    let path = join_paths(vec![location.to_str().unwrap(), name]);
    create_folder(&path);

    let location = process_path(location);

    Vault::load(name, &location);
    config.add_vault(name.to_owned(), location);

    print!("vault {} created", name)
}

pub fn enter_vault(name: &String, config: &mut Config) {
    if !config.vault_exists(name) {
        panic!("vault named {} doesn't exist", name)
    }

    if let Some(current_vault) = config.get_current_vault() {
        if name == current_vault {
            return print!("already in {}", name);
        }
    }

    config.set_current_vault(Some(name.to_owned()));
    print!("switched to {}", name)
}

pub fn rename_vault(name: &String, new_name: &String, config: &mut Config) {
    if !valid_name(new_name) {
        panic!("not a valid name")
    }

    if new_name == name {
        panic!("vault is already named {}", name)
    }

    if config.vault_exists(new_name) {
        panic!("vaule named {} already exists", name)
    }

    if let Some(location) = config.get_vault_location(name) {
        rename_item(name, new_name, location);
        Vault::load(new_name, location).set_name(new_name.to_owned());
        config.rename_vault(name, new_name.to_owned());

        if let Some(current_vault) = config.get_current_vault() {
            if name == current_vault {
                config.set_current_vault(Some(new_name.to_owned()));
            }
        }

        print!("vault {} renamed to {}", name, new_name)
    } else {
        panic!("vault {} doesn't exist", name)
    }
}

pub fn delete_vault(name: &String, config: &mut Config) {
    if let Some(location) = config.get_vault_location(name) {
        let path = join_paths(vec![location.to_str().unwrap(), name]);
        delete_folder(&path);
        config.delete_vault(name);

        if let Some(current_vault) = config.get_current_vault() {
            if name == current_vault {
                config.set_current_vault(None)
            }
        }

        print!("vault {} deleted", name)
    } else {
        panic!("vault {} doesn't exist", name)
    }
}

pub fn move_vault(name: &String, new_location: &PathBuf, config: &mut Config) {
    if let Some(original_location) = config.get_vault_location(name) {
        if new_location == original_location {
            panic!("vault {} already exists in this location", name)
        }

        if join_paths(vec![new_location.to_str().unwrap(), name]).exists() {
            panic!("a folder named {} already exists in this location", name)
        }

        move_item(name, original_location, new_location);

        let new_location = process_path(new_location);

        Vault::load(name, &new_location).set_location(new_location.clone());
        config.set_vault_location(name, new_location);

        print!("vault {} moved", name);
    } else {
        panic!("vault {} doesn't exist", name);
    }
}
