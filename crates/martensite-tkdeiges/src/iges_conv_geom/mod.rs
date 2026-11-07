//! Conversions between the spline definitions of IGES and B-splines, and the builder of entities.
mod continuity;
mod geom_builder;
mod geom_builder_entities;
mod spline_curve;
mod spline_surface;

pub use continuity::{
    increase_curve_continuity, increase_curve2d_continuity, increase_surface_continuity,
};
pub use geom_builder::IgesConvGeomGeomBuilder;
pub use spline_curve::{SplineCurveConversion, spline_curve_from_iges};
pub use spline_surface::{SplineSurfaceConversion, spline_surface_from_iges};
