//! OpenPBR Surface v1.1 material conversion for the unified compute path tracer.
use crate::RenderMaterialColorSpace;
use crate::openpbr_material::{OpenPbrGpuMaterial, OpenPbrTextureIndices};
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsMembers as _, Graphic3dAspectsSource, Graphic3dTypeOfMaterial,
    Graphic3dTypeOfReflection,
};

/// One OpenPBR v1.1 material uploaded into the compute path tracer's `MaterialBuf` SSBO (256 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RaytraceMaterial {
    pub gpu: OpenPbrGpuMaterial,
    pub tex: OpenPbrTextureIndices,
}

impl RaytraceMaterial {
    /// Converts `aspects` front material (and interior colour for `Aspect` materials) into an
    /// `OpenPbrGpuMaterial` ready for the unified compute path tracer.
    pub fn of_aspects(
        aspects: &dyn Graphic3dAspectsSource,
        space: RenderMaterialColorSpace,
    ) -> Self {
        let source = aspects.front_material();
        let mut pbr = *source.openpbr_material();
        if source.material_type() == Graphic3dTypeOfMaterial::Aspect {
            let interior = aspects.interior_color().rgb();
            let tint = |c: [f32; 3]| [c[0] * interior[0], c[1] * interior[1], c[2] * interior[2]];
            pbr.base.color = tint(pbr.base.color);
            if source.reflection_mode(Graphic3dTypeOfReflection::Emission) {
                pbr.emission.color = tint(pbr.emission.color);
            }
        }
        if space == RenderMaterialColorSpace::GammaEncoded {
            let c = ColorRgba::linear_to_srgb_vec4([
                pbr.base.color[0],
                pbr.base.color[1],
                pbr.base.color[2],
                1.0,
            ]);
            pbr.base.color = [c[0], c[1], c[2]];
        }
        Self {
            gpu: OpenPbrGpuMaterial::from_openpbr(&pbr),
            tex: OpenPbrTextureIndices::default(),
        }
    }

    /// Records a texture slot index on this OpenPBR material.
    pub fn set_texture(
        &mut self,
        record: super::textures::TexturedRecord,
        index: i32,
        _transform: [[f32; 4]; 4],
    ) {
        use super::textures::TexturedRecord::*;
        let idx = index.clamp(-1, i16::MAX as i32) as i16;
        match record {
            BaseColor => self.tex.slots[0] = idx,
            Emissive => self.tex.slots[1] = idx,
            Normal => self.tex.slots[3] = idx,
            MetallicRoughness => self.tex.slots[4] = idx,
        }
        self.gpu.tex_indices = self.tex.pack().map(f32::from_bits);
    }
}

#[cfg(test)]
#[path = "material_tests.rs"]
mod tests;
