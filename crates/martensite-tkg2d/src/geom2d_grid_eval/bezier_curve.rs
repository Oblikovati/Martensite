//! `Geom2dGridEvalBezierCurve`: a Bezier curve on a grid through one span cache.
use super::bezier_arrays::BezierCurveArrays;
use super::grid::{constant_grid, empty, try_map_grid};
use super::{Geom2dGridEvalCurveD1, Geom2dGridEvalCurveD2, Geom2dGridEvalCurveD3};
use crate::geom2d::Geom2dBezierCurve;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::bsplclib::Cache;
use martensite_tkmath::gp::{Point2, Vector2};

/// Batch evaluator of a `Geom2dBezierCurve` (`Geom2dGridEvalBezierCurve`).
///
/// `EvaluateGrid` to `EvaluateGridD3` build a `bsplclib::Cache` of the single span at
/// `0.5` and evaluate every parameter through it; `EvaluateGridDN` calls
/// `bsplclib::dn` on the flat knots. Evaluation policies: the evaluation representation of the
/// curve is ignored, and orders above the degree give zero vectors even for a rational
/// curve, whose derivatives never vanish there;
/// `Geom2dGridEvalBezierCurve::new(bezier).evaluate_grid(&params)?`.
pub struct Geom2dGridEvalBezierCurve {
    geom: Handle<Geom2dBezierCurve>,
}

// The arrays of the curve, read once per call (shared with the 3D evaluator).
fn read_arrays(bezier: &Geom2dBezierCurve) -> Result<BezierCurveArrays<Point2>, Failure> {
    let flat_knots: Array1<'static, f64> = bezier.knot_sequence()?;
    Ok(BezierCurveArrays {
        degree: bezier.degree(),
        poles: bezier.poles(),
        weights: bezier.weights(),
        flat_knots,
    })
}

impl Geom2dGridEvalBezierCurve {
    /// Evaluator of `bezier` (`Geom2dGridEvalBezierCurve(theBezier)`); a null handle is
    /// accepted; `Geom2dGridEvalBezierCurve::new(bezier)`.
    pub fn new(bezier: Handle<Geom2dBezierCurve>) -> Self {
        Self { geom: bezier }
    }
    /// The evaluated curve (`Geometry`); `evaluator.geometry().is_null()`.
    pub fn geometry(&self) -> &Handle<Geom2dBezierCurve> {
        &self.geom
    }
    // The span cache when there is something to evaluate.
    fn cache(&self, params: &Array1<'_, f64>) -> Result<Option<Cache>, Failure> {
        let Some(bezier) = self.geom.get() else {
            return Ok(None);
        };
        if params.is_empty() {
            return Ok(None);
        }
        Ok(Some(read_arrays(bezier)?.cache()?))
    }
    crate::bezier_grid_methods!(
        Point2,
        Geom2dGridEvalCurveD1,
        Geom2dGridEvalCurveD2,
        Geom2dGridEvalCurveD3
    );
    /// Derivative of order `n` through `bsplclib::dn`, zero above the degree; empty for
    /// `n < 1` (`EvaluateGridDN`); `evaluator.evaluate_grid_dn(&params, 2)?`.
    pub fn evaluate_grid_dn(
        &self,
        params: &Array1<'_, f64>,
        n: i32,
    ) -> Result<Array1<'static, Vector2>, Failure> {
        let Some(bezier) = self.geom.get() else {
            return Ok(empty());
        };
        if params.is_empty() || n < 1 {
            return Ok(empty());
        }
        if n > bezier.degree() {
            return Ok(constant_grid(params.len(), Vector2::new(0.0, 0.0)));
        }
        let arrays: BezierCurveArrays<Point2> = read_arrays(bezier)?;
        try_map_grid(params, |u| arrays.dn(u, n))
    }
}
