pub(super) fn scale_factor(value: f64, degree: i32) -> f64 {
    let exponent: i32 = base_exponent(value) / degree;
    martensite_cmath::pow(2.0_f64, f64::from(exponent))
}
fn base_exponent(value: f64) -> i32 {
    let inverse_log: f64 = 1.0 / martensite_cmath::log(2.0_f64);
    if value > 1.0 {
        return (martensite_cmath::log(value) * inverse_log) as i32;
    }
    if value < -1.0 {
        return (-martensite_cmath::log(-value) * inverse_log) as i32;
    }
    0
}
pub(super) use super::super::real_epsilon as epsilon;
