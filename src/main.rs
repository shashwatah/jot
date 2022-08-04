mod app;
mod args;
mod config;
mod vault;

use app::App;

fn main() {
    let mut app: App = App::new();
    app.handle_args()
}
