//! Frozen correspondence is verified without inverse arithmetic or coordinate matching.
use super::*;
use crate::surface_sketch::encoding;
pub(super) struct Physical {
    pub bytes: Vec<u8>,
    pub bindings: Vec<(BindingKey, Shape)>,
}
pub(super) fn place(family: &family::UnqualifiedFamily, plan: &Plan) -> Checked<Physical> {
    let (root, bindings) = at(
        "canonical decode",
        pipeline::decode(&family.bytes, &family.keys),
    )?;
    let location = Location::new(plan.transform);
    let root = at("root placement", root.moved(&location, true))?;
    let bindings = bindings
        .into_iter()
        .map(|(key, shape)| Ok((key, at("semantic placement", shape.moved(&location, true))?)))
        .collect::<Checked<Vec<_>>>()?;
    let bytes = encoding::encode(&root, &bindings).map_err(encoding_failure)?;
    let (root, bindings) = at("physical decode", pipeline::decode(&bytes, &family.keys))?;
    correspondence(family, plan, &root, &bindings)?;
    Ok(Physical { bytes, bindings })
}
fn correspondence(
    family: &family::UnqualifiedFamily,
    plan: &Plan,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<()> {
    check_location(root.location(), plan)?;
    let canonical_root = strip(root)?;
    let canonical_bindings = strip_bindings(root.location(), bindings)?;
    let bytes = encoding::encode(&canonical_root, &canonical_bindings).map_err(encoding_failure)?;
    compare_archive(&bytes, &family.bytes)
}
fn compare_archive(actual: &[u8], expected: &[u8]) -> Checked<()> {
    if actual == expected {
        return Ok(());
    }
    let offset = actual
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    Err(fail(PlacementFailure::FrozenGraph {
        actual_length: actual.len(),
        expected_length: expected.len(),
        offset,
        actual: actual.get(offset).copied(),
        expected: expected.get(offset).copied(),
    }))
}
fn strip(shape: &Shape) -> Checked<Shape> {
    at(
        "strip placement",
        shape.located(&Location::default(), false),
    )
}
fn strip_bindings(
    location: &Location,
    bindings: &[(BindingKey, Shape)],
) -> Checked<Vec<(BindingKey, Shape)>> {
    bindings
        .iter()
        .map(|(key, shape)| {
            if !location.is_equal(shape.location()) {
                return Err(fail(PlacementFailure::SemanticDatum(*key)));
            }
            Ok((*key, strip(shape)?))
        })
        .collect()
}
fn check_location(location: &Location, plan: &Plan) -> Checked<()> {
    let identity = location.is_identity();
    let power = if identity {
        None
    } else {
        Some(at("datum power", location.first_power())?)
    };
    let tail_identity = location.next_location().is_identity();
    if identity || power != Some(1) || !tail_identity {
        return Err(fail(PlacementFailure::LocationChain {
            identity,
            power,
            tail_identity,
        }));
    }
    check_coefficients(location.transformation(), plan.transform)
}
fn check_coefficients(actual: Transform3, expected: Transform3) -> Checked<()> {
    if actual.scale_factor() != 1. || coefficient_bits(actual) != coefficient_bits(expected) {
        return Err(fail(PlacementFailure::Coefficients {
            actual: coefficients(actual),
            expected: coefficients(expected),
            scale: actual.scale_factor(),
        }));
    }
    Ok(())
}
fn coefficients(transform: Transform3) -> [[f64; 4]; 3] {
    std::array::from_fn(|i| {
        let [a, b, c] = transform.matrix().coefficients()[i];
        [a, b, c, transform.translation_part().coordinates()[i]]
    })
}
fn coefficient_bits(transform: Transform3) -> [[u64; 4]; 3] {
    coefficients(transform).map(|row| row.map(f64::to_bits))
}
fn encoding_failure(
    cause: crate::surface_sketch::failure::SurfaceFailureCause,
) -> WrappedNormalFailure {
    fail(PlacementFailure::Encoding(cause))
}
#[cfg(test)]
#[path = "freeze_tests.rs"]
mod tests;
