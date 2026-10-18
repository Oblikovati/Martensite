//! The state and result queries of `Variational`.
use super::{Variational, not_done};
use crate::app_par_curves::MultiBSpCurve;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::geom_abs::Shape;

impl Variational {
    // not-done failure before a successful Approximate.
    fn done(&self, member: &str) -> Result<(), Failure> {
        if self.is_done {
            Ok(())
        } else {
            Err(not_done(member))
        }
    }

    /// `IsCreated()`: the constraints fit the degree and segments;
    /// `smoothing.is_created()`.
    pub fn is_created(&self) -> bool {
        self.is_created
    }

    /// `IsDone()`: an approximation succeeded (never reset); `smoothing.is_done()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `IsOverConstrained()`: the last Init found the problem over-constrained (never
    /// reset by a later valid Init); `smoothing.is_over_constrained()`.
    pub fn is_over_constrained(&self) -> bool {
        self.is_over_constr
    }

    /// `Value()`: the B-spline MultiCurve (a copy sharing its poles);
    /// `smoothing.value()?`.
    pub fn value(&self) -> Result<MultiBSpCurve, Failure> {
        self.done("Value")?;
        Ok(self.results.mbsp_curve.clone())
    }

    /// `MaxError()`: the largest point distance; `smoothing.max_error()?`.
    pub fn max_error(&self) -> Result<f64, Failure> {
        self.done("MaxError")?;
        Ok(self.results.max_error)
    }

    /// `MaxErrorIndex()`: the multipoint of MaxError; `smoothing.max_error_index()?`.
    pub fn max_error_index(&self) -> Result<i32, Failure> {
        self.done("MaxErrorIndex")?;
        Ok(self.results.max_error_index)
    }

    /// `QuadraticError()`: the sum of the squared distances;
    /// `smoothing.quadratic_error()?`.
    pub fn quadratic_error(&self) -> Result<f64, Failure> {
        self.done("QuadraticError")?;
        Ok(self.results.criterium[0])
    }

    /// `AverageError()`: the distance sum over the unconstrained points;
    /// `smoothing.average_error()?`.
    pub fn average_error(&self) -> Result<f64, Failure> {
        self.done("AverageError")?;
        Ok(self.results.average_error)
    }

    /// `Parameters()`: the point parameters (the shared array, rebased on 1 by an
    /// approximation); `smoothing.parameters()?`.
    pub fn parameters(&self) -> Result<&Handle<HArray1<f64>>, Failure> {
        self.done("Parameters")?;
        Ok(&self.parameters)
    }

    /// `Knots()`: the knots of the B-spline; `smoothing.knots()?`.
    pub fn knots(&self) -> Result<&Handle<HArray1<f64>>, Failure> {
        self.done("Knots")?;
        Ok(&self.knots)
    }

    /// `Criterium(VFirstOrder, VSecondOrder, VThirdOrder)`: the square roots of the
    /// tension, flexion and jerk criteria; `smoothing.criterium()?`.
    pub fn criterium(&self) -> Result<[f64; 3], Failure> {
        self.done("Criterium")?;
        let c: [f64; 4] = self.results.criterium;
        Ok([c[1], c[2], c[3]])
    }

    /// `CriteriumWeight(Percent1, Percent2, Percent3)`; `smoothing.criterium_weight()`.
    pub fn criterium_weight(&self) -> [f64; 3] {
        self.percent
    }

    /// `MaxDegree()`; `smoothing.max_degree()`.
    pub fn max_degree(&self) -> i32 {
        self.options.max_degree
    }

    /// `MaxSegment()`; `smoothing.max_segment()`.
    pub fn max_segment(&self) -> i32 {
        self.options.max_segment
    }

    /// `Continuity()`; `smoothing.continuity()`.
    pub fn continuity(&self) -> Shape {
        self.options.continuity
    }

    /// `WithMinMax()`; `smoothing.with_min_max()`.
    pub fn with_min_max(&self) -> bool {
        self.options.with_min_max
    }

    /// `WithCutting()`; `smoothing.with_cutting()`.
    pub fn with_cutting(&self) -> bool {
        self.options.with_cutting
    }

    /// `Tolerance()`; `smoothing.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.options.tolerance
    }

    /// `NbIterations()`; `smoothing.nb_iterations()`.
    pub fn nb_iterations(&self) -> i32 {
        self.options.nb_iterations
    }
}
