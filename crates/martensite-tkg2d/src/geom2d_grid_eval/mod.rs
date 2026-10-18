//! The Geom2dGridEval package: batch evaluation of 2D curves on parameter grids.
//!
//! Each evaluator takes an array of parameters (any bounds) and returns the points or
//! derivatives in an array indexed from 1; empty parameters, a null curve or an
//! order below 1 give an empty array. `Geom2dGridEvalCurve` picks the specialized
//! evaluator from the curve class. Evaluators that can raise internally (through the
//! gp getters, bsplclib, the offset utilities or an adaptor) return `Result`.
//!
//! These low-level arrays do not report modelling-operation completion or validity.
//! In particular, `Ok(empty)` cannot distinguish an unbound evaluator from no
//! requested samples. Where `geometry()` is exposed, inspect that handle before
//! evaluation and compare output coverage with the requested samples. An operation publishing
//! geometry must report its own typed completion, diagnostics and validation.
use crate::geom2d::{Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3};

/// Point and first derivative of a grid evaluation (`Geom2dGridEval::CurveD1`); the
/// same fields as `Geom2dCurve::ResD1`, so the implementation shares the struct.
pub type Geom2dGridEvalCurveD1 = Geom2dCurveD1;
/// Point and two derivatives of a grid evaluation (`Geom2dGridEval::CurveD2`).
pub type Geom2dGridEvalCurveD2 = Geom2dCurveD2;
/// Point and three derivatives of a grid evaluation (`Geom2dGridEval::CurveD3`).
pub type Geom2dGridEvalCurveD3 = Geom2dCurveD3;

pub mod bezier_arrays;
mod conic_grid;
mod grid;
pub mod grid_loops;

mod line;
pub use line::Geom2dGridEvalLine;
mod circle;
pub use circle::Geom2dGridEvalCircle;
mod ellipse;
pub use ellipse::Geom2dGridEvalEllipse;
mod hyperbola;
pub use hyperbola::Geom2dGridEvalHyperbola;
mod parabola;
pub use parabola::Geom2dGridEvalParabola;
mod bezier_curve;
pub use bezier_curve::Geom2dGridEvalBezierCurve;
mod bspline_curve;
mod bspline_spans;
pub use bspline_curve::Geom2dGridEvalBSplineCurve;
mod offset_curve;
pub use offset_curve::Geom2dGridEvalOffsetCurve;
mod other_curve;
pub use other_curve::Geom2dGridEvalOtherCurve;
mod curve;
pub use curve::{Geom2dGridEvalCurve, Geom2dGridEvalEvaluator};
