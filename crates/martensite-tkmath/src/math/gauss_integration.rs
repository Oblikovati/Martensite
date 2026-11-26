//! Gauss quadrature and uniform subdivision convergence.
use super::{Function, quadrature_tables::fill_gauss_half_tables, stream_real};
use martensite_tkernel::standard::Failure;
use std::fmt;
/// Single-variable Gauss integral; `GaussSingleIntegration::new(&mut f,0.0,1.0,10)`.
///
/// `Default` is the empty constructor: not done.
#[derive(Default)]
pub struct GaussSingleIntegration {
    done: bool,
    value: f64,
}
impl GaussSingleIntegration {
    /// Caps order at 61; `GaussSingleIntegration::new(&mut f,0.0,1.0,10)`.
    pub fn new(f: &mut impl Function, lower: f64, upper: f64, order: i32) -> Self {
        let mut result: Self = Self::default();
        let order: i32 = order.min(61);
        if order < 1 {
            return result;
        }
        let mut points: [f64; 61] = [0.0; 61];
        let mut weights: [f64; 61] = [0.0; 61];
        fill_gauss_half_tables(order, &mut points, &mut weights);
        result.perform_with_tables(f, lower, upper, order, &points, &weights);
        result
    }
    /// Subdivides at least once, up to iteration limit; `GaussSingleIntegration::with_tolerance(&mut f,0.0,1.0,5,1e-8)`.
    pub fn with_tolerance(
        f: &mut impl Function,
        lower: f64,
        upper: f64,
        order: i32,
        tolerance: f64,
    ) -> Self {
        let order: i32 = order.min(61);
        let mut result: Self = Self::default();
        if order < 1 {
            return result;
        }
        let mut points: [f64; 61] = [0.0; 61];
        let mut weights: [f64; 61] = [0.0; 61];
        fill_gauss_half_tables(order, &mut points, &mut weights);
        result.perform_with_tables(f, lower, upper, order, &points, &weights);
        let mut sum: f64 = result.value;
        let mut count: i32 = 1;
        let mut iteration: i32 = 1;
        loop {
            let old: f64 = sum;
            sum = 0.0;
            count *= 2;
            let step: f64 = (upper - lower) / f64::from(count);
            for i in 1..=count {
                result.perform_with_tables(
                    f,
                    lower + f64::from(i - 1) * step,
                    lower + f64::from(i) * step,
                    order,
                    &points,
                    &weights,
                );
                if !result.done {
                    return result;
                }
                sum += result.value;
            }
            iteration += 1;
            if (old - sum).abs().partial_cmp(&tolerance) != Some(std::cmp::Ordering::Greater)
                || iteration > 13
            {
                break;
            }
        }
        result.value = sum;
        result
    }
    fn perform_with_tables(
        &mut self,
        f: &mut impl Function,
        lower: f64,
        upper: f64,
        order: i32,
        points: &[f64; 61],
        weights: &[f64; 61],
    ) {
        self.done = false;
        let middle: f64 = 0.5 * (upper + lower);
        let radius: f64 = 0.5 * (upper - lower);
        self.value = 0.0;
        let half_floor: usize = (order / 2) as usize;
        if (order & 1) == 1 {
            if !f.value(middle, &mut self.value) {
                return;
            }
            self.value *= weights[half_floor];
        }
        let (mut first, mut second): (f64, f64) = (0.0, 0.0);
        for index in 0..half_floor {
            let offset: f64 = radius * points[index];
            if !f.value(middle - offset, &mut first) || !f.value(middle + offset, &mut second) {
                return;
            }
            self.value += weights[index] * (first + second);
        }
        self.value *= radius;
        self.done = true;
    }
    /// Reports successful callbacks; `integral.is_done()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// Reads a completed integral; `integral.value()?`.
    pub fn value(&self) -> Result<f64, Failure> {
        if !self.done {
            return Err(Failure::with_type("StdFail_NotDone", " ", ""));
        }
        Ok(self.value)
    }
    /// Writes the status text; `integral.dump(&mut text)?`.
    pub fn dump(&self, stream: &mut impl fmt::Write) -> fmt::Result {
        write!(stream, "math_GaussSingleIntegration ")?;
        if !self.done {
            return writeln!(stream, "Status = not Done ");
        }
        writeln!(stream, " Status = Done ")?;
        writeln!(stream, "Integration Value = {}", stream_real(self.value))
    }
}
display_via_dump!(GaussSingleIntegration);
