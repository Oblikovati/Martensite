//! Physical publication uses the shared archive correspondence checks.
use super::*;
use crate::surface_sketch::wrapped_profile::frozen_placement::{
    self as archive, ArchivePlacementFailure,
};
pub(super) struct Physical {
    pub bytes: Vec<u8>,
    pub root: Shape,
    pub bindings: Vec<(BindingKey, Shape)>,
}

pub(super) fn place<B: PlacementBudget>(
    family: &family::UnqualifiedFamily,
    plan: &Plan<B>,
) -> Checked<Physical> {
    place_bytes(&family.bytes, &family.keys, plan.transform)
}
pub(super) fn place_bytes(
    bytes: &[u8],
    keys: &[BindingKey],
    transform: Transform3,
) -> Checked<Physical> {
    archive::place(bytes, keys, transform, pipeline::decode)
        .map(physical)
        .map_err(project)
}
pub(super) fn unchanged_bytes(bytes: &[u8], keys: &[BindingKey]) -> Checked<Physical> {
    archive::unchanged(bytes, keys, pipeline::decode)
        .map(physical)
        .map_err(project)
}

pub(super) fn unchanged(family: &family::UnqualifiedFamily) -> Checked<Physical> {
    unchanged_bytes(&family.bytes, &family.keys)
}
pub(super) fn identity(transform: Transform3) -> bool {
    transform.matrix().coefficients() == &[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
        && transform.translation_part().coordinates() == &[0.; 3]
}
fn physical(archive: archive::PhysicalArchive) -> Physical {
    Physical {
        bytes: archive.bytes,
        root: archive.root,
        bindings: archive.bindings,
    }
}

pub(super) fn project(cause: ArchivePlacementFailure<Failure>) -> WrappedNormalFailure {
    use ArchivePlacementFailure as A;
    let cause = match cause {
        A::Decode { step, cause } => PlacementFailure::Native { step, cause },
        A::Native { step, cause } => PlacementFailure::Native {
            step,
            cause: *cause,
        },
        A::Encoding(cause) => PlacementFailure::Encoding(cause),
        A::LocationChain(value) => chain_failure(value),
        A::Coefficients(value) => coefficient_failure(*value),
        A::SemanticDatum(key) => PlacementFailure::SemanticDatum(key),
        A::FrozenGraph(value) => graph_failure(value),
    };
    fail(cause)
}
fn chain_failure(value: archive::DatumChain) -> PlacementFailure {
    PlacementFailure::LocationChain {
        identity: value.identity,
        power: value.power,
        tail_identity: value.tail_identity,
    }
}
fn coefficient_failure(value: archive::MatrixReadout) -> PlacementFailure {
    PlacementFailure::Coefficients {
        actual: value.actual,
        expected: value.expected,
        scale: value.scale,
    }
}
fn graph_failure(value: archive::GraphReadout) -> PlacementFailure {
    PlacementFailure::FrozenGraph {
        actual_length: value.actual_length,
        expected_length: value.expected_length,
        offset: value.offset,
        actual: value.actual,
        expected: value.expected,
    }
}

#[cfg(test)]
#[path = "freeze_tests.rs"]
mod tests;

#[cfg(test)]
fn check_location<B: PlacementBudget>(location: &Location, plan: &Plan<B>) -> Checked<()> {
    archive::check_location(location, plan.transform).map_err(project)
}
#[cfg(test)]
fn strip_bindings(
    location: &Location,
    bindings: &[(BindingKey, Shape)],
) -> Checked<Vec<(BindingKey, Shape)>> {
    archive::strip_bindings(location, bindings).map_err(project)
}
#[cfg(test)]
fn compare_archive(actual: &[u8], expected: &[u8]) -> Checked<()> {
    archive::compare_archive(actual, expected).map_err(project)
}
#[cfg(test)]
fn correspondence<B: PlacementBudget>(
    family: &family::UnqualifiedFamily,
    plan: &Plan<B>,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<()> {
    archive::correspondence(&family.bytes, plan.transform, root, bindings).map_err(project)
}
