//! This module takes care of the effect and lighting calculation. It computes a low pass filtered
//! Iterations field and from this on it generates a pseudo shading and a fog effect.

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use rayon::prelude::*;
use std::f32::consts::PI;
use std::sync::LazyLock;

/// The half size we have for the convolution kernel..
const CONV_OFFSET: i32 = 5;
const CONV_SIZE: usize = 2 * CONV_OFFSET as usize + 1;

/// The sigma for the loe pass filter.
const SIGMA: f32 = 1.0;

/// The ambient lighting we use for shading.
const AMBIENT: f32 = 0.1;

const STEEP_FACTOR: f32 = 3.0;

static LOW_PASS_KERNEL: LazyLock<Vec<f32>> = LazyLock::new(generate_convolution_kernel);

fn generate_convolution_kernel() -> Vec<f32> {
    (-CONV_OFFSET..=CONV_OFFSET)
        .flat_map(|y| {
            (-CONV_OFFSET..=CONV_OFFSET).map(move |x| {
                1.0 / (2.0 * PI * SIGMA * SIGMA)
                    * f32::exp(-(x * x + y * y) as f32 / (2.0 * SIGMA * SIGMA))
            })
        })
        .collect()
}

pub fn create_low_pass_filtered_density_field(iteration_field: &[f32]) -> Vec<f32> {
    (0..WINDOW_WIDTH * WINDOW_HEIGHT)
        .into_par_iter()
        .map(|i| {
            let y_pos = i / WINDOW_WIDTH;
            let x_pos = i % WINDOW_WIDTH;
            (-CONV_OFFSET..=CONV_OFFSET)
                .flat_map(|dy| {
                    (-CONV_OFFSET..=CONV_OFFSET).map(move |dx| {
                        let source_x = x_pos + dx;
                        let source_y = y_pos + dy;

                        if (source_x < 0) || (source_y < 0) || (source_x >= WINDOW_WIDTH) || (source_y >= WINDOW_HEIGHT) {
                            0.0
                        }
                        else {
                            let source_index = (source_x + source_y * WINDOW_WIDTH) as usize;
                            let kernel_index =
                                (dx + CONV_OFFSET) as usize + (dy + CONV_OFFSET) as usize * CONV_SIZE;
                            iteration_field[source_index]  * LOW_PASS_KERNEL[kernel_index]
                        }

                    })
                })
                .sum::<f32>()
        })
        .collect()
}

pub fn create_shading_field(in_field: &[f32]) -> Vec<f32> {
    (0..WINDOW_WIDTH * WINDOW_HEIGHT)
        .into_par_iter()
        .map(|i| {
            let y_pos = i / WINDOW_WIDTH;
            let x_pos = i % WINDOW_WIDTH;

            let mut light = AMBIENT;

            if (x_pos != 0)
                && (y_pos != 0)
                && (x_pos != WINDOW_WIDTH - 1)
                && (y_pos != WINDOW_HEIGHT - 1)
            {
                let grad_x = 0.5
                    * (in_field[(x_pos + 1 + y_pos * WINDOW_WIDTH) as usize]
                        - in_field[(x_pos - 1 + y_pos * WINDOW_WIDTH) as usize]);
                let grad_y = 0.5
                    * (in_field[(x_pos + (y_pos + 1) * WINDOW_WIDTH) as usize]
                        - in_field[(x_pos + (y_pos - 1) * WINDOW_WIDTH) as usize]);

                light += STEEP_FACTOR
                    / f32::sqrt(grad_x * grad_x + grad_y * grad_y + STEEP_FACTOR * STEEP_FACTOR)
                    * (1.0 - AMBIENT);
            }
            light
        })
        .collect()
}
