mod app;
mod args;

use app::App;

fn main() {
    let app: App = App::new();
    app.display_args()
}
