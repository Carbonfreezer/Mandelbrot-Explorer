//! Contains the real mandelbrot caclulations.

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use rayon::prelude::*;
use num_traits::Float;

/// The maximum amount of iterations we want to do for a complex number in Mandelbbrot to check for divergence.
pub const MAX_ITER: u16 = 100;

/// Complex number used in Mandelbrot in double precision.
#[derive(Default, Debug, Clone)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    /// Constructor.
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    /// Does the next step on a complex number and returns true if we still need to iterate.
    fn next_step(&mut self, offset: &ComplexNumber) -> bool {
        let sq_real = self.real * self.real;
        let sq_imag = self.imag * self.imag;
        (self.real, self.imag) = (
            sq_real - sq_imag + offset.real,
            2.0 * self.real * self.imag + offset.imag,
        );
        sq_real + sq_imag < 4.0
    }

    /// Gets the amount of iterations we need till divergence.
    pub fn get_iteration_till_termination(&self) -> u16 {
        let mut iter = 0;
        let mut scan = ComplexNumber::default();
        while iter < MAX_ITER && scan.next_step(&self) {
            iter += 1;
        }
        iter
    }

    /// Adds another complex number into the current one.
    pub fn add_into(&mut self, other: &ComplexNumber) {
        self.real += other.real;
        self.imag += other.imag;
    }

    /// Does a smooth damp with critical damped spring to a target complex number.
    pub fn smooth_damp_to(&mut self, target: &ComplexNumber, velocity : &mut (f64, f64), smooth_time: f32, delta_time: f32) {
        self.real = smooth_damp(self.real, target.real, &mut velocity.0, smooth_time as f64, delta_time as f64);
        self.imag = smooth_damp(self.imag, target.imag, &mut velocity.0, smooth_time as f64, delta_time as f64);
    }
}

/// Generates an iteration field for the given complex number as a center and an extension given as a radius.
pub fn get_iteration_field(center: ComplexNumber, extension: f64) -> Vec<u16> {
    let window_height = WINDOW_HEIGHT as f64;
    let step_increment = extension / (window_height * 0.5);

    (0..WINDOW_WIDTH * WINDOW_HEIGHT)
        .into_par_iter()
        .map(|x| {
            let y_pos = x / WINDOW_WIDTH - WINDOW_HEIGHT / 2;
            let x_pos = x % WINDOW_WIDTH - WINDOW_WIDTH / 2;
            let mut scan =
                ComplexNumber::new(x_pos as f64 * step_increment, y_pos as f64 * step_increment);
            scan.add_into(&center);
            scan.get_iteration_till_termination()
        })
        .collect::<Vec<u16>>()
}


/// Generic smooth damping function that works on a critically damped spring.
/// Works with both f32 and f64.
pub fn smooth_damp<T: Float>(
    current: T,
    target: T,
    current_velocity: &mut T,
    smooth_time: T,
    delta_time: T,
) -> T {
    let zero = T::zero();
    let two = T::from(2.0).unwrap();
    let min_smooth = T::from(0.0001).unwrap();

    // Sicherstellen, dass smooth_time nicht 0 ist
    let smooth_time = if smooth_time < min_smooth {
        min_smooth
    } else {
        smooth_time
    };

    let omega = two / smooth_time;
    let exp = (-omega * delta_time).exp();
    let change = current - target;

    let temp = (*current_velocity + omega * change) * delta_time;
    *current_velocity = (*current_velocity - omega * temp) * exp;

    let mut output = target + (change + temp) * exp;

    // Über-shooting verhindern
    if (target - current > zero) == (output > target) {
        output = target;
        *current_velocity = zero;
    }

    output
}
