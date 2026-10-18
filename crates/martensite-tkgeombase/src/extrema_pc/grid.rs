// the implementation at .

//! `ExtremaPC_GridEvaluator::Perform`: unlike the 2D evaluator, a failing curve
//! evaluation during the classification escapes it (only the stationarity signs are
//! caught), so the Rust member returns `Result`.
use super::{Domain1D, ExtremaResult, GridEvaluator, SearchMode};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::gp::Point3;

impl GridEvaluator {
    /// The extrema of `point` against `curve` over `domain` with the cached partition
    /// (`Perform`); InvalidInput when the partition does not cover the domain;
    /// `evaluator.perform(&adaptor, &point, &domain, 1e-9, SearchMode::MinMax)?`.
    pub fn perform(
        &mut self,
        curve: &dyn Adaptor3dCurve,
        point: &Point3,
        domain: &Domain1D,
        tol: f64,
        mode: SearchMode,
    ) -> Result<&ExtremaResult, Failure> {
        self.perform_core(curve, point, domain, (tol, mode))?;
        Ok(self.result())
    }
}
