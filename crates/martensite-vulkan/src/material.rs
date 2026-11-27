//! Martensite shader material packing for Phong (`RenderMaterialCommon`) and ASWF OpenPBR
//! Surface v1.1 (`OpenPbrGpuMaterial`). Initialization requires no graphics context.
use crate::{OpenPbrGpuMaterial, RenderMaterialCommon};
use martensite_tkernel::quantity::{Color, ColorRgba};
use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dTypeOfMaterial};

/// Color policy of the target framebuffer; `RenderMaterialColorSpace::Linear`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderMaterialColorSpace {
    /// An sRGB attachment encodes the linear shader result on write.
    Linear,
    /// Legacy non-sRGB attachments require gamma-encoded common coefficients.
    GammaEncoded,
}

impl RenderMaterialColorSpace {
    fn convert(self, color: [f32; 4]) -> [f32; 4] {
        match self {
            Self::Linear => color,
            Self::GammaEncoded => ColorRgba::linear_to_srgb_vec4(color),
        }
    }
}

/// Front/back material snapshots for Phong and OpenPBR Surface v1.1 shading.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderMaterial {
    /// Common Phong coefficients for front and back faces.
    pub common: [RenderMaterialCommon; 2],
    /// Unified 256-byte OpenPBR Surface v1.1 records for front and back faces.
    pub openpbr: [OpenPbrGpuMaterial; 2],
}

impl RenderMaterial {
    /// Initialize both faces from Graphic3d materials; `material.init(space, &front, red, &back, blue)`.
    /// Identical source references copy the front record, including its interior color.
    pub fn init(
        &mut self,
        space: RenderMaterialColorSpace,
        front: &Graphic3dMaterialAspect,
        front_color: Color,
        back: &Graphic3dMaterialAspect,
        back_color: Color,
    ) {
        self.init_face(space, front, front_color, 0);
        if std::ptr::eq(front, back) {
            self.common[1] = self.common[0];
            self.openpbr[1] = self.openpbr[0];
            return;
        }
        self.init_face(space, back, back_color, 1);
    }

    fn init_face(
        &mut self,
        space: RenderMaterialColorSpace,
        source: &Graphic3dMaterialAspect,
        interior: Color,
        index: usize,
    ) {
        self.common[index] = common_coefficients(space, source, interior);
        let mut gpu = OpenPbrGpuMaterial::from_openpbr(source.openpbr_material());
        if source.material_type() == Graphic3dTypeOfMaterial::Aspect {
            gpu.set_base_color(interior.rgb());
            gpu.set_opacity(source.alpha());
        }
        self.openpbr[index] = gpu;
    }

    /// Override both faces for highlight, preserving gloss/emission/alpha; `material.set_color([1.,0.,0.])`.
    pub fn set_color(&mut self, color: [f32; 3]) {
        for common in &mut self.common {
            common.set_color(color);
        }
        for pbr in &mut self.openpbr {
            pbr.set_base_color(color);
        }
    }

    /// Override opacity on both faces for translucent highlights; `material.set_alpha(0.5)`.
    pub fn set_alpha(&mut self, alpha: f32) {
        for common in &mut self.common {
            common.diffuse[3] = alpha;
        }
        for pbr in &mut self.openpbr {
            pbr.set_opacity(alpha);
        }
    }

    /// Serialize front then back Phong records; `material.packed_common()`.
    pub fn packed_common(&self) -> [[f32; 4]; 8] {
        let [front, back] = self.common.map(|face| face.packed());
        [
            front[0], front[1], front[2], front[3], back[0], back[1], back[2], back[3],
        ]
    }

    /// Serialize front then back OpenPBR records (32 `vec4`s = 512 bytes); `material.packed_openpbr()`.
    pub fn packed_openpbr(&self) -> [[f32; 4]; 32] {
        let front = self.openpbr[0].packed();
        let back = self.openpbr[1].packed();
        std::array::from_fn(|i| if i < 16 { front[i] } else { back[i - 16] })
    }

    /// Number of vec4 records in the Phong block; `RenderMaterial::nb_of_vec4_common()`.
    pub const fn nb_of_vec4_common() -> usize {
        8
    }

    /// Number of vec4 records in the OpenPBR block (front + back); `RenderMaterial::nb_of_vec4_openpbr()`.
    pub const fn nb_of_vec4_openpbr() -> usize {
        32
    }

    /// Compare exact bit patterns, including signed zeros and NaNs; `material.is_equal(&other)`.
    pub fn is_equal(&self, other: &Self) -> bool {
        let left = self
            .packed_common()
            .into_iter()
            .chain(self.packed_openpbr());
        let right = other
            .packed_common()
            .into_iter()
            .chain(other.packed_openpbr());
        left.zip(right)
            .all(|(a, b)| a.map(f32::to_bits) == b.map(f32::to_bits))
    }
}

impl PartialEq for RenderMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}
impl Eq for RenderMaterial {}

pub(crate) fn common_coefficients(
    space: RenderMaterialColorSpace,
    source: &Graphic3dMaterialAspect,
    interior: Color,
) -> RenderMaterialCommon {
    let tint = if source.material_type() == Graphic3dTypeOfMaterial::Aspect {
        Some(interior.rgb())
    } else {
        None
    };
    RenderMaterialCommon {
        diffuse: space.convert(tinted(source.diffuse_color(), tint, source.alpha())),
        emission: space.convert(tinted(source.emissive_color(), tint, 1.)),
        ambient: space.convert(tinted(source.ambient_color(), tint, 1.)),
        specular_shininess: space.convert(with_fourth(
            source.specular_color().rgb(),
            128. * source.shininess(),
        )),
    }
}

fn tinted(color: Color, tint: Option<[f32; 3]>, fourth: f32) -> [f32; 4] {
    let rgb = color.rgb();
    with_fourth(
        tint.map_or(rgb, |factor| {
            std::array::from_fn(|index| rgb[index] * factor[index])
        }),
        fourth,
    )
}

fn with_fourth([r, g, b]: [f32; 3], fourth: f32) -> [f32; 4] {
    [r, g, b, fourth]
}
