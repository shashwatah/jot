use crate::fs::{collapse_path, create_folder, join_paths, path_exists, unix_path, rename_folder};
use crate::vault::Vault;
use walkdir::WalkDir;

pub fn create_dir(name: &str, current_vault: &mut Vault) {
    let vault_name = current_vault.get_name();
    let vault_location = current_vault.get_location();
    let current_location = current_vault.get_current_location();

    let full_path = join_paths(vec![vault_location, vault_name, current_location, name]);

    create_folder(&full_path);

    println!("{} created at {}", name, unix_path(&full_path))
}

pub fn print_dir_tree(current_vault: &Vault) {
    let vault_name = current_vault.get_name();
    let vault_location = current_vault.get_location();
    let current_location = current_vault.get_current_location();

    let full_path = join_paths(vec![vault_location, vault_name, current_location]);

    for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", unix_path(&entry.path().to_str().unwrap().to_string()));
    }
}

pub fn change_dir(location: &str, current_vault: &mut Vault) {
    let vault_name = current_vault.get_name();
    let vault_location = current_vault.get_location();
    let current_locaton = current_vault.get_current_location();

    let vault_path = join_paths(vec![vault_location, vault_name]);
    let vault_path = unix_path(&vault_path);

    let full_path = join_paths(vec![&vault_path, current_locaton, location]);
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
    let vault_name = current_vault.get_name();
    let vault_location = current_vault.get_location();
    let current_location = current_vault.get_current_location();

    let path = join_paths(vec![vault_location, vault_name, current_location]);
    rename_folder(name, new_name, &path);
    println!("folder {} renamed to {}", name, new_name)
}


// pub fn delete_dir() {}
// pub fn move_dir() {}
// pub fn movev_dir() {}
