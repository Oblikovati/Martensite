//! Curve and surface evaluation interfaces, restricted curves and sampling tools.
//! Trait objects are reached through shared handles; concrete base adaptors retain
//! their documented defaults independently of extension implementations.
mod curve;
pub use curve::{Adaptor3dCurve, Adaptor3dCurveBase, Adaptor3dD3};
mod surface;
pub use surface::{Adaptor3dSurface, Adaptor3dSurfaceBase};
mod h_vertex;
pub use h_vertex::{Adaptor3dHVertex, Adaptor3dHVertexBase};
mod topol_tool;
pub use topol_tool::Adaptor3dTopolTool;
mod iso_curve;
pub use iso_curve::Adaptor3dIsoCurve;
pub mod h_surface_tool;
mod inter_func;
pub use inter_func::Adaptor3dInterFunc;
mod curve_on_surface;
pub use curve_on_surface::Adaptor3dCurveOnSurface;
