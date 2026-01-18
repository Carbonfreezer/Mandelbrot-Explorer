#![windows_subsystem = "windows"]

use macroquad::prelude::{clear_background, get_frame_time, next_frame, Conf, BLACK};

/// Width of the window in stand-alone mode.
const WINDOW_WIDTH: i32 = 1280;
/// Height of the window in stand-alone mode.
const WINDOW_HEIGHT: i32 = 1024;

/// Sets the windows name and the required size.
fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbort".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        let delta_time = get_frame_time();

        clear_background(BLACK);


        next_frame().await;
    }
}