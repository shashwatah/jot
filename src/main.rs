mod app;
mod enums;
mod error;
mod output;
mod state;
mod traits;
mod utils;

use crate::app::App;

fn main() {
    let mut app = App::new();

    if let Err(error) = app.handle_args() {
        println!("{}", error)
    }
}
