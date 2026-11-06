use super::*;
use martensite_tkbrep::{topexp, topods::ShapeIterator, toptools::IndexedMapOfShape};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSurface};
use martensite_tkmath::gp::Axis3;
use std::f64::consts::PI;

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found).unwrap();
    found.extent()
}

pub(crate) fn assert_count(source: &str, shape: &Shape, kind: ShapeEnum, expected: usize) {
    assert_eq!(count(shape, kind), expected, "{source}: {kind:?} count");
}

pub(crate) fn direct_count(shape: &Shape, kind: ShapeEnum) -> usize {
    ShapeIterator::new(shape, true, true)
        .filter(|part| part.shape_type().unwrap() == kind)
        .count()
}

pub(crate) fn direct_child(shape: &Shape, kind: ShapeEnum) -> Shape {
    ShapeIterator::new(shape, true, true)
        .find(|part| part.shape_type().unwrap() == kind)
        .unwrap_or_else(|| panic!("expected direct {kind:?} child"))
}

pub(crate) fn assert_volume(source: &str, shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    assert!(
        (properties.mass() - expected).abs() / expected <= 1e-2,
        "{source}: volume {}; expected {expected} within 1%",
        properties.mass()
    );
}

fn box_face_cells(bounds: (f64, f64, f64, f64)) -> (BOPAlgoCellsBuilder, [Shape; 2]) {
    let box_shape = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let plane = Plane3::new(Point3::new(5.0, 5.0, 5.0), gp::dx()).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, bounds)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&box_shape, &face]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "box face cells: bfillds failed"
    );
    (cells, [box_shape, face])
}

#[test]
fn CellsTest__J1() {
    let (mut cells, shapes) = box_face_cells((-10.0, 10.0, -10.0, 10.0));
    cells.add_all_to_result(0, false).unwrap();
    assert_count(
        "tests/boolean/cells_test/J1",
        cells.shape(),
        ShapeEnum::Solid,
        2,
    );
    assert_count(
        "tests/boolean/cells_test/J1",
        cells.shape(),
        ShapeEnum::Face,
        12,
    );
    assert_volume("tests/boolean/cells_test/J1", cells.shape(), 1000.0);
    assert_area("tests/boolean/cells_test/J1", cells.shape(), 1200.0);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J1 cut face",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[&shapes[1]], 0);
    assert_count(
        "tests/boolean/cells_test/J1 cut solid",
        cells.shape(),
        ShapeEnum::Solid,
        2,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0], &shapes[1]], &[], 0);
    assert_count(
        "tests/boolean/cells_test/J1 common",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0]], &[], 1, true);
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J1 fuse",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J1 fuse",
        cells.shape(),
        ShapeEnum::Face,
        11,
    );
}

#[test]
fn CellsTest__J2() {
    let (mut cells, shapes) = box_face_cells((0.0, 10.0, 0.0, 10.0));
    cells.add_all_to_result(0, false).unwrap();
    assert_count(
        "tests/boolean/cells_test/J2",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J2",
        cells.shape(),
        ShapeEnum::Face,
        8,
    );
    assert_volume("tests/boolean/cells_test/J2", cells.shape(), 1000.0);
    assert_area("tests/boolean/cells_test/J2", cells.shape(), 725.0);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J2 cut face",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[&shapes[1]], 0);
    assert_count(
        "tests/boolean/cells_test/J2 cut solid",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J2 cut solid",
        cells.shape(),
        ShapeEnum::Face,
        7,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0], &shapes[1]], &[], 0);
    assert_count(
        "tests/boolean/cells_test/J2 common",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0]], &[], 1, true);
    assert_count(
        "tests/boolean/cells_test/J2 unified",
        cells.shape(),
        ShapeEnum::Face,
        7,
    );
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J2 fuse",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J2 fuse",
        cells.shape(),
        ShapeEnum::Face,
        8,
    );
}

fn assert_length(source: &str, shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(shape, &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - expected).abs() / expected <= 1e-2,
        "{source}: length {}; expected {expected} within 1%",
        properties.mass()
    );
}

fn sphere_edge_cells() -> (BOPAlgoCellsBuilder, [Shape; 2]) {
    let sphere = BRepPrimAPIMakeSphere::new(10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let edge = BRepBuilderAPIMakeEdge::from_lin_params(
        Line3::new(Point3::default(), gp::dy()),
        -15.0,
        15.0,
    )
    .unwrap()
    .edge()
    .unwrap()
    .into_shape();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&sphere, &edge]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "sphere edge cells: bfillds failed"
    );
    (cells, [sphere, edge])
}

#[test]
fn CellsTest__J4() {
    let (mut cells, shapes) = sphere_edge_cells();
    cells.add_all_to_result(0, false).unwrap();
    assert_count(
        "tests/boolean/cells_test/J4",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J4",
        cells.shape(),
        ShapeEnum::Edge,
        6,
    );
    assert_volume("tests/boolean/cells_test/J4", cells.shape(), 4188.79);
    assert_length("tests/boolean/cells_test/J4", cells.shape(), 112.832);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J4 cut edge",
        cells.shape(),
        ShapeEnum::Edge,
        2,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[&shapes[1]], 0);
    assert_count(
        "tests/boolean/cells_test/J4 cut solid",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0], &shapes[1]], &[], 0);
    assert_count(
        "tests/boolean/cells_test/J4 common",
        cells.shape(),
        ShapeEnum::Edge,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0]], &[], 1, true);
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        "tests/boolean/cells_test/J4 fuse",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );
    assert_count(
        "tests/boolean/cells_test/J4 fuse",
        cells.shape(),
        ShapeEnum::Edge,
        6,
    );
}

fn cylinder_surface() -> Handle<dyn GeomSurface> {
    Handle::new(GeomCylindricalSurface::new(Axis3::from_axis2(gp::xoy()), 10.0).unwrap())
        .into_cast()
}

fn cylinder_face_edge(range: Option<(f64, f64)>) -> (BOPAlgoCellsBuilder, [Shape; 2]) {
    let surface = cylinder_surface();
    let face = BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0.0, 2.0 * PI, -20.0, 20.0),
        precision::confusion(),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let curve = match range {
        Some(_) => surface.get().unwrap().u_iso(PI),
        None => surface.get().unwrap().v_iso(0.0),
    }
    .unwrap();
    let edge = match range {
        Some((first, last)) => BRepBuilderAPIMakeEdge::from_curve_params(&curve, first, last),
        None => BRepBuilderAPIMakeEdge::from_curve(&curve),
    }
    .unwrap()
    .edge()
    .unwrap()
    .into_shape();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&face, &edge]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "cylinder face edge cells: bfillds failed"
    );
    (cells, [face, edge])
}

fn cylinder_face_edge_case(
    source: &str,
    range: Option<(f64, f64)>,
    edge_count: usize,
    length: f64,
) {
    let (mut cells, shapes) = cylinder_face_edge(range);
    cells.add_all_to_result(0, false).unwrap();
    assert_count(source, cells.shape(), ShapeEnum::Face, 2);
    assert_count(source, cells.shape(), ShapeEnum::Edge, edge_count);
    assert_area(source, cells.shape(), 2513.27);
    assert_length(source, cells.shape(), length);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[1]], &[&shapes[0]], 0);
    assert_count(
        source,
        cells.shape(),
        ShapeEnum::Edge,
        range.map_or(0, |_| 2),
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[&shapes[1]], 0);
    assert_count(source, cells.shape(), ShapeEnum::Face, 2);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0], &shapes[1]], &[], 0);
    assert_count(source, cells.shape(), ShapeEnum::Edge, 1);

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[], 0);
    add(&mut cells, &[&shapes[1]], &[], 0);
    assert_count(source, cells.shape(), ShapeEnum::Face, 2);
    assert_count(source, cells.shape(), ShapeEnum::Edge, edge_count);
}

#[test]
fn CellsTest__J5() {
    cylinder_face_edge_case("tests/boolean/cells_test/J5", None, 5, 394.159);
}

#[test]
fn CellsTest__J6() {
    cylinder_face_edge_case(
        "tests/boolean/cells_test/J6",
        Some((-25.0, 25.0)),
        8,
        335.664,
    );
}
