//! `LogSample`: a logarithmic sampling of an interval.
use crate::native_failure::failure;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{FunctionSample, FunctionSampling};

/// The FunctionSample of `n` points of `[a, b]` whose inner points follow
/// `a - 1 + (b - a)^(i / n)`.
///
/// `let sample = LogSample::new(1e-3, 50.0, 100); let t = sample.get_parameter(2)?;`
pub struct LogSample {
    base: FunctionSample,
    my_f: f64,
    myexp: f64,
}

impl LogSample {
    /// The sampling of `[a, b]` by `n` points (`LogSample(A, B, N)`);
    /// `LogSample::new(1e-3, 50.0, 100)`.
    pub fn new(a: f64, b: f64, n: i32) -> Self {
        Self {
            base: FunctionSample::new(a, b, n),
            my_f: a - 1.0,
            myexp: martensite_cmath::log(b - a) / f64::from(n),
        }
    }

    /// The parameter of index `index`: A at 1, B at N, `myF + exp(myexp Index)`
    /// between; index-range failure for an index outside `1..=N`
    /// (`GetParameter(Index)`); `sample.get_parameter(2)?`.
    pub fn get_parameter(&self, index: i32) -> Result<f64, Failure> {
        let a_n: i32 = self.base.number_of_points();
        if index >= a_n || index <= 1 {
            let [a_a, a_b] = self.base.bounds();
            if index == 1 {
                return Ok(a_a);
            }
            if index == a_n {
                return Ok(a_b);
            }
            return Err(failure(
                "Standard_OutOfRange",
                &format!("GeomLib_LogSample::GetParameter: Index = {index}; expected 1..={a_n}"),
            ));
        }
        Ok(self.my_f + martensite_cmath::exp(self.myexp * f64::from(index)))
    }
}

impl FunctionSampling for LogSample {
    fn bounds(&self) -> [f64; 2] {
        self.base.bounds()
    }

    fn number_of_points(&self) -> i32 {
        self.base.number_of_points()
    }

    /// The trait cannot raise: FunctionAllRoots only asks for `1..=N`, where
    /// GetParameter succeeds; an index outside gives NaN instead of
    /// index-range failure.
    fn parameter(&self, index: i32) -> f64 {
        self.get_parameter(index).unwrap_or(f64::NAN)
    }
}
