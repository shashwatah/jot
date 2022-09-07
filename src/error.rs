use std::fmt::Display;
use std::io::{Error as OSError, ErrorKind};

// writing boilerplate error code atm, will probably replace this with thiserror in future

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    IOError {
        // converted from io::std::Error > namely NotFound & AlreadyExists
        source: OSError,
    },
    FileError,     // errors concering FileIO trait
    InternalError, // internal erros: unwrap calls that fail, internal err result matches
    InvalidName,
    VaultAlreadyExists(String),
    VaultNotFound(String),
    NotInsideVault,
    PathOutOfBounds,
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
                _ => Error::Misc,
            }
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound | ErrorKind::AlreadyExists => Error::IOError { source: error },
            _ => Error::Misc,
        }
    }
}
