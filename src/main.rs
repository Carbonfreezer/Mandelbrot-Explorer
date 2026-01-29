#![windows_subsystem = "windows"]

mod color_generation;
mod focus_system;
mod math;

use crate::color_generation::generate_colors;
use crate::focus_system::FocusPoint;
use crate::math::{ComplexNumber, get_iteration_field};
use png::Encoder;
use std::fs::File;
use std::io::BufWriter;
use std::sync::LazyLock;

/// Width of the window in stand-alone mode.
const PICTURE_WIDTH: i32 = 1920;
/// Height of the window in stand-alone mode.
const PICTURE_HEIGHT: i32 = 1080;

/// The amount of scaling we want to have per step.
static START_RADIUS_SCALING_PER_STEP: LazyLock<f64> =
    LazyLock::new(|| (0.5_f32).powf(DELTA_TIME as f32) as f64);

/// The scaling factor we have for in scaling per second.
const START_RADIUS: f64 = 0.05;

/// The smooth time we use for the autofocus.
const FOCUS_SMOOTH_TIME: f64 = 1.25;

/// The amount of fps we want to have in the video.
const FPS: f64 = 50.0;

/// The delta time we have for every frame.
const DELTA_TIME: f64 = 1.0 / FPS;

fn main() {
    let mut center = ComplexNumber::new(-0.75, 0.11);
    let mut radius = START_RADIUS;
    let mut velocity = (0.0, 0.0);

    let mut frame_counter: u32 = 0;

    loop {
        let num_array = get_iteration_field(center, radius);

        // compute the target center we want to approach
        let focus = FocusPoint::new(&num_array);
        let target_center = focus.get_absolute_focus_in_complex_number_pane(center, radius);

        // smoothly move center towards target_center using the existing ComplexNumber smoothing
        center.smooth_damp_to(target_center, &mut velocity, FOCUS_SMOOTH_TIME, DELTA_TIME);

        let color_array = generate_colors(&num_array);
        save_image(&color_array, frame_counter);

        // Check if we reach the limit of precision. Later on we have to switch the precision level here.
        if radius < 1e-13 {
            break;
        }
        radius *= *START_RADIUS_SCALING_PER_STEP;
        frame_counter += 1;
    }
}

fn save_image(color_vec: &[u8], serial_number: u32) {
    let path = format!("Image_{:06}", serial_number);
    let file = File::create(path).expect("Failed to create image file");
    let writer = BufWriter::new(file);

    let mut encoder = Encoder::new(writer, PICTURE_WIDTH as u32, PICTURE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder
        .write_header()
        .expect("Failed to write image header");
    writer
        .write_image_data(color_vec)
        .expect("Failed to write image data");
}
