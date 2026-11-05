use super::*;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

pub(crate) fn circle_face(center: Point3) -> Shape {
    circle_face_radius(center, 15.0)
}

pub(crate) fn circle_face_radius(center: Point3, radius: f64) -> Shape {
    let axis = Axis2::new(center, gp::dz()).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_circ(Circle3::new(axis, radius).unwrap())
        .unwrap()
        .edge()
        .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap();
    BRepBuilderAPIMakeFace::from_wire(&wire, false)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn planar_cell_case(
    source: &str,
    choices: &[(&[usize], &[usize], i32)],
    remove_boundaries: bool,
    expected_area: f64,
) {
    let shapes = [
        circle_face(Point3::new(0.0, 10.0, 0.0)),
        circle_face(Point3::new(20.0, 10.0, 0.0)),
        circle_face(Point3::new(10.0, 0.0, 0.0)),
    ];
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1], &shapes[2]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "{source}: bfillds failed");
    for &(take, avoid, material) in choices {
        let take: Vec<_> = take.iter().map(|&index| &shapes[index]).collect();
        let avoid: Vec<_> = avoid.iter().map(|&index| &shapes[index]).collect();
        add(&mut cells, &take, &avoid, material);
    }
    if remove_boundaries {
        cells.remove_internal_boundaries().unwrap();
    }
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cells.shape(), &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!(
        (area - expected_area).abs() / expected_area <= 1e-2,
        "{source}: area {area}; expected {expected_area} within 1%"
    );
}

macro_rules! planar_case {
    ($name:ident, $source:literal, $choices:expr, $remove:expr, $area:expr) => {
        #[test]
        fn $name() {
            planar_cell_case($source, $choices, $remove, $area);
        }
    };
}

planar_case!(
    CellsTest__C1,
    "tests/boolean/cells_test/C1",
    &[(&[0, 2], &[1], 1)],
    false,
    175.992
);
planar_case!(
    CellsTest__C2,
    "tests/boolean/cells_test/C2",
    &[(&[1, 2], &[0], 1)],
    false,
    175.992
);
planar_case!(
    CellsTest__C3,
    "tests/boolean/cells_test/C3",
    &[(&[0, 2], &[1], 1), (&[1, 2], &[0], 1)],
    false,
    351.984
);
planar_case!(
    CellsTest__C4,
    "tests/boolean/cells_test/C4",
    &[(&[0, 1, 2], &[], 1)],
    false,
    122.886
);
planar_case!(
    CellsTest__C5,
    "tests/boolean/cells_test/C5",
    &[(&[0, 2], &[], 1)],
    false,
    298.878
);
planar_case!(
    CellsTest__C6,
    "tests/boolean/cells_test/C6",
    &[(&[1, 2], &[], 1)],
    false,
    298.878
);
planar_case!(
    CellsTest__C7,
    "tests/boolean/cells_test/C7",
    &[(&[0, 2], &[], 1), (&[1, 2], &[], 1)],
    true,
    474.87
);
planar_case!(
    CellsTest__C8,
    "tests/boolean/cells_test/C8",
    &[(&[2], &[0, 1], 1)],
    false,
    231.988
);
planar_case!(
    CellsTest__C9,
    "tests/boolean/cells_test/C9",
    &[(&[2], &[1], 1)],
    false,
    407.98
);

planar_case!(
    CellsTest__D1,
    "tests/boolean/cells_test/D1",
    &[(&[2], &[0], 1)],
    false,
    407.98
);
planar_case!(
    CellsTest__D2,
    "tests/boolean/cells_test/D2",
    &[(&[2], &[1], 1), (&[2], &[0], 1)],
    false,
    583.972
);
planar_case!(
    CellsTest__D3,
    "tests/boolean/cells_test/D3",
    &[(&[2], &[], 1)],
    false,
    706.858
);

#[test]
fn CellsTest__D4() {
    let shapes = [
        circle_face(Point3::new(0.0, 10.0, 0.0)),
        circle_face(Point3::new(20.0, 10.0, 0.0)),
        circle_face(Point3::new(10.0, 0.0, 0.0)),
    ];
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1], &shapes[2]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "tests/boolean/cells_test/D4: bfillds failed"
    );
    cells.add_all_to_result(0, false).unwrap();
    cells
        .remove_from_result(
            &selected(&[&shapes[0], &shapes[1], &shapes[2]]),
            &List::default(),
        )
        .unwrap();
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cells.shape(), &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!(
        (area - 1367.94).abs() / 1367.94 <= 1e-2,
        "tests/boolean/cells_test/D4: area {area}; expected 1367.94 within 1%"
    );
}

#[test]
fn CellsTest__D5() {
    let shapes = [
        circle_face(Point3::new(0.0, 10.0, 0.0)),
        circle_face(Point3::new(20.0, 10.0, 0.0)),
        circle_face(Point3::new(10.0, 0.0, 0.0)),
    ];
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1], &shapes[2]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(!cells.options().has_errors(), "D5 cells: bfillds failed");
    for &(take, avoid, material) in [
        (&[0][..], &[1, 2][..], 1),
        (&[0, 2][..], &[1][..], 1),
        (&[0, 1][..], &[2][..], 2),
        (&[0, 1, 2][..], &[][..], 2),
        (&[1, 2][..], &[0][..], 2),
    ]
    .iter()
    {
        let take: Vec<_> = take.iter().map(|&index| &shapes[index]).collect();
        let avoid: Vec<_> = avoid.iter().map(|&index| &shapes[index]).collect();
        add(&mut cells, &take, &avoid, material);
    }
    cells.remove_internal_boundaries().unwrap();
    let checker = BRepCheckAnalyzer::new(cells.shape(), true, false, false).unwrap();
    assert!(
        checker.is_valid().unwrap(),
        "tests/boolean/cells_test/D5 validity"
    );
    assert_area("tests/boolean/cells_test/D5", cells.shape(), 882.85);
}
