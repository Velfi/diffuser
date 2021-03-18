use crate::constants::{DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W};
use crate::Model;
use nannou::prelude::Rect;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::{LogicalPosition, PhysicalSize}, window::Fullscreen};
use winit::event_loop::EventLoop;
use winit::{dpi::LogicalSize, window::Window};
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
        let (window, width, height, mut _hidpi_factor) = create_window("Diffuser", &event_loop);

        let pixel_w = width / 1;
        let pixel_h = height / 1;

        let pixels = {
            let surface_texture = SurfaceTexture::new(width, height, &window);
            Pixels::new(pixel_w, pixel_h, surface_texture).unwrap()
        };

        let model = Model::new(Rect::from_w_h(pixel_w as i32, pixel_h as i32));

        Self {
            event_loop,
            window,
            pixels,
            model,
            input,
        }
    }
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
pub fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = DEFAULT_RESOLUTION_W as f64;
    let height = DEFAULT_RESOLUTION_H as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let LogicalSize { width, height }= monitor.size().to_logical(hidpi_factor);
            (width, height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height).round().max(1.0);

    // Resize, center, and display the window
    let min_size = PhysicalSize::new(width, height).to_logical::<f64>(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        monitor_width - width * scale,
        monitor_height - height * scale,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}
