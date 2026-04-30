//! Module is responsible for mapping the iteration field to a color value. It us using a cyclicle color map here.

use crate::effect_system::{create_low_pass_filtered_density_field, create_shading_field};
use crate::math::MAX_ITER;
use colorous::MAGMA;
use macroquad::color::Color;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

/// Takes a field with iterations and converts it into a color array.
/// It colorizes the iteration field with a magma color scale. It diminishes the color with
/// increasing distance and takes the shading effects into account.
pub fn generate_colors(in_field: &[f32]) -> Vec<Color> {
    let low_pass = create_low_pass_filtered_density_field(in_field);
    let shading = create_shading_field(&low_pass);

    in_field
        .par_iter()
        .zip(shading.par_iter())
        .map(|(l, r)| {
            let t = (l / MAX_ITER as f32).sqrt() as f64;
            let c = MAGMA.eval_continuous(t);
            let brightness = r * f32::exp(-0.5 * (MAX_ITER as f32 - l) / MAX_ITER as f32);
            Color::new(
                brightness * c.r as f32 / 255.0,
                brightness * c.g as f32 / 255.0,
                brightness * c.b as f32 / 255.0,
                1.0,
            )
        })
        .collect()
}
