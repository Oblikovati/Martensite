//! Reuse canonical toolkit fixtures in private library tests.
// Other owned/shaded paths remain tested by the toolkit executable, without a copied fixture.
include!("../tests/support/support_unit_tests_elsewhere.rs");
#[allow(dead_code, unused_imports)]
#[path = "../tests/support/hatch_scene.rs"]
pub(crate) mod scene;
