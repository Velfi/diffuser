use crate::{
    constants::{DEFAULT_DECAY_FACTOR, DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W},
    matrix::Matrix2D,
};
use nannou::{color::rgb::Rgb, prelude::*};

pub struct Model {
    pub _window: window::Id,
    pub base_matrix: Matrix2D,
    pub decay_factor: f32,
    pub left_click_is_held_down: bool,
    pub modifier_matrix: Matrix2D,
    pub mouse_xy: Vector2<f32>,
    pub right_click_is_held_down: bool,
    pub rng: rand::rngs::ThreadRng,
    pub window_rect: Rect<f32>,
}

impl Model {
    pub fn new(app: &nannou::App) -> Self {
        let window_rect = Rect::from_w_h(DEFAULT_RESOLUTION_W as f32, DEFAULT_RESOLUTION_W as f32);

        let _window = app
            .new_window()
            .size(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H)
            .view(view)
            .mouse_moved(mouse_moved)
            .mouse_pressed(mouse_pressed)
            .mouse_released(mouse_released)
            // .key_pressed(update::key_pressed)
            // .resized(update::resized)
            .build()
            .unwrap();

        let rng = rand::thread_rng();

        let base_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);
        let modifier_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);

        let model = Self {
            _window,
            base_matrix,
            decay_factor: DEFAULT_DECAY_FACTOR,
            left_click_is_held_down: false,
            modifier_matrix,
            mouse_xy: Vector2::new(0.0, 0.0),
            right_click_is_held_down: false,
            rng,
            window_rect,
        };

        model
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.left_click_is_held_down {
        let Vector2 { x, y } = model.mouse_xy;
        let (x, y) = (x.round() as usize, y.round() as usize);
        model.base_matrix.get_mut(x, y).map(|value| { *value = 1.0 });
    }

    // model.base_matrix.iter_mut().for_each(|(x, y, value)| {

    // });
}

fn mouse_moved(_app: &App, model: &mut Model, xy: Point2) {
    model.mouse_xy = xy;
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.left_click_is_held_down = true,
        MouseButton::Right => model.right_click_is_held_down = true,
        _ => (),
    }
}

fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.left_click_is_held_down = false,
        MouseButton::Right => model.right_click_is_held_down = false,
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // draw.background().color(BLACK);
    let mut rgb_value = 255u8;
    let mut color = WHITE;

    model.base_matrix.iter().for_each(|(x, y, value)| {
        rgb_value = (*value * 255.0).round() as u8;
        color = Rgb::new(rgb_value, rgb_value, rgb_value);
        draw.rect().w_h(1.0, 1.0).x_y(x as f32, y as f32).color(color);
    });

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
