mod app_error;
mod constants;
mod matrix;

use std::time::Instant;

use app_error::AppError;
use circular_queue::CircularQueue;
use constants::{
    DEFAULT_DECAY_FACTOR, DEFAULT_MAX_VALUE, DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W,
    DEFAULT_VALUE_CUTOFF,
};
use log::error;
use matrix::{Direction, Matrix2D};
use nannou::prelude::{Rect, Vector2};
use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::{dpi::LogicalSize, window::Window};
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

pub struct App {
    pub event_loop: EventLoop<()>,
    pub input: WinitInputHelper,
    pub model: Model,
    pub pixels: Pixels<Window>,
    pub window: Window,
}

impl App {
    pub fn new() -> App {
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(DEFAULT_RESOLUTION_W as f64, DEFAULT_RESOLUTION_H as f64);
            WindowBuilder::new()
                .with_title("Diffuser")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H, surface_texture).unwrap()
        };

        let model = Model::new(Rect::from_w_h(
            window.inner_size().width as i32,
            window.inner_size().height as i32,
        ));

        Self {
            event_loop,
            window,
            pixels,
            model,
            input,
        }
    }
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

            if let Some((x, y)) = input.mouse() {
                model.mouse_xy.x = x;
                model.mouse_xy.y = y;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
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
    pub window_rect: Rect<i32>,
}

impl Model {
    /// Create a new `World` instance that can draw a moving box.
    fn new(window_rect: Rect<i32>) -> Self {
        let base_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);
        let modifier_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);

        println!(
            "Created new base_matrix with dimensions (w: {}, h: {})",
            base_matrix.w(),
            base_matrix.h()
        );
        println!(
            "Created new modifier_matrix with dimensions (w: {}, h: {})",
            modifier_matrix.w(),
            modifier_matrix.h()
        );

        Self {
            base_matrix,
            left_click_is_held_down: false,
            modifier_matrix,
            mouse_xy: Vector2::new(0.0, 0.0),
            right_click_is_held_down: false,
            window_rect,
        }
    }

    fn update(&mut self, frame_time: f32) {
        assert_eq!(self.base_matrix.len(), self.modifier_matrix.len(), "matrices should be identical length but they are not: base_matrix.len() == {}, modifier_matrix.len() == {}", self.base_matrix.len(), self.modifier_matrix.len());
        if self.left_click_is_held_down || self.right_click_is_held_down {
            let Vector2 { x, y } = self.mouse_xy;
            let (x, y) = (x.round() as usize, y.round() as usize);

            if (0..self.window_rect.w()).contains(&(x as i32))
                && (0..self.window_rect.h()).contains(&(y as i32))
            {
                // can't fail because we've already checked that coords are in bounds
                *self.base_matrix.get_mut(x, y).expect("invalid xy coords") =
                    match (self.left_click_is_held_down, self.right_click_is_held_down) {
                        (true, _) => DEFAULT_MAX_VALUE,
                        (_, true) => 0.0,
                        _ => unreachable!("No other combinations need to be considered"),
                    };

                println!("Painting {{x: {}, y: {}}}", x, y);
            } else {
                println!("Mouse outside canvas bounds {{x: {}, y: {}}}", x, y);
            }
        }

        let base_matrix = &mut self.base_matrix;
        let modifier_matrix = &mut self.modifier_matrix;

        /*
        paint in a bucket
        spills into neighbouring cells
        affecting their shade
        */
        for (index, spillover) in base_matrix
            .iter_mut()
            .enumerate()
            .filter_map(|(index, value)| {
                // for cells with paint, darken the cell, calculate spillover
                if *value > DEFAULT_VALUE_CUTOFF {
                    // cell spills over into its four neighbours, so it gets divided into five parts
                    // that's four parts for the neighbours, and one part to keep
                    *value /= 9.0;

                    // the current value will also be the amount that pours over into the neighbours
                    return Some((index, *value));
                }

                if *value <= DEFAULT_VALUE_CUTOFF {
                    // For values below the VALUE_CUTOFF, set them to zero in order to avoid ever-shrinking (but non-zero) float values
                    *value = 0.0;
                }

                // No spillover
                None
            })
        {
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
                        .get_neighbouring_cell_mut_by_index(index, *direction)
                        .map(spillover_fn);
                }
            }
        }

        // Apply the value of every cell in the modifier matrix to the corresponding cell in the base matrix
        self.modifier_matrix
            .iter_mut()
            .enumerate()
            .for_each(|(i, mod_value)| {
                if let Some(value) = base_matrix.get_mut_by_index(i) {
                    *value = (*value + *mod_value + (DEFAULT_DECAY_FACTOR * frame_time)).max(0.0);
                }

                // Reset each mod cells once we've used it up
                *mod_value = 0.0;
            });
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        assert_eq!(frame.len() / 4, self.base_matrix.len());

        frame
            .par_chunks_mut(4)
            .enumerate()
            .for_each(|(index, pixel)| {
                let value = *self
                    .base_matrix
                    .get_by_index(index)
                    .ok_or_else(|| AppError::InvalidIndex {
                        list_name: "base_matrix".to_owned(),
                        index,
                        len: self.base_matrix.len(),
                    })
                    .unwrap();
                let value = (value.min(1.0) * 255.0).round();
                let value = (255.0 - value).clamp(0.0, 255.0) as u8;

                pixel.copy_from_slice(&[value, value, value, 0xff]);
            })
    }
}
