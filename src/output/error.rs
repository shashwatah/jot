use crate::enums::Item;
use std::fmt::Display;

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    InternalError, // internal errors: unwrap calls that fail, internal err result matches
    FileError(String, std::io::Error), // errors concering FileIO trait
    InvalidName,
    SameName,
    SameLocation,
    PathNotFound, // PathNotFound and PathNotAbsolute might be subject to change in output type
    PathNotAbsolute,
    ItemAlreadyExists(Item, String),
    ItemNotFound(Item, String),
    VaultAlreadyExists(String),
    VaultNotFound(String),
    NotInsideVault,
    AlreadyInVault(String),
    OutOfBounds,
    EditorNotFound,
    MoveError(String), // this will be removed upon switching to custom recursive move fn
    Undefined(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::FileError(title, error) => format!(
                    "{} file error: {}",
                    title,
                    process_io_error(error.to_string())
                ),
                Error::InvalidName => "invalid name".to_string(),
                Error::SameName => "new name is same as old name".to_string(),
                Error::SameLocation => "new location is same as old location".to_string(),
                Error::PathNotFound => "couldn't find the path specified".to_string(),
                Error::PathNotAbsolute => "specified path isn't absolute".to_string(),
                Error::ItemAlreadyExists(item_type, name) => format!(
                    "a {} named {name} already exists in this location",
                    item_type.fs_name()
                ),
                Error::ItemNotFound(item_type, name) =>
                    format!("{} {name} not found", item_type.fs_name()),
                Error::VaultAlreadyExists(name) => format!("vault {name} already exists"),
                Error::VaultNotFound(name) => format!("vault {name} doesn't exist"),
                Error::NotInsideVault => "not inside a vault".to_string(),
                Error::AlreadyInVault(name) => format!("already in vault {name}"),
                Error::OutOfBounds => "path crosses the bounds of vault".to_string(),
                Error::EditorNotFound => "editor not found".to_string(),
                Error::MoveError(msg) => msg.to_owned(),
                Error::Undefined(error) => format!("undefined error: {error}"),
                _ => "error msg not set".to_string(),
            }
        )
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(error: fs_extra::error::Error) -> Self {
        Error::MoveError(process_io_error(error.to_string()))
    }
}

fn process_io_error(error: String) -> String {
    let mut error = error.to_lowercase();
    if let Some(dot) = error.find('.') {
        error.replace_range(dot.., "");
    }
    error
}
