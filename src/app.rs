use crate::args::{Args, Command};
use crate::config::Config;
use crate::vault::handle_vlt_command;
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
        match &self.args.command {
            Command::VLT {
                name,
                path,
                command,
            } => {
                handle_vlt_command(name, path, command, &mut self.config);
            }
            _ => {
                self.display_config();
                self.display_args()
            }
        }
    }
}
