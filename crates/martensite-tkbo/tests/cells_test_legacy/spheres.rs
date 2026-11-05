use super::*;

fn cell_case(source: &str, selections: &[(&[usize], &[usize], i32)], expected_area: f64) {
    let shapes = [
        sphere(Point3::new(0.0, 0.0, 10.0)),
        sphere(Point3::new(20.0, 0.0, 10.0)),
        sphere(Point3::new(10.0, 0.0, 0.0)),
    ];
    let arguments = selected(&[&shapes[0], &shapes[1], &shapes[2]]);
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&arguments);
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "{source}: bfillds failed");
    for &(parts, excluded, material) in selections {
        let take: Vec<_> = parts.iter().map(|&index| &shapes[index]).collect();
        let avoid: Vec<_> = excluded.iter().map(|&index| &shapes[index]).collect();
        add(&mut cells, &take, &avoid, material);
    }
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cells.shape(), &mut properties, false, false).unwrap();
    let actual = properties.mass();
    assert!(
        (actual - expected_area).abs() / expected_area <= 1e-2,
        "{source}: area {actual}; expected {expected_area} within 1%"
    );
}

macro_rules! case {
    ($name:ident, $source:literal, $parts:expr, $area:expr) => {
        #[test]
        fn $name() {
            cell_case($source, $parts, $area);
        }
    };
}

case!(
    CellsTest__A1,
    "tests/boolean/cells_test/A1",
    &[(&[0, 2], &[1], 1)],
    1345.27
);
case!(
    CellsTest__A2,
    "tests/boolean/cells_test/A2",
    &[(&[1, 2], &[0], 1)],
    1345.27
);
case!(
    CellsTest__A3,
    "tests/boolean/cells_test/A3",
    &[(&[0, 2], &[1], 1), (&[1, 2], &[0], 1)],
    2690.54
);
case!(
    CellsTest__A4,
    "tests/boolean/cells_test/A4",
    &[(&[0, 1, 2], &[], 1)],
    769.835
);
case!(
    CellsTest__A5,
    "tests/boolean/cells_test/A5",
    &[(&[0, 2], &[], 1)],
    2115.11
);
case!(
    CellsTest__A6,
    "tests/boolean/cells_test/A6",
    &[(&[1, 2], &[], 1)],
    2115.11
);
case!(
    CellsTest__A7,
    "tests/boolean/cells_test/A7",
    &[(&[0, 2], &[], 1), (&[1, 2], &[], 2)],
    3460.38
);
case!(
    CellsTest__A8,
    "tests/boolean/cells_test/A8",
    &[(&[2], &[0, 1], 1)],
    2356.19
);
case!(
    CellsTest__A9,
    "tests/boolean/cells_test/A9",
    &[(&[2], &[1], 1)],
    3701.46
);

case!(
    CellsTest__B1,
    "tests/boolean/cells_test/B1",
    &[(&[2], &[0], 1)],
    3701.46
);
case!(
    CellsTest__B2,
    "tests/boolean/cells_test/B2",
    &[(&[2], &[1], 1), (&[2], &[0], 1)],
    5046.74
);
case!(
    CellsTest__B3,
    "tests/boolean/cells_test/B3",
    &[(&[2], &[], 1)],
    5816.57
);

fn material_case(source: &str, updates: &[bool]) {
    let shapes = [
        sphere(Point3::new(0.0, 0.0, 10.0)),
        sphere(Point3::new(20.0, 0.0, 10.0)),
        sphere(Point3::new(10.0, 0.0, 0.0)),
    ];
    let arguments = selected(&[&shapes[0], &shapes[1], &shapes[2]]);
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&arguments);
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "{source}: bfillds failed");
    let choices = [
        (&[0][..], &[1, 2][..], 1),
        (&[0, 2][..], &[1][..], 1),
        (&[0, 1][..], &[2][..], 2),
        (&[0, 1, 2][..], &[][..], 2),
        (&[1, 2][..], &[0][..], 2),
    ];
    for ((take, avoid, material), update) in choices.into_iter().zip(updates) {
        let take: Vec<_> = take.iter().map(|&index| &shapes[index]).collect();
        let avoid: Vec<_> = avoid.iter().map(|&index| &shapes[index]).collect();
        add_with_update(&mut cells, &take, &avoid, material, *update);
    }
    cells.remove_internal_boundaries().unwrap();
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cells.shape(), &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!(
        (area - 4494.64).abs() / 4494.64 <= 1e-2,
        "{source}: area {area}; expected 4494.64 within 1%"
    );
}

#[test]
fn CellsTest__B5() {
    material_case("tests/boolean/cells_test/B5", &[false; 5]);
}

#[test]
fn CellsTest__B6() {
    material_case(
        "tests/boolean/cells_test/B6",
        &[false, true, false, false, true],
    );
}

#[test]
fn CellsTest__B4() {
    let shapes = [
        sphere(Point3::new(0.0, 0.0, 10.0)),
        sphere(Point3::new(20.0, 0.0, 10.0)),
        sphere(Point3::new(10.0, 0.0, 0.0)),
    ];
    let arguments = selected(&[&shapes[0], &shapes[1], &shapes[2]]);
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&arguments);
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "tests/boolean/cells_test/B4: bfillds failed"
    );
    cells.add_all_to_result(0, false).unwrap();
    cells
        .remove_from_result(
            &selected(&[&shapes[0], &shapes[1], &shapes[2]]),
            &List::default(),
        )
        .unwrap();
    cells
        .remove_from_result(&selected(&[&shapes[2]]), &selected(&[&shapes[1]]))
        .unwrap();
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cells.shape(), &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!(
        (area - 7172.78).abs() / 7172.78 <= 1e-2,
        "tests/boolean/cells_test/B4: area {area}; expected 7172.78 within 1%"
    );
}
