//! Reuse canonical toolkit fixtures in private library tests.
// Other owned/shaded paths remain tested by the toolkit executable, without a copied fixture.
#[allow(dead_code, unused_imports)]
#[path = "../tests/support/hatch_scene.rs"]
pub(crate) mod scene;

// Canonical nested source controls use the toolkit's device selector.
#[path = "../examples/support/device_options.rs"]
pub(crate) mod device_options;
