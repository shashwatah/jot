use crate::states::config::Config;
use crate::states::vault::Vault;

use crate::fs::{
    create_folder, delete_folder, join_paths, move_item, process_path, rename_item, valid_name,
};
use std::path::PathBuf;

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
