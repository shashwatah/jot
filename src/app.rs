use crate::config::Config;
use crate::args::Args;
use clap::Parser;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
struct Vault {
    name: String,
    location: PathBuf,
    history: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    config: Config,
    args: Args,
    vaults: Vec<String>,
    current_vault: Option<Vault>,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::load_config(),
            args: Args::parse(),
            vaults: vec![],
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
