//! The textures of ray-traced materials (GlRaytraceGeometry::AddTexture,
//! HasTextures and MAX_TEX_NUMBER; GlView::convertMaterial's texture branch and
//! buildTextureTransform). Native passes 64-bit bindless handles in `uTextureSamplers`; here each
//! texture is one element of the sampler array at set 0 binding 12 (backend/raytrace_resources.rs).
use super::material::RaytraceMaterial;
use crate::RenderError;
use crate::retained_texture_images::RetainedTextureImages;
use crate::surface_texture::{SurfaceSamplerPolicy, SurfaceTextureImage};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// `GlRaytraceGeometry::MAX_TEX_NUMBER`: the size of the sampler array.
pub(crate) const MAX_TEX_NUMBER: usize = 32;

/// One texture of the scene: its frozen image and the sampler it is read with.
#[derive(Clone, Debug)]
pub(crate) struct RaytraceTexture {
    pub image: Arc<SurfaceTextureImage>,
    pub sampler: SurfaceSamplerPolicy,
}
impl PartialEq for RaytraceTexture {
    /// The same image snapshot under the same sampler (native compares GlTexture handles).
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.image, &other.image) && self.sampler == other.sampler
    }
}

/// The textures of one ray-traced scene (`myTextures`), in sampler-array order.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RaytraceTextures {
    pub textures: Vec<RaytraceTexture>,
}
impl RaytraceTextures {
    /// `AddTexture`: the index of `texture`, appended when new; -1 once MAX_TEX_NUMBER distinct
    /// textures are held. Example: `textures.add(texture)`.
    pub fn add(&mut self, texture: RaytraceTexture) -> i32 {
        if let Some(index) = self.textures.iter().position(|held| *held == texture) {
            return index as i32;
        }
        if self.textures.len() >= MAX_TEX_NUMBER {
            return -1;
        }
        self.textures.push(texture);
        self.textures.len() as i32 - 1
    }
    /// `HasTextures`: whether any material is textured (selects the USE_TEXTURES programs).
    pub fn has_textures(&self) -> bool {
        !self.textures.is_empty()
    }
}

/// Material records that take a texture index in their w component, by texture unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TexturedRecord {
    /// Diffuse and BSDF Kd (`Graphic3dTextureUnit_BaseColor`).
    BaseColor,
    /// BSDF Kt (`Graphic3dTextureUnit_MetallicRoughness`).
    MetallicRoughness,
    /// BSDF Le (`Graphic3dTextureUnit_Emissive`).
    Emissive,
    /// BSDF FresnelBase (`Graphic3dTextureUnit_Normal`).
    Normal,
}
impl TexturedRecord {
    /// The record a texture unit feeds, or `None` for units ray tracing ignores (occlusion).
    pub fn of_unit(unit: Graphic3dTextureUnit) -> Option<Self> {
        match unit {
            Graphic3dTextureUnit::Unit0 => Some(Self::BaseColor),
            Graphic3dTextureUnit::Unit4 => Some(Self::MetallicRoughness),
            Graphic3dTextureUnit::Unit1 => Some(Self::Emissive),
            Graphic3dTextureUnit::Unit3 => Some(Self::Normal),
            _ => None,
        }
    }
}

/// `buildTextureTransform`: scale, then translation by the negated offset (scaled), times a
/// rotation by the negated angle; identity without parameters. Returned as the 16 floats of the
/// column-major BVHMat4f that convertMaterial copies into the material buffer.
/// Example: `texture_transform(&params)`.
pub(crate) fn texture_transform(params: &Graphic3dTextureParams) -> [[f32; 4]; 4] {
    let (scale, translation) = (params.scale(), params.translation());
    let (sx, sy) = (scale.x(), scale.y());
    let (tx, ty) = (-translation.x(), -translation.y());
    let angle = -params.rotation() * (std::f64::consts::PI / 180.0) as f32;
    let (sin, cos) = (
        (martensite_cmath::sin(f64::from(angle)) as f32),
        (martensite_cmath::cos(f64::from(angle)) as f32),
    );
    // [S | S*T] * R with R = [[cos, -sin], [sin, cos]]; rows 2 and 3 stay identity.
    let row0 = [sx * cos, -(sx * sin), 0., sx * tx];
    let row1 = [sy * sin, sy * cos, 0., sy * ty];
    let rows = [row0, row1, [0., 0., 1., 0.], [0., 0., 0., 1.]];
    std::array::from_fn(|column| std::array::from_fn(|row| rows[row][column]))
}

/// convertMaterial's texture branch: each texture of `aspects`' set that the frozen `images`
/// can load is added to `textures`, and its index and transform written to `material`.
/// Example: `apply_material_textures(&mut material, aspects, images, &mut textures)?`.
pub(crate) fn apply_material_textures(
    material: &mut RaytraceMaterial,
    aspects: &dyn Graphic3dAspectsSource,
    images: &RetainedTextureImages,
    textures: &mut RaytraceTextures,
) -> Result<(), RenderError> {
    if !aspects.to_map_texture() {
        return Ok(());
    }
    let set = aspects.texture_set();
    let Some(set) = set.get() else {
        return Ok(());
    };
    let list = set.textures();
    if list.first().is_none_or(|first| first.is_null()) {
        return Ok(());
    }
    for texture in list.iter().filter(|texture| !texture.is_null()) {
        let params = texture.get_params();
        let Some(record) = TexturedRecord::of_unit(params.texture_unit()) else {
            continue;
        };
        let index = match images.snapshot(texture)? {
            Some(image) => {
                let sampler = images.samplers.policy(&image, &params)?;
                textures.add(RaytraceTexture { image, sampler })
            }
            None => -1, // AddTexture of a texture without a GL texture object.
        };
        material.set_texture(record, index, texture_transform(&params));
    }
    Ok(())
}

#[cfg(test)]
#[path = "textures_tests.rs"]
mod tests;
