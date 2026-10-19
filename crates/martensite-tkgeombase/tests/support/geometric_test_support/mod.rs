//! Shared geometry fixtures and assertions for the owned numerical tests.
#[path = "assertions.rs"]
mod assertions;
#[path = "parameters.rs"]
mod parameters;
#[path = "planar_fixture.rs"]
mod planar_fixture;
#[allow(unused_imports)] // Each retained test imports its required subset of this interface.
pub use assertions::{EXPECT_DOUBLE_EQ, EXPECT_NEAR, EXPECT_THROW};
#[allow(unused_imports)]
pub use parameters::CreateUniformParams;
#[allow(unused_imports)]
pub use planar_fixture::modeling_circle;
