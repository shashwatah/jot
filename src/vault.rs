use crate::args::VltCommand;
use crate::config::Config;
use crate::fs::{check_path, create_directory, create_path_string, delete_directory};

pub fn handle_vlt_command(
    name: &Option<String>,
    path: &Option<String>,
    command: &Option<VltCommand>,
    config: &mut Config,
) -> () {
    // if name and path are some -> create vault with name and path
    if let Some(name_value) = name {
        if let Some(path_value) = path {
            create_vault(name_value, path_value, config);
            return ();
        }
    }

    if let Some(VltCommand::ENT { name }) = command {
        enter_vault(name, config);
        return ();
    }

    if let Some(VltCommand::DEL { name }) = command {
        delete_vault(name, config);
        return ();
    }

    // no arg passed -> display all vaults, highlight current vault
    println!("vaults: {:#?}", config.get_vaults().keys());
    match config.get_current_vault() {
        Some(current_vault) => println!("current vault: {}", current_vault),
        None => println!("not inside a vault"),
    }
}

fn create_vault(name: &str, path: &str, config: &mut Config) {
    // check if vault name doesn't already exist
    if config.check_vault(name) == false {
        // check if path is exists
        if check_path(path) == true {
            // create finale path string
            let final_path = create_path_string(name, path);
            // create directory at path
            create_directory(&final_path);
            // create .jot inside the folder
            // add vault to config
            config.add_vault(String::from(name), String::from(path));

            println!("{} created", name)
        } else {
            println!("path doesn't exist")
        }
    } else {
        println!("vault already exists")
    }
}

fn enter_vault(name: &str, config: &mut Config) {
    if config.check_vault(name) == true {
        if let Some(current_vault) = config.get_current_vault() {
            if current_vault == name {
                println!("already in {}", name);
                return ();
            }
        }
        config.update_current_vault(Some(name.to_string()));
        println!("switched to {}", name)
    } else {
        println!("vault doesn't exist")
    }
}

fn delete_vault(name: &str, config: &mut Config) {
    if config.check_vault(name) == true {
        if let Some(vault) = config.get_current_vault() {
            if name == vault {
                config.update_current_vault(None)
            }
        }

        // using unwrap because vault check has already been performed and the vault
        // definitely exists at this point
        let final_path = create_path_string(name, config.get_vault_path(name).unwrap());
        delete_directory(&final_path);
        config.delete_vault(name);
        println!("{} deleted", name)
    } else {
        println!("vault doesn't exist")
    }
}
