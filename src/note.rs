use crate::{
    config::Config,
    fs::{delete_file, join_paths, move_item, process_path, rename_item},
    helpers::generate_location,
};
use core::panic;
use std::{path::PathBuf, process::Command};

use crate::{
    fs::{create_file, valid_name},
    vault::Vault,
};

pub fn create_note(name: &String, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let mut name_ext = PathBuf::from(name);
    name_ext.set_extension("md");

    let location = generate_location(current_vault);
    let path = join_paths(vec![&location, &name_ext]);

    if path.exists() {
        panic!("note named {} already exists", name);
    }

    create_file(&path);
    print!("note {} created", name);
}

// this needs to be improved
pub fn open_note(name: &String, config: &Config, current_vault: &Vault) {
    let location = generate_location(current_vault);

    let mut name = PathBuf::from(name);
    name.set_extension("md");

    let path = join_paths(vec![&location, &name]);

    if !path.exists() {
        panic!("note doesn't exist")
    }

    let app = config.get_editor();

    let mut cmd = Command::new(app)
        .arg(path.to_str().unwrap())
        .spawn()
        .unwrap();

    if config.editor_conflict() == true {
        cmd.wait().unwrap();
    }
}

pub fn rename_note(name: &String, new_name: &String, current_vault: &Vault) {
    if !valid_name(name) || !valid_name(new_name) {
        panic!("not a valid name")
    }

    if new_name == name {
        panic!("note is already named {}", name)
    }

    let location = generate_location(current_vault);

    let name_ext = name.to_owned() + ".md";
    let new_name_ext = new_name.to_owned() + ".md";

    if join_paths(vec![location.to_str().unwrap(), &new_name_ext]).exists() {
        panic!("note named {} already exists", new_name)
    }

    rename_item(&name_ext, &new_name_ext, &location);
    print!("note {} renamed to {}", name, new_name)
}

pub fn delete_note(name: &String, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let mut name_ext = PathBuf::from(name);
    name_ext.set_extension("md");

    let location = generate_location(current_vault);
    let path = join_paths(vec![&location, &name_ext]);

    delete_file(&path);
    print!("note {} deleted", name)
}

pub fn move_note(name: &String, new_location: &PathBuf, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let name_ext = name.to_owned() + ".md";

    let vault_path = join_paths(vec![
        current_vault.get_location().to_str().unwrap(),
        current_vault.get_name(),
    ]);
    let location = join_paths(vec![&vault_path, current_vault.get_folder()]);

    let new_location = join_paths(vec![&location, new_location]);
    let new_location = process_path(&new_location);

    if new_location == location {
        panic!("note {} already exists in this location", name)
    }

    if join_paths(vec![&new_location, &PathBuf::from(&name_ext)]).exists() {
        panic!("note named {} already exists in this location", name);
    }

    if !new_location
        .to_str()
        .unwrap()
        .contains(vault_path.to_str().unwrap())
    {
        panic!("location crosses the bounds of vault")
    }

    move_item(&name_ext, &location, &new_location);
    print!("note {} moved", name)
}

pub fn movev_note(name: &String, vault_name: &str, config: &Config, current_vault: &Vault) {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    if vault_name == current_vault.get_name() {
        panic!(
            "note {} already exists in vault {}, use mov instead",
            name, vault_name
        )
    }

    let mut name_ext = PathBuf::from(&name);
    name_ext.set_extension("md");

    if let Some(vault_location) = config.get_vault_location(vault_name) {
        let location = generate_location(current_vault);
        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);

        if join_paths(vec![&new_location, &name_ext]).exists() {
            panic!("note named {} already exists in vault", name)
        }

        move_item(name_ext.to_str().unwrap(), &location, &new_location);
        print!("note {} moved to vault {}", name, vault_name)
    } else {
        panic!("vault doesn't exist")
    }
}
