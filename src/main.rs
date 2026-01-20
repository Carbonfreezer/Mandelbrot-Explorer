#![windows_subsystem = "windows"]

mod color_generation;
mod focus_system;
mod math;

use crate::color_generation::generate_colors;
use crate::focus_system::get_focus_point;
use crate::math::{ComplexNumber, get_iteration_field};
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

/// Width of the window in stand-alone mode.
const WINDOW_WIDTH: i32 = 1280;
/// Height of the window in stand-alone mode.
const WINDOW_HEIGHT: i32 = 720;

/// The score we minimally want to get as a starting position.
const START_SCORE: f32 = 800.0;

/// The radius at which we start zooming and to which we zoom out.
const BASE_RADIUS: f64 = 0.2;

/// The scaling factor we have for in scaling per second.
const RADIUS_SCALING: f64 = 0.5;

/// Zoom-out speed multiplier (how fast we zoom out during transition).
const ZOOM_OUT_SPEED: f64 = 4.0;

/// Smooth time for panning between positions (in seconds).
const PAN_SMOOTH_TIME: f32 = 0.5;

/// The smooth time we use for the autofocus.
const SMOOTH_TIME: f32 = 1.25;


/// Threshold for considering the pan complete (in complex plane units).
const PAN_COMPLETE_THRESHOLD: f64 = 0.001;

/// Represents the current state of the zoom system.
enum ZoomState {
    /// Normal operation: zooming in and following focus.
    ZoomingIn,
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

/// Finds a suitable random starting position with good variance score.
fn find_interesting_start() -> ComplexNumber {
    let mut iter_count = 0;
    loop {
        iter_count += 1;
        if iter_count == 20 {
            break ComplexNumber::new(-1.4, 0.0);
        }
        let test = ComplexNumber::new(gen_range(-2.0, 1.0), gen_range(-1.0, 1.0));
        let num_array = get_iteration_field(&test, BASE_RADIUS);
        let value = get_focus_point(&num_array).score;
        if value > START_SCORE {
            break test;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    srand(miniquad::date::now() as _);

    let mut center = find_interesting_start();
    let mut radius = BASE_RADIUS;
    let mut velocity = (0.0, 0.0);
    let mut next_center = ComplexNumber::new(0.0, 0.0);
    let mut zoom_state = ZoomState::ZoomingIn;

    let mut image = Image::gen_image_color(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, BLANK);
    let texture = Texture2D::from_image(&image);

    loop {
        let delta_time = get_frame_time();
        let num_array = get_iteration_field(&center, radius);

        // State machine logic
        match zoom_state {
            ZoomState::ZoomingIn => {
                // compute the target center we want to approach
                let focus = get_focus_point(&num_array);
                let step = radius / (WINDOW_HEIGHT as f64 * 0.5);
                let target_center = ComplexNumber::new(
                    center.real + focus.x_pos as f64 * step,
                    center.imag + focus.y_pos as f64 * step,
                );

                // smoothly move center towards target_center using the existing ComplexNumber smoothing
                center.smooth_damp_to(&target_center, &mut velocity, SMOOTH_TIME, delta_time);

                // Check if we need to transition out
                if radius < 1e-13  {
                    next_center = find_interesting_start();
                    velocity = (0.0, 0.0);
                    zoom_state = ZoomState::ZoomingOut
                }
                radius *= RADIUS_SCALING.powf(delta_time as f64);
            }
            ZoomState::ZoomingOut  => {
                // Check if we've reached BASE_RADIUS
                if radius >= BASE_RADIUS {
                    radius = BASE_RADIUS;
                    zoom_state = ZoomState::Panning
                }
                else {
                    radius *= RADIUS_SCALING.powf(-delta_time as f64 * ZOOM_OUT_SPEED);
                }
            }
            ZoomState::Panning  => {
                // Smooth damp center towards next_center
                center.smooth_damp_to(&next_center, &mut velocity, PAN_SMOOTH_TIME, delta_time);

                let dist_sq = (&center - &next_center).sq_mag();
                if dist_sq < PAN_COMPLETE_THRESHOLD * PAN_COMPLETE_THRESHOLD {
                    center = next_center.clone();
                    zoom_state = ZoomState::ZoomingIn
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
