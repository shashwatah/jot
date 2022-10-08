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
                    return Ok(Message::ItemCreated(Item::Vl, name.to_owned()));
                } else if name.is_some() && *show_loc {
                    let name = name.clone().unwrap();
                    self.vaults.show_vault_location(name);
                    return Ok(Message::Empty);
                } else {
                    self.vaults.list_vaults(show_loc);
                    return Ok(Message::Empty);
                }
            }
            Command::Enter { name } => {
                self.vaults.enter_vault(name)?;
                return Ok(Message::VaultEntered(name.to_owned()));
            }
            Command::Note { name } => {
                self.vaults
                    .ref_current()?
                    .create_vault_item(VaultItem::Nt, name)?;
                return Ok(Message::ItemCreated(Item::Nt, name.to_owned()));
            }
            Command::Alias { name, alias } => {
                // TODO: make name and alias parameters &str
                self.vaults
                    .mut_current()?
                    .set_alias(name.to_string(), alias.to_string())?;

                // TODO: add a better note
                return Ok(Message::Empty);
            }
            Command::Open { name } => {
                self.vaults
                    .ref_current()?
                    .open_note(name, self.config.get_editor_data())?;
                return Ok(Message::Empty);
            }
            Command::Folder { name } => {
                self.vaults
                    .ref_current()?
                    .create_vault_item(VaultItem::Fd, name)?;
                return Ok(Message::ItemCreated(Item::Fd, name.to_owned()));
            }
            Command::Chdir { path } => {
                self.vaults.mut_current()?.change_folder(path)?;
                return Ok(Message::FolderChanged);
            }
            Command::Remove { item_type, name } => {
                match item_type {
                    Item::Vl | Item::Vault => self.vaults.remove_vault(name)?,
                    _ => self
                        .vaults
                        .ref_current()?
                        .remove_vault_item(item_type.to_vault_item(), name)?,
                };
                return Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()));
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
                return Ok(Message::ItemRenamed(
                    item_type.to_owned(),
                    name.to_owned(),
                    new_name.to_owned(),
                ));
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
                return Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()));
            }
            Command::Vmove {
                item_type,
                name,
                vault_name,
            } => {
                self.vaults.move_to_vault(item_type, name, vault_name)?;
                return Ok(Message::ItemVMoved(
                    item_type.to_owned(),
                    name.to_owned(),
                    vault_name.to_owned(),
                ));
            }
            Command::List => {
                self.vaults.ref_current()?.list();
                return Ok(Message::Empty);
            }
            Command::Config { config_type, value } => {
                if let Some(value) = value {
                    self.config.set_config(config_type, value);
                    return Ok(Message::ConfigSet(config_type.to_owned(), value.to_owned()));
                } else {
                    let value = self.config.get_config(config_type);
                    return Ok(Message::Config(config_type.to_owned(), value));
                }
            }
            _ => Ok(Message::Empty),
        }
    }
}
