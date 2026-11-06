//! `nexplode name kind`: the faces, edges or vertices of a shape, bound in the order of their
//! weighted centres.
use super::{commands::distinct_parts, session::Session};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The weight the command sorts by: the centre of the part's boundary, x first.
fn weight(part: &Shape) -> f64 {
    let centre = if part.shape_type().unwrap() == ShapeEnum::Vertex {
        BRepTool::pnt(&topods::vertex(part).unwrap()).unwrap()
    } else {
        let mut properties = GProps::default();
        BRepGProp::linear_properties(part, &mut properties, false, false).unwrap();
        properties.centre_of_mass()
    };
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

/// `nexplode name kind`: binds `name_1...` and returns which found part each one is, from 0.
/// A name that holds no shape answers nothing, as the command does.
/// Example: `nexplode(s, "b", ShapeEnum::face)`.
pub fn nexplode(s: &mut dyn Session, name: &str, kind: ShapeEnum) -> Vec<usize> {
    let Some(shape) = s.shape(name).cloned() else {
        return Vec::new();
    };
    let parts = distinct_parts(&shape, kind);
    let weights: Vec<f64> = parts.iter().map(weight).collect();
    let mut order: Vec<usize> = (0..parts.len()).collect();
    while sort_pass(&mut order, &weights) {}
    warn_equal_weights(&order, &weights);
    for (slot, found) in order.iter().enumerate() {
        s.bind(&format!("{name}_{}", slot + 1), &parts[*found]);
    }
    order
}

/// Equal neighbours after the sort leave the order to the walk, and the command says so.
fn warn_equal_weights(order: &[usize], weights: &[f64]) {
    for pair in order.windows(2) {
        if weights[pair[1]] == weights[pair[0]] {
            println!("Warning! For this shape the results may be incorrect.");
        }
    }
}
