//! Native nonmesh BRepBndLib regressions with original builders and assertions.
//! The two mesher and cylinder cases run with the mesher crate.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCircle, GeomCurve};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis2, Direction3, Plane3, Point3};
use martensite_tkmath::poly::Polygon3D;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

fn expect_same_bounds(first: &BoundingBox3, second: &BoundingBox3) {
    assert_eq!(first.get().unwrap(), second.get().unwrap());
}

fn make_circle_edge() -> Edge {
    let frame = Axis2::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let circle: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(frame, 1.0).unwrap()).cast();
    BRepBuilderAPIMakeEdge::from_curve(&circle)
        .unwrap()
        .edge()
        .unwrap()
}

fn make_polygon3d_edge() -> Edge {
    let mut nodes = Array1::with_bounds(1, 5).unwrap();
    nodes[1] = Point3::new(1.0, 0.0, 0.0);
    nodes[2] = Point3::new(0.0, 1.0, 0.0);
    nodes[3] = Point3::new(-1.0, 0.0, 0.0);
    nodes[4] = Point3::new(0.0, -1.0, 0.0);
    nodes[5] = Point3::new(1.0, 0.0, 0.0);
    let polygon = Handle::new(Polygon3D::from_nodes(&nodes));
    let mut edge = Edge::default();
    BRepBuilder::default()
        .make_edge_with_polygon3d(&mut edge, &polygon)
        .unwrap();
    edge
}

fn add_both(edge: &Edge) -> (BoundingBox3, BoundingBox3) {
    let (mut with, mut without) = (BoundingBox3::default(), BoundingBox3::default());
    BRepBndLib::add(edge, &mut with, true).unwrap();
    BRepBndLib::add(edge, &mut without, false).unwrap();
    (with, without)
}

#[test]
// GeometricEdgeWithoutDiscreteRepresentation
fn occ30292_geometric_edge_without_discrete_representation() {
    let (with, without) = add_both(&make_circle_edge());
    expect_same_bounds(&with, &without);
}

#[test]
// Polygon3dEdgeIgnoresRepresentationFlag
fn occ30292_polygon3d_edge_ignores_representation_flag() {
    let (with, without) = add_both(&make_polygon3d_edge());
    expect_same_bounds(&with, &without);
}

#[test]
// PlanarBSplineEdgeBounds
fn occ23575_planar_bspline_edge_bounds() {
    let mut poles = Array1::with_bounds(1, 4).unwrap();
    poles[1] = Point3::new(0.0, 0.0, 0.0);
    poles[2] = Point3::new(0.0, 1.0, 0.0);
    poles[3] = Point3::new(1.0, 1.0, 0.0);
    poles[4] = Point3::new(1.0, 0.0, 0.0);
    let mut weights = Array1::with_bounds(1, 4).unwrap();
    weights.init(1.0);
    let mut knots = Array1::with_bounds(1, 2).unwrap();
    knots[1] = 0.0;
    knots[2] = 1.0;
    let mut mults = Array1::with_bounds(1, 2).unwrap();
    mults[1] = 4;
    mults[2] = 4;
    let curve = GeomBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 3, false).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&Handle::new(curve).cast())
        .unwrap()
        .edge()
        .unwrap();
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&edge, &mut bounds, true).unwrap();
    assert!(!bounds.is_void());
    let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds.get().unwrap();
    let tolerance = 0.1;
    assert!((xmin - 0.0).abs() <= tolerance);
    assert!((ymin - 0.0).abs() <= tolerance);
    assert!((zmin - 0.0).abs() <= tolerance);
    assert!((xmax - 1.0).abs() <= tolerance);
    assert!((ymax - 1.0).abs() <= tolerance);
    assert!((zmax - 0.0).abs() <= tolerance);
    assert!(zmax - zmin <= 1.0e-6);
}

#[test]
// InfinitePlaneBounds
fn occ6503_infinite_plane_bounds() {
    let plane = Plane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane(plane)
        .unwrap()
        .face()
        .unwrap();
    assert!(!face.is_null());
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&face, &mut bounds, true).unwrap();
    assert!(!bounds.is_void());
    let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds.get().unwrap();
    let open_bound = precision::infinite() / 2.0;
    assert_eq!(xmin, -open_bound);
    assert_eq!(xmax, open_bound);
    assert_eq!(ymin, -open_bound);
    assert_eq!(ymax, open_bound);
    assert!((zmin - -1.0e-7).abs() <= 1.0e-12);
    assert!((zmax - 1.0e-7).abs() <= 1.0e-12);
}
