use super::*;

fn four_solids() -> [Shape; 4] {
    [
        BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
            .unwrap()
            .shape()
            .unwrap()
            .clone(),
        BRepPrimAPIMakeSphere::new(12.0)
            .unwrap()
            .shape()
            .unwrap()
            .clone(),
        BRepPrimAPIMakeCylinder::new(10.0, 20.0)
            .shape()
            .unwrap()
            .clone(),
        BRepPrimAPIMakeCone::new(10.0, 0.0, 30.0)
            .unwrap()
            .shape()
            .unwrap()
            .clone(),
    ]
}

fn solid_cells() -> (BOPAlgoCellsBuilder, [Shape; 4]) {
    let shapes = four_solids();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1], &shapes[2], &shapes[3]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "four solid cells: bfillds failed"
    );
    (cells, shapes)
}

fn split_cylinder_cells() -> (BOPAlgoCellsBuilder, [Shape; 3]) {
    let cylinder = BRepPrimAPIMakeCylinder::new(10.0, 20.0)
        .shape()
        .unwrap()
        .clone();
    let plane = Plane3::new(Point3::default(), gp::dx()).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-20.0, 20.0, -20.0, 20.0))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut splitter = BOPAlgoCellsBuilder::new();
    splitter.set_arguments(&selected(&[&cylinder, &face]));
    splitter.options_mut().set_run_parallel(false);
    splitter.perform();
    assert!(
        !splitter.options().has_errors(),
        "cylinder split: bfillds failed"
    );
    let parts: Vec<_> = Explorer::new(splitter.all_parts(), ShapeEnum::Solid, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert_eq!(parts.len(), 2, "cylinder split must produce two solids");
    let small = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(0.0, -10.0, 0.0), gp::dz()).unwrap(),
        5.0,
        20.0,
    )
    .shape()
    .unwrap()
    .clone();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&parts[0], &parts[1], &small]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "split cylinder cells: bfillds failed"
    );
    (cells, [parts[0].clone(), parts[1].clone(), small])
}

fn solid_selection_case(source: &str, parts: &[usize], material: i32, update: bool, expected: f64) {
    let (mut cells, shapes) = solid_cells();
    let take: Vec<_> = parts.iter().map(|&index| &shapes[index]).collect();
    add_with_update(&mut cells, &take, &[], material, update);
    assert_area(source, cells.shape(), expected);
}

#[test]
fn CellsTest__E1() {
    let (mut cells, _) = solid_cells();
    cells.add_all_to_result(0, false).unwrap();
    assert_area("tests/boolean/cells_test/E1", cells.shape(), 7729.72);
}

#[test]
fn CellsTest__E2() {
    let (mut cells, _) = solid_cells();
    cells.add_all_to_result(1, true).unwrap();
    assert_area("tests/boolean/cells_test/E2", cells.shape(), 2689.14);
}

#[test]
fn CellsTest__E3() {
    solid_selection_case(
        "tests/boolean/cells_test/E3",
        &[0, 1, 2, 3],
        1,
        false,
        417.967,
    );
}

#[test]
fn CellsTest__E4() {
    solid_selection_case("tests/boolean/cells_test/E4", &[0], 1, false, 1416.31);
}

#[test]
fn CellsTest__E5() {
    solid_selection_case("tests/boolean/cells_test/E5", &[1], 1, false, 4993.56);
}

#[test]
fn CellsTest__E6() {
    solid_selection_case("tests/boolean/cells_test/E6", &[2], 1, false, 5017.47);
}

#[test]
fn CellsTest__E7() {
    solid_selection_case("tests/boolean/cells_test/E7", &[3], 1, false, 2086.49);
}

#[test]
fn CellsTest__E8() {
    let (mut cells, shapes) = solid_cells();
    cells.add_all_to_result(0, false).unwrap();
    cells
        .remove_from_result(
            &selected(&[&shapes[0], &shapes[1], &shapes[2], &shapes[3]]),
            &List::default(),
        )
        .unwrap();
    assert_area("tests/boolean/cells_test/E8", cells.shape(), 7311.75);
}

#[test]
fn CellsTest__E9() {
    solid_selection_case("tests/boolean/cells_test/E9", &[0, 1], 1, true, 502.043);
}

fn solid_cells_case(
    source: &str,
    take_indices: &[usize],
    avoid_indices: &[usize],
    material: i32,
    update: bool,
    expected_area: f64,
) {
    let (mut cells, shapes) = solid_cells();
    let take: Vec<_> = take_indices.iter().map(|&index| &shapes[index]).collect();
    let avoid: Vec<_> = avoid_indices.iter().map(|&index| &shapes[index]).collect();
    add_with_update(&mut cells, &take, &avoid, material, update);
    assert_area(source, cells.shape(), expected_area);
}

macro_rules! solid_case {
    ($name:ident, $source:literal, $take:expr, $avoid:expr, $material:expr, $update:expr, $area:expr) => {
        #[test]
        fn $name() {
            solid_cells_case($source, $take, $avoid, $material, $update, $area);
        }
    };
}

solid_case!(
    CellsTest__F1,
    "tests/boolean/cells_test/F1",
    &[0, 2],
    &[],
    1,
    true,
    514.159
);
solid_case!(
    CellsTest__F2,
    "tests/boolean/cells_test/F2",
    &[0, 3],
    &[],
    1,
    true,
    418.093
);
solid_case!(
    CellsTest__F3,
    "tests/boolean/cells_test/F3",
    &[1, 2],
    &[],
    1,
    true,
    1135.58
);
solid_case!(
    CellsTest__F4,
    "tests/boolean/cells_test/F4",
    &[1, 3],
    &[],
    1,
    true,
    1017.77
);
solid_case!(
    CellsTest__F5,
    "tests/boolean/cells_test/F5",
    &[2, 3],
    &[],
    1,
    true,
    1232.14
);
solid_case!(
    CellsTest__F6,
    "tests/boolean/cells_test/F6",
    &[0],
    &[1, 2, 3],
    1,
    false,
    259.774
);
solid_case!(
    CellsTest__F7,
    "tests/boolean/cells_test/F7",
    &[1],
    &[0, 2, 3],
    1,
    false,
    2067.69
);
solid_case!(
    CellsTest__F8,
    "tests/boolean/cells_test/F8",
    &[2],
    &[0, 1, 3],
    1,
    false,
    1640.54
);
solid_case!(
    CellsTest__F9,
    "tests/boolean/cells_test/F9",
    &[3],
    &[0, 1, 2],
    1,
    false,
    145.291
);

#[test]
fn CellsTest__G1() {
    let (mut cells, shapes) = solid_cells();
    for &(take_index, avoid_indices) in [
        (0, &[1, 2, 3][..]),
        (1, &[0, 2, 3][..]),
        (2, &[0, 1, 3][..]),
        (3, &[0, 1, 2][..]),
    ]
    .iter()
    {
        let avoid: Vec<_> = avoid_indices.iter().map(|&index| &shapes[index]).collect();
        add(&mut cells, &[&shapes[take_index]], &avoid, 1);
    }
    assert_area("tests/boolean/cells_test/G1", cells.shape(), 4113.3);
}

fn removed_solid_case(source: &str, selected_index: usize, expected_area: f64) {
    let (mut cells, shapes) = solid_cells();
    add(&mut cells, &[&shapes[selected_index]], &[], 1);
    let avoid: Vec<_> = (0..shapes.len())
        .filter(|&index| index != selected_index)
        .map(|index| &shapes[index])
        .collect();
    cells
        .remove_from_result(&selected(&[&shapes[selected_index]]), &selected(&avoid))
        .unwrap();
    cells.remove_internal_boundaries().unwrap();
    assert_area(source, cells.shape(), expected_area);
}

#[test]
fn CellsTest__G2() {
    removed_solid_case("tests/boolean/cells_test/G2", 0, 545.266);
}

#[test]
fn CellsTest__G3() {
    removed_solid_case("tests/boolean/cells_test/G3", 1, 1166.69);
}

#[test]
fn CellsTest__G4() {
    removed_solid_case("tests/boolean/cells_test/G4", 2, 1393.05);
}

#[test]
fn CellsTest__G5() {
    removed_solid_case("tests/boolean/cells_test/G5", 3, 1232.14);
}

#[test]
fn CellsTest__G6() {
    let (mut cells, shapes) = solid_cells();
    cells.add_all_to_result(1, false).unwrap();
    for index in 0..shapes.len() {
        let avoid: Vec<_> = (0..shapes.len())
            .filter(|&other| other != index)
            .map(|other| &shapes[other])
            .collect();
        cells
            .remove_from_result(&selected(&[&shapes[index]]), &selected(&avoid))
            .unwrap();
    }
    cells.remove_internal_boundaries().unwrap();
    assert_area("tests/boolean/cells_test/G6", cells.shape(), 1424.16);
}

#[test]
fn CellsTest__G7() {
    let (mut cells, shapes) = solid_cells();
    for (index, material) in (0..shapes.len()).zip(1..=4) {
        add(&mut cells, &[&shapes[index]], &[], material);
    }
    cells.remove_internal_boundaries().unwrap();
    assert_area("tests/boolean/cells_test/G7", cells.shape(), 4326.81);
}

#[test]
fn CellsTest__G8() {
    let (mut cells, shapes) = solid_cells();
    cells.add_all_to_result(0, false).unwrap();
    cells.make_containers().unwrap();
    cells
        .remove_from_result(&selected(&[&shapes[0]]), &List::default())
        .unwrap();
    add(&mut cells, &[&shapes[0], &shapes[1]], &[], 0);
    assert_area("tests/boolean/cells_test/G8", cells.shape(), 7299.17);
}

#[test]
fn CellsTest__I1() {
    let (mut cells, shapes) = split_cylinder_cells();
    add(&mut cells, &[&shapes[1]], &[&shapes[0], &shapes[2]], 0);
    assert_area("tests/boolean/cells_test/I1", cells.shape(), 1238.14);
}

#[test]
fn CellsTest__I2() {
    let (mut cells, shapes) = split_cylinder_cells();
    cells.add_all_to_result(1, false).unwrap();
    cells
        .remove_from_result(
            &selected(&[&shapes[1]]),
            &selected(&[&shapes[0], &shapes[2]]),
        )
        .unwrap();
    add(&mut cells, &[&shapes[1]], &[&shapes[0], &shapes[2]], 0);
    cells.remove_internal_boundaries().unwrap();
    cells.make_containers().unwrap();
    assert_area("tests/boolean/cells_test/I2", cells.shape(), 2998.06);
}
