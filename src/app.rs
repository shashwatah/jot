use crate::args::{Args, Command, Item, VaultItem};
use crate::config::Config;
use crate::dir::{
    change_dir, create_dir, delete_dir, move_dir, movev_dir, print_dir_tree, rename_dir,
};
use crate::note::{create_note, delete_note, rename_note, move_note};
use crate::vault::{create_vault, delete_vault, enter_vault, move_vault, rename_vault, Vault};
use clap::Parser;

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
            config: Config::load(),
            current_vault: None,
            args: Args::parse(),
        }
    }

    pub fn load_current_vault(&mut self) {
        self.current_vault = match self.config.get_current_vault() {
            Some(current_vault_name) => {
                let current_vault_location =
                    self.config.get_vault_location(current_vault_name).unwrap();
                Some(Vault::load(current_vault_location, current_vault_name))
            }
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
            Command::NTE { name } => {
                match name {
                    Some(name_value) => create_note(name_value, self.current_vault.as_ref().unwrap()),
                    None => self.display_app_data()
                }
            }
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
            Command::DIR { name } => match self.current_vault {
                Some(_) => match name {
                    Some(name_value) => {
                        create_dir(name_value, self.current_vault.as_mut().unwrap())
                    }
                    None => print_dir_tree(self.current_vault.as_ref().unwrap()),
                },
                None => {
                    panic!("not inside a vault")
                }
            },
            Command::CDR { location } => change_dir(location, self.current_vault.as_mut().unwrap()),
            Command::REN {
                item_type,
                name,
                new_name,
            } => match item_type {
                Item::VLT => rename_vault(name, new_name, &mut self.config),
                Item::NTE => rename_note(name, new_name, self.current_vault.as_ref().unwrap()),
                Item::DIR => rename_dir(name, new_name, self.current_vault.as_ref().unwrap())
            },
            Command::DEL { item_type, name } => match item_type {
                Item::VLT => delete_vault(name, &mut self.config),
                Item::NTE => delete_note(name, self.current_vault.as_ref().unwrap()),
                Item::DIR => delete_dir(name, self.current_vault.as_ref().unwrap())
            },
            Command::MOV {
                item_type,
                name,
                new_location,
            } => match item_type {
                Item::VLT => move_vault(name, new_location, &mut self.config),
                Item::NTE => move_note(name, new_location, self.current_vault.as_ref().unwrap()),
                Item::DIR => move_dir(name, new_location, self.current_vault.as_ref().unwrap())
            },
            Command::MVV {
                item_type,
                name,
                vault_name,
            } => match item_type {
                VaultItem::DIR => movev_dir(
                    name,
                    vault_name,
                    &self.config,
                    self.current_vault.as_ref().unwrap(),
                ),
                _ => self.display_app_data(),
            },
            _ => {
                self.display_app_data();
            }
        }
    }
}
