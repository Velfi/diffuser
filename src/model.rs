use crate::app;
use crate::constants::{DEFAULT_DECAY_FACTOR, DEFAULT_MAX_VALUE, DEFAULT_VALUE_CUTOFF};
use crate::matrix::{calculate_index_from_xy, Direction, Matrix2D};
use crate::{rect::Rect, vector2::Vector2};
use line_drawing::Bresenham;
use log::debug;
use rayon::prelude::*;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct Model {
    pub base_matrix: Matrix2D,
    pub left_click_is_held_down: bool,
    pub modifier_matrix: Matrix2D,
    pub mouse_xy: Vector2<f32>,
    pub previous_mouse_xy: Option<Vector2<f32>>,
    pub right_click_is_held_down: bool,
    pub window_rect: Rect<usize>,
}

impl Model {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(window_rect: Rect<usize>) -> Self {
        let base_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);
        let modifier_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);

        debug!(
            "Created new base_matrix with dimensions (w: {}, h: {})",
            base_matrix.w(),
            base_matrix.h()
        );
        debug!(
            "Created new modifier_matrix with dimensions (w: {}, h: {})",
            modifier_matrix.w(),
            modifier_matrix.h()
        );

        Self {
            base_matrix,
            left_click_is_held_down: false,
            modifier_matrix,
            mouse_xy: Vector2::new(0.0, 0.0),
            previous_mouse_xy: None,
            right_click_is_held_down: false,
            window_rect,
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        assert_eq!(self.base_matrix.len(), self.modifier_matrix.len(), "matrices should be identical length but they are not: base_matrix.len() == {}, modifier_matrix.len() == {}", self.base_matrix.len(), self.modifier_matrix.len());
        let mouse_buttons_are_held_down =
            self.left_click_is_held_down || self.right_click_is_held_down;
        if mouse_buttons_are_held_down {
            let Vector2 { x, y } = self.mouse_xy;
            let (x, y) = (x.round() as usize, y.round() as usize);

            if self.window_rect.contains(x, y) {
                if let Some(Vector2 {
                    x: prev_x,
                    y: prev_y,
                }) = self.previous_mouse_xy
                {
                    let (prev_x, prev_y, x, y) = (
                        prev_x.round() as isize,
                        prev_y.round() as isize,
                        x as isize,
                        y as isize,
                    );
                    let line_points = Bresenham::new((prev_x, prev_y), (x, y));
                    for (line_x, line_y) in line_points {
                        if line_x < 0
                            || line_y < 0
                            || line_x > self.window_rect.w() as isize
                            || line_y > self.window_rect.h() as isize
                        {
                            continue;
                        }

                        let index = calculate_index_from_xy(
                            line_x as usize,
                            line_y as usize,
                            self.window_rect.w() as usize,
                        );

                        *self.base_matrix.get_mut(index).expect("invalid index") =
                            match (self.left_click_is_held_down, self.right_click_is_held_down) {
                                (true, _) => DEFAULT_MAX_VALUE,
                                (_, true) => 0.0,
                                _ => {
                                    unreachable!("No other combinations need to be considered")
                                }
                            };
                    }

                    debug!(
                        "Painting from {{x: {}, y: {}}} to {{x: {}, y: {}}}",
                        prev_x, prev_y, x, y
                    );
                } else {
                    let index = calculate_index_from_xy(x, y, self.window_rect.w() as usize);

                    // can't fail because we've already checked that coords are in bounds
                    *self.base_matrix.get_mut(index).expect("invalid index") =
                        match (self.left_click_is_held_down, self.right_click_is_held_down) {
                            (true, _) => DEFAULT_MAX_VALUE,
                            (_, true) => 0.0,
                            _ => unreachable!("No other combinations need to be considered"),
                        };

                    debug!("Painting {{x: {}, y: {}}}", x, y);
                }

                // We need to store previous mouse positions so we can line draw when the mouse button is held down
                self.previous_mouse_xy = Some(self.mouse_xy);
            } else {
                debug!("Mouse outside canvas bounds {{x: {}, y: {}}}", x, y);
                self.previous_mouse_xy = None;
            }
        } else {
            self.previous_mouse_xy = None;
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
                        .get_neighbouring_cell_mut(index, *direction)
                        .map(spillover_fn);
                }
            }
        }

        // Apply the value of every cell in the modifier matrix to the corresponding cell in the base matrix
        self.modifier_matrix
            .iter_mut()
            .enumerate()
            .for_each(|(i, mod_value)| {
                if let Some(value) = base_matrix.get_mut(i) {
                    *value = (*value + *mod_value - (DEFAULT_DECAY_FACTOR * frame_time)).clamp(0.0, DEFAULT_MAX_VALUE);
                }

                // Reset each mod cells once we've used it up
                *mod_value = 0.0;
            });
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        assert_eq!(frame.len() / 4, self.base_matrix.len());

        frame
            .par_chunks_mut(4)
            .enumerate()
            .for_each(|(index, pixel)| {
                let value = *self
                    .base_matrix
                    .get(index)
                    .ok_or_else(|| app::Error::InvalidIndex {
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
