//! Geometry references and raw numeric fields are checked without evaluating points.
use super::{binding::InputBinding, spline_binding, types::*};
use martensite_tkbrep::brep::face_input_capture::CapturedFaceCell as Cell;
pub(super) fn verify(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    binding.count(actual, 1);
    match input {
        OwnerInput::Extrusion { basis, direction } => extrusion(binding, actual, basis, direction),
        OwnerInput::Spline3(input) => spline3(binding, actual, input),
        OwnerInput::Spline2(input) => spline2(binding, actual, input),
        OwnerInput::Line3 { origin, direction } => line3(binding, actual, origin, direction),
        OwnerInput::Line2 { origin, direction } => line2(binding, actual, origin, direction),
        OwnerInput::Plane {
            origin,
            x,
            y,
            normal,
        } => plane(binding, actual, origin, x, y, normal),
        _ => unreachable!(),
    }
}
fn extrusion(binding: &mut InputBinding<'_>, actual: usize, basis: usize, direction: [u64; 3]) {
    let Cell::Extrusion {
        basis: actual_basis,
        direction: actual_direction,
    } = binding.cell(actual, |c| matches!(c, Cell::Extrusion { .. }))
    else {
        unreachable!()
    };
    assert_eq!(actual_direction.map(f64::to_bits), direction);
    binding.owner(basis, actual_basis);
}
fn spline3(binding: &InputBinding<'_>, actual: usize, expected: SplineInput<3>) {
    let Cell::Spline(input) = binding.cell(actual, |c| matches!(c, Cell::Spline(_))) else {
        unreachable!()
    };
    spline_binding::spline3(&input, expected);
}
fn spline2(binding: &InputBinding<'_>, actual: usize, expected: SplineInput<2>) {
    let Cell::PlanarSpline(input) = binding.cell(actual, |c| matches!(c, Cell::PlanarSpline(_)))
    else {
        unreachable!()
    };
    spline_binding::spline2(&input, expected);
}
fn line3(binding: &InputBinding<'_>, actual: usize, origin: [u64; 3], direction: [u64; 3]) {
    let Cell::Line(axis) = binding.cell(actual, |c| matches!(c, Cell::Line(_))) else {
        unreachable!()
    };
    let p = axis.location();
    let d = axis.direction();
    assert_eq!([p.x(), p.y(), p.z()].map(f64::to_bits), origin);
    assert_eq!([d.x(), d.y(), d.z()].map(f64::to_bits), direction);
}
fn line2(binding: &InputBinding<'_>, actual: usize, origin: [u64; 2], direction: [u64; 2]) {
    let Cell::PlanarLine(axis) = binding.cell(actual, |c| matches!(c, Cell::PlanarLine(_))) else {
        unreachable!()
    };
    let p = axis.location();
    let d = axis.direction();
    assert_eq!([p.x(), p.y()].map(f64::to_bits), origin);
    assert_eq!([d.x(), d.y()].map(f64::to_bits), direction);
}
fn plane(
    binding: &InputBinding<'_>,
    actual: usize,
    origin: [u64; 3],
    x: [u64; 3],
    y: [u64; 3],
    normal: [u64; 3],
) {
    let Cell::Plane(axis) = binding.cell(actual, |c| matches!(c, Cell::Plane(_))) else {
        unreachable!()
    };
    let p = axis.location();
    let a = axis.x_direction();
    let b = axis.y_direction();
    let n = axis.direction();
    assert_eq!([p.x(), p.y(), p.z()].map(f64::to_bits), origin);
    assert_eq!([a.x(), a.y(), a.z()].map(f64::to_bits), x);
    assert_eq!([b.x(), b.y(), b.z()].map(f64::to_bits), y);
    assert_eq!([n.x(), n.y(), n.z()].map(f64::to_bits), normal);
}
