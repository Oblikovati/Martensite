//! All raw children, flags and local geometry fields must match the owned input.
use super::{binding::InputBinding, topology, types::*};
use martensite_tkbrep::brep::face_input_capture::CapturedFaceCell as Cell;
pub(super) fn verify(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let (state, children) = topology::fields(input).unwrap();
    let Cell::Topology {
        state: actual_state,
        children: actual_children,
    } = binding.cell(actual, |c| matches!(c, Cell::Topology { .. }))
    else {
        unreachable!()
    };
    assert_eq!(state, actual_state);
    assert_eq!(children.len(), actual_children.len());
    for (expected, actual) in children.iter().zip(&actual_children) {
        binding.occurrence(*expected, actual);
    }
    fields(binding, input, actual);
}
fn fields(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    match input {
        OwnerInput::Face {
            surface,
            location,
            tolerance,
            natural,
            ..
        } => face(binding, actual, surface, location, tolerance, natural),
        OwnerInput::Wire { .. } => binding.count(actual, 1),
        OwnerInput::Edge {
            flags,
            tolerance,
            representations,
            ..
        } => edge(binding, actual, flags, tolerance, representations),
        OwnerInput::Vertex {
            point, tolerance, ..
        } => vertex(binding, actual, point, tolerance),
        _ => unreachable!(),
    }
}
fn face(
    binding: &mut InputBinding<'_>,
    actual: usize,
    surface: usize,
    location: LocationInput,
    tolerance: u64,
    natural: bool,
) {
    binding.count(actual, 2);
    let Cell::Face {
        surface: actual_surface,
        location: actual_location,
        tolerance: actual_tolerance,
        natural_restriction,
    } = binding.cell(actual, |c| matches!(c, Cell::Face { .. }))
    else {
        unreachable!()
    };
    assert_eq!(
        (actual_tolerance.to_bits(), natural_restriction),
        (tolerance, natural)
    );
    binding.location(location, &actual_location);
    binding.owner(surface, actual_surface);
}
fn edge(
    binding: &mut InputBinding<'_>,
    actual: usize,
    flags: i32,
    tolerance: u64,
    representations: &[usize],
) {
    binding.count(actual, 2);
    let Cell::Edge {
        flags: actual_flags,
        tolerance: actual_tolerance,
        representations: actual_reps,
    } = binding.cell(actual, |c| matches!(c, Cell::Edge { .. }))
    else {
        unreachable!()
    };
    assert_eq!(
        (actual_flags, actual_tolerance.to_bits()),
        (flags, tolerance)
    );
    binding.references(representations, &actual_reps);
}
fn vertex(binding: &mut InputBinding<'_>, actual: usize, point: [u64; 3], tolerance: u64) {
    binding.count(actual, 2);
    let Cell::Vertex {
        point: actual_point,
        tolerance: actual_tolerance,
    } = binding.cell(actual, |c| matches!(c, Cell::Vertex { .. }))
    else {
        unreachable!()
    };
    assert_eq!(actual_point.map(f64::to_bits), point);
    assert_eq!(actual_tolerance.to_bits(), tolerance);
}
