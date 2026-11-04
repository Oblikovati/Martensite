//! User GLSL programs (Graphic3dShaderProgram) as precompiled SPIR-V: the program key, the
//! per-driver module library with its typed refusal of GLSL source, module reflection, and the
//! per-draw uniform block filled from the scene state and the program variables.
mod frag_color_output;
mod library;
mod phong_sample;
mod program_draw;
mod program_variables;
mod source_key;
mod spirv_broadcast;
#[cfg(test)]
mod spirv_broadcast_tests;
mod spirv_reflect;
#[cfg(test)]
mod spirv_reflect_tests;
mod uniform_values;
#[cfg(test)]
mod uniform_values_tests;
pub(crate) use frag_color_output::broadcasts_frag_color;
pub(crate) use library::refuse_user_program;
pub use library::{RenderUserProgramFailure, RenderUserProgramLibrary, UserProgramModules};
pub use phong_sample::phong_shading_program;
pub(crate) use phong_sample::register_samples;
pub use program_draw::UserProgramDraw;
pub(crate) use program_variables::program_variables;
pub use source_key::UserProgramKey;
pub(crate) use spirv_broadcast::broadcast_frag_color;
pub(crate) use spirv_reflect::SPIRV_DIM_3D;
pub use spirv_reflect::{
    SamplerBinding, StageReflection, UniformMember, UniformScalar, UniformShape, UserStage,
};
pub use uniform_values::UserUniformScene;
