use crate::args::VltCommand;
use crate::config::Config;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vault {
    name: String,
    location: String,
}

pub fn handle_vault_cmd(
    name: &Option<String>,
    path: &Option<String>,
    command: &Option<VltCommand>,
    config: &mut Config,
) -> () {
    if let Some(name_str) = name {
        if let Some(path_str) = path {
            config.add_vault(String::from(name_str), String::from(path_str));
            return ();
        }
    }

    if let Some(VltCommand::ENT { name }) = command {
        config.update_current_vault(Some(name));
        return ();
    }

    if let Some(VltCommand::DEL { name }) = command {
        delete_vault(name);
        config.delete_vault(name);
        return ();
    }

    println!("vaults: {:#?}", config.get_vaults().keys());
    match config.get_current_vault() {
        Some(current_vault) => println!("current vault: {}", current_vault),
        None => println!("Not inside a vault"),
    }
}

pub fn create_vault() {}
pub fn delete_vault(name: &str) {}
pub fn move_vault() {}
