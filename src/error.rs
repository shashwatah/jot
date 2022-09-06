/*
- Errors:
    - DataError:
        - couldn't load config/data
    - VaultExists
    - ItemDoesntExist
        - Folder/Note
    - ItemAlreadyExists
    - InvalidName
    - PathDoesntExist
    - InvalidPath
    - SameName
    - InvalidEditor
*/

use std::io::ErrorKind;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    AlreadyExists,
    NotFound,
    Unspecified
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::AlreadyExists => "already exists",
            Error::NotFound => "not found",
            Error::Unspecified => "unspecified error",
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::AlreadyExists => Error::AlreadyExists,
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Unspecified            
        }
    }
}

