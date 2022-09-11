pub mod error;
pub mod message;

use colored::Colorize;
use std::fmt::Display;

use error::Error;
use message::Message;

pub enum Output {
    Message(Message),
    Error(Error),
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Output::Message(msg) => msg.to_string(),
                Output::Error(err) => format!("{}: {}", "error".red(), err),
            }
        )
    }
}
