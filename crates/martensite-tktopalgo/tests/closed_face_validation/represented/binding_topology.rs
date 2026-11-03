//! All raw children, flags and local geometry fields must match the owned input.
use super::{binding::InputBinding, topology, types::*};
use martensite_tkbrep::brep::face_input_capture::{
    CapturedFaceCell as Cell, CapturedLocation, CapturedOccurrence,
};
pub(super) fn verify(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let (state, children) = topology::fields(input).unwrap();
    let (actual_state, actual_children) = topology_fields(binding, actual);
    assert_eq!(state, actual_state);
    assert_eq!(children.len(), actual_children.len());
    for (expected, actual) in children.iter().zip(&actual_children) {
        binding.occurrence(*expected, actual);
    }
    fields(binding, input, actual);
}
fn topology_fields(binding: &InputBinding<'_>, actual: usize) -> (u16, Vec<CapturedOccurrence>) {
    let Cell::Topology { state, children } =
        binding.cell(actual, |c| matches!(c, Cell::Topology { .. }))
    else {
        unreachable!()
    };
    (state, children)
}
fn fields(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    match input {
        OwnerInput::Face { .. } => face(binding, input, actual),
        OwnerInput::Wire { .. } => binding.count(actual, 1),
        OwnerInput::Edge { .. } => edge(binding, input, actual),
        OwnerInput::Vertex { .. } => vertex(binding, input, actual),
        _ => unreachable!(),
    }
}
fn face(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Face {
        surface,
        location,
        tolerance,
        natural,
        ..
    } = input
    else {
        unreachable!()
    };
    binding.count(actual, 2);
    let (actual_surface, actual_location, actual_tolerance, natural_restriction) =
        face_fields(binding, actual);
    face_limits(actual_tolerance, natural_restriction, tolerance, natural);
    binding.location(location, &actual_location);
    binding.owner(surface, actual_surface);
}
fn face_limits(actual_tolerance: f64, natural_restriction: bool, tolerance: u64, natural: bool) {
    assert_eq!(
        (actual_tolerance.to_bits(), natural_restriction),
        (tolerance, natural)
    );
}

fn face_fields(binding: &InputBinding<'_>, actual: usize) -> (usize, CapturedLocation, f64, bool) {
    let Cell::Face {
        surface,
        location,
        tolerance,
        natural_restriction,
    } = binding.cell(actual, |c| matches!(c, Cell::Face { .. }))
    else {
        unreachable!()
    };
    (surface, location, tolerance, natural_restriction)
}
fn edge(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Edge {
        flags,
        tolerance,
        representations,
        ..
    } = input
    else {
        unreachable!()
    };
    binding.count(actual, 2);
    let (actual_flags, actual_tolerance, actual_reps) = edge_fields(binding, actual);
    assert_eq!(
        (actual_flags, actual_tolerance.to_bits()),
        (flags, tolerance)
    );
    binding.references(representations, &actual_reps);
}
fn edge_fields(binding: &InputBinding<'_>, actual: usize) -> (i32, f64, Vec<usize>) {
    let Cell::Edge {
        flags,
        tolerance,
        representations,
    } = binding.cell(actual, |c| matches!(c, Cell::Edge { .. }))
    else {
        unreachable!()
    };
    (flags, tolerance, representations)
}
fn vertex(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Vertex {
        point, tolerance, ..
    } = input
    else {
        unreachable!()
    };
    binding.count(actual, 2);
    let (actual_point, actual_tolerance) = vertex_fields(binding, actual);
    assert_eq!(actual_point.map(f64::to_bits), point);
    assert_eq!(actual_tolerance.to_bits(), tolerance);
}
fn vertex_fields(binding: &InputBinding<'_>, actual: usize) -> ([f64; 3], f64) {
    let Cell::Vertex { point, tolerance } =
        binding.cell(actual, |c| matches!(c, Cell::Vertex { .. }))
    else {
        unreachable!()
    };
    (point, tolerance)
}
