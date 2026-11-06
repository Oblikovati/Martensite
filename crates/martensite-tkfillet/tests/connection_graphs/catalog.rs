//! Fixed fixture ordering is authored input provenance, not returned topology rank.
#[path = "catalog_0.rs"]
mod first;
#[path = "catalog_1.rs"]
mod second;
use super::spec::FixtureSpec;

/// Iterate the exact22 authored fixtures; `fixtures().count() == 22`.
pub(super) fn fixtures() -> impl Iterator<Item = &'static FixtureSpec> {
    let first = first::FIXTURES.iter();
    first.chain(second::FIXTURES)
}
