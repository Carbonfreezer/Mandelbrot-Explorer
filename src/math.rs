use rayon::prelude::*;
use macroquad::prelude::{Color, BLACK};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::viridis::get_color;

/// The maximum amount of iterations we want to do for a complex number in Mandelbbrot to check for divergence.
pub const MAX_ITER:u16 = 100;

/// Complex number used in Mandelbrot in double precision.
#[derive(Default,Debug,Clone)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}


impl ComplexNumber {
    /// Constructor.
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    /// Checks if we are already too large to continue iteration.
    fn is_too_large(&self) -> bool {
        self.real * self.real + self.imag * self.imag > 2.0 * 2.0
    }

    /// Does the next step on a complex number.
    fn next_step(&mut self, offset: &ComplexNumber) {
        (self.real, self.imag) = (self.real * self.real - self.imag * self.imag + offset.real,
                                  2.0 * self.real * self.imag + offset.imag);
    }

    /// Gets the amount of iterations we need till divergence.
    pub fn get_iteration_till_termination(&self) -> u16 {
        let mut iter = 0;
        let mut scan = ComplexNumber::default();
        while iter < MAX_ITER && !scan.is_too_large() {
            scan.next_step(self);
            iter += 1;
        }
        iter
    }

    pub fn add_into(&mut self, other: &ComplexNumber) {
        self.real += other.real;
        self.imag += other.imag;
    }
}


/// Generates an iteration field for the given complex number as a center and an extension given as a radius.
pub fn get_iteration_field(center: ComplexNumber, extension : f64) -> Vec<u16> {
    let window_height = WINDOW_HEIGHT as f64;
    let step_increment = extension / (window_height * 0.5);

    (0..WINDOW_WIDTH * WINDOW_HEIGHT).into_par_iter().map(|x| {
        let y_pos = x / WINDOW_WIDTH - WINDOW_HEIGHT / 2;
        let x_pos = x % WINDOW_WIDTH - WINDOW_WIDTH / 2;
        let mut scan = ComplexNumber::new(x_pos as f64 * step_increment, y_pos as f64 * step_increment);
        scan.add_into(&center);
        scan.get_iteration_till_termination()
    }).collect::<Vec<u16>>()
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
            0 => { r = v; g = t; b = p; }
            1 => { r = q; g = v; b = p; }
            2 => { r = p; g = v; b = t; }
            3 => { r = p; g = q; b = v; }
            4 => { r = t; g = p; b = v; }
            5 => { r = v; g = p; b = q; }
            _ => {}
        }
    }
    Color::new(r, g, b, 1.0)
}

/// The amount of complete cycles we do on the hue for the complete stretch.
const HUE_CYCLES : f32 = 5.0;
/// The light intensity we use on the color.
const COLOR_VALUE : f32 = 0.8;
/// The color saturation we use.
const COLOR_SATURATION : f32 = 0.7;

/// Takes a field with iterations and converts it into a color array.
pub fn generate_colors(in_field: &[u16]) -> Vec< Color> {
    in_field.par_iter().map( |i| {
        if *i == MAX_ITER {BLACK} else {
            let rel_val = (*i as f32 * HUE_CYCLES / MAX_ITER as f32).fract();
            hsv_to_rgb_color(rel_val, COLOR_SATURATION, COLOR_VALUE)
        }
    }).collect()
}


const WINDOW_STEP : i32 = 6;
const SAMPLE_SIZE : f32 = ((2 * WINDOW_STEP + 1) * (2 * WINDOW_STEP + 1)) as f32;

const MAX_DIST_SQ : f32 =  ((WINDOW_WIDTH / 2).pow(2) + (WINDOW_HEIGHT / 2).pow(2)) as f32;

pub fn get_focus_point(in_field: &[u16], extension: f64) -> ComplexNumber {
    let best_index = (0..WINDOW_WIDTH * WINDOW_HEIGHT)
        .into_par_iter()
        .map(|idx| {
            let x = idx % WINDOW_WIDTH;
            let y = idx / WINDOW_WIDTH;

            // Randbereich ausschließen
            if x < WINDOW_STEP || y < WINDOW_STEP
                || x >= WINDOW_WIDTH - WINDOW_STEP
                || y >= WINDOW_HEIGHT - WINDOW_STEP
            {
                return 0.0;
            }

            // Varianz im Fenster berechnen
            let (sum, sq_sum) = (-WINDOW_STEP..=WINDOW_STEP)
                .flat_map(|dx| (-WINDOW_STEP..=WINDOW_STEP).map(move |dy| (dx, dy)))
                .map(|(dx, dy)| {
                    in_field[(x + dx) as usize + ((y + dy) * WINDOW_WIDTH) as usize] as f32
                })
                .fold((0.0, 0.0), |(s, sq), v| (s + v, sq + v * v));

            let mean = sum / SAMPLE_SIZE;
            let variance = sq_sum / SAMPLE_SIZE - mean * mean;

            // Zentrum bevorzugen
            let dx = (x - WINDOW_WIDTH / 2) as f32;
            let dy = (y - WINDOW_HEIGHT / 2) as f32;
            let center_bias = 1.0 - 0.5 * (dx * dx + dy * dy) / MAX_DIST_SQ;

            variance * center_bias
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap()
        .0 as i32;

    let step = extension / (WINDOW_HEIGHT as f64 * 0.5);
    ComplexNumber::new(
        (best_index % WINDOW_WIDTH - WINDOW_WIDTH / 2) as f64 * step,
        (best_index / WINDOW_WIDTH - WINDOW_HEIGHT / 2) as f64 * step,
    )
}