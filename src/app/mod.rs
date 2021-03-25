mod error;

use crate::{
    constants::{DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W},
    model::Model,
    rect::Rect,
};
use circular_queue::CircularQueue;
pub use error::Error;
use log::{debug, error, info};
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{dpi::LogicalSize, event::VirtualKeyCode, event_loop::ControlFlow, window::Window};
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

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

        let model = Model::new(Rect::new(
            window.inner_size().height as usize,
            window.inner_size().width as usize,
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

pub fn run(app: App) {
    let App {
        event_loop,
        mut input,
        mut model,
        mut pixels,
        window,
    } = app;

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
                debug!("Pressed LMB");
                model.left_click_is_held_down = true
            } else if input.mouse_released(0) {
                debug!("Released LMB");
                model.left_click_is_held_down = false
            }

            if input.mouse_pressed(1) {
                debug!("Pressed RMB");
                model.right_click_is_held_down = true
            } else if input.mouse_released(1) {
                debug!("Released RMB");
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
                info!("FPS {}", avg_fps.trunc());
            }
        }
    })
}
