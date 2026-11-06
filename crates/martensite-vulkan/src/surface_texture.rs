//! Sampled surface images stay independent from coverage atlases and point sprites.
use crate::{RenderError, error::rejected};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceImageEncoding {
    Srgb8,
    Linear8,
    Linear16,
    Float16,
    Float32,
    Bc1Rgb(bool),
    Bc1Rgba(bool),
    Bc2(bool),
    Bc3(bool),
}
/// Whether an uncompressed image format uploads with an alpha component (RGBA, BGRA and their
/// float forms, and the alpha-only formats); `format_has_alpha(ImageFormat::Rgba)`.
pub(crate) fn format_has_alpha(format: martensite_tkservice::image::ImageFormat) -> bool {
    use martensite_tkservice::image::ImageFormat::*;
    matches!(
        format,
        Rgba | Bgra | Rgbaf | Bgraf | RgbafHalf | Alpha | AlphaF
    )
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceTextureImage {
    pub lifetime: Arc<crate::surface_texture_lifetime::SurfaceTextureLifetime>,
    pub id: Vec<u8>,
    pub revision: usize,
    pub extent: [u32; 2],
    /// The slice count of a 3D texture (`Graphic3dTypeOfTexture_3D`, sampled as a 3D image);
    /// `None` for a 2D image. `bytes` then holds the slices one after another.
    pub depth: Option<u32>,
    pub encoding: SurfaceImageEncoding,
    pub alpha_only: bool,
    /// The GL texture format has an alpha component (GL_REPLACE then takes the texel alpha).
    pub has_alpha: bool,
    pub bytes: Arc<Vec<u8>>,
    pub top_down: bool,
    pub mipmaps: bool,
    pub supplied_levels: Vec<usize>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceSamplerPolicy {
    pub repeat: bool,
    pub filter: Graphic3dTypeOfTextureFilter,
    pub anisotropy: Graphic3dLevelOfTextureAnisotropy,
    pub first_level: i32,
    pub last_level: i32,
}
impl SurfaceSamplerPolicy {
    pub fn read(parameters: &Graphic3dTextureParams) -> Self {
        Self {
            repeat: parameters.is_repeat(),
            filter: parameters.filter(),
            anisotropy: parameters.aniso_filter(),
            first_level: parameters.base_level(),
            last_level: parameters.max_level(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceTextureStyle {
    pub image: Arc<SurfaceTextureImage>,
    pub pending_texgen: Option<Box<crate::surface_texgen::SurfaceTexgen>>,
    pub fixed_environment: bool,
    pub sampler: SurfaceSamplerPolicy,
    pub uv: Vec<[f32; 4]>,
    pub back_vertex: bool,
    pub vertex_override: bool,
    pub modulate: bool,
    pub native_gen_mode: Graphic3dTypeOfTextureMode,
    /// Emissive, occlusion, normal and metallic-roughness maps of the texture set.
    pub material_maps: Option<Arc<crate::surface_material_maps::SurfaceMaterialMaps>>,
    /// The view's environment texture mapped by the sphere-map vertex stage (the native
    /// `Graphic3dShaderFlags_TextureEnv` program); `uv` is then unused.
    pub sphere_map: Option<Arc<crate::environment_texture_map::SphereMapTransform>>,
}
impl SurfaceTextureStyle {
    pub fn validate(&self, vertices: usize) -> Result<(), RenderError> {
        if self.uv.len() != vertices {
            return Err(rejected(
                (self.uv.len(), vertices),
                "one texture coordinate per vertex",
            ));
        }
        if let Some(uv) = self
            .uv
            .iter()
            .find(|uv| uv.iter().any(|value| !value.is_finite()))
        {
            return Err(rejected(uv, "finite Vec4 texture coordinates"));
        }
        Ok(())
    }
}

/// Explicit legacy incomplete-sampler policy; no image was decoded or uploaded.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceTextureUnavailable {
    pub resource_id: Vec<u8>,
    pub fixed_environment: bool,
    pub modulate: bool,
    pub vertex_override: bool,
    pub back_vertex: bool,
}
