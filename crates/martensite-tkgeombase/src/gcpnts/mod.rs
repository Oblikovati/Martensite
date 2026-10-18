//! GCPnts: points on a curve by abscissa (uniform, quasi-uniform), by deflection
//! (uniform, quasi-uniform, tangential) and the curve length.
//!
//! The retained classes implement each member once as a template over
//! `Adaptor3dCurve` and `Adaptor2dCurve2d` (`TCurveTypes`); this implementation has one
//! generic member over [`TCurveTypes`], implemented for both adaptor traits and the
//! GeomAdaptor and Geom2dAdaptor curves. 2D points are returned in the plane Z = 0, as
//! natively. Every retained exception, including the ones the adaptors raise, is `Err`.
mod abscissa_point;
mod deflection_common;
mod dist_function;
mod intervals;
mod point_sequence;
mod quasi_uniform_abscissa;
mod quasi_uniform_deflection;
pub(crate) mod t_curve_types;
mod tangential_deflection;
mod types;
mod uniform_abscissa;
mod uniform_deflection;

pub use abscissa_point::AbscissaPoint;
pub use dist_function::{
    DistFunction, DistFunction2d, DistFunction2dMV, DistFunctionMV, TDistFunction, TDistFunctionMV,
};
pub use quasi_uniform_abscissa::QuasiUniformAbscissa;
pub use quasi_uniform_deflection::QuasiUniformDeflection;
pub use t_curve_types::{PoleCurve, TCurveTypes};
pub use tangential_deflection::{TangentialCriteria, TangentialDeflection};
pub use types::{AbscissaType, DeflectionType};
pub use uniform_abscissa::UniformAbscissa;
pub use uniform_deflection::UniformDeflection;
