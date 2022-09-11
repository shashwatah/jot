mod app;
mod enums;
mod output;
mod state;
mod traits;
mod utils;

use crate::{app::App, output::Output};
fn main() {
    let mut app = App::new();
    println!(
        "{}",
        match app.handle_args() {
            Ok(msg) => Output::Message(msg),
            Err(err) => Output::Error(err),
        }
    );
}
