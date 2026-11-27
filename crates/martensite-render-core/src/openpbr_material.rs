//! Unified 256-byte `std430` GPU representation of [`OpenPbrMaterial`] shared identically by
//! OpenPBR rasterization and compute path tracing (hardware `VK_KHR_ray_query` and software BVH).
use martensite_tkservice::graphic3d::OpenPbrMaterial;

/// Active OpenPBR slab bitmask packed in `OpenPbrGpuMaterial::meta[2]`.
pub mod lobe_flags {
    /// Microfiber fuzz / sheen layer (`fuzz.weight > 0`).
    pub const FUZZ: u32 = 1 << 0;
    /// Clear-coat dielectric layer (`coat.weight > 0`).
    pub const COAT: u32 = 1 << 1;
    /// Metallic conductor component (`base.metalness > 0`).
    pub const METAL: u32 = 1 << 2;
    /// Dielectric transmission component (`transmission.weight > 0 && base.metalness < 1`).
    pub const TRANSMISSION: u32 = 1 << 3;
    /// Airy thin-film interference (`thin_film.weight > 0 && thin_film.thickness_um > 0`).
    pub const THIN_FILM: u32 = 1 << 4;
    /// Subsurface scattering (`subsurface.weight > 0 && base.metalness < 1 && transmission.weight < 1`).
    pub const SSS: u32 = 1 << 5;
    /// Self-emission (`emission.luminance > 0`).
    pub const EMISSION: u32 = 1 << 6;
    /// Infinitely thin double-sided slab (`geometry.thin_walled`).
    pub const THIN_WALLED: u32 = 1 << 7;
    /// Anisotropic specular or coat roughness.
    pub const ANISO: u32 = 1 << 8;
    /// Energy-preserving Oren-Nayar diffuse (`base.diffuse_roughness > 1e-5`).
    pub const DIFFUSE_ROUGH: u32 = 1 << 9;
    /// Interior transmission volume absorption or scattering (`depth > 0 || scatter > 0`).
    pub const VOLUME: u32 = 1 << 10;
    /// Chromatic Abbe dispersion (`transmission.dispersion_scale > 0`).
    pub const DISPERSION: u32 = 1 << 11;
}

/// Bindless texture slots packed into `OpenPbrGpuMaterial::tex_indices` (eight `i16` values,
/// `-1` = untextured).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenPbrTextureIndices {
    /// Texture indices for `[base_color, emissive, occlusion, normal, metallic_roughness, specular_color, coat_weight_roughness, transmission]`.
    pub slots: [i16; 8],
}

impl Default for OpenPbrTextureIndices {
    fn default() -> Self {
        Self { slots: [-1; 8] }
    }
}

impl OpenPbrTextureIndices {
    /// Packs the eight signed 16-bit texture indices into four `u32` words.
    pub fn pack(self) -> [u32; 4] {
        [
            pack_i16_pair(self.slots[0], self.slots[1]),
            pack_i16_pair(self.slots[2], self.slots[3]),
            pack_i16_pair(self.slots[4], self.slots[5]),
            pack_i16_pair(self.slots[6], self.slots[7]),
        ]
    }
}

fn pack_i16_pair(lo: i16, hi: i16) -> u32 {
    (u32::from(lo as u16)) | (u32::from(hi as u16) << 16)
}

/// Unified `std430` 16-`vec4` (256-byte) OpenPBR Surface v1.1 GPU material record.
///
/// Layout matches `OpenPbrGpuMaterial` in `shaders/openpbr/openpbr_types.glsl` word-for-word.
///
/// ```
/// use martensite_render_core::OpenPbrGpuMaterial;
/// use martensite_tkservice::graphic3d::OpenPbrMaterial;
/// let gpu = OpenPbrGpuMaterial::from_openpbr(&OpenPbrMaterial::default());
/// assert_eq!(std::mem::size_of::<OpenPbrGpuMaterial>(), 256);
/// assert_eq!(gpu.specular_ior(), 1.5);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OpenPbrGpuMaterial {
    /// `[base_color.r, g, b, base_weight]`.
    pub base_color_weight: [f32; 4],
    /// `[base_metalness, base_diffuse_roughness, specular_weight, specular_roughness]`.
    pub base_spec_params: [f32; 4],
    /// `[specular_color.r, g, b, specular_roughness_anisotropy]`.
    pub specular_color_aniso: [f32; 4],
    /// `[specular_ior, transmission_weight, transmission_depth, transmission_scatter_anisotropy]`.
    pub specular_ior_trans: [f32; 4],
    /// `[transmission_color.r, g, b, transmission_dispersion_scale]`.
    pub transmission_color_disp: [f32; 4],
    /// `[transmission_scatter.r, g, b, transmission_dispersion_abbe_number]`.
    pub transmission_scatter_abbe: [f32; 4],
    /// `[subsurface_color.r, g, b, subsurface_weight]`.
    pub subsurface_color_weight: [f32; 4],
    /// `[subsurface_radius_scale.r, g, b, subsurface_radius]`.
    pub subsurface_radius_scale: [f32; 4],
    /// `[coat_color.r, g, b, coat_weight]`.
    pub coat_color_weight: [f32; 4],
    /// `[coat_roughness, coat_roughness_anisotropy, coat_ior, coat_darkening]`.
    pub coat_params: [f32; 4],
    /// `[fuzz_color.r, g, b, fuzz_weight]`.
    pub fuzz_color_weight: [f32; 4],
    /// `[fuzz_roughness, thin_film_weight, thin_film_thickness_um, thin_film_ior]`.
    pub fuzz_thin_film: [f32; 4],
    /// `[emission_color.r, g, b, emission_luminance]`.
    pub emission_color_lum: [f32; 4],
    /// `[geometry_opacity, subsurface_scatter_anisotropy, bitcast(lobe_flags), bitcast(tex_mask)]`.
    pub geometry_meta: [f32; 4],
    /// `[uv_scale_u, uv_scale_v, uv_offset_u, uv_offset_v]`.
    pub uv_transform: [f32; 4],
    /// Four `u32` words bitcast to `f32` packing eight `i16` bindless texture indices (`-1` = none).
    pub tex_indices: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<OpenPbrGpuMaterial>() == 256);

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<OpenPbrGpuMaterial>();
};

impl Default for OpenPbrGpuMaterial {
    fn default() -> Self {
        Self::from_openpbr(&OpenPbrMaterial::default())
    }
}

impl PartialEq for OpenPbrGpuMaterial {
    /// Bitwise equality so bitcast integer metadata (`lobe_flags`, `tex_indices` with `-1` = `0xFFFF_FFFF`)
    /// compares identically regardless of IEEE NaN bit patterns.
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for OpenPbrGpuMaterial {}

impl OpenPbrGpuMaterial {
    /// Number of `vec4` rows per face (`16`).
    pub const VEC4_COUNT: usize = 16;

    /// Packs an [`OpenPbrMaterial`] into its 256-byte GPU record with active lobe flags.
    pub fn from_openpbr(m: &OpenPbrMaterial) -> Self {
        let flags = compute_lobe_flags(m);
        let packed_tex = OpenPbrTextureIndices::default().pack().map(f32::from_bits);
        Self {
            base_color_weight: [
                m.base.color[0],
                m.base.color[1],
                m.base.color[2],
                m.base.weight,
            ],
            base_spec_params: [
                m.base.metalness,
                m.base.diffuse_roughness,
                m.specular.weight,
                m.specular.roughness,
            ],
            specular_color_aniso: [
                m.specular.color[0],
                m.specular.color[1],
                m.specular.color[2],
                m.specular.roughness_anisotropy,
            ],
            specular_ior_trans: [
                m.specular.ior,
                m.transmission.weight,
                m.transmission.depth,
                m.transmission.scatter_anisotropy,
            ],
            transmission_color_disp: [
                m.transmission.color[0],
                m.transmission.color[1],
                m.transmission.color[2],
                m.transmission.dispersion_scale,
            ],
            transmission_scatter_abbe: [
                m.transmission.scatter[0],
                m.transmission.scatter[1],
                m.transmission.scatter[2],
                m.transmission.dispersion_abbe_number,
            ],
            subsurface_color_weight: [
                m.subsurface.color[0],
                m.subsurface.color[1],
                m.subsurface.color[2],
                m.subsurface.weight,
            ],
            subsurface_radius_scale: [
                m.subsurface.radius_scale[0],
                m.subsurface.radius_scale[1],
                m.subsurface.radius_scale[2],
                m.subsurface.radius,
            ],
            coat_color_weight: [
                m.coat.color[0],
                m.coat.color[1],
                m.coat.color[2],
                m.coat.weight,
            ],
            coat_params: [
                m.coat.roughness,
                m.coat.roughness_anisotropy,
                m.coat.ior,
                m.coat.darkening,
            ],
            fuzz_color_weight: [
                m.fuzz.color[0],
                m.fuzz.color[1],
                m.fuzz.color[2],
                m.fuzz.weight,
            ],
            fuzz_thin_film: [
                m.fuzz.roughness,
                m.thin_film.weight,
                m.thin_film.thickness_um,
                m.thin_film.ior,
            ],
            emission_color_lum: [
                m.emission.color[0],
                m.emission.color[1],
                m.emission.color[2],
                m.emission.luminance,
            ],
            geometry_meta: [
                m.geometry.opacity,
                m.subsurface.scatter_anisotropy,
                f32::from_bits(flags),
                f32::from_bits(0),
            ],
            uv_transform: [1.0, 1.0, 0.0, 0.0],
            tex_indices: packed_tex,
        }
    }

    /// Sets bindless texture indices and active texture bitmask.
    pub fn with_textures(mut self, indices: OpenPbrTextureIndices, uv_transform: [f32; 4]) -> Self {
        let mut mask = 0u32;
        for (bit, slot) in indices.slots.iter().enumerate() {
            if *slot >= 0 {
                mask |= 1u32 << bit;
            }
        }
        self.geometry_meta[3] = f32::from_bits(mask);
        self.uv_transform = uv_transform;
        self.tex_indices = indices.pack().map(f32::from_bits);
        self
    }

    /// Overrides base RGB color while preserving weight, opacity, and other slabs.
    pub fn set_base_color(&mut self, color: [f32; 3]) {
        self.base_color_weight[..3].copy_from_slice(&color);
    }

    /// Overrides surface opacity (`geometry.opacity`).
    pub fn set_opacity(&mut self, opacity: f32) {
        self.geometry_meta[0] = opacity;
    }

    /// Reads surface opacity (`geometry.opacity`).
    pub fn opacity(&self) -> f32 {
        self.geometry_meta[0]
    }

    /// Reads base metalness (`[0, 1]`).
    pub fn metalness(&self) -> f32 {
        self.base_spec_params[0]
    }

    /// Reads primary specular roughness (`[0, 1]`).
    pub fn roughness(&self) -> f32 {
        self.base_spec_params[3]
    }

    /// Reads primary specular IOR.
    pub fn specular_ior(&self) -> f32 {
        self.specular_ior_trans[0]
    }

    /// Reads active lobe bitmask.
    pub fn lobe_flags(&self) -> u32 {
        self.geometry_meta[2].to_bits()
    }

    /// Serializes as 16 `vec4` rows (`std430` order).
    pub fn packed(&self) -> [[f32; 4]; 16] {
        [
            self.base_color_weight,
            self.base_spec_params,
            self.specular_color_aniso,
            self.specular_ior_trans,
            self.transmission_color_disp,
            self.transmission_scatter_abbe,
            self.subsurface_color_weight,
            self.subsurface_radius_scale,
            self.coat_color_weight,
            self.coat_params,
            self.fuzz_color_weight,
            self.fuzz_thin_film,
            self.emission_color_lum,
            self.geometry_meta,
            self.uv_transform,
            self.tex_indices,
        ]
    }

    /// Serializes as 256 raw little-endian bytes for SSBO upload.
    pub fn to_bytes(&self) -> [u8; 256] {
        let mut out = [0u8; 256];
        for (row_idx, row) in self.packed().iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                let offset = (row_idx * 4 + col_idx) * 4;
                out[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
            }
        }
        out
    }
}

fn compute_lobe_flags(m: &OpenPbrMaterial) -> u32 {
    let mut flags = 0u32;
    if m.has_fuzz() {
        flags |= lobe_flags::FUZZ;
    }
    if m.has_coat() {
        flags |= lobe_flags::COAT;
    }
    if m.base.metalness > 0.0 {
        flags |= lobe_flags::METAL;
    }
    if m.has_transmission() {
        flags |= lobe_flags::TRANSMISSION;
        if m.transmission.depth > 0.0 || m.transmission.scatter.iter().any(|&c| c > 0.0) {
            flags |= lobe_flags::VOLUME;
        }
        if m.transmission.dispersion_scale > 0.0 {
            flags |= lobe_flags::DISPERSION;
        }
    }
    if m.has_thin_film() && m.thin_film.thickness_um > 0.0 {
        flags |= lobe_flags::THIN_FILM;
    }
    if m.has_subsurface() {
        flags |= lobe_flags::SSS;
    }
    if m.has_emission() {
        flags |= lobe_flags::EMISSION;
    }
    if m.geometry.thin_walled {
        flags |= lobe_flags::THIN_WALLED;
    }
    if m.specular.roughness_anisotropy > 0.0 || m.coat.roughness_anisotropy > 0.0 {
        flags |= lobe_flags::ANISO;
    }
    if m.base.diffuse_roughness > 1e-5 {
        flags |= lobe_flags::DIFFUSE_ROUGH;
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpu_material_is_256_bytes_and_sets_expected_lobe_flags() {
        assert_eq!(std::mem::size_of::<OpenPbrGpuMaterial>(), 256);
        let mut mat = OpenPbrMaterial::default();
        mat.coat.weight = 0.8;
        mat.fuzz.weight = 0.4;
        mat.thin_film.weight = 1.0;
        mat.thin_film.thickness_um = 0.45;
        mat.emission.luminance = 2.0;
        mat.geometry.thin_walled = true;
        let gpu = OpenPbrGpuMaterial::from_openpbr(&mat);
        let flags = gpu.lobe_flags();
        assert_ne!(flags & lobe_flags::COAT, 0);
        assert_ne!(flags & lobe_flags::FUZZ, 0);
        assert_ne!(flags & lobe_flags::THIN_FILM, 0);
        assert_ne!(flags & lobe_flags::EMISSION, 0);
        assert_ne!(flags & lobe_flags::THIN_WALLED, 0);
        assert_eq!(flags & lobe_flags::METAL, 0);
        assert_eq!(gpu.to_bytes().len(), 256);
    }
}
