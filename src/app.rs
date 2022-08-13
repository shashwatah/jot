use crate::args::{Args, Command, Item};
use crate::config::Config;
use crate::fs::create_path_with_name;
use crate::vault::{create_vault, delete_vault, enter_vault, move_vault, rename_vault, Vault};
use clap::Parser;
use walkdir::WalkDir;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    config: Config,
    current_vault: Option<Vault>,
    args: Args,
}

impl App {
    pub fn new() -> Self {
        App {
            config: Config::load_config(),
            current_vault: None,
            args: Args::parse(),
        }
    }

    pub fn load_current_vault(&mut self) {
        self.current_vault = match self.config.get_current_vault() {
            Some(current_vault_name) => Some(Vault::load_data(&self.config, current_vault_name)),
            None => None,
        }
    }

    pub fn display_app_data(&self) {
        println!(
            "{:#?}\n{:#?}\n{:#?}",
            self.config, self.current_vault, self.args
        );
    }

    pub fn handle_args(&mut self) {
        match &self.args.command {
            Command::VLT { name, location } => {
                // if name and path are some -> create vault with name and path
                match name {
                    Some(name_value) => {
                        if let Some(path_value) = location {
                            create_vault(name_value, path_value, &mut self.config);
                            return;
                        }
                    }
                    None => {
                        println!("vaults: {:#?}", self.config.get_vaults().keys());
                        match self.config.get_current_vault() {
                            Some(current_vault) => println!("current vault: {}", current_vault),
                            None => println!("not inside a vault"),
                        }
                    }
                }
            }
            Command::ENT { name } => enter_vault(name, &mut self.config),
            Command::DIR { name } => match name {
                Some(name_value) => {
                    println!("create vault: {}", name_value);
                    self.display_app_data();
                }
                None => {
                    // will pretty print the tree later, using as-tree right now
                    // cargo install as-tree -> jot dir | as-tree
                    if let Some(_) = self.current_vault {
                        let current_vault_name = self.config.get_current_vault().unwrap();
                        let current_vault_location =
                            self.config.get_vault_locaton(current_vault_name).unwrap();
                        let current_vault_path =
                            create_path_with_name(current_vault_location, current_vault_name);
                        for entry in WalkDir::new(current_vault_path)
                            .into_iter()
                            .filter_map(|e| e.ok())
                        {
                            println!("{}", entry.path().display());
                        }
                    }
                }
            },
            Command::REN {
                item_type,
                name,
                new_name,
            } => match item_type {
                Item::VLT => rename_vault(name, new_name, &mut self.config),
                _ => {
                    self.display_app_data();
                }
            },
            Command::DEL { item_type, name } => match item_type {
                Item::VLT => delete_vault(name, &mut self.config),
                _ => {
                    self.display_app_data();
                }
            },
            Command::MOV {
                item_type,
                name,
                new_location,
            } => match item_type {
                Item::VLT => move_vault(name, new_location, &mut self.config),
                _ => {
                    self.display_app_data();
                }
            },
            _ => {
                self.display_app_data();
            }
        }
    }
}
