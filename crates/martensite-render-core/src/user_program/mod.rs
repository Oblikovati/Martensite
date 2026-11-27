//! User GLSL/WGSL programs (Graphic3dShaderProgram): program keys, module library,
//! SPIR-V reflection, and per-draw uniform blocks.
mod frag_color_output;
mod library;
mod program_draw;
mod program_variables;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod source_key;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod spirv_broadcast;
#[doc(hidden)]
#[allow(missing_docs)]
pub mod spirv_reflect;
mod uniform_values;
#[cfg(test)]
mod uniform_values_tests;
pub use frag_color_output::broadcasts_frag_color;
pub use library::refuse_user_program;
pub use library::{
    RenderUserProgramFailure, RenderUserProgramLibrary, UserProgramModules, WgslUserProgram,
};
pub use program_draw::UserProgramDraw;
pub use program_variables::program_variables;
pub use source_key::UserProgramKey;
pub use spirv_broadcast::broadcast_frag_color;
pub use spirv_reflect::SPIRV_DIM_3D;
pub use spirv_reflect::{
    SamplerBinding, StageReflection, UniformMember, UniformScalar, UniformShape, UserStage,
};
pub use uniform_values::UserUniformScene;
