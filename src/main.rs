mod app;
mod constants;
mod matrix;
mod model;
mod rect;
mod vector2;

use app::{run, App};

fn main() {
    env_logger::init();

    let app = App::new();
    run(app);
}
