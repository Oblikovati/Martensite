//! Reuse canonical toolkit fixtures in private library tests.
// Other owned/shaded paths remain tested by the toolkit executable, without a copied fixture.
macro_rules! support_unit_tests {
    ($($tests:tt)*) => {};
}
#[allow(dead_code, unused_imports)]
#[path = "../tests/support/hatch_scene.rs"]
pub(crate) mod scene;
