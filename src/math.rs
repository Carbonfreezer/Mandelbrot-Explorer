//! Contains the real mandelbrot caclulations.

use crate::{PICTURE_HEIGHT, PICTURE_WIDTH, PRECISION};
use rayon::prelude::*;
use std::ops::{AddAssign, Sub};
use rug::Float;

/// The maximum amount of iterations we want to do for a complex number in Mandelbrot to check for divergence.
pub const MAX_ITER: u16 = 100;

/// Complex number used in Mandelbrot in double precision.
#[derive(Debug, Clone)]
pub struct ComplexNumber {
    pub real: Float,
    pub imag: Float,
}

impl ComplexNumber {
    /// Constructor.
    pub fn new(real: Float, imag: Float) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    /// Does the next step on a complex number and returns true if we still need to iterate.
    /// We change ourselves.
    fn next_step(&mut self, offset: &ComplexNumber) -> bool {
        let sq_real = self.real * self.real;
        let sq_imag = self.imag * self.imag;


        (self.real, self.imag) = (
            Float::with_val(PRECISION, &sq_real - &sq_imag) + offset.real,
            2.0 * self.real * self.imag + offset.imag,
        );
        sq_real + sq_imag < 4.0
    }

    /// Gets the amount of iterations we need till divergence.
    pub fn get_iteration_till_termination(&self) -> u16 {
        let mut iter = 0;
        let mut scan = ComplexNumber::new (Float::with_val(PRECISION, 0.0), Float::with_val(PRECISION, 0.0) );
        while iter < MAX_ITER && scan.next_step(&self) {
            iter += 1;
        }
        iter
    }

    /// Does a smooth damp with critical damped spring to a target complex number.
    pub fn smooth_damp_to(
        &mut self,
        target: &ComplexNumber,
        velocity: &mut (Float, Float),
        smooth_time: &Float,
        delta_time: &Float,
    ) {
        self.real = smooth_damp(
            &self.real,
            &target.real,
            &mut velocity.0,
            smooth_time,
            delta_time,
        );
        self.imag = smooth_damp(
            &self.imag,
            &target.imag,
            &mut velocity.1,
            smooth_time,
            delta_time,
        );
    }
}

impl AddAssign<ComplexNumber> for ComplexNumber {
    fn add_assign(&mut self, other: ComplexNumber) {
        self.real += other.real;
        self.imag += other.imag;
    }
}

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: ComplexNumber) -> Self::Output {
        ComplexNumber::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}

/// Generates an iteration field for the given complex number as a center and an extension given as a radius.
/// The window half height corresponds to the radius.
pub fn get_iteration_field(center: &ComplexNumber, extension: &Float) -> Vec<u16> {
    let window_height = Float::with_val(PRECISION, PICTURE_HEIGHT);
    let step_increment = extension / (window_height * 0.5);

    (0..PICTURE_WIDTH * PICTURE_HEIGHT)
        .into_par_iter()
        .map(|x| {
            let y_pos = x / PICTURE_WIDTH - PICTURE_HEIGHT / 2;
            let x_pos = x % PICTURE_WIDTH - PICTURE_WIDTH / 2;
            let mut scan =
                ComplexNumber::new(x_pos  * &step_increment, y_pos * &step_increment);
            scan += *center;
            scan.get_iteration_till_termination()
        })
        .collect::<Vec<u16>>()
}

/// Generic smooth damping function that works on a critically damped spring.
fn smooth_damp(
    current: &Float,
    target: &Float,
    current_velocity: &mut Float,
    smooth_time: &Float,
    delta_time: &Float,
) -> Float {
    let omega = Float::with_val(PRECISION, 2.0) / smooth_time;
    let exp = (-&omega * delta_time).exp();
    let change = current - target;

    let temp = (current_velocity + &omega * &change) * delta_time;
    *current_velocity = (*current_velocity - omega * temp) * exp;
    target + (change + temp) * exp
}
