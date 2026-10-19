//! CPnts: the curve length, the point at a curvilinear abscissa and the uniform
//! deflection iterator on an `Adaptor3dCurve` or an `Adaptor2dCurve2d`.
//!
//! Each retained member has a 3D and a 2D overload with the same body; this implementation has one
//! member taking a [`PntsCurve`] (convertible from either adaptor). retained exceptions,
//! including the ones the adaptors raise while they are evaluated, are `Err` values.
mod abscissa_point;
mod curve;
mod my_gauss_function;
mod my_root_function;
mod uniform_deflection;

pub use abscissa_point::AbscissaPoint;
pub use curve::PntsCurve;
pub use my_gauss_function::{MyGaussFunction, RealFunction};
pub use my_root_function::MyRootFunction;
pub use uniform_deflection::UniformDeflection;
