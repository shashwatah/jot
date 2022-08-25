use crate::config::Config;
use crate::fs::{
    create_folder, delete_folder, join_paths, move_item, process_path, rename_item, valid_name,
};
use crate::helpers::generate_location;
use crate::vault::Vault;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn create_dir(name: &String, current_vault: &mut Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let location = generate_location(current_vault);
    let path = join_paths(vec![location, PathBuf::from(name)]);

    create_folder(&path);
    print!("folder {} created", name);
}

pub fn dir_tree(current_vault: &Vault) {
    let location = generate_location(current_vault);

    for entry in WalkDir::new(location).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}

pub fn change_dir(path: &PathBuf, current_vault: &mut Vault) {
    let (vault_name, vault_location, folder) = current_vault.get_path_data();

    let vault_path = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);

    let full_path = join_paths(vec![&vault_path, folder, path]);
    let full_path = process_path(&full_path);

    if !full_path.exists() {
        panic!("path doesn't exist")
    }

    let vault_path = vault_path.to_str().unwrap();
    let full_path = full_path.to_str().unwrap();

    if !full_path.contains(vault_path) {
        panic!("path crosses the bounds of vault")
    }

    let mut dest_folder = full_path.replace(vault_path, "");
    if dest_folder.starts_with('\\') || dest_folder.starts_with('/') {
        dest_folder = dest_folder[1..].to_string();
    }
    let dest_folder = PathBuf::from(dest_folder);

    current_vault.set_folder(dest_folder);
    print!("changed folder");
}

pub fn rename_dir(name: &String, new_name: &String, current_vault: &Vault) {
    if !valid_name(new_name) {
        panic!("not a valid name")
    }

    if new_name == name {
        panic!("folder is already named {}", name)
    }

    let location = generate_location(current_vault);

    rename_item(name, new_name, &location);
    print!("folder {} renamed to {}", name, new_name)
}

pub fn delete_dir(name: &String, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let location = generate_location(current_vault);
    let path = join_paths(vec![location, PathBuf::from(name)]);

    delete_folder(&path);
    print!("folder {} deleted", name);
}

pub fn move_dir(name: &String, new_location: &PathBuf, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let location = generate_location(current_vault);

    let new_location = join_paths(vec![&location, new_location]);
    let new_location = process_path(&new_location);

    if new_location == location {
        panic!("folder {} already exists in this location", name)
    }

    if join_paths(vec![&new_location, &PathBuf::from(name)]).exists() {
        panic!("folder named {} already exists in this location", name);
    }

    if !new_location
        .to_str()
        .unwrap()
        .contains(location.to_str().unwrap())
    {
        panic!("location crosses the bounds of vault")
    }

    move_item(name, &location, &new_location);
    print!("folder {} moved", name)
}

pub fn movev_dir(name: &String, vault_name: &String, config: &Config, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    if vault_name == current_vault.get_name() {
        panic!(
            "folder {} already exists in vault {}, use mov instead",
            name, vault_name
        )
    }

    if let Some(vault_location) = config.get_vault_location(vault_name) {
        let location = generate_location(current_vault);
        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);

        if join_paths(vec![&new_location, &PathBuf::from(name)]).exists() {
            panic!("folder named {} already exists in vault", name)
        }

        move_item(name, &location, &new_location);
        print!("folder {} moved to vault {}", name, vault_name)
    } else {
        panic!("vault doesn't exist")
    }
}
