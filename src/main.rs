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

/// The score we minimally want till we terminate.
const MIN_SCORE: f32 = 150.0;

/// The radius at which we start zooming and to which we zoom out.
const BASE_RADIUS: f64 = 0.2;

/// The scaling factor we have for in scaling per second.
const RADIUS_SCALING: f64 = 0.5;

/// Zoom-out speed multiplier (how fast we zoom out during transition).
const ZOOM_OUT_SPEED: f64 = 4.0;

/// Smooth time for panning between positions (in seconds).
const PAN_SMOOTH_TIME: f32 = 0.5;

/// Threshold for considering the pan complete (in complex plane units).
const PAN_COMPLETE_THRESHOLD: f64 = 0.001;

/// Represents the current state of the zoom system.
enum ZoomState {
    /// Normal operation: zooming in and following focus.
    ZoomingIn,
    /// Transitioning out: zooming back to BASE_RADIUS before jumping.
    ZoomingOut { next_center: ComplexNumber },
    /// Panning to new position at BASE_RADIUS before zooming in again.
    Panning {
        next_center: ComplexNumber,
        velocity: (f64, f64),
    },
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
    let mut zoom_state = ZoomState::ZoomingIn;

    let mut image = Image::gen_image_color(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, BLANK);
    let texture = Texture2D::from_image(&image);

    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);

        // Update radius based on current state
        match &zoom_state {
            ZoomState::ZoomingIn => {
                radius *= RADIUS_SCALING.powf(delta_time as f64);
            }
            ZoomState::ZoomingOut { .. } => {
                radius *= RADIUS_SCALING.powf(-delta_time as f64 * ZOOM_OUT_SPEED);
            }
            ZoomState::Panning { .. } => {
                // Hold radius constant during panning
            }
        }

        let num_array = get_iteration_field(&center, radius);
        let mut focus = get_focus_point(&num_array);

        // State machine logic
        zoom_state = match zoom_state {
            ZoomState::ZoomingIn => {
                // Apply smooth damping and follow focus
                focus.smooth_damp(&mut velocity, delta_time);

                let step = radius / (WINDOW_HEIGHT as f64 * 0.5);
                center.real += focus.x_pos as f64 * step;
                center.imag += focus.y_pos as f64 * step;

                // Check if we need to transition out
                if radius < 1e-13 || focus.score < MIN_SCORE {
                    let next_center = find_interesting_start();
                    velocity = (0.0, 0.0);
                    ZoomState::ZoomingOut { next_center }
                } else {
                    ZoomState::ZoomingIn
                }
            }
            ZoomState::ZoomingOut { next_center } => {
                // Check if we've reached BASE_RADIUS
                if radius >= BASE_RADIUS {
                    radius = BASE_RADIUS;
                    ZoomState::Panning {
                        next_center,
                        velocity: (0.0, 0.0),
                    }
                } else {
                    ZoomState::ZoomingOut { next_center }
                }
            }
            ZoomState::Panning {
                next_center,
                mut velocity,
            } => {
                // Smooth damp center towards next_center
                center.smooth_damp_to(&next_center, &mut velocity, PAN_SMOOTH_TIME, delta_time);

                let dist_sq = (&center - &next_center).sq_mag();
                if dist_sq < PAN_COMPLETE_THRESHOLD * PAN_COMPLETE_THRESHOLD {
                    center = next_center;
                    ZoomState::ZoomingIn
                } else {
                    ZoomState::Panning {
                        next_center,
                        velocity,
                    }
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
