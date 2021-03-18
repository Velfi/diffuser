mod app;
mod app_error;
mod constants;
mod matrix;

use app::App;
use app_error::AppError;
use circular_queue::CircularQueue;
use constants::{DEFAULT_DECAY_FACTOR, DEFAULT_MAX_VALUE, DEFAULT_VALUE_CUTOFF};
use log::error;
use matrix::{Direction, Matrix2D};
use nannou::prelude::{Rect, Vector2};
use pixels::{Error, Pixels};
use rayon::prelude::*;
use std::time::Instant;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    env_logger::init();

    let App {
        event_loop,
        input,
        model,
        pixels,
        window,
    } = App::new();
    run(event_loop, input, model, pixels, window);

    Ok(())
}

fn run(
    event_loop: EventLoop<()>,
    mut input: WinitInputHelper,
    mut model: Model,
    mut pixels: Pixels<Window>,
    window: Window,
) {
    let mut frame_time = 0.16;
    let mut time_of_last_frame_start = Instant::now();

    let mut frame_counter = 0;
    let mut fps_values = CircularQueue::with_capacity(5);
    let mut time_of_last_fps_counter_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            model.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.mouse_pressed(0) {
                log::info!("Pressed LMB");
                model.left_click_is_held_down = true
            } else if input.mouse_released(0) {
                log::info!("Released LMB");
                model.left_click_is_held_down = false
            }

            if input.mouse_pressed(1) {
                log::info!("Pressed RMB");
                model.right_click_is_held_down = true
            } else if input.mouse_released(1) {
                log::info!("Released RMB");
                model.right_click_is_held_down = false
            }

            if let Some((x, y)) = input
                .mouse()
                .map(|xy| pixels.window_pos_to_pixel(xy).ok())
                .flatten()
            {
                {
                    model.mouse_xy.x = x as f32;
                    model.mouse_xy.y = y as f32;
                }
            }

            // Resize the window
            if let Some(PhysicalSize { width, height }) = input.window_resized() {
                pixels.resize(width, height);
                model.window_rect = Rect::from_w_h(width as i32, height as i32);
                model.base_matrix = Matrix2D::new(height as usize, width as usize);
                model.modifier_matrix = Matrix2D::new(height as usize, width as usize);
            }

            // Update internal state and request a redraw
            window.request_redraw();
            model.update(frame_time);

            frame_time = time_of_last_frame_start.elapsed().as_secs_f32();
            time_of_last_frame_start = Instant::now();

            frame_counter += 1;

            if time_of_last_fps_counter_update.elapsed().as_secs() > 1 {
                time_of_last_fps_counter_update = Instant::now();
                let _ = fps_values.push(frame_counter);
                frame_counter = 0;

                let fps_sum: i32 = fps_values.iter().sum();
                let avg_fps = fps_sum as f32 / fps_values.len() as f32;
                println!("FPS {}", avg_fps.trunc());
            }
        }
    })
}

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct Model {
    pub base_matrix: Matrix2D,
    pub left_click_is_held_down: bool,
    pub modifier_matrix: Matrix2D,
    pub mouse_xy: Vector2<f32>,
    pub right_click_is_held_down: bool,
    pub rng: rand::rngs::ThreadRng,
    pub window_rect: Rect<i32>,
}

impl Model {
    /// Create a new `World` instance that can draw a moving box.
    fn new(window_rect: Rect<i32>) -> Self {
        let rng = rand::thread_rng();

        let base_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);
        let modifier_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);

        let model = Self {
            base_matrix,
            left_click_is_held_down: false,
            modifier_matrix,
            mouse_xy: Vector2::new(0.0, 0.0),
            right_click_is_held_down: false,
            rng,
            window_rect,
        };

        model
    }

    fn update(&mut self, frame_time: f32) {
        assert_eq!(self.base_matrix.len(), self.modifier_matrix.len(), "matrices should be identical length but they are not: base_matrix.len() == {}, modifier_matrix.len() == {}", self.base_matrix.len(), self.modifier_matrix.len());
        if self.left_click_is_held_down {
            let Vector2 { x, y } = self.mouse_xy;
            let (x, y) = (x.round() as usize, y.round() as usize);
            self.base_matrix
                .get_mut(x, y)
                .map(|value| *value = DEFAULT_MAX_VALUE);

            // println!("Painting {{x: {}, y: {}}}", x, y);
        }

        let base_matrix = &mut self.base_matrix;
        let modifier_matrix = &mut self.modifier_matrix;

        /*
        paint in a bucket
        spills into neighbouring cells
        lightening their shade
        */
        base_matrix
            .iter_mut()
            .filter_map(|(x, y, value)| {
                // for cells with paint, darken the cell, calculate spillover
                if *value > DEFAULT_VALUE_CUTOFF {
                    // cell spills over into its four neighbours, so it gets divided into five parts
                    // that's four parts for the neighbours, and one part to keep
                    *value = *value / 9.0;

                    // the current value will also be the amount that pours over into the neighbours
                    return Some((x, y, *value));
                }

                if *value <= DEFAULT_VALUE_CUTOFF {
                    // For values below the VALUE_CUTOFF, set them to zero in order to avoid ever-shrinking float values
                    *value = 0.0;
                }

                // No spillover
                None
            })
            .for_each(|(x, y, spillover)| {
                // All neighbours are updated in the same way, so we define the closure once
                // Spillover is added to the current value of each affected neighbour,
                let spillover_fn = |value: &mut f32| *value += spillover;
                {
                    use Direction::*;
                    for direction in &[
                        NorthWest, North, NorthEast, West, East, SouthEast, South, SouthWest,
                    ] {
                        // For each neighbouring cell in the modifier matrix, add the spillover value
                        modifier_matrix
                            .get_neighbouring_cell_mut(x, y, *direction)
                            .map(spillover_fn);
                    }
                }
            });

        let decay_amount = DEFAULT_DECAY_FACTOR * frame_time;
        // Apply the value of every cell in the modifier matrix to the corresponding cell in the base matrix
        modifier_matrix
            .iter_mut()
            .zip(base_matrix.iter_mut())
            .for_each(|((mx, my, mod_value), (bx, by, base_value))| {
                assert_eq!(mx, bx);
                assert_eq!(my, by);

                *base_value = (*base_value + *mod_value + decay_amount).max(0.0);

                // Reset each mod cells once we've used it up
                *mod_value = 0.0;
            });
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .zip(self.base_matrix.iter())
            .for_each(|(pixel, (_x, _y, value))| {
                let value = ((1.0 - (*value).min(1.0)).abs() * 255.0).round() as u8;

                pixel.copy_from_slice(&[value, value, value, 0xff]);
            });
    }
}
