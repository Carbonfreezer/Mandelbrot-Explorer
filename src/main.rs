#![windows_subsystem = "windows"]

mod math;

use macroquad::prelude::*;
use crate::math::{generate_colors, get_focus_point, get_iteration_field, smooth_damp, ComplexNumber};

/// Width of the window in stand-alone mode.
const WINDOW_WIDTH: i32 = 1280;
/// Height of the window in stand-alone mode.
const WINDOW_HEIGHT: i32 = 720;

/// Sets the windows name and the required size.
fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbort".to_owned(),
        window_width: WINDOW_WIDTH ,
        window_height: WINDOW_HEIGHT,
        // fullscreen: true,
        ..Default::default()
    }
}

const SMOOTH_TIME: f32 = 1.0;

#[macroquad::main(window_conf)]
async fn main() {

    // let mut center = ComplexNumber::new(-0.9, 0.3); // Meine
    let mut center = ComplexNumber::new(-1.4, 0.0); // Feigenbaum Punkt


    let radius_scaling: f64 = 0.5;
    let mut radius: f64 = 0.1;
    let mut velocity = (0.0, 0.0);

    let mut image = Image::gen_image_color(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, BLANK);
    let texture = Texture2D::from_image(&image);



    loop {
        let delta_time = get_frame_time();
        radius *= radius_scaling.powf(delta_time as f64);

        clear_background(BLACK);

        let num_array = get_iteration_field(center.clone(), radius);

        let mut focus  = get_focus_point(&num_array);
        focus.0 = smooth_damp(0.0, focus.0, &mut velocity.0, SMOOTH_TIME, delta_time);
        focus.1 = smooth_damp(0.0, focus.1, &mut velocity.1, SMOOTH_TIME, delta_time);
        let step = radius / (WINDOW_HEIGHT as f64 * 0.5);

        center.real += focus.0 as f64 * step;
        center.imag += focus.1 as f64 * step;

        let color_array = generate_colors(&num_array);

        image.update(&color_array);
        texture.update(&image);

        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            ..Default::default()
        });


        let time_str = format!("Zeit: {:.2}s  Radius: {:.2e}", delta_time, radius);
        // let time_str = format!("Zeit: {:.2}s", delta_time);
        draw_text(&time_str, 20.0, 50.0, 30.0, WHITE);

        next_frame().await;
    }
}