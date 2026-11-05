//! Regressions of the face-face intersection.
#![allow(non_snake_case, clippy::excessive_precision)]
use martensite_tkbo::int_tools::FaceFace;
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomCylindricalSurface, GeomPlane, GeomSurface,
    GeomSurfaceOfLinearExtrusion,
};
use martensite_tkmath::gp::{Axis2, Axis3, Circle3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use std::f64::consts::{PI, TAU};
include!("face_face_legacy/fixtures.rs");
include!("face_face_legacy/circular.rs");
include!("face_face_legacy/regressions.rs");
