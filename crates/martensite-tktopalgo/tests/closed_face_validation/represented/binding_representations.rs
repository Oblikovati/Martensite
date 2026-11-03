//! Spatial and both seam branches retain their actual references and cached values.
use super::{binding::InputBinding, types::*};
use martensite_tkbrep::brep::face_input_capture::{CapturedFaceCell as Cell, CapturedLocation};
pub(super) fn verify(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    match input {
        OwnerInput::Spatial { .. } => spatial(binding, input, actual),
        OwnerInput::Parametric { .. } => parametric(binding, input, actual),
        OwnerInput::Seam { .. } => seam(binding, input, actual),
        _ => unreachable!(),
    }
}
fn spatial(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Spatial {
        location,
        range,
        curve,
    } = input
    else {
        unreachable!()
    };
    common(binding, actual, location, range, 3);
    let actual_curve = spatial_curve(binding, actual);
    binding.owner(curve, actual_curve);
}
fn spatial_curve(binding: &InputBinding<'_>, actual: usize) -> usize {
    let Cell::Spatial { curve } = binding.cell(actual, |c| matches!(c, Cell::Spatial { .. }))
    else {
        unreachable!()
    };
    curve
}
fn parametric(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Parametric {
        location,
        range,
        curve,
        surface,
        endpoints,
    } = input
    else {
        unreachable!()
    };
    common(binding, actual, location, range, 3);
    branch(binding, actual, curve, surface, endpoints);
}
fn common(
    binding: &mut InputBinding<'_>,
    actual: usize,
    location: LocationInput,
    range: [u64; 2],
    count: usize,
) {
    binding.count(actual, count);
    let actual_range = range_fields(binding, actual);
    let actual_location = location_fields(binding, actual);
    assert_eq!(actual_range.map(f64::to_bits), range);
    binding.location(location, &actual_location);
}
fn range_fields(binding: &InputBinding<'_>, actual: usize) -> [f64; 2] {
    let Cell::Range(range) = binding.cell(actual, |c| matches!(c, Cell::Range(_))) else {
        unreachable!()
    };
    range
}
fn location_fields(binding: &InputBinding<'_>, actual: usize) -> CapturedLocation {
    let Cell::Location(location) = binding.cell(actual, |c| matches!(c, Cell::Location(_))) else {
        unreachable!()
    };
    location
}
fn branch(
    binding: &mut InputBinding<'_>,
    actual: usize,
    curve: usize,
    surface: usize,
    endpoints: [[u64; 2]; 2],
) {
    let (actual_curve, actual_surface, actual_uv) = branch_fields(binding, actual);
    assert_eq!(actual_uv.map(|p| p.map(f64::to_bits)), endpoints);
    binding.owner(curve, actual_curve);
    binding.owner(surface, actual_surface);
}
fn branch_fields(binding: &InputBinding<'_>, actual: usize) -> (usize, usize, [[f64; 2]; 2]) {
    let Cell::Parametric {
        curve,
        surface,
        endpoints,
    } = binding.cell(actual, |c| matches!(c, Cell::Parametric { .. }))
    else {
        unreachable!()
    };
    (curve, surface, endpoints)
}
fn seam(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    seam_first(binding, input, actual);
    seam_second(binding, input, actual);
}
fn seam_first(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Seam {
        location,
        range,
        curves,
        surface,
        endpoints,
        ..
    } = input
    else {
        unreachable!()
    };
    common(binding, actual, location, range, 4);
    branch(binding, actual, curves[0], surface, endpoints[0]);
}
fn seam_second(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    let OwnerInput::Seam {
        curves,
        continuity,
        endpoints,
        ..
    } = input
    else {
        unreachable!()
    };
    let (curve, actual_continuity, actual_uv) = seam_fields(binding, actual);
    assert_eq!(actual_continuity, continuity);
    assert_eq!(actual_uv.map(|p| p.map(f64::to_bits)), endpoints[1]);
    binding.owner(curves[1], curve);
}
fn seam_fields(binding: &InputBinding<'_>, actual: usize) -> (usize, i32, [[f64; 2]; 2]) {
    let Cell::Seam {
        curve,
        continuity,
        endpoints,
    } = binding.cell(actual, |c| matches!(c, Cell::Seam { .. }))
    else {
        unreachable!()
    };
    (curve, continuity as i32, endpoints)
}
