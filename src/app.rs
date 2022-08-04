use crate::args::{Args, Command, VltCommand};
use crate::config::Config;
use crate::vault::Vault;
use clap::Parser;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    config: Config,
    args: Args,
    current_vault: Option<Vault>,
}

impl App {
    pub fn new() -> Self {
        App {
            config: Config::load_config(),
            args: Args::parse(),
            current_vault: None,
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
        match &self.args.command {
            Command::VLT {
                name,
                path,
                command,
            } => {
                if let Some(VltCommand::ENT { name }) = command {
                    self.config.update_current_vault(String::from(name));
                    println!("vaults: {:#?}", self.config.get_vaults());
                    match self.config.get_current_vault() {
                        Some(current_vault) => println!("current vault: {}", current_vault),
                        None => println!("Not inside a vault"),
                    }
                }

                if let Some(VltCommand::DEL { name }) = command {
                    if let Some(vault) = self.config.get_current_vault() {
                        if name == vault {
                            self.config.exit_current_vault()
                        }
                    }
                    self.config.del_vault(name);
                    println!("vaults: {:#?}", self.config.get_vaults());
                    match self.config.get_current_vault() {
                        Some(current_vault) => println!("current vault: {}", current_vault),
                        None => println!("Not inside a vault"),
                    }
                }

                if let Some(name_str) = name {
                    if let Some(path_str) = path {
                        self.config
                            .add_vault(String::from(name_str), String::from(path_str));
                        println!("vaults: {:#?}", self.config.get_vaults())
                    }
                }

                if let None = name {
                    if let None = command {
                        println!("vaults: {:#?}", self.config.get_vaults().keys());
                        match self.config.get_current_vault() {
                            Some(current_vault) => println!("current vault: {}", current_vault),
                            None => println!("Not inside a vault"),
                        }
                    }
                }
            }
            _ => {
                self.display_config();
                self.display_args()
            }
        }
    }
}
