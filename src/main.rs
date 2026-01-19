#![windows_subsystem = "windows"]

mod math;

use macroquad::prelude::{clear_background, draw_text, draw_texture, get_frame_time, next_frame, Conf, Image, Texture2D, BLACK, BLANK, GREEN, WHITE};
use crate::math::{convert_iteration_array, get_iteration_field, ComplexNumber};

/// Width of the window in stand-alone mode.
const WINDOW_WIDTH: i32 = 1280;
/// Height of the window in stand-alone mode.
const WINDOW_HEIGHT: i32 = 1024;

/// Sets the windows name and the required size.
fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbort".to_owned(),
        window_width: WINDOW_WIDTH ,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let center = ComplexNumber::new(-1.0, 0.0);
    let radius = 2.0;

    let mut image = Image::gen_image_color(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, BLANK);
    let texture = Texture2D::from_image(&image);



    loop {
        let delta_time = get_frame_time();

        clear_background(BLACK);

        let num_array = get_iteration_field(center.clone(), radius);
        let color_array = convert_iteration_array(&num_array);

        image.update(&color_array);
        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        let time_str = format!("Zeit: {:.2}s", delta_time);
        draw_text(&time_str, 20.0, 50.0, 30.0, GREEN);

        next_frame().await;
    }
}