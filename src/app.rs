use crate::state::{args::Args, config::Config, data::Data, vault::Vault};
use clap::Parser;

pub struct App {
    args: Args,
    config: Config,
    data: Data,
    vault: Option<Vault>
}

impl App {
    pub fn new() -> Self {
        App {
            args: Args::parse(),
            config: Config {},
            data: Data {},
            vault: None
        }
    }

    pub fn handle_args(&self) {}

    fn load_vault(&mut self) {}
    fn create_vault(&self) {}
    fn remove_vault(&self) {}
    fn rename_vault(&self) {}
    fn move_vault(&self) {}
}