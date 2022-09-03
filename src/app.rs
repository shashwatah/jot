use crate::{
    enums::{Item, VaultItem},
    state::{
        args::{Args, Command},
        config::Config,
        vaults::Vaults,
    },
    traits::FileIO,
};
use clap::Parser;

pub struct App {
    args: Args,
    config: Config,
    vaults: Vaults,
}

impl App {
    pub fn new() -> Self {
        App {
            args: Args::parse(),
            config: Config::load(),
            vaults: Vaults::load(),
        }
    }

    pub fn display_app_state(&self) {
        println!("{:#?}\n{:#?}\n{:#?}", self.args, self.config, self.vaults);
    }

    pub fn handle_args(&mut self) {
        match &self.args.command {
            Command::Vl { name, location } => {
                if let (Some(name), Some(location)) = (name, location) {
                    self.vaults.create_vault(name, location)
                } else {
                    self.vaults.list_vaults()
                }
            }
            Command::En { name } => self.vaults.enter_vault(name),
            Command::Nt { name } => self
                .vaults
                .ref_current()
                .create_vault_item(VaultItem::Nt, &name),
            Command::Op { name } => self
                .vaults
                .ref_current()
                .open_note(name, self.config.get_editor_data()),
            Command::Fd { name } => self
                .vaults
                .ref_current()
                .create_vault_item(VaultItem::Fd, &name),
            Command::Cd { path } => self.vaults.mut_current().change_folder(path),
            Command::Rm { item_type, name } => match item_type {
                Item::Vl => self.vaults.remove_vault(name),
                Item::Nt | Item::Fd => self
                    .vaults
                    .ref_current()
                    .remove_vault_item(item_type.to_vault_item(), name),
            },
            Command::Rn {
                item_type,
                name,
                new_name,
            } => match item_type {
                Item::Vl => self.vaults.rename_vault(name, new_name),
                Item::Nt | Item::Fd => self.vaults.ref_current().rename_vault_item(
                    item_type.to_vault_item(),
                    name,
                    new_name,
                ),
            },
            Command::Mv {
                item_type,
                name,
                new_location,
            } => match item_type {
                Item::Vl => self.vaults.move_vault(name, new_location),
                Item::Nt | Item::Fd => self.vaults.ref_current().move_vault_item(
                    item_type.to_vault_item(),
                    name,
                    new_location,
                ),
            },
            Command::Vm {
                item_type,
                name,
                vault_name,
            } => self.vaults.move_to_vault(item_type, name, vault_name),
            Command::Ls => self.vaults.ref_current().list(),
            Command::Cf { config_type, value } => {
                if let Some(value) = value {
                    self.config.set_config(config_type, value)
                } else {
                    self.config.display_config(config_type)
                }
            }
            _ => self.display_app_state(),
        }
    }
}
