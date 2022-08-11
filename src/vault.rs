use crate::config::Config;
use crate::fs::{
    create_folder, create_path_with_name, delete_folder, move_folder, path_exists, rename_folder,
};

pub fn create_vault(name: &str, location: &str, config: &mut Config) {
    // check if vault name doesn't already exist
    if config.vault_exists(name) == false {
        // check if path is exists
        if path_exists(location) == true {
            // create path string with name of vault
            let path_with_name = create_path_with_name(location, name);
            // create folder at path
            create_folder(&path_with_name);
            // create .jot inside the folder -> will do this later
            // add vault to config
            config.add_vault(name.to_string(), location.to_string());
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
            if current_vault == name {
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
    // check if vault exists
    if config.vault_exists(name) == true {
        if name != new_name {
            rename_folder(name, new_name, config.get_vault_locaton(name).unwrap());
            config.rename_vault(name, new_name.to_string());
            // check if its the current vault, update if it is
            if let Some(vault) = config.get_current_vault() {
                if name == vault {
                    config.update_current_vault(Some(new_name.to_string()));
                }
            }
            println!("vault {} renamed to {}", name, new_name)
        } else {
            panic!("new name can't be same as old name")
        }
    } else {
        panic!("vault doesn't exist")
    }
}

pub fn delete_vault(name: &str, config: &mut Config) {
    if config.vault_exists(name) == true {
        // using unwrap because vault check has already been performed and the vault
        // definitely exists at this point
        let final_path = create_path_with_name(config.get_vault_locaton(name).unwrap(), name);
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
    if config.vault_exists(name) == true {
        if path_exists(new_location) == true {
            let original_location = config.get_vault_locaton(name).unwrap();
            if new_location != original_location {
                move_folder(name, original_location, new_location);
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
