//! Existing geometry drawing algorithms over explicit rendering state and reports.
//! All caches, geometry handles, draw records, reports and pick contexts here are
//! thread-confined: they share mutable view state and do not claim isolated published geometry.
//! Plain copied style values are independently checked as `Send + Sync`.
pub mod edge_face;
mod fill_gaps;
pub mod hide_data;
pub mod iso_builder;
pub mod picks;
pub mod shape;
pub use shape::{
    HlrSettings, MeshNormals, NormalSegment, ShapeColors, TopologyRendering, color_orientation,
};

pub mod axes;
mod state;
mod topology_drawable;
pub use topology_drawable::{TopologyDrawable, TopologyStyle};

mod point_glyph;
mod segment_glyph;
pub use point_glyph::PointGlyph;
pub use segment_glyph::{PlanarSegment, SpatialSegment};

mod curve_sampling;
pub use curve_sampling::CurveSampling;

pub mod curve_properties;
pub mod curve_style;
pub mod spline_style;
pub use curve_style::{CurveLook, CurveStyle};
pub use spline_style::{KnotsStyle, PolesStyle};

mod curve_rendering;
pub mod geometry_access;
pub use curve_rendering::{PlanarCurveRendering, SpatialCurveRendering};

mod curve_drawable;
pub use curve_drawable::{PlanarCurveDrawable, SpatialCurveDrawable};

pub mod bezier;
pub use bezier::{PlanarBezierDrawable, SpatialBezierDrawable};

pub mod bspline;
pub use bspline::{PlanarSplineDrawable, SpatialSplineDrawable, SplineCurveLook};

pub mod surface;
pub use surface::{SurfaceDrawable, SurfaceLook, SurfaceRendering, SurfaceStyle};

pub use bezier::BezierSurfaceDrawable;

mod property_report;
pub use property_report::property_report;

mod box_glyph;
mod box_rendering;
pub use box_glyph::BoxGlyph;
pub use box_rendering::BoxRendering;

#[cfg(test)]
mod box_glyph_tests;
