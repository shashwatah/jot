mod app;
mod enums;
mod output;
mod state;
mod traits;
mod utils;

use crate::{
    app::App,
    output::{Message, Output},
};
fn main() {
    let mut app = App::new();

    match app.handle_args() {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("{}", Output::Message(msg)),
        },
        Err(err) => println!("{}", Output::Error(err)),
    }
}
