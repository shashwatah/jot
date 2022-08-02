use crate::args::Args;
use clap::Parser;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    config: String,
    args: Args,
    vaults: Vec<String>,
    current_vault: Option<Vault>,
}

impl App {
    pub fn new() -> App {
        App {
            config: String::from("config"),
            args: Args::parse(),
            vaults: vec![],
            current_vault: None,
        }
    }

    pub fn display_args(&self) {
        println!("{:#?}", self.args)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Vault {
    name: String,
    location: PathBuf,
    history: String,
}
