// (GFuncExtCC, GGenExtCC, GenLocateExtCC, the
// GlobOptFuncCC functions, ExtCC, ExtCC2d,
// LocateExtCC and the ECC, ECC2d, LocECC and
// CCLocFOfLocECC aliases) at .

//! The curve-curve extrema.
mod ext_cc;
mod ext_cc2d;
mod func_ext_cc;
mod func_ext_cc_values;
mod gen_ext_cc;
mod gen_locate_ext_cc;
mod glob_opt_func_cc;

pub use ext_cc::ExtCC;
pub use ext_cc2d::ExtCC2d;
pub use func_ext_cc::GFuncExtCC;
pub use gen_ext_cc::GGenExtCC;
pub use gen_locate_ext_cc::{GenLocateExtCC, LocECC, LocateExtCC};
pub use glob_opt_func_cc::{GlobOptFuncCCC0, GlobOptFuncCCC1, GlobOptFuncCCC2};

use super::curve_tools::{Curve2dTool, CurveTool};

/// `ECC`: `GGenExtCC` over Adaptor3d_Curve.
pub type ECC<'a> = GGenExtCC<'a, CurveTool>;
/// `ECC2d`: `GGenExtCC` over Adaptor2d_Curve2d.
pub type ECC2d<'a> = GGenExtCC<'a, Curve2dTool>;
/// `CCLocFOfLocECC`: `GFuncExtCC` over Adaptor3d_Curve.
pub type CCLocFOfLocECC<'a> = GFuncExtCC<'a, CurveTool>;
