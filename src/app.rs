use crate::args::{Args, Command};
use crate::config::Config;
use crate::vault::{handle_vault_cmd, Vault};
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
                let _ = handle_vault_cmd(name, path, command, &mut self.config);
            }
            _ => {
                self.display_config();
                self.display_args()
            }
        }
    }
}
