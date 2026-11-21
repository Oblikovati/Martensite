//! Frozen correspondence is verified without inverse arithmetic or coordinate matching.
use super::{BindingKey, Failure, Shape};
use crate::surface_sketch::{encoding, failure::SurfaceFailureCause};
use martensite_tkmath::{gp::Transform3, toploc::Location};

pub(super) type DecodedGraph = (Shape, Vec<(BindingKey, Shape)>);
pub(super) struct PhysicalArchive {
    pub bytes: Vec<u8>,
    pub root: Shape,
    pub bindings: Vec<(BindingKey, Shape)>,
}
#[derive(Debug)]
pub(super) enum ArchivePlacementFailure<E> {
    Decode {
        step: &'static str,
        cause: E,
    },
    Native {
        step: &'static str,
        cause: Box<Failure>,
    },
    Encoding(SurfaceFailureCause),
    LocationChain(DatumChain),
    Coefficients(Box<MatrixReadout>),
    SemanticDatum(BindingKey),
    FrozenGraph(GraphReadout),
}
#[derive(Debug)]
pub(super) struct DatumChain {
    pub identity: bool,
    pub power: Option<i32>,
    pub tail_identity: bool,
}
#[derive(Debug)]
pub(super) struct MatrixReadout {
    pub actual: [[f64; 4]; 3],
    pub expected: [[f64; 4]; 3],
    pub scale: f64,
}
#[derive(Debug)]
pub(super) struct GraphReadout {
    pub actual_length: usize,
    pub expected_length: usize,
    pub offset: usize,
    pub actual: Option<u8>,
    pub expected: Option<u8>,
}
type Checked<T, E> = Result<T, ArchivePlacementFailure<E>>;

pub(super) fn place<E>(
    bytes: &[u8],
    keys: &[BindingKey],
    transform: Transform3,
    decode: impl Fn(&[u8], &[BindingKey]) -> Result<DecodedGraph, E>,
) -> Checked<PhysicalArchive, E> {
    let (root, bindings) = decode_at("canonical decode", bytes, keys, &decode)?;
    let location = Location::new(transform);
    let root = native("root placement", root.moved(&location, true))?;
    let bindings = move_bindings(bindings, &location)?;
    let physical = encoding::encode(&root, &bindings).map_err(ArchivePlacementFailure::Encoding)?;
    let (root, bindings) = decode_at("physical decode", &physical, keys, &decode)?;
    let result = PhysicalArchive {
        bytes: physical,
        root,
        bindings,
    };
    correspondence(bytes, transform, &result.root, &result.bindings)?;
    Ok(result)
}

fn move_bindings<E>(
    bindings: Vec<(BindingKey, Shape)>,
    location: &Location,
) -> Checked<Vec<(BindingKey, Shape)>, E> {
    bindings
        .into_iter()
        .map(|(key, shape)| {
            Ok((
                key,
                native("semantic placement", shape.moved(location, true))?,
            ))
        })
        .collect()
}

pub(super) fn correspondence<E>(
    bytes: &[u8],
    transform: Transform3,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<(), E> {
    check_location(root.location(), transform)?;
    let canonical_root = strip(root)?;
    let canonical_bindings = strip_bindings(root.location(), bindings)?;
    let actual = encoding::encode(&canonical_root, &canonical_bindings)
        .map_err(ArchivePlacementFailure::Encoding)?;
    compare_archive(&actual, bytes)
}

pub(super) fn compare_archive<E>(actual: &[u8], expected: &[u8]) -> Checked<(), E> {
    if actual == expected {
        return Ok(());
    }
    let offset = actual
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    Err(ArchivePlacementFailure::FrozenGraph(GraphReadout {
        actual_length: actual.len(),
        expected_length: expected.len(),
        offset,
        actual: actual.get(offset).copied(),
        expected: expected.get(offset).copied(),
    }))
}

fn strip<E>(shape: &Shape) -> Checked<Shape, E> {
    native(
        "strip placement",
        shape.located(&Location::default(), false),
    )
}
pub(super) fn strip_bindings<E>(
    location: &Location,
    bindings: &[(BindingKey, Shape)],
) -> Checked<Vec<(BindingKey, Shape)>, E> {
    bindings
        .iter()
        .map(|(key, shape)| {
            if !location.is_equal(shape.location()) {
                return Err(ArchivePlacementFailure::SemanticDatum(*key));
            }
            Ok((*key, strip(shape)?))
        })
        .collect()
}

pub(super) fn check_location<E>(location: &Location, transform: Transform3) -> Checked<(), E> {
    let identity = location.is_identity();
    let power = if identity {
        None
    } else {
        Some(native("datum power", location.first_power())?)
    };
    let tail_identity = location.next_location().is_identity();
    if identity || power != Some(1) || !tail_identity {
        return Err(ArchivePlacementFailure::LocationChain(DatumChain {
            identity,
            power,
            tail_identity,
        }));
    }
    check_coefficients(location.transformation(), transform)
}
fn check_coefficients<E>(actual: Transform3, expected: Transform3) -> Checked<(), E> {
    if actual.scale_factor() != 1. || coefficient_bits(actual) != coefficient_bits(expected) {
        return Err(ArchivePlacementFailure::Coefficients(Box::new(
            MatrixReadout {
                actual: coefficients(actual),
                expected: coefficients(expected),
                scale: actual.scale_factor(),
            },
        )));
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
fn native<T, E>(step: &'static str, result: Result<T, Failure>) -> Checked<T, E> {
    result.map_err(|cause| ArchivePlacementFailure::Native {
        step,
        cause: Box::new(cause),
    })
}
fn decode_at<E>(
    step: &'static str,
    bytes: &[u8],
    keys: &[BindingKey],
    decode: &impl Fn(&[u8], &[BindingKey]) -> Result<DecodedGraph, E>,
) -> Checked<DecodedGraph, E> {
    decode(bytes, keys).map_err(|cause| ArchivePlacementFailure::Decode { step, cause })
}

pub(super) fn unchanged<E>(
    bytes: &[u8],
    keys: &[BindingKey],
    decode: impl Fn(&[u8], &[BindingKey]) -> Result<DecodedGraph, E>,
) -> Checked<PhysicalArchive, E> {
    let (root, bindings) = decode_at("unchanged canonical decode", bytes, keys, &decode)?;
    let canonical_root = strip(&root)?;
    let canonical_bindings = strip_bindings(root.location(), &bindings)?;
    let actual = encoding::encode(&canonical_root, &canonical_bindings)
        .map_err(ArchivePlacementFailure::Encoding)?;
    compare_archive(&actual, bytes)?;
    Ok(PhysicalArchive {
        bytes: actual,
        root: canonical_root,
        bindings,
    })
}

#[cfg(test)]
mod tests;
