use crate::config::Config;
use crate::fs::{
    create_folder, delete_folder, join_paths, move_item, path_exists, rename_item,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<String>,
    current_location: String,
    last_note: Option<(String, String)>,
    history: Vec<(String, String)>,
}

// by default when a vault is created it doesn't store any data
// when user enters a vault a data file is generated within the vault
impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            current_location: "".to_string(),
            last_note: None,
            history: vec![],
        }
    }
}

// using confy right now but will eventually use standard toml parsing
impl Vault {
    pub fn load(location: &str, name: &str) -> Self {
        let data_path = join_paths(vec![location, name, ".jot/data"]);
        let mut vault: Vault = confy::load_path(data_path).unwrap();
        // if name or location = None = new data file -> set name and location
        if let None = vault.name {
            vault.name = Some(name.to_string());
            vault.location = Some(location.to_string());
            vault.update_vault_file()
        }
        vault
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref().unwrap()
    }

    pub fn get_location(&self) -> &str {
        self.location.as_ref().unwrap()
    }

    pub fn get_current_location(&self) -> &str {
        &self.current_location
    }

    pub fn get_location_data(&self) -> (&str, &str, &str) {
        (
            self.get_name(),
            self.get_location(),
            self.get_current_location(),
        )
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
        self.update_vault_file()
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = Some(location.to_string());
        self.update_vault_file()
    }

    pub fn update_current_location(&mut self, location: &str) {
        self.current_location = location.to_string();
        self.update_vault_file()
    }

    fn update_vault_file(&self) {
        let file_path = join_paths(vec![
            self.location.as_ref().unwrap(),
            self.name.as_ref().unwrap(),
            ".jot/data",
        ]);
        confy::store_path(file_path, self).unwrap()
    }
}

pub fn create_vault(name: &str, location: &str, config: &mut Config) {
    // check if vault name doesn't already exist
    if config.vault_exists(name) == false {
        // check if path is exists
        if path_exists(location) == true {
            // create path string with name of vault
            let path_with_name = join_paths(vec![location, name]);
            // create folder at path
            create_folder(&path_with_name);
            // create .jot inside the folder -> will do this later
            let jot_path = join_paths(vec![&path_with_name, ".jot"]);
            create_folder(&jot_path);
            // add vault to config
            config.add_vault(name.to_string(), location.to_string());
            // generate data file by calling load_data on vault
            Vault::load(location, name);
            println!("{} created", name)
        } else {
            panic!("path doesn't exist")
        }
    } else {
        panic!("vault already exists")
    }
}

pub fn enter_vault(name: &str, config: &mut Config) {
    if config.vault_exists(name) == true {
        if let Some(current_vault) = config.get_current_vault() {
            if name == current_vault {
                println!("already in {}", name);
                return;
            }
        }
        config.update_current_vault(Some(name.to_string()));
        println!("switched to {}", name)
    } else {
        panic!("vault doesn't exist")
    }
}

pub fn rename_vault(name: &str, new_name: &str, config: &mut Config) {
    // replacing vault_exists call with get_vault_location (in rename, delete and move)
    // since it returns an option i.e it does the same thing while also returning vault location
    if let Some(vault_location) = config.get_vault_location(name) {
        if name != new_name {
            if config.vault_exists(new_name) != true {
                rename_item(name, new_name, vault_location);
                Vault::load(vault_location, new_name).set_name(new_name);
                config.rename_vault(name, new_name.to_string());
                // check if its the current vault, update if it is
                if let Some(vault) = config.get_current_vault() {
                    if name == vault {
                        config.update_current_vault(Some(new_name.to_string()));
                    }
                }
                println!("vault {} renamed to {}", name, new_name)
            } else {
                panic!("a vault with name same as new name already exists")
            }
        } else {
            panic!("new name can't be same as old name")
        }
    } else {
        panic!("vault doesn't exist")
    }
}

pub fn delete_vault(name: &str, config: &mut Config) {
    if let Some(vault_location) = config.get_vault_location(name) {
        let final_path = join_paths(vec![vault_location, name]);
        delete_folder(&final_path);
        config.delete_vault(name);
        if let Some(vault) = config.get_current_vault() {
            if name == vault {
                config.update_current_vault(None)
            }
        }
        println!("{} deleted", name)
    } else {
        panic!("vault doesn't exist")
    }
}

pub fn move_vault(name: &str, new_location: &str, config: &mut Config) {
    if let Some(original_location) = config.get_vault_location(name) {
        if path_exists(new_location) == true {
            if new_location != original_location {
                move_item(name, original_location, new_location);
                Vault::load(new_location, name).set_location(new_location);
                config.update_vault_location(name.to_string(), new_location.to_string());
                println!("vault {} moved", name);
            } else {
                panic!("new location can't be same as original location")
            }
        } else {
            panic!("path doesn't exist");
        }
    } else {
        panic!("vault doesn't exist");
    }
}
