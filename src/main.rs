mod app;
mod args;
mod config;

use app::App;

fn main() {
    let app: App = App::new();
    app.display_config();
    app.display_args()
}
