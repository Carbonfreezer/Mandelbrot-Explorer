use crate::math::MAX_ITER;
use macroquad::prelude::{BLACK, Color};

const CTRL_POINTS: usize = 17;
/// Reference control points from matplotlib's viridis colormap (every 16th entry)
const VIRIDIS: [[f32; 3]; CTRL_POINTS] = [
    [0.267004, 0.004874, 0.329415], // 0
    [0.282327, 0.140926, 0.457517], // 16
    [0.253935, 0.265254, 0.529983], // 32
    [0.206756, 0.371758, 0.553117], // 48
    [0.163625, 0.471133, 0.558148], // 64
    [0.127568, 0.566949, 0.550556], // 80
    [0.134692, 0.658636, 0.517649], // 96
    [0.220057, 0.743517, 0.456192], // 112
    [0.364929, 0.815712, 0.367757], // 128
    [0.525776, 0.870588, 0.271225], // 144
    [0.692840, 0.907950, 0.168856], // 160
    [0.845561, 0.926419, 0.094695], // 176
    [0.964394, 0.927318, 0.104071], // 192
    [0.993248, 0.906157, 0.143936], // 208
    [0.987053, 0.862323, 0.196354], // 224
    [0.974417, 0.813768, 0.247040], // 240
    [0.993248, 0.906157, 0.143936], // 255 (endpoint)
];

const SCALING : f32 = (CTRL_POINTS - 1) as f32 / MAX_ITER as f32;

pub fn get_color(iter: u16) -> Color {
    if iter == MAX_ITER {
        BLACK
    } else {
        let base = iter as f32 * SCALING;
        let alpha = base.fract();
        let base = base.floor() as usize;

        Color::new(
            VIRIDIS[base][0] * (1.0 - alpha) + VIRIDIS[base + 1][0] * alpha,
            VIRIDIS[base][1] * (1.0 - alpha) + VIRIDIS[base + 1][1] * alpha,
            VIRIDIS[base][2] * (1.0 - alpha) + VIRIDIS[base + 1][2] * alpha,
            1.0,
        )
    }
}
