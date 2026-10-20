//! The intersection of two Geom surfaces as Geom curves. [`GeomIntIntSS`] runs `IntPatch_Intersection`,
//! splits its lines with [`GeomIntLineConstructor`] and makes the curves, approximating
//! the walking lines with [`GeomIntWLApprox`] (the ApproxInt instances of `instances`);
//! [`GeomIntLineTool`] reads the lines and decomposes the walking lines at the seams.
mod adjust_periodic;
mod instances;
mod int_ss;
pub(crate) mod line_constructor;
pub(crate) mod line_tool;
mod parameter_and_orientation;

pub use adjust_periodic::adjust_periodic;
pub use instances::*;
pub use int_ss::{GeomIntIntSS, GeomIntIntSSApprox, GeomIntRLineCurves};
pub use line_constructor::GeomIntLineConstructor;
pub use line_tool::GeomIntLineTool;
pub use parameter_and_orientation::GeomIntParameterAndOrientation;

pub use line_constructor::helpers::quadric_parameters;
