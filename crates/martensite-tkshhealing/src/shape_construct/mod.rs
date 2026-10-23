//! ShapeConstruct: curve adjustment, B-spline conversion and projection.
//! Curve construction and the sampled surface projector share the geometric foundations.
mod curve;
pub use curve::{
    ShapeConstructCurve, ShapeConstructCurveJoin2d, ShapeConstructCurveJoin2dRequest,
    ShapeConstructCurveJoin3d,
};

mod package;
pub use package::{ShapeConstruct, ShapeConstructApproxSettings, ShapeConstructSurfaceBounds};

mod project_curve_on_surface;
pub use project_curve_on_surface::ShapeConstructProjectCurveOnSurface;
