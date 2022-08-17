use crate::config::Config;
use crate::fs::{
    collapse_path, create_file, delete_file, join_paths, move_item, path_exists, rename_item,
    unix_path,
};
use crate::vault::Vault;
use std::process::Command;

fn create_note_path(location_data: (&str, &str, &str), name: &str) -> String {
    let (vault_name, vault_location, current_location) = location_data;
    join_paths(vec![vault_location, vault_name, current_location, name])
}

pub fn create_note(name: &str, current_vault: &Vault) {
    let name = name.to_string() + ".md";
    let full_path = create_note_path(current_vault.get_location_data(), &name);

    if path_exists(&full_path) == false {
        create_file(&full_path);
        print!("note {} created", name)
    } else {
        panic!("note named {} aready exists", name)
    }
}

pub fn open_note(name: &str, config: &Config, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();
    let name = name.to_string() + ".md";
    
    let path = join_paths(vec![vault_location, vault_name, current_location, &name]);
    
    if path_exists(&path) == false {
        panic!("note doesn't exist")
    } 

    let app = config.get_editor();

    let mut cmd = Command::new(app).arg(path).spawn().unwrap();

    if config.editor_conflict() == true {
        cmd.wait().unwrap();     
    }
}

pub fn rename_note(name: &str, new_name: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let name = name.to_string() + ".md";
    let new_name = new_name.to_string() + ".md";

    if name == new_name {
        panic!("new name can't be same as old name")
    }

    let path = join_paths(vec![vault_location, vault_name, current_location]);

    if path_exists(&join_paths(vec![&path, &new_name])) == true {
        panic!("note named {} already exists", new_name)
    }

    rename_item(&name, &new_name, &path);

    println!("note {} renamed to {}", name, new_name)
}

pub fn move_note(name: &str, new_location: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let name = name.to_string() + ".md";

    let path = join_paths(vec![vault_location, vault_name, current_location]);
    let path = unix_path(&path);

    let new_path = join_paths(vec![&path, new_location]);
    let new_path = collapse_path(&new_path);
    let new_path = unix_path(&new_path);

    if path_exists(&join_paths(vec![&new_path, &name])) {
        panic!("note named {} already exists at new path", name)
    }

    if new_path.contains(&path) == false {
        panic!("invalid path")
    }

    move_item(&name, &path, &new_path);
    print!("note {} moved", name)
}

pub fn movev_note(name: &str, new_vault: &str, config: &Config, current_vault: &Vault) {
    let name = name.to_string() + ".md";
    
    if let Some(new_vault_location) = config.get_vault_location(new_vault) {
        let (current_vault_name, current_vault_location, current_location) =
            current_vault.get_location_data();

        if current_vault_name == new_vault {
            panic!("new vault can't be the same as old vault");
        }

        let new_path = join_paths(vec![new_vault_location, new_vault]);

        // this will be bypassed if the user enters a path instead of name for a note
        if path_exists(&join_paths(vec![&new_path, &name])) == true {
            panic!("note named {} already exists in vault {}", name, new_vault)
        }

        let original_path = join_paths(vec![
            current_vault_location,
            current_vault_name,
            current_location,
        ]);

        move_item(&name, &original_path, &new_path);
        print!("note {} moved to {}", name, new_vault)
    } else {
        panic!("vault {} doesn't exist", new_vault)
    }
}

pub fn delete_note(name: &str, current_vault: &Vault) {
    let name = name.to_string() + ".md";

    let full_path = create_note_path(current_vault.get_location_data(), &name);

    delete_file(&full_path);

    print!("note {} deleted", name)
}
