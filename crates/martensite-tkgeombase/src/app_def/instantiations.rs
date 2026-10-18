//! Multiline fitting aliases over [`MultiLine`].
//!
//! The shared fitting algorithms accept any [`crate::app_par_curves::LineTool`].
//! These aliases select the multiline representation; algorithms receiving their
//! line per call retain the same type for every supported line.
use super::multi_line::MultiLine;
use crate::app_par_curves::{
    BSpFunction, BSpGradient, Function, Gradient, GradientBfgs, LeastSquare, ResolConstraint,
};
use crate::approx::{BSplComputeLine, ComputeLine};

/// `Compute`: `ComputeLine` over MultiLine, the Bezier
/// approximation of a MultiLine with cutting;
/// `Compute::with_line(&line, ComputeLineOptions::default())?`.
pub type Compute = ComputeLine;
/// `BSplineCompute`: `BSplComputeLine` over MultiLine, the
/// B-spline approximation; `BSplineCompute::with_line(&line, options)?`.
pub type BSplineCompute = BSplComputeLine;

/// `TheLeastSquares`: `LeastSquare` over MultiLine;
/// `TheLeastSquares::new(&line, 1, 5, first, last, &u, 4)?`.
pub type TheLeastSquares = LeastSquare;
/// `ParLeastSquareOfTheGradient`: the least squares of [`TheGradient`].
pub type ParLeastSquareOfTheGradient = LeastSquare;
/// `ParLeastSquareOfMyGradientOfCompute`: the least squares of
/// [`MyGradientOfCompute`].
pub type ParLeastSquareOfMyGradientOfCompute = LeastSquare;
/// `ParLeastSquareOfMyGradientbisOfBSplineCompute`: the least squares of
/// [`MyGradientbisOfBSplineCompute`].
pub type ParLeastSquareOfMyGradientbisOfBSplineCompute = LeastSquare;
/// `BSpParLeastSquareOfMyBSplGradientOfBSplineCompute`: the B-spline least
/// squares of [`MyBSplGradientOfBSplineCompute`].
pub type BSpParLeastSquareOfMyBSplGradientOfBSplineCompute = LeastSquare;

/// `TheResol`: `ResolConstraint` over MultiLine;
/// `TheResol::new(&line, &mut curve, 1, 5, &constraints, &a, &da, 1e-10)?`.
pub type TheResol = ResolConstraint;
/// `ResConstraintOfTheGradient`: the constraint solver of [`TheGradient`].
pub type ResConstraintOfTheGradient = ResolConstraint;
/// `ResConstraintOfMyGradientOfCompute`: the constraint solver of
/// [`MyGradientOfCompute`].
pub type ResConstraintOfMyGradientOfCompute = ResolConstraint;
/// `ResConstraintOfMyGradientbisOfBSplineCompute`: the constraint solver of
/// [`MyGradientbisOfBSplineCompute`].
pub type ResConstraintOfMyGradientbisOfBSplineCompute = ResolConstraint;

/// `TheFunction`: `Function` over MultiLine;
/// `TheFunction::new(&line, 1, 5, &constraints, &u, 3)?`.
pub type TheFunction = Function<MultiLine>;
/// `ParFunctionOfTheGradient`: the function of [`TheGradient`].
pub type ParFunctionOfTheGradient = Function<MultiLine>;
/// `ParFunctionOfMyGradientOfCompute`: the function of [`MyGradientOfCompute`].
pub type ParFunctionOfMyGradientOfCompute = Function<MultiLine>;
/// `ParFunctionOfMyGradientbisOfBSplineCompute`: the function of
/// [`MyGradientbisOfBSplineCompute`].
pub type ParFunctionOfMyGradientbisOfBSplineCompute = Function<MultiLine>;
/// `BSpParFunctionOfMyBSplGradientOfBSplineCompute`: `BSpFunction`
/// over MultiLine, the function of [`MyBSplGradientOfBSplineCompute`].
pub type BSpParFunctionOfMyBSplGradientOfBSplineCompute = BSpFunction<MultiLine>;

/// `TheGradient`: `Gradient` over MultiLine;
/// `TheGradient::new(&line, 1, 5, &constraints, &mut u, 3, 1e-3, 1e-6, 100)?`.
pub type TheGradient = Gradient;
/// `MyGradientOfCompute`: the parameter optimizer of [`Compute`].
pub type MyGradientOfCompute = Gradient;
/// `MyGradientbisOfBSplineCompute`: the Bezier parameter optimizer of
/// [`BSplineCompute`].
pub type MyGradientbisOfBSplineCompute = Gradient;
/// `MyBSplGradientOfBSplineCompute`: `BSpGradient` over
/// MultiLine, the B-spline parameter optimizer of [`BSplineCompute`].
pub type MyBSplGradientOfBSplineCompute = BSpGradient;

/// `Gradient_BFGSOfTheGradient`: `Gradient_BFGS`, math_BFGS
/// stopping on the fit tolerances too.
pub type GradientBfgsOfTheGradient = GradientBfgs;
/// `Gradient_BFGSOfMyGradientOfCompute`: see [`GradientBfgsOfTheGradient`].
pub type GradientBfgsOfMyGradientOfCompute = GradientBfgs;
/// `Gradient_BFGSOfMyGradientbisOfBSplineCompute`: see
/// [`GradientBfgsOfTheGradient`].
pub type GradientBfgsOfMyGradientbisOfBSplineCompute = GradientBfgs;
/// `BSpGradient_BFGSOfMyBSplGradientOfBSplineCompute`:
/// `BSpGradient_BFGS`, the same criterion over the B-spline function.
pub type BSpGradientBfgsOfMyBSplGradientOfBSplineCompute = GradientBfgs;
