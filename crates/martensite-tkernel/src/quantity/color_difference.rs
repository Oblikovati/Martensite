//! The CIEDE2000 formula, including its epsilon-based hue handling.
use super::{Color, ColorSpace};
const RAD: f64 = std::f64::consts::PI / 180.0;
const DEG: f64 = 180.0 / std::f64::consts::PI;
const POW_25_7: f64 = 6103515625.0;
impl Color {
    /// Evaluates perceptual difference through the Lab conversion; `color.delta_e2000(color) == 0.0`.
    pub fn delta_e2000(self, other: Self) -> f64 {
        let [l1, a1, b1] = self.values(ColorSpace::Lab);
        let [l2, a2, b2] = other.values(ColorSpace::Lab);
        let (c1, c2, h1, h2) = adjusted_chromas(a1, b1, a2, b2);
        let (mean_hue, delta_hue) = mean_delta_hue(h1, h2);
        let mean_light = 0.5 * (l1 + l2);
        let mean_chroma = 0.5 * (c1 + c2);
        let delta_h = 2.0 * (c1 * c2).sqrt() * martensite_cmath::sin(0.5 * delta_hue * RAD);
        let [sl, sc, sh, rotation] = difference_factors(mean_light, mean_chroma, mean_hue);
        let dl = (l2 - l1) / sl;
        let dc = (c2 - c1) / sc;
        let dh = delta_h / sh;
        (dl * dl + dc * dc + dh * dh + rotation * dc * dh).sqrt()
    }
}
fn adjusted_chromas(a1: f64, b1: f64, a2: f64, b2: f64) -> (f64, f64, f64, f64) {
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    let power = martensite_cmath::pow(0.5 * (c1 + c2), 7.0);
    let correction = 0.5 * (1.0 - (power / (power + POW_25_7)).sqrt());
    let a1 = a1 * (1.0 + correction);
    let a2 = a2 * (1.0 + correction);
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    (c1, c2, adjusted_hue(a1, b1, c1), adjusted_hue(a2, b2, c2))
}
fn adjusted_hue(a: f64, b: f64, chroma: f64) -> f64 {
    let mut hue = if chroma > Color::epsilon() {
        martensite_cmath::atan2(b, a) * DEG
    } else {
        270.0
    };
    if hue < 0.0 {
        hue += 360.0;
    }
    hue
}
fn mean_delta_hue(first: f64, second: f64) -> (f64, f64) {
    let mut mean = 0.5 * (first + second);
    let mut delta = second - first;
    if delta.abs() > 180.0 {
        mean += if mean < 180.0 { 180.0 } else { -180.0 };
        delta += if first >= second { 360.0 } else { -360.0 };
    }
    (mean, delta)
}
fn difference_factors(light: f64, chroma: f64, hue: f64) -> [f64; 4] {
    let t = 1.0 - 0.17 * martensite_cmath::cos((hue - 30.0) * RAD)
        + 0.24 * martensite_cmath::cos((2.0 * hue) * RAD)
        + 0.32 * martensite_cmath::cos((3.0 * hue + 6.0) * RAD)
        - 0.20 * martensite_cmath::cos((4.0 * hue - 63.0) * RAD);
    let light50 = (light - 50.0) * (light - 50.0);
    let sl = 1.0 + 0.015 * light50 / (20.0 + light50).sqrt();
    let sc = 1.0 + 0.045 * chroma;
    let sh = 1.0 + 0.015 * chroma * t;
    let theta = 30.0 * martensite_cmath::exp(-(hue - 275.0) * (hue - 275.0) / 625.0);
    let power = martensite_cmath::pow(chroma, 7.0);
    let rc = 2.0 * (power / (power + POW_25_7)).sqrt();
    [sl, sc, sh, -rc * martensite_cmath::sin(2.0 * theta * RAD)]
}
