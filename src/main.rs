mod app;
mod state;
mod traits;
mod types;
mod utils;

use crate::app::App;

fn main() {
    let mut app = App::new();
    app.handle_args();
}
