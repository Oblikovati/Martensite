//! `nexplode name f`: the faces of a shape, bound in the order of their weighted centres.
use super::{commands::distinct_parts, session::Session};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The weight the command sorts by: the centre of the part's boundary, x first.
fn weight(part: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(part, &mut properties, false, false).unwrap();
    let centre = properties.centre_of_mass();
    centre.x() * 999. + centre.y() * 99. + centre.z() * 0.9
}

/// The bubble pass the command makes: only strictly greater neighbours swap, so equal weights
/// stay in the order they were found.
fn sort_pass(order: &mut [usize], weights: &[f64]) -> bool {
    let mut swapped = false;
    for at in 1..order.len() {
        if weights[order[at - 1]] > weights[order[at]] {
            order.swap(at - 1, at);
            swapped = true;
        }
    }
    swapped
}

/// `nexplode name f`: binds `name_1...` and returns which found face each one is, from 0.
/// Example: `nexplode_faces(s, "b")`.
pub fn nexplode_faces(s: &mut dyn Session, name: &str) -> Vec<usize> {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("nexplode {name}: expected a bound shape"))
        .clone();
    let parts = distinct_parts(&shape, ShapeEnum::Face);
    let weights: Vec<f64> = parts.iter().map(weight).collect();
    let mut order: Vec<usize> = (0..parts.len()).collect();
    while sort_pass(&mut order, &weights) {}
    for (slot, found) in order.iter().enumerate() {
        s.bind(&format!("{name}_{}", slot + 1), &parts[*found]);
    }
    order
}
