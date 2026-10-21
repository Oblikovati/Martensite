//! The package class `BRepBuilderAPI`: the default plane and precision of the makers,
//! which are the process-wide settings of BRepLib.
use crate::brep_lib;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomPlane;

/// `BRepBuilderAPI::Plane(P)`: sets the plane of the 2d makers (`BRepLib::Plane(P)`);
/// `brep_builder_api::set_plane(&plane)`.
pub fn set_plane(plane: &Handle<GeomPlane>) {
    brep_lib::set_plane(plane);
}

/// `BRepBuilderAPI::Plane()`: the current plane (`BRepLib::Plane()`), created on the
/// XOY axes when none was set; `assert!(martensite_tktopalgo::brep_builder_api::plane().is_some())`.
pub fn plane() -> Handle<GeomPlane> {
    brep_lib::plane()
}

/// `BRepBuilderAPI::Precision(P)`: sets the default precision of the makers
/// (`BRepLib::Precision(P)`); `brep_builder_api::set_precision(1e-6)`.
pub fn set_precision(value: f64) {
    brep_lib::set_precision(value);
}

/// `BRepBuilderAPI::Precision()`: the default precision (`BRepLib::Precision()`);
/// `assert_eq!(martensite_tktopalgo::brep_builder_api::precision(), 1e-7)`.
pub fn precision() -> f64 {
    brep_lib::precision()
}
