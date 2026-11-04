//! GlRaytraceMaterial and GlView::convertMaterial: the 19 vec4 records
//! of the material texture buffer that RaytraceBase.fs reads through its MATERIAL_* offsets.
use crate::RenderMaterialColorSpace;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsMembers as _, Graphic3dAspectsSource, Graphic3dMaterialAspect,
    Graphic3dTypeOfMaterial,
};

/// vec4 records of one material in the buffer (`19 * index` in RaytraceBase.fs).
pub(crate) const MATERIAL_VEC4S: usize = 19;

/// One material as the shaders read it: ambient, diffuse, specular, emission, reflection,
/// refraction, transparency, the texture transform rows and the physical BSDF records.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RaytraceMaterial {
    pub records: [[f32; 4]; MATERIAL_VEC4S],
}

/// Record indices of GlRaytraceMaterial, in declaration order.
const AMBIENT: usize = 0;
const DIFFUSE: usize = 1;
const SPECULAR: usize = 2;
const EMISSION: usize = 3;
const REFLECTION: usize = 4;
const TRANSPARENCY: usize = 6;
const TEXTURE_TRANSFORM: usize = 7;
const BSDF: usize = 11;

impl Default for RaytraceMaterial {
    /// The empty constructor: zero colours, an identity texture transform, a zero BSDF.
    fn default() -> Self {
        let mut records = [[0.; 4]; MATERIAL_VEC4S];
        (0..4).for_each(|i| records[TEXTURE_TRANSFORM + i][i] = 1.);
        Self { records }
    }
}

fn scaled(color: [f32; 3], by: [f32; 3], w: f32) -> [f32; 4] {
    [color[0] * by[0], color[1] * by[1], color[2] * by[2], w]
}

fn with_w(color: [f32; 3], w: f32) -> [f32; 4] {
    [color[0], color[1], color[2], w]
}

/// `Vec4FromQuantityColor`: linear for sRGB targets, gamma-encoded otherwise.
fn target_color(space: RenderMaterialColorSpace, color: [f32; 4]) -> [f32; 4] {
    match space {
        RenderMaterialColorSpace::Linear => color,
        RenderMaterialColorSpace::GammaEncoded => ColorRgba::linear_to_srgb_vec4(color),
    }
}

impl RaytraceMaterial {
    /// `convertMaterial` without textures: the aspect's front material and interior colour.
    /// Example: `RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::Linear)`.
    pub fn of_aspects(
        aspects: &dyn Graphic3dAspectsSource,
        space: RenderMaterialColorSpace,
    ) -> Self {
        let mut material = Self::default();
        let source = aspects.front_material();
        material.set_colors(&source, aspects.interior_color().rgb());
        material.set_reflection(&source);
        material.records[TRANSPARENCY] = transparency(&source);
        for index in [AMBIENT, DIFFUSE, SPECULAR, EMISSION] {
            material.records[index] = target_color(space, material.records[index]);
        }
        material.set_bsdf(&source);
        material
    }
    fn set_colors(&mut self, source: &Graphic3dMaterialAspect, interior: [f32; 3]) {
        let (ambient, diffuse) = (source.ambient_color().rgb(), source.diffuse_color().rgb());
        let emissive = source.emissive_color().rgb();
        // -1 marks the diffuse record as untextured.
        (
            self.records[AMBIENT],
            self.records[DIFFUSE],
            self.records[EMISSION],
        ) = match source.material_type() {
            Graphic3dTypeOfMaterial::Aspect => (
                scaled(ambient, interior, 1.),
                scaled(diffuse, interior, -1.),
                scaled(emissive, interior, 1.),
            ),
            Graphic3dTypeOfMaterial::Physic => (
                with_w(ambient, 1.),
                with_w(diffuse, -1.),
                with_w(emissive, 1.),
            ),
        };
    }
    /// The specular record (shininess scaled to 128) and the reflection scaled to 0.75 of the
    /// largest diffuse plus specular channel.
    fn set_reflection(&mut self, source: &Graphic3dMaterialAspect) {
        let specular = source.specular_color().rgb();
        self.records[SPECULAR] = with_w(specular, 128. * source.shininess());
        let (d, s) = (self.records[DIFFUSE], self.records[SPECULAR]);
        let max_reflection = (d[0] + s[0]).max((d[1] + s[1]).max(d[2] + s[2]));
        let scale = 0.75 / max_reflection;
        self.records[REFLECTION] = with_w(specular.map(|c| c * scale), 0.);
    }
    /// Kc, Kd, Ks, Kt, Le, FresnelCoat, FresnelBase, Absorption; -1 marks untextured maps.
    fn set_bsdf(&mut self, source: &Graphic3dMaterialAspect) {
        let bsdf = source.bsdf();
        let mut base = bsdf.fresnel_base.serialize().data();
        base[3] = -1.;
        let records = [
            bsdf.kc.data(),
            with_w(bsdf.kd.data(), -1.),
            bsdf.ks.data(),
            with_w(bsdf.kt.data(), -1.),
            with_w(bsdf.le.data(), -1.),
            bsdf.fresnel_coat.serialize().data(),
            base,
            bsdf.absorption.data(),
        ];
        self.records[BSDF..].copy_from_slice(&records);
    }
}

impl RaytraceMaterial {
    /// Write one texture of convertMaterial's texture branch: the texture transform (each
    /// textured unit rewrites it) and the texture index in the w component of `record`.
    /// Example: `material.set_texture(TexturedRecord::BaseColor, 0, transform)`.
    pub fn set_texture(
        &mut self,
        record: super::textures::TexturedRecord,
        index: i32,
        transform: [[f32; 4]; 4],
    ) {
        use super::textures::TexturedRecord::*;
        self.records[TEXTURE_TRANSFORM..TEXTURE_TRANSFORM + 4].copy_from_slice(&transform);
        let records: &[usize] = match record {
            BaseColor => &[DIFFUSE, BSDF + 1],
            MetallicRoughness => &[BSDF + 3],
            Emissive => &[BSDF + 4],
            Normal => &[BSDF + 6],
        };
        for &slot in records {
            self.records[slot][3] = index as f32;
        }
    }
}

/// `(Alpha, Transparency, index, 1 / index)` with a zero refraction index read as 1.
fn transparency(source: &Graphic3dMaterialAspect) -> [f32; 4] {
    let index = source.refraction_index();
    let (index, inverse) = if index == 0. {
        (1., 1.)
    } else {
        (index, 1. / index)
    };
    [source.alpha(), source.transparency(), index, inverse]
}

#[cfg(test)]
#[path = "material_tests.rs"]
mod tests;
