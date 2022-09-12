use crate::enums::{ConfigType, Item, VaultItem};
use std::fmt::Display;

pub enum Message {
    VaultEntered(String),
    ItemCreated(Item, String),
    ItemRemoved(Item, String),
    ItemRenamed(Item, String, String),
    ItemMoved(Item, String),
    ItemVMoved(VaultItem, String, String),
    FolderChanged,
    Config(ConfigType, String),
    ConfigSet(ConfigType, String),
    Empty,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Message::VaultEntered(name) => format!("entered \x1b[0;34m{}\x1b[0m", name),
                Message::ItemCreated(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m created", item_type.full(), name),
                Message::ItemRemoved(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m removed", item_type.full(), name),
                Message::ItemRenamed(item_type, name, new_name) => format!(
                    "{} \x1b[0;34m{}\x1b[0m renamed to \x1b[0;34m{}\x1b[0m",
                    item_type.full(),
                    name,
                    new_name
                ),
                Message::ItemMoved(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m moved", item_type.full(), name),
                Message::ItemVMoved(item_type, name, vault_name) => format!(
                    "{} \x1b[0;34m{}\x1b[0m moved to vault \x1b[0;34m{}\x1b[0m",
                    item_type.full(),
                    name,
                    vault_name
                ),
                Message::FolderChanged => "changed folder".to_string(),
                Message::Config(config_type, value) =>
                    format!("{}: \x1b[0;34m{}\x1b[0m", config_type.to_str(), value),
                Message::ConfigSet(config_type, value) => format!(
                    "set \x1b[0;34m{}\x1b[0m to \x1b[0;34m{}\x1b[0m",
                    config_type.to_str(),
                    value
                ),
                Message::Empty => "".to_string(),
            }
        )
    }
}
