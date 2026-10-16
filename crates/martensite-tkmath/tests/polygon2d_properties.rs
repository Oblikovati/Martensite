//! 2D polygon properties: area, perimeter and node insertion.
//!
//! `Polygon2D` builds the polygon, `poly::polygon_properties` reads its properties
//! and the polygon is rebuilt with one more node; the `checkreal`
//! assertions keep their tolerances.
use martensite_tkernel::ncollection::Array1;
use martensite_tkmath::gp::Point2;
use martensite_tkmath::poly::{Polygon2D, polygon_properties};

/// The combined absolute and relative comparison fails when |value - expected| > tol_abs + tol_rel * |expected|.
fn checkreal(name: &str, value: f64, expected: f64, tol_abs: f64, tol_rel: f64) {
    assert!(
        (value - expected).abs() <= tol_abs + tol_rel * expected.abs(),
        "Error: {name} = {value} is not equal to expected {expected}"
    );
}

/// `polygon2d name n x1 y1 ...`: a polygon of the given nodes, numbered from 1.
fn polygon2d(coordinates: &[(f64, f64)]) -> Polygon2D {
    let mut nodes = Array1::with_bounds(1, coordinates.len() as i32).unwrap();
    for (node, &(x, y)) in nodes.as_mut_slice().iter_mut().zip(coordinates) {
        *node = Point2::new(x, y);
    }
    Polygon2D::from_nodes(&nodes)
}

/// `addpolygonnode name x y`: a new polygon with the old nodes and `(x, y)` appended.
fn add_polygon_node(polygon: &Polygon2D, x: f64, y: f64) -> Polygon2D {
    let old = polygon.nodes();
    let mut nodes = Array1::with_bounds(old.lower(), old.upper() + 1).unwrap();
    nodes.as_mut_slice()[..old.len()].clone_from_slice(old.as_slice());
    *nodes.as_mut_slice().last_mut().unwrap() = Point2::new(x, y);
    Polygon2D::from_nodes(&nodes)
}

/// `polygonprops name -area a -perimeter p`.
fn polygon_props(polygon: &Polygon2D) -> (f64, f64) {
    polygon_properties(polygon.nodes().as_slice())
}

/// tests/geometry/2dpolygon/A1: the unit right triangle, open and then closed by
/// repeating its first node; the area and perimeter must not change.
#[test]
fn geometry_2dpolygon_a1() {
    let result = polygon2d(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
    let (area1, perimeter1) = polygon_props(&result);
    checkreal("Area1", area1, 0.5, 1.0e-14, 0.0);
    checkreal("Perimeter1", perimeter1, 2.0 + 2.0_f64.sqrt(), 1.0e-7, 0.0);
    let result = add_polygon_node(&result, 0.0, 0.0);
    let (area2, perimeter2) = polygon_props(&result);
    checkreal("Area2", area2, 0.5, 1.0e-14, 0.0);
    checkreal("Perimeter2", perimeter2, 2.0 + 2.0_f64.sqrt(), 1.0e-7, 0.0);
}
