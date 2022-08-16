use core::panic;

use crate::config::Config;
use crate::fs::{
    collapse_path, create_folder, delete_folder, join_paths, move_folder, path_exists,
    rename_folder, unix_path,
};
use crate::vault::Vault;
use walkdir::WalkDir;

// these fns assume that the paths generated are valid (folders haven't been tampered with externally)
// if file/folder not found error is thrown then jot fix (will be added later) can be used

pub fn create_dir(name: &str, current_vault: &mut Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let full_path = join_paths(vec![vault_location, vault_name, current_location, name]);

    create_folder(&full_path);

    println!("{} created at {}", name, unix_path(&full_path))
}

pub fn print_dir_tree(current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let full_path = join_paths(vec![vault_location, vault_name, current_location]);

    for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", unix_path(&entry.path().to_str().unwrap().to_string()));
    }
}

pub fn change_dir(location: &str, current_vault: &mut Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let vault_path = join_paths(vec![vault_location, vault_name]);
    let vault_path = unix_path(&vault_path);

    let full_path = join_paths(vec![&vault_path, current_location, location]);
    let full_path = collapse_path(&full_path);
    let full_path = unix_path(&full_path);

    if path_exists(&full_path) & full_path.contains(&vault_path) {
        let mut new_location = full_path.replace(&vault_path, "");
        if new_location.starts_with("/") {
            new_location = new_location[1..].to_string();
        }
        current_vault.update_current_location(&new_location);
        println!("changed dir");
    } else {
        panic!("invalid path")
    }
}

pub fn rename_dir(name: &str, new_name: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let path = join_paths(vec![vault_location, vault_name, current_location]);

    rename_folder(name, new_name, &path);

    println!("folder {} renamed to {}", name, new_name)
}

pub fn delete_dir(name: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let path = join_paths(vec![vault_location, vault_name, current_location, name]);

    delete_folder(&path);

    println!("folder {} deleted", name)
}

pub fn move_dir(name: &str, new_location: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let path = join_paths(vec![vault_location, vault_name, current_location]);
    let new_path = join_paths(vec![&path, new_location]);

    move_folder(name, &path, &new_path);

    println!("folder {} moved", name)
}

pub fn movev_dir(name: &str, vault: &str, config: &Config, current_vault: &Vault) {
    if vault != current_vault.get_name() {
        if let Some(new_vault_location) = config.get_vault_locaton(vault) {
            let (current_vault_name, current_vault_location, current_location) =
                current_vault.get_location_data();

            let path = join_paths(vec![
                current_vault_location,
                current_vault_name,
                current_location,
            ]);

            let new_path = join_paths(vec![new_vault_location, vault]);

            if path_exists(&join_paths(vec![&new_path, name])) {
                panic!("folder named {} already exists in {}", name, vault)
            }

            if path_exists(&join_paths(vec![&path, name])) {
                move_folder(name, &path, &new_path);
                println!("folder {} moved to {}", name, vault)
            } else {
                panic!("folder doesn't exist")
            }
        } else {
            panic!("vault doesn't exist")
        }
    } else {
        panic!("new vault can't be the same as old one")
    }
}
