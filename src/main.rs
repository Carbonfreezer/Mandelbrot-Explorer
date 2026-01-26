#![windows_subsystem = "windows"]

mod color_generation;
mod focus_system;
mod math;

use crate::color_generation::generate_colors;
use crate::focus_system::{FocusPointWithScore, StartPointForZoom};
use crate::math::{ComplexNumber, get_iteration_field};
use macroquad::prelude::*;
use macroquad::rand::srand;
use std::default::Default;

/// Width of the window in stand-alone mode.
const WINDOW_WIDTH: i32 = 1280;
/// Height of the window in stand-alone mode.
const WINDOW_HEIGHT: i32 = 720;

/// The radius at which we start using the autofocus.
const START_FOCUS_RADIUS: f64 = 0.05;

/// The radius where we start with.
const START_RADIUS: f64 = 1.5;

/// The scaling factor we have for in scaling per second.
const RADIUS_SCALING: f64 = 0.5;

/// Zoom-out speed multiplier (how fast we zoom out during transition).
const ZOOM_OUT_SPEED: f64 = 4.0;

/// Smooth time for panning between positions (in seconds).
const PAN_SMOOTH_TIME: f64 = 0.25;

/// The smooth time we use for the autofocus.
const FOCUS_SMOOTH_TIME: f64 = 1.25;

/// Threshold for considering the pan complete (in complex plane units).
const PAN_COMPLETE_THRESHOLD: f64 = 0.01;

/// Represents the current state of the zoom system.
enum ZoomState {
    /// The start zooming phase, where we do not follow a focus.
    StartZooming,
    /// Normal operation: zooming in and following focus.
    ZoomingInAndFollowing,
    /// Transitioning out: zooming back to BASE_RADIUS before jumping.
    ZoomingOut,
    /// Panning to new position at BASE_RADIUS before zooming in again.
    Panning,
}

/// Sets the windows name and the required size.
fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fullscreen = true;
    srand(miniquad::date::now() as _);
    show_mouse(false);

    let mut center = ComplexNumber::new(-0.5, 0.0);
    let mut radius = START_RADIUS;
    let mut velocity = (0.0, 0.0);
    let mut best_start_candidate = StartPointForZoom::prepare_start();
    let mut zoom_state = ZoomState::Panning;

    let mut image = Image::gen_image_color(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, BLANK);
    let texture = Texture2D::from_image(&image);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
				}
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            miniquad::window::set_fullscreen(fullscreen);
        }

        let delta_time = get_frame_time() as f64;
        let num_array = get_iteration_field(center, radius);

        // State machine logic
        match zoom_state {
            ZoomState::StartZooming => {
                radius *= RADIUS_SCALING.powf(delta_time);
                if radius <= START_FOCUS_RADIUS {
                    radius = START_FOCUS_RADIUS;
                    zoom_state = ZoomState::ZoomingInAndFollowing;
                }
            }
            ZoomState::ZoomingInAndFollowing => {
                // compute the target center we want to approach
                let focus = FocusPointWithScore::new(&num_array);
                let target_center =
                    focus.get_absolute_focus_in_complex_number_pane(center, radius);

                // smoothly move center towards target_center using the existing ComplexNumber smoothing
                center.smooth_damp_to(target_center, &mut velocity, FOCUS_SMOOTH_TIME, delta_time);

                // Check if we need to transition out
                if radius < 1e-13 {
                    velocity = (0.0, 0.0);
                    // In zooming out we search our new point.
                    best_start_candidate.reset_iteration();
                    zoom_state = ZoomState::ZoomingOut
                }
                radius *= RADIUS_SCALING.powf(delta_time);
            }
            ZoomState::ZoomingOut => {
                best_start_candidate.try_improve();
                radius *= RADIUS_SCALING.powf(-delta_time * ZOOM_OUT_SPEED);
                // Check if we've reached START_RADIUS
                if radius >= START_RADIUS {
                    radius = START_RADIUS;
                    zoom_state = ZoomState::Panning
                }
            }
            ZoomState::Panning => {
                // Smooth damp center towards next_center
                center.smooth_damp_to(
                    best_start_candidate.starting_point(),
                    &mut velocity,
                    PAN_SMOOTH_TIME,
                    delta_time,
                );

                let dist_sq = (center - best_start_candidate.starting_point()).sq_mag();
                if dist_sq < PAN_COMPLETE_THRESHOLD * PAN_COMPLETE_THRESHOLD {
                    center = best_start_candidate.starting_point();
                    zoom_state = ZoomState::StartZooming;
                }
            }
        };

        let color_array = generate_colors(&num_array);

        image.update(&color_array);
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
