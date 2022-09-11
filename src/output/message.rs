use crate::enums::{ConfigType, Item, VaultItem};
use colored::Colorize;
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
                Message::VaultEntered(name) => format!("entered {}", name.blue()),
                Message::ItemCreated(item_type, name) =>
                    format!("{} {} created", item_type.full(), name.blue()),
                Message::ItemRemoved(item_type, name) =>
                    format!("{} {} removed", item_type.full(), name.blue()),
                Message::ItemRenamed(item_type, name, new_name) => format!(
                    "{} {} renamed to {}",
                    item_type.full(),
                    name.blue(),
                    new_name.blue()
                ),
                Message::ItemMoved(item_type, name) =>
                    format!("{} {} moved", item_type.full(), name.blue()),
                Message::ItemVMoved(item_type, name, vault_name) => format!(
                    "{} {} moved to vault {}",
                    item_type.full(),
                    name.blue(),
                    vault_name.blue()
                ),
                Message::FolderChanged => "changed folder".to_string(),
                Message::Config(config_type, value) =>
                    format!("{}: {}", config_type.to_str(), value.blue()),
                Message::ConfigSet(config_type, value) =>
                    format!("set {} to {}", config_type.to_str().blue(), value.blue()),
                Message::Empty => "".to_string(),
            }
        )
    }
}
