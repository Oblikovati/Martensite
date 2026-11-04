//! Ray tracing of GlView_Raytrace: the scene geometry, its two-level quad
//! BVH, materials, lights and uniforms that the ray-tracing passes of backend/raytrace_pass.rs
//! read (shaders/raytrace_*.glsl, ports of RaytraceBase.fs, RaytraceRender.fs and friends).
mod adaptive;
mod bvh_cache;
mod environment;
mod frame;
mod halton_sampler;
mod lights;
mod material;
mod pathtrace;
mod primitive_triangles;
mod scene;
mod scene_buffers;
mod structure_check;
mod textures;
mod tile_sampler;
mod triangle_set;
mod uniforms;
pub(crate) use adaptive::{AdaptivePasses, admit_view_adaptive_sampling};
pub(crate) use bvh_cache::RaytraceBvhCache;
pub(crate) use frame::{RaytraceConstants, RaytraceFrame, RaytraceFrameInputs, prepare_raytrace};
pub use halton_sampler::RaytraceHaltonSampler;
pub(crate) use pathtrace::PathtraceFrame;
pub(crate) use pathtrace::{RaytraceAccumulation, eye_targets};
pub(crate) use primitive_triangles::is_raytraced as is_raytraced_primitive;
pub(crate) use textures::{MAX_TEX_NUMBER, RaytraceTexture, RaytraceTextures};
pub use tile_sampler::{RaytraceTileOffset, RaytraceTileSampler};
