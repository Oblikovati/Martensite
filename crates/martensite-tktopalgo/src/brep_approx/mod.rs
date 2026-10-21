//! The approximation of the walking lines of two faces by
//! B-spline curves. It is the second instantiation of the ApproxInt templates (GeomInt is
//! the first), over `BRepAdaptor_Surface`:
//!
//! - `BRepApprox_SurfaceTool` is [`BRepApproxSurfaceTool`], the `ThePSurfaceTool` of the
//!   instances;
//! - `BRepApprox_ApproxLine` is [`BRepApproxApproxLine`], their `TheWLine`;
//! - `BRepApprox_Approx` and the other instance classes are aliases of the generics of the
//!   geometric algorithm and construction crates (`instances`).
mod approx_line;
mod instances;
mod surface_tool;
mod surface_tool_traits;

pub use approx_line::BRepApproxApproxLine;
pub use instances::*;
pub use surface_tool::{BRepApproxSurface, BRepApproxSurfaceTool};
