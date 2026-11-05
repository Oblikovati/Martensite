use super::*;

fn compound(parts: &[Shape]) -> Shape {
    use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
    let builder = BRepBuilder::default();
    let mut result = Compound::default();
    builder.make_compound(&mut result);
    for part in parts {
        builder.add(&mut result, part).unwrap();
    }
    result.into_shape()
}

fn compound_cells() -> (BOPAlgoCellsBuilder, [Shape; 2]) {
    let first = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let second = BRepPrimAPIMakeBox::at_point(Point3::new(0.0, 30.0, 0.0), 10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let third = BRepPrimAPIMakeBox::at_point(Point3::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let fourth = BRepPrimAPIMakeBox::at_point(Point3::new(30.0, 5.0, 5.0), 10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let shapes = [
        compound(&[first.clone(), second, first]),
        compound(&[third, fourth]),
    ];
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&selected(&[&shapes[0], &shapes[1]]));
    cells.options_mut().set_run_parallel(false);
    cells.perform();
    assert!(
        !cells.options().has_errors(),
        "compound cells: bfillds failed"
    );
    (cells, shapes)
}

fn compound_case(source: &str, take: &[usize], avoid: &[usize], expected_area: f64) {
    let (mut cells, shapes) = compound_cells();
    let take: Vec<_> = take.iter().map(|&index| &shapes[index]).collect();
    let avoid: Vec<_> = avoid.iter().map(|&index| &shapes[index]).collect();
    add(&mut cells, &take, &avoid, 1);
    assert_area(source, cells.shape(), expected_area);
}

#[test]
fn CellsTest__H1() {
    compound_case("tests/boolean/cells_test/H1", &[0], &[], 1350.0);
}

#[test]
fn CellsTest__H2() {
    compound_case("tests/boolean/cells_test/H2", &[1], &[], 1350.0);
}

#[test]
fn CellsTest__H3() {
    compound_case("tests/boolean/cells_test/H3", &[0, 1], &[], 150.0);
}

#[test]
fn CellsTest__H4() {
    compound_case("tests/boolean/cells_test/H4", &[0], &[1], 1200.0);
}
