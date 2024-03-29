use crate::{
    enums::{Item, VaultItem},
    output::{error::Error, message::Message},
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

    pub fn handle_args(&mut self) -> Result<Message, Error> {
        match &self.args.command {
            Command::Vault {
                show_loc,
                name,
                location,
            } => {
                if let (Some(name), Some(location)) = (name, location) {
                    self.vaults.create_vault(name, location)?;
                    Ok(Message::ItemCreated(Item::Vl, name.to_owned()))
                } else {
                    self.vaults.list_vaults(show_loc);
                    Ok(Message::Empty)
                }
            }
            Command::Enter { name } => {
                self.vaults.enter_vault(name)?;
                Ok(Message::VaultEntered(name.to_owned()))
            }
            Command::Note { name } => {
                self.vaults
                    .ref_current()?
                    .create_vault_item(VaultItem::Nt, name)?;
                Ok(Message::ItemCreated(Item::Nt, name.to_owned()))
            }
            Command::Open { name } => {
                self.vaults
                    .ref_current()?
                    .open_note(name, self.config.get_editor_data())?;
                Ok(Message::Empty)
            }
            Command::Folder { name } => {
                self.vaults
                    .ref_current()?
                    .create_vault_item(VaultItem::Fd, name)?;
                Ok(Message::ItemCreated(Item::Fd, name.to_owned()))
            }
            Command::Opdir => {
                self.vaults.ref_current()?.open_folder()?;
                Ok(Message::Empty)
            }
            Command::Chdir { path } => {
                self.vaults.mut_current()?.change_folder(path)?;
                Ok(Message::FolderChanged)
            }
            Command::Remove { item_type, name } => {
                match item_type {
                    Item::Vl | Item::Vault => self.vaults.remove_vault(name)?,
                    _ => self
                        .vaults
                        .ref_current()?
                        .remove_vault_item(item_type.to_vault_item(), name)?,
                };
                Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()))
            }
            Command::Rename {
                item_type,
                name,
                new_name,
            } => {
                match item_type {
                    Item::Vl | Item::Vault => self.vaults.rename_vault(name, new_name)?,
                    _ => self.vaults.ref_current()?.rename_vault_item(
                        item_type.to_vault_item(),
                        name,
                        new_name,
                    )?,
                };
                Ok(Message::ItemRenamed(
                    item_type.to_owned(),
                    name.to_owned(),
                    new_name.to_owned(),
                ))
            }
            Command::Move {
                item_type,
                name,
                new_location,
            } => {
                match item_type {
                    Item::Vl | Item::Vault => self.vaults.move_vault(name, new_location)?,
                    _ => self.vaults.ref_current()?.move_vault_item(
                        item_type.to_vault_item(),
                        name,
                        new_location,
                    )?,
                };
                Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()))
            }
            Command::Vmove {
                item_type,
                name,
                vault_name,
            } => {
                self.vaults.move_to_vault(item_type, name, vault_name)?;
                Ok(Message::ItemVMoved(
                    item_type.to_owned(),
                    name.to_owned(),
                    vault_name.to_owned(),
                ))
            }
            Command::List { item_type } => {
                self.vaults.ref_current()?.list(item_type);
                Ok(Message::Empty)
            }
            Command::Config { config_type, value } => {
                if config_type.is_none() {
                    self.config.open_config()?;
                    return Ok(Message::Empty);
                }

                let config_type = config_type.as_ref().unwrap();

                if let Some(value) = value {
                    self.config.set_config(config_type, value);
                    Ok(Message::ConfigSet(config_type.to_owned(), value.to_owned()))
                } else {
                    let value = self.config.get_config(config_type);
                    Ok(Message::Config(config_type.to_owned(), value))
                }
            }
            _ => Ok(Message::Empty),
        }
    }
}
