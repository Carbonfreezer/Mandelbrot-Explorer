//! The focus system searches for interesting spots based on variance.

use rayon::iter::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const WINDOW_STEP : i32 = 5;
const SAMPLE_SIZE : f32 = ((2 * WINDOW_STEP + 1) * (2 * WINDOW_STEP + 1)) as f32;

const MAX_DIST_SQ : f32 =  ((WINDOW_WIDTH / 2).pow(2) + (WINDOW_HEIGHT / 2).pow(2)) as f32;

/// Contains a point to focus on with an evaluation-
pub struct FocusPointWithScore {
    pub x_pos: f32,
    pub y_pos: f32,
    pub score: f32,
}
const SMOOTH_TIME: f32 = 1.25;
impl FocusPointWithScore {
    /// Makes the origin gravitate towards the focus point.
    pub fn smooth_damp(&mut self, velocity : &mut (f32, f32), delta_time : f32) {
        self.x_pos = smooth_damp(0.0, self.x_pos, &mut velocity.0, SMOOTH_TIME, delta_time);
        self.y_pos = smooth_damp(0.0, self.y_pos, &mut velocity.1, SMOOTH_TIME, delta_time);
    }
}


/// Gets a focus point (including score) from the iteration field handed over.
pub fn get_focus_point(in_field: &[u16]) -> FocusPointWithScore {
    let (best_index, score) = (0..WINDOW_WIDTH * WINDOW_HEIGHT)
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
        .unwrap();

    let best_index = best_index as i32;

    FocusPointWithScore { x_pos : (best_index % WINDOW_WIDTH - WINDOW_WIDTH / 2) as f32,
        y_pos: (best_index / WINDOW_WIDTH - WINDOW_HEIGHT / 2) as f32, score}

}


/// Helper smooth damping function that works on a critically damped spring.
fn smooth_damp(
    current: f32,
    target: f32,
    current_velocity: &mut f32,
    smooth_time: f32,
    delta_time: f32,
) -> f32 {
    // Sicherstellen, dass smooth_time nicht 0 ist, um Division durch Null zu vermeiden
    let smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;
    let exp = (-omega * delta_time).exp();
    let change = current - target;


    let temp = (*current_velocity + omega * change) * delta_time;
    *current_velocity = (*current_velocity - omega * temp) * exp;

    let mut output = target + (change + temp) * exp;

    // Über-shooting verhindern
    if (target - current > 0.0) == (output > target) {
        output = target;
        *current_velocity = 0.0;
    }

    output
}