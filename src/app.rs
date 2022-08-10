use crate::args::{Args, ItemType, SubCommand};
use crate::config::Config;
use crate::vault::{create_vault, delete_vault, enter_vault, move_vault, rename_vault};
use clap::Parser;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    config: Config,
    args: Args,
}

impl App {
    pub fn new() -> Self {
        App {
            config: Config::load_config(),
            args: Args::parse(),
        }
    }

    pub fn display_config(&self) {
        println!("{:#?}", self.config)
    }

    pub fn display_args(&self) {
        println!("{:#?}", self.args)
    }

    // absoluut spaghet 🤌
    pub fn handle_args(&mut self) {
        match &self.args.subcommand {
            SubCommand::VLT { name, location } => {
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
            SubCommand::ENT { name } => enter_vault(name, &mut self.config),
            SubCommand::REN {
                item_type,
                name,
                new_name,
            } => match item_type {
                ItemType::VLT => rename_vault(name, new_name, &mut self.config),
                _ => {
                    self.display_config();
                    self.display_args()
                }
            },
            SubCommand::DEL { item_type, name } => match item_type {
                ItemType::VLT => delete_vault(name, &mut self.config),
                _ => {
                    self.display_config();
                    self.display_args()
                }
            },
            SubCommand::MOV {
                item_type,
                name,
                new_location,
            } => match item_type {
                ItemType::VLT => move_vault(name, new_location, &mut self.config),
                _ => {
                    self.display_config();
                    self.display_args()
                }
            },
            _ => {
                self.display_config();
                self.display_args()
            }
        }
    }
}
