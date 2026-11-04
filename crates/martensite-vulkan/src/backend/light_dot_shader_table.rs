//! Generated stage groups share exact byte identities with the canonical recipe catalog.
use super::fragment_variant_registry::FragmentVariant;
macro_rules! numeric_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/", $name)).as_slice(),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/numeric-max-v2.",
                $name
            ))
            .as_slice(),
        )
    };
}
mod initial;
mod ordinary;
mod peeling;
mod vertices;
mod weighted;
pub(super) const FRAGMENT_GROUPS: &[&[FragmentVariant]] = &[
    ordinary::MODULES,
    initial::MODULES,
    weighted::MODULES,
    peeling::MODULES,
];
pub(super) const VERTEX_MODULES: &[FragmentVariant] = vertices::MODULES;
