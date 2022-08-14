use crate::config::Config;
use crate::fs::{create_folder, create_path_with_fslash, join_paths};
use crate::vault::Vault;
use walkdir::WalkDir;

pub fn create_dir(name: &str, config: &Config, current_vault: &mut Vault) {
    let current_vault_name = config.get_current_vault().unwrap();
    let current_vault_path = config.get_vault_locaton(current_vault_name).unwrap();
    let current_location = current_vault.get_current_location();
    let full_path = join_paths(vec![
        current_vault_path,
        current_vault_name,
        current_location,
        name,
    ]);
    create_folder(&full_path);
    println!(
        "folder {} created at {}",
        name,
        create_path_with_fslash(&full_path)
    )
}

pub fn print_dir_tree(config: &Config, current_vault: &Vault) {
    let current_vault_name = config.get_current_vault().unwrap();
    let current_vault_location = config.get_vault_locaton(current_vault_name).unwrap();
    let current_location = current_vault.get_current_location();
    let full_path = join_paths(vec![
        current_vault_location,
        current_vault_name,
        current_location,
    ]);
    for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
        println!(
            "{}",
            create_path_with_fslash(&entry.path().to_str().unwrap().to_string())
        );
    }
}

// pub fn change_dir() {}
// pub fn rename_dir() {}
// pub fn delete_dir() {}
// pub fn move_dir() {}
// pub fn movev_dir() {}