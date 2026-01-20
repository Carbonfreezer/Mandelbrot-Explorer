//! Module is responsible for mapping the iteration field to a color value. It us using a cyclicle color map here.

use crate::math::MAX_ITER;
use macroquad::color::{BLACK, Color};
use std::sync::LazyLock;

/// The amount of complete cycles we do on the hue for the complete stretch.
const HUE_CYCLES: f32 = 10.0;
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
            hsv_to_rgb_color(rel_val, COLOR_SATURATION, COLOR_VALUE)
        })
        .collect();
    vec.push(BLACK);
    vec
}

/// Converts hsv to rgb color.
fn hsv_to_rgb_color(h: f32, s: f32, v: f32) -> Color {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if s == 0.0 {
        r = v;
        g = v;
        b = v;
    } else {
        let h_i = (h * 6.0).floor();
        let f = h * 6.0 - h_i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match h_i as i32 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => {}
        }
    }
    Color::new(r, g, b, 1.0)
}

/// Takes a field with iterations and converts it into a color array.
pub fn generate_colors(in_field: &[u16]) -> Vec<Color> {
    in_field.iter().map(|i| COLOR_ARRAY[*i as usize]).collect()
}
