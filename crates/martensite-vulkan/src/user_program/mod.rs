//! Vulkan precompiled SPIR-V user-program sample and SPIR-V fixture tests (#633).
#[allow(unused_imports)]
pub use martensite_render_core::user_program::*;

mod phong_sample;
#[cfg(test)]
mod spirv_broadcast_tests;
#[cfg(test)]
mod spirv_reflect_tests;

pub use phong_sample::phong_shading_program;
pub(crate) use phong_sample::register_samples;
