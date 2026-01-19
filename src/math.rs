use rayon::prelude::*;
use macroquad::prelude::{Color};
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
            scan.next_step(&self);
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



/// Takes a field with iterations and converts it into a color array.
pub fn convert_iteration_array(in_field: &[u16]) -> Vec< Color> {
    in_field.par_iter().map( |i| { get_color(*i) }).collect()
}