//! Module is responsible for mapping the iteration field to a color value. It us using a cyclicle color map here.

use crate::effect_system::{create_low_pass_filtered_density_field, create_shading_field};
use crate::math::MAX_ITER;
use macroquad::color::{BLACK, Color, hsl_to_rgb};
use std::sync::LazyLock;

/// The amount of complete cycles we do on the hue for the complete stretch.
const HUE_CYCLES: f32 = 2.0;
/// The light intensity we use on the color.
const COLOR_VALUE: f32 = 0.8;
/// The color saturation we use.
const COLOR_SATURATION: f32 = 0.8;

/// The lookup table for all entries as static array.
static COLOR_ARRAY: LazyLock<Vec<Color>> = LazyLock::new(create_all_colors);

/// Helper function to build the lookup table.
fn create_all_colors() -> Vec<Color> {
    let mut vec: Vec<_> = (0..MAX_ITER)
        .map(|i| {
            let rel_val = (i as f32 * HUE_CYCLES / MAX_ITER as f32).fract();
            hsl_to_rgb(rel_val, COLOR_SATURATION, (i as f32 / MAX_ITER as f32) * COLOR_VALUE)
        })
        .collect();
   // vec.push(BLACK);
    vec.push( hsl_to_rgb(1.0, COLOR_SATURATION, COLOR_VALUE));
    vec
}



/// Takes a field with iterations and converts it into a color array.
pub fn generate_colors(in_field: &[u16]) -> Vec<Color> {
    let low_pass = create_low_pass_filtered_density_field(in_field);
    let shading = create_shading_field(&low_pass);

    in_field
        .iter()
        .zip(shading)
        .map(|(i, luminance)| {
            let mut color = COLOR_ARRAY[*i as usize];
            color.r *= luminance;
            color.g *= luminance;
            color.b *= luminance;
            color
        })
        .collect()
}
