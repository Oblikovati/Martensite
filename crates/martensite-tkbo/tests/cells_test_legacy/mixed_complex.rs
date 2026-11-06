use super::multidimensional::{assert_count, assert_volume, direct_child, direct_count};
use super::planar::circle_face_radius;
use super::*;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

fn j3_cells() -> (BOPAlgoCellsBuilder, [Shape; 4]) {
    let first = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let second = BRepPrimAPIMakeBox::at_point(Point3::new(7.0, 0.0, 0.0), 10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let third = BRepPrimAPIMakeBox::at_point(Point3::new(5.0, 0.0, 5.0), 10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let plane = Plane3::new(Point3::new(5.0, 5.0, 5.0), gp::dy()).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-20.0, 20.0, -20.0, 20.0))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&first, &second, &third, &face]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "J3 cells: bfillds failed");
    (cells, [first, second, third, face])
}

#[test]
fn CellsTest__J3() {
    let (mut cells, shapes) = j3_cells();
    cells.add_all_to_result(0, false).unwrap();
    assert_count(
        "tests/boolean/cells_test/J3",
        cells.shape(),
        ShapeEnum::Solid,
        14,
    );
    assert_count(
        "tests/boolean/cells_test/J3",
        cells.shape(),
        ShapeEnum::Face,
        66,
    );
    assert_volume("tests/boolean/cells_test/J3", cells.shape(), 2200.0);
    assert_area("tests/boolean/cells_test/J3", cells.shape(), 4120.0);

    cells
        .remove_from_result(&selected(&[&shapes[3]]), &List::default())
        .unwrap();
    assert_count(
        "tests/boolean/cells_test/J3 solid splits",
        cells.shape(),
        ShapeEnum::Solid,
        14,
    );
    assert_count(
        "tests/boolean/cells_test/J3 solid splits",
        cells.shape(),
        ShapeEnum::Face,
        65,
    );
    assert_area(
        "tests/boolean/cells_test/J3 solid splits",
        cells.shape(),
        2520.0,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[3]], &[], 0);
    assert_count(
        "tests/boolean/cells_test/J3 face splits",
        cells.shape(),
        ShapeEnum::Face,
        8,
    );
    assert_area(
        "tests/boolean/cells_test/J3 face splits",
        cells.shape(),
        1600.0,
    );

    cells.remove_all_from_result().unwrap();
    add(
        &mut cells,
        &[&shapes[3]],
        &[&shapes[0], &shapes[1], &shapes[2]],
        0,
    );
    assert_count(
        "tests/boolean/cells_test/J3 cut face",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );
    assert_area(
        "tests/boolean/cells_test/J3 cut face",
        cells.shape(),
        1380.0,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[], 0);
    assert_count(
        "tests/boolean/cells_test/J3 first solid",
        cells.shape(),
        ShapeEnum::Solid,
        8,
    );
    assert_volume(
        "tests/boolean/cells_test/J3 first solid",
        cells.shape(),
        1000.0,
    );

    cells.remove_all_from_result().unwrap();
    cells.add_all_to_result(0, false).unwrap();
    cells.make_containers().unwrap();
    assert_eq!(
        direct_count(cells.shape(), ShapeEnum::Shell),
        1,
        "tests/boolean/cells_test/J3 containers",
    );
    assert_eq!(
        direct_count(cells.shape(), ShapeEnum::CompSolid),
        1,
        "tests/boolean/cells_test/J3 containers",
    );
    cells
        .remove_from_result(&selected(&[&shapes[0]]), &List::default())
        .unwrap();
    assert_count(
        "tests/boolean/cells_test/J3 removed solid",
        cells.shape(),
        ShapeEnum::Solid,
        6,
    );
    cells
        .remove_from_result(&selected(&[&shapes[3]]), &selected(&[&shapes[0]]))
        .unwrap();
    let shell = direct_child(cells.shape(), ShapeEnum::Shell);
    assert_count(
        "tests/boolean/cells_test/J3 removed face",
        &shell,
        ShapeEnum::Face,
        4,
    );
    cells
        .remove_from_result(&selected(&[&shapes[3]]), &List::default())
        .unwrap();
    assert_eq!(
        direct_count(cells.shape(), ShapeEnum::Shell),
        0,
        "tests/boolean/cells_test/J3 destroyed shell",
    );
    assert_eq!(
        direct_count(cells.shape(), ShapeEnum::CompSolid),
        1,
        "tests/boolean/cells_test/J3 retained compsolid",
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[], 1);
    add(&mut cells, &[&shapes[2]], &[], 1);
    add(&mut cells, &[&shapes[3]], &[&shapes[0], &shapes[2]], 2);
    cells.make_containers().unwrap();
    cells.remove_internal_boundaries().unwrap();
    assert_eq!(direct_count(cells.shape(), ShapeEnum::Solid), 1);
    assert_eq!(direct_count(cells.shape(), ShapeEnum::Face), 1);

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0], &shapes[1]], &[], 1, true);
    assert_count(
        "tests/boolean/cells_test/J3 unified solids",
        cells.shape(),
        ShapeEnum::Solid,
        1,
    );

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0], &shapes[3]], &[], 1, true);
    assert_count(
        "tests/boolean/cells_test/J3 unified face",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );

    cells.remove_all_from_result().unwrap();
    for (index, material) in (0..3).zip(1..=3) {
        add_with_update(&mut cells, &[&shapes[index]], &[], material, true);
    }
    assert_count(
        "tests/boolean/cells_test/J3 material solids",
        cells.shape(),
        ShapeEnum::Solid,
        3,
    );
}

#[test]
fn CellsTest__K1() {
    let shapes = [
        circle_face_radius(Point3::new(0.0, 0.0, 0.0), 10.0),
        circle_face_radius(Point3::new(10.0, 0.0, 0.0), 10.0),
        circle_face_radius(Point3::new(5.0, 9.0, 0.0), 10.0),
    ];
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1], &shapes[2]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "K1 cells: bfillds failed");

    add_with_update(&mut cells, &[&shapes[0]], &[], 1, true);
    add_with_update(&mut cells, &[&shapes[1]], &[], 2, true);
    assert_count(
        "tests/boolean/cells_test/K1 split materials",
        cells.shape(),
        ShapeEnum::Face,
        2,
    );
    assert_area(
        "tests/boolean/cells_test/K1 split materials",
        cells.shape(),
        505.482,
    );

    cells.remove_all_from_result().unwrap();
    add_with_update(&mut cells, &[&shapes[0]], &[], 1, true);
    add_with_update(&mut cells, &[&shapes[1]], &[], 1, true);
    assert_count(
        "tests/boolean/cells_test/K1 shared material",
        cells.shape(),
        ShapeEnum::Face,
        1,
    );
    assert_area(
        "tests/boolean/cells_test/K1 shared material",
        cells.shape(),
        505.482,
    );

    cells.remove_all_from_result().unwrap();
    add(&mut cells, &[&shapes[0]], &[], 1);
    add(&mut cells, &[&shapes[1]], &[&shapes[2]], 1);
    add(&mut cells, &[&shapes[2]], &[&shapes[0]], 2);
    cells.remove_internal_boundaries().unwrap();
    let checker = BRepCheckAnalyzer::new(cells.shape(), true, false, false).unwrap();
    assert!(
        checker.is_valid().unwrap(),
        "tests/boolean/cells_test/K1 validity"
    );
    assert_count(
        "tests/boolean/cells_test/K1",
        cells.shape(),
        ShapeEnum::Edge,
        8,
    );
    assert_count(
        "tests/boolean/cells_test/K1",
        cells.shape(),
        ShapeEnum::Face,
        2,
    );
    assert_area("tests/boolean/cells_test/K1", cells.shape(), 651.238);
}
