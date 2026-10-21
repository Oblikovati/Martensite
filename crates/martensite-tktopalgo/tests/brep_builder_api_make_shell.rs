//! Tests of `BRepBuilderAPI_MakeShell`.
#![allow(non_snake_case)]

use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShell};

// Regression case `bugs_modalg_5::bug25591`.  The second unique edge of a
// sphere shell created from a surface must not acquire a spurious 3D curve.
#[test]
fn BRepBuilderAPI_MakeShellTest__ModalgBug_25591_SphereSeamHasNo3dCurve() {
    let frame = Axis3::from_normal(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let a_sphere: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(frame, 100.0).unwrap()).cast();
    let a_maker = BRepBuilderAPIMakeShell::with_surface(&a_sphere, false).unwrap();
    assert!(a_maker.is_done());

    let a_shell = a_maker.shell().unwrap();
    assert!(!a_shell.is_null());

    let mut an_edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&a_shell, ShapeEnum::Edge, &mut an_edges).unwrap();
    assert!(an_edges.extent() >= 2);

    let edge = topods::edge(an_edges.find_key(2).unwrap()).unwrap();
    let a_curve = BRepTool::curve_with_location(&edge).unwrap().curve;
    assert!(a_curve.is_null());
}
