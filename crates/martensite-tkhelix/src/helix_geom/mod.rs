//! The analytic helix and its B-spline approximation.
//!
//! [`HelixGeomHelixCurve`] evaluates the helix as an `Adaptor3d_Curve`,
//! [`HelixGeomTools`] approximates it (or any adapted curve) with AdvApprox, and the
//! builders [`HelixGeomBuilderHelixCoil`] (one coil) and [`HelixGeomBuilderHelix`]
//! (one coil per turn, placed in a frame) implement the abstract builder traits
//! [`HelixGeomBuilderApproxCurve`] and [`HelixGeomBuilderHelixGen`].
mod builder_approx_curve;
mod builder_helix;
mod builder_helix_coil;
mod builder_helix_gen;
pub(crate) mod errors;
mod helix_curve;
mod tools;

pub use builder_approx_curve::{HelixGeomBuilderApproxCurve, HelixGeomBuilderApproxCurveBase};
pub use builder_helix::HelixGeomBuilderHelix;
pub use builder_helix_coil::HelixGeomBuilderHelixCoil;
pub use builder_helix_gen::{HelixGeomBuilderHelixGen, HelixGeomBuilderHelixGenBase};
pub use helix_curve::{HelixGeomHelixCurve, HelixParameters};
pub use tools::HelixGeomTools;
