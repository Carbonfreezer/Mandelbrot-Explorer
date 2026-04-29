//! Module is responsible for mapping the iteration field to a color value. It us using a cyclicle color map here.

use rayon::iter::ParallelIterator;
use rayon::iter::IndexedParallelIterator;
use crate::effect_system::{create_low_pass_filtered_density_field, create_shading_field};
use crate::math::MAX_ITER;
use macroquad::color::{Color, hsl_to_rgb};
use rayon::prelude::IntoParallelRefIterator;

/// The amount of complete cycles we do on the hue for the complete stretch.
const HUE_CYCLES: f32 = 1.0;
/// The light intensity we use on the color.
const COLOR_VALUE: f32 = 0.8;
/// The color saturation we use.
const COLOR_SATURATION: f32 = 0.8;




/// Takes a field with iterations and converts it into a color array.
pub fn generate_colors(in_field: &[f32]) -> Vec<Color> {
    let low_pass = create_low_pass_filtered_density_field(in_field);
    let shading = create_shading_field(&low_pass);


    in_field.par_iter().zip(shading.par_iter()).map(|(l, r)|
        {
            let rel_val = (l *HUE_CYCLES / MAX_ITER as f32).fract();
            hsl_to_rgb(rel_val, COLOR_SATURATION, r * COLOR_VALUE * f32::exp(- 1.5 *  (MAX_ITER as f32 - l) / MAX_ITER as f32) )
        }).collect()

}
