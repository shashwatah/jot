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
    // println!("***************************");
    // println!("**********JOT DEV**********");
    // println!("***************************");
    let mut app = App::new();

    match app.handle_args() {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("TEST {}", Output::Message(msg)),
        },
        Err(err) => println!("TEST{}", Output::Error(err)),
    }
}
