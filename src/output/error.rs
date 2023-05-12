use crate::enums::Item;
use std::fmt::Display;

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    InvalidName,
    SameName,
    SameLocation,
    // PathNotFound and PathNotAbsolute might get converted from errors to a different output type.
    PathNotFound, 
    PathNotAbsolute,
    ItemAlreadyExists(Item, String),
    ItemNotFound(Item, String),
    VaultAlreadyExists(String),
    VaultNotFound(String),
    NotInsideVault,
    AlreadyInVault(String),
    OutOfBounds,
    EditorNotFound,
    // MoveError will be removed if and when fs_extra::move_items() is replaced with a custom function.
    MoveError(String), 
    Undefined(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
                Error::Undefined(error) => format!("undefined error: {error}")
            }
        )
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(error: fs_extra::error::Error) -> Self {
        Error::MoveError(process_io_error(error.to_string()))
    }
}


// @desc: Converts error message to jot's native format by removing redundant information, i.e. 
//        error code after the first full-stop.
//        Used (specifically) above to convert error message returned by fs_extra::move_items().
fn process_io_error(error: String) -> String {
    let mut error = error.to_lowercase();
    if let Some(dot) = error.find('.') {
        error.replace_range(dot.., "");
    }
    error
}
