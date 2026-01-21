//! The focus system searches for interesting spots based on variance.

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use itertools::Itertools;
use rayon::iter::*;
use crate::math::ComplexNumber;

/// The window size we use for variance calculation is this size * 2 + 1
const WINDOW_STEP: i32 = 5;
/// The amount of samples we generate in the window.
const SAMPLE_SIZE: f32 = ((2 * WINDOW_STEP + 1) * (2 * WINDOW_STEP + 1)) as f32;
/// The maximum distance a pixel can be away from the center squared.
const MAX_DIST_SQ: f32 = ((WINDOW_WIDTH / 2).pow(2) + (WINDOW_HEIGHT / 2).pow(2)) as f32;

/// Contains a point to focus on with an evaluation-
pub struct FocusPointWithScore {
    /// Contains the x position of the focus-point in screen space pixel coordinates.
    x_pos: f32,
    /// Contains the y position of the focus-point in screen space pixel coordinates.
    y_pos: f32,
    /// The score we have for the point..
    score: f32,
}

impl FocusPointWithScore {

    /// Gets a focus point (including score) from the iteration field handed over.
    pub fn new(in_field: &[u16]) -> FocusPointWithScore {
        let (best_index, score) = (0..WINDOW_WIDTH * WINDOW_HEIGHT)
            .into_par_iter()
            .map(|idx| {
                let x = idx % WINDOW_WIDTH;
                let y = idx / WINDOW_WIDTH;

                // Exclude border stripe.
                if x < WINDOW_STEP
                    || y < WINDOW_STEP
                    || x >= WINDOW_WIDTH - WINDOW_STEP
                    || y >= WINDOW_HEIGHT - WINDOW_STEP
                {
                    return 0.0;
                }

                // Calculate variance in window.
                let (sum, sq_sum) = (-WINDOW_STEP..=WINDOW_STEP)
                    .cartesian_product(-WINDOW_STEP..=WINDOW_STEP)
                    .map(|(dx, dy)| {
                        in_field[(x + dx) as usize + ((y + dy) * WINDOW_WIDTH) as usize] as f32
                    })
                    .fold((0.0, 0.0), |(s, sq), v| (s + v, sq + v * v));

                let mean = sum / SAMPLE_SIZE;
                let variance = sq_sum / SAMPLE_SIZE - mean * mean;

                // Get center bias.
                let dx = (x - WINDOW_WIDTH / 2) as f32;
                let dy = (y - WINDOW_HEIGHT / 2) as f32;
                let center_bias = 1.0 - 0.5 * (dx * dx + dy * dy) / MAX_DIST_SQ;

                variance * center_bias
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        let best_index = best_index as i32;

        FocusPointWithScore {
            x_pos: (best_index % WINDOW_WIDTH - WINDOW_WIDTH / 2) as f32,
            y_pos: (best_index / WINDOW_WIDTH - WINDOW_HEIGHT / 2) as f32,
            score,
        }
    }

    /// Given a screen center in the complex number pane and an applied radius the focus gets converted into a target position in the complex number pane.
    pub fn get_absolute_focus_in_complex_number_pane(&self, center : &ComplexNumber, radius : f64) -> ComplexNumber {
        let step = radius / (WINDOW_HEIGHT as f64 * 0.5);
         ComplexNumber::new(
            center.real + self.x_pos as f64 * step,
            center.imag + self.y_pos as f64 * step,
        )
    }


    pub fn score(&self) -> f32 {self.score}
}
