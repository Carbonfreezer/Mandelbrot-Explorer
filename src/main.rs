//! This program makes use of the rug number crate for arbitrary large number (especially floats)
//! I got this only running on linux. To get it compiled you will also need the following components:
//! ```bash
//! sudo apt update
//! sudo apt install libgmp-dev libmpfr-dev
//! sudo apt install build-essential m4
//! ```

mod color_generation;
mod focus_system;
mod math;
mod precision;

use crate::color_generation::generate_colors;
use crate::focus_system::FocusPoint;
use crate::math::{ComplexNumber, get_iteration_field};
use crate::precision::DYNAMIC_PRECISION;
use png::Encoder;
use rug::Float;
use std::fs::File;
use std::io::BufWriter;

/// Width of the window in stand-alone mode.
const PICTURE_WIDTH: i32 = 1920;
/// Height of the window in stand-alone mode.
const PICTURE_HEIGHT: i32 = 1080;

/// The scaling factor we have for in scaling per second.
const START_RADIUS: f64 = 0.05;

/// The smooth time we use for the autofocus.
const FOCUS_SMOOTH_TIME: f64 = 1.25;

/// The amount of fps we want to have in the video.
const FPS: f64 = 50.0;

/// The delta time we have for every frame.
const DELTA_TIME: f64 = 1.0 / FPS;

/// The maximum number of precision we use.
const MAX_PRECISION: u32 = 64;

/// The delta we use for a precision increment in bits.
const PRECISION_INCREMENT: u32 = 8;

/// This contains all the data in the main program that have to change with dynamic precision.
struct PrecisionChangingData {
    /// The current center of the window in the complex number pane.
    center: ComplexNumber,
    /// The current velocity of the center point in complex number pane for smooth damp.
    velocity: (Float, Float),
    /// The radius in complex number pane used for half window height.
    radius: Float,
    /// The radius where we have to change precision again.
    min_radius: f64,
    /// The time between two frames in the correct precision representation.
    delta_time: Float,
    /// The smooth damp time for the focus point.
    focus_smooth_time: Float,
}

impl PrecisionChangingData {
    /// Initializes everything with the start values.
    pub fn new() -> Self {
        Self {
            center: ComplexNumber::new(float!(-0.75), float!(0.11)),
            velocity: (float!(0.0), float!(0.0)),
            radius: float!(START_RADIUS),
            min_radius: 10_f64.powf(-((precision!() - 10) as f64 * 0.3)),
            delta_time: float!(DELTA_TIME),
            focus_smooth_time: float!(FOCUS_SMOOTH_TIME),
        }
    }

    /// Checks if we need to upgrade the precision and eventually does so, all relevant data will get upgraded to the new precision.
    /// 
    /// #unsafe
    /// We manipulate the global variable DYNAMIC_PRECISION here, assuming that all threads are not accessing here. 
    pub fn check_upgrade_precision(&mut self) -> bool {
        if self.radius >= self.min_radius {
            return false;
        }

        let current_precision = precision!();
        if current_precision >= MAX_PRECISION {
            return true;
        }

        {
            unsafe {
                DYNAMIC_PRECISION = current_precision + PRECISION_INCREMENT;
            }
        }
        self.center = ComplexNumber::new(float!(&self.center.real), float!(&self.center.imag));
        self.velocity = (float!(&self.velocity.0), float!(&self.velocity.1));
        self.radius = float!(&self.radius);
        self.min_radius = 10_f64.powf(-((precision!() - 10) as f64 * 0.3));
        self.delta_time = float!(DELTA_TIME);
        self.focus_smooth_time = float!(FOCUS_SMOOTH_TIME);

        false
    }
}

fn main() {
    let start_radius_scaling_per_step = 0.5_f64.powf(DELTA_TIME);
    let mut data = PrecisionChangingData::new();
    let mut frame_counter: u32 = 0;

    loop {
        let num_array = get_iteration_field(&data.center, &data.radius);

        // compute the target center we want to approach
        let focus = FocusPoint::new(&num_array);
        let target_center =
            focus.get_absolute_focus_in_complex_number_pane(&data.center, &data.radius);

        // smoothly move center towards target_center using the existing ComplexNumber smoothing
        data.center.smooth_damp_to(
            &target_center,
            &mut data.velocity,
            &data.focus_smooth_time,
            &data.delta_time,
        );

        let color_array = generate_colors(&num_array);
        save_image(&color_array, frame_counter);

        // Check if we reach the limit of precision. Later on we have to switch the precision level here.
        if data.check_upgrade_precision() {
            break;
        }
        data.radius *= start_radius_scaling_per_step;
        frame_counter += 1;
    }
}

/// Generates an image file to be saved to disc from the serial number.
/// All image files can then be accumulated into one video with
/// ```bash
/// ffmpeg -framerate 50 -i Image_%06d.png -c:v libx264 -crf 18 -preset slow -pix_fmt yuv420p fractal.mp4
/// ```
fn save_image(color_vec: &[u8], serial_number: u32) {
    let path = format!("Image_{:06}.png", serial_number);
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
