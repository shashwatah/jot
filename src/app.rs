use crate::args::{Args, Command, Item, VaultItem};
use crate::config::Config;
use crate::dir::{change_dir, create_dir, delete_dir, dir_tree, move_dir, movev_dir, rename_dir};
use crate::note::{create_note, delete_note, move_note, movev_note, open_note, rename_note};
use crate::vault::{create_vault, delete_vault, enter_vault, move_vault, rename_vault, Vault};
use clap::Parser;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    args: Args,
    config: Config,
    vault: Option<Vault>,
}

impl App {
    pub fn new() -> Self {
        App {
            args: Args::parse(),
            config: Config::load(),
            vault: None,
        }
    }

    pub fn load_vault(&mut self) {
        self.vault = if let Some(vault_name) = self.config.get_current_vault() {
            let vault_location = self.config.get_vault_location(vault_name).unwrap();
            Some(Vault::load(vault_name, vault_location))
        } else {
            None
        }
    }

    pub fn display_app_data(&self) {
        println!("{:#?}\n{:#?}\n{:#?}", self.config, self.vault, self.args);
    }

    pub fn handle_args(&mut self) {
        match &self.args.command {
            Command::NTE { name } => {
                if let Some(name) = name {
                    create_note(name, self.vault.as_ref().unwrap())
                } else {
                    self.display_app_data()
                }
            }
            Command::OPN { name } => open_note(name, &self.config, self.vault.as_ref().unwrap()),
            Command::VLT { name, location } => {
                // name is some (i.e. location is also some) => create_vault
                if let Some(name) = name {
                    create_vault(name, location.as_ref().unwrap(), &mut self.config)
                } else {
                    // list vaults fn
                    println!("vaults: {:#?}", self.config.get_vaults().keys());
                    if let Some(vault) = self.config.get_current_vault() {
                        print!("current vault: {}", vault)
                    } else {
                        print!("not inside a vault")
                    }
                }
            }
            Command::ENT { name } => enter_vault(name, &mut self.config),
            Command::DIR { name } => {
                if let Some(vault) = &mut self.vault {
                    if let Some(name) = name {
                        create_dir(name, vault)
                    } else {
                        dir_tree(vault)
                    }
                } else {
                    panic!("not inside a vault")
                }
            }
            Command::CDR { path } => change_dir(path, self.vault.as_mut().unwrap()),
            Command::REN {
                item_type,
                name,
                new_name,
            } => match item_type {
                Item::VLT => rename_vault(name, new_name, &mut self.config),
                Item::NTE => rename_note(name, new_name, self.vault.as_ref().unwrap()),
                Item::DIR => rename_dir(name, new_name, self.vault.as_ref().unwrap()),
            },
            Command::DEL { item_type, name } => match item_type {
                Item::VLT => delete_vault(name, &mut self.config),
                Item::NTE => delete_note(name, self.vault.as_ref().unwrap()),
                Item::DIR => delete_dir(name, self.vault.as_ref().unwrap()),
            },
            Command::MOV {
                item_type,
                name,
                new_location,
            } => match item_type {
                Item::VLT => move_vault(name, new_location, &mut self.config),
                Item::NTE => move_note(name, new_location, self.vault.as_ref().unwrap()),
                Item::DIR => move_dir(name, new_location, self.vault.as_ref().unwrap()),
            },
            Command::MVV {
                item_type,
                name,
                vault_name,
            } => match item_type {
                VaultItem::NTE => {
                    movev_note(name, vault_name, &self.config, self.vault.as_ref().unwrap())
                }
                VaultItem::DIR => {
                    movev_dir(name, vault_name, &self.config, self.vault.as_ref().unwrap())
                }
            },
            _ => {
                self.display_app_data();
            }
        }
    }
}
