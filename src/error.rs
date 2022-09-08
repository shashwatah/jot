use crate::enums::{Item, VaultItem};
use std::fmt::Display;
use std::io::{Error as IOError, ErrorKind};

// writing boilerplate error code atm, will probably replace this with thiserror in future

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    FSError(IOError), // converted from io::std::Error > namely NotFound & AlreadyExists
    FileError,        // errors concering FileIO trait
    InternalError(String), // internal errors: unwrap calls that fail, internal err result matches
    InvalidName(String),
    SameName,
    SameLocation(Item),
    VaultAlreadyExists(String),
    VaultNotFound(String),
    NotInsideVault,
    PathOutOfBounds,
    ItemAlreadyExists(VaultItem, String),
    EditorNotFound,
    Misc,
}

/*
    errors that are not exactly errors:
        already in vault {} -- upon en_vault
        {item} {}  already exists in vault {} -- upon v_move
        {item} {} already exist in this location -- upon move

*/

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::FSError(io_err) => io_err.to_string().to_ascii_lowercase(),
                Error::SameName => "new can't be same as old name".to_string(),
                Error::SameLocation(item_type) =>
                    format!("{} already exists in this location", item_type.full()),
                Error::VaultAlreadyExists(name) => format!("vault {} already exists", name),
                Error::VaultNotFound(name) => format!("vault {} doesn't exist", name),
                Error::NotInsideVault => "not inside a vault".to_string(),
                Error::PathOutOfBounds => "path crosses the bounds of vault".to_string(),
                Error::ItemAlreadyExists(item_type, name) => format!(
                    "a {} named {} already exists in this location",
                    item_type.full(),
                    name
                ),
                Error::Misc | _ => "not set".to_string(),
            }
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound | ErrorKind::AlreadyExists => Error::FSError(error),
            _ => Error::Misc,
        }
    }
}
