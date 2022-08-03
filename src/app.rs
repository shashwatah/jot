use crate::args::Args;
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
            config: Config::load_config().unwrap(),
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
}
