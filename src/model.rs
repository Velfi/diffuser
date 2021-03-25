use crate::app;
use crate::constants::{DEFAULT_DECAY_FACTOR, DEFAULT_MAX_VALUE, DEFAULT_VALUE_CUTOFF};
use crate::matrix::{calculate_index_from_xy, Direction, Matrix2D};
use crate::{rect::Rect, vector2::Vector2};
use rayon::prelude::*;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct Model {
    pub base_matrix: Matrix2D,
    pub left_click_is_held_down: bool,
    pub modifier_matrix: Matrix2D,
    pub mouse_xy: Vector2<f32>,
    pub right_click_is_held_down: bool,
    pub window_rect: Rect<usize>,
}

impl Model {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(window_rect: Rect<usize>) -> Self {
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

    pub fn update(&mut self, frame_time: f32) {
        assert_eq!(self.base_matrix.len(), self.modifier_matrix.len(), "matrices should be identical length but they are not: base_matrix.len() == {}, modifier_matrix.len() == {}", self.base_matrix.len(), self.modifier_matrix.len());
        if self.left_click_is_held_down || self.right_click_is_held_down {
            let Vector2 { x, y } = self.mouse_xy;
            let (x, y) = (x.round() as usize, y.round() as usize);

            if self.window_rect.contains(x, y) {
                let index = calculate_index_from_xy(
                    x,
                    y,
                    self.window_rect.w() as usize,
                );
                // can't fail because we've already checked that coords are in bounds
                *self.base_matrix.get_mut(index).expect("invalid index") =
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
                    *value = (*value + *mod_value + (DEFAULT_DECAY_FACTOR * frame_time)).max(0.0);
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
