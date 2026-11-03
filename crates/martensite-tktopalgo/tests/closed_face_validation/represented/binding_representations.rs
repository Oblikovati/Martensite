//! Spatial and both seam branches retain their actual references and cached values.
use super::{binding::InputBinding, types::*};
use martensite_tkbrep::brep::face_input_capture::CapturedFaceCell as Cell;
pub(super) fn verify(binding: &mut InputBinding<'_>, input: OwnerInput, actual: usize) {
    match input {
        OwnerInput::Spatial {
            location,
            range,
            curve,
        } => {
            common(binding, actual, location, range, 3);
            let Cell::Spatial {
                curve: actual_curve,
            } = binding.cell(actual, |c| matches!(c, Cell::Spatial { .. }))
            else {
                unreachable!()
            };
            binding.owner(curve, actual_curve);
        }
        OwnerInput::Parametric {
            location,
            range,
            curve,
            surface,
            endpoints,
        } => {
            common(binding, actual, location, range, 3);
            branch(binding, actual, curve, surface, endpoints);
        }
        OwnerInput::Seam {
            location,
            range,
            curves,
            surface,
            continuity,
            endpoints,
        } => seam(
            binding, actual, location, range, curves, surface, continuity, endpoints,
        ),
        _ => unreachable!(),
    }
}
fn common(
    binding: &mut InputBinding<'_>,
    actual: usize,
    location: LocationInput,
    range: [u64; 2],
    count: usize,
) {
    binding.count(actual, count);
    let Cell::Range(actual_range) = binding.cell(actual, |c| matches!(c, Cell::Range(_))) else {
        unreachable!()
    };
    let Cell::Location(actual_location) = binding.cell(actual, |c| matches!(c, Cell::Location(_)))
    else {
        unreachable!()
    };
    assert_eq!(actual_range.map(f64::to_bits), range);
    binding.location(location, &actual_location);
}
fn branch(
    binding: &mut InputBinding<'_>,
    actual: usize,
    curve: usize,
    surface: usize,
    endpoints: [[u64; 2]; 2],
) {
    let Cell::Parametric {
        curve: actual_curve,
        surface: actual_surface,
        endpoints: actual_uv,
    } = binding.cell(actual, |c| matches!(c, Cell::Parametric { .. }))
    else {
        unreachable!()
    };
    assert_eq!(actual_uv.map(|p| p.map(f64::to_bits)), endpoints);
    binding.owner(curve, actual_curve);
    binding.owner(surface, actual_surface);
}
fn seam(
    binding: &mut InputBinding<'_>,
    actual: usize,
    location: LocationInput,
    range: [u64; 2],
    curves: [usize; 2],
    surface: usize,
    continuity: i32,
    endpoints: [[[u64; 2]; 2]; 2],
) {
    common(binding, actual, location, range, 4);
    branch(binding, actual, curves[0], surface, endpoints[0]);
    let Cell::Seam {
        curve,
        continuity: actual_continuity,
        endpoints: actual_uv,
    } = binding.cell(actual, |c| matches!(c, Cell::Seam { .. }))
    else {
        unreachable!()
    };
    assert_eq!(actual_continuity as i32, continuity);
    assert_eq!(actual_uv.map(|p| p.map(f64::to_bits)), endpoints[1]);
    binding.owner(curves[1], curve);
}
