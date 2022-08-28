mod app;
mod dir;
mod fs;
mod helpers;
mod note;
mod states;
mod vault;

use app::App;

fn main() {
    let mut app: App = App::new();
    app.load_vault();
    app.handle_args()
}
