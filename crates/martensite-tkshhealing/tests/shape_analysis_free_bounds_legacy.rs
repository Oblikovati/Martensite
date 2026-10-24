//! Free-boundary analysis regression cases.
#![allow(non_snake_case)]
mod healing_support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{Face, Shape, Vertex, Wire},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point3;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisFreeBounds;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
include!("free_bounds_legacy/support.rs");
include!("free_bounds_legacy/wires.rs");
include!("free_bounds_legacy/edges.rs");
include!("free_bounds_legacy/faces.rs");
