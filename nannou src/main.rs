mod model;
mod constants;
mod matrix;

use model::{update, Model};

fn main() {
    println!("Starting the diffuser...");

    nannou::app(model).update(update).run();
}

fn model(app: &nannou::App) -> Model {
    Model::new(app)
}