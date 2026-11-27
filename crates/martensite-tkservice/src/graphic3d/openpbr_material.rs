//! ASWF OpenPBR Surface v1.1 material model (`OpenPbrMaterial`).
//!
//! Parameter groups, defaults, and physical domains match the OpenPBR Surface v1.1 specification
//! and Blender's OpenPBR Surface node (`node_shader_bsdf_open_pbr.cc`).
use super::openpbr_validate::validate_material;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, ColorRgba};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::dump::DumpStream;
use std::io;

/// Base layer: bulk dielectric/metallic color, metalness, and diffuse roughness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrBase {
    /// Scalar multiplier of `color` in `[0, 1]`.
    pub weight: f32,
    /// Linear RGB diffuse albedo / metallic normal-incidence color (`F0`) in `[0, 1]`.
    pub color: [f32; 3],
    /// Blend between dielectric (`0`) and conductor (`1`) in `[0, 1]`.
    pub metalness: f32,
    /// Oren-Nayar diffuse roughness (`0` = Lambertian, `1` = full EON) in `[0, 1]`.
    pub diffuse_roughness: f32,
}

impl Default for OpenPbrBase {
    fn default() -> Self {
        Self {
            weight: 1.0,
            color: [0.8, 0.8, 0.8],
            metalness: 0.0,
            diffuse_roughness: 0.0,
        }
    }
}

/// Primary specular layer: dielectric reflectivity / metallic F82 tint and microfacet roughness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrSpecular {
    /// Modulates dielectric normal-incidence reflectivity and metallic reflection (`>= 0`).
    pub weight: f32,
    /// Linear RGB dielectric specular tint / metallic near-grazing (`F82`) tint in `[0, 1]`.
    pub color: [f32; 3],
    /// Microfacet roughness in `[0, 1]` (`0` = mirror, `1` = maximally rough).
    pub roughness: f32,
    /// Anisotropy in `[0, 1]` elongating highlights along the surface tangent.
    pub roughness_anisotropy: f32,
    /// Base dielectric index of refraction (`>= 1e-5`, default `1.5`).
    pub ior: f32,
}

impl Default for OpenPbrSpecular {
    fn default() -> Self {
        Self {
            weight: 1.0,
            color: [1.0, 1.0, 1.0],
            roughness: 0.3,
            roughness_anisotropy: 0.0,
            ior: 1.5,
        }
    }
}

/// Transmission layer: solid dielectric refraction, interior volume medium, and Abbe dispersion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrTransmission {
    /// Blend between transmissive dielectric and opaque base in `[0, 1]`.
    pub weight: f32,
    /// Linear RGB transmission tint or Beer-Lambert color at `depth` in `[0, 1]`.
    pub color: [f32; 3],
    /// Depth at which `color` is realized (`0` = on-surface tint only, `>= 0`).
    pub depth: f32,
    /// Interior homogeneous scattering coefficient per RGB channel (`>= 0`).
    pub scatter: [f32; 3],
    /// Henyey-Greenstein phase asymmetry in `[-1, 1]`.
    pub scatter_anisotropy: f32,
    /// Chromatic dispersion scale in `[0, 1]`.
    pub dispersion_scale: f32,
    /// Abbe number controlling refractive dispersion (`>= 1`, default `20`).
    pub dispersion_abbe_number: f32,
}

impl Default for OpenPbrTransmission {
    fn default() -> Self {
        Self {
            weight: 0.0,
            color: [1.0, 1.0, 1.0],
            depth: 0.0,
            scatter: [0.0, 0.0, 0.0],
            scatter_anisotropy: 0.0,
            dispersion_scale: 0.0,
            dispersion_abbe_number: 20.0,
        }
    }
}

/// Subsurface scattering layer: thin-walled translucency or solid random-walk BSSRDF.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrSubsurface {
    /// Blend between diffuse base and subsurface scattering in `[0, 1]`.
    pub weight: f32,
    /// Observed linear RGB subsurface reflection albedo in `[0, 1]`.
    pub color: [f32; 3],
    /// Mean free path length multiplier (`>= 0`).
    pub radius: f32,
    /// Per-channel mean free path scale in `[0, 1]`.
    pub radius_scale: [f32; 3],
    /// Henyey-Greenstein scattering asymmetry in `[-1, 1]`.
    pub scatter_anisotropy: f32,
}

impl Default for OpenPbrSubsurface {
    fn default() -> Self {
        Self {
            weight: 0.0,
            color: [0.8, 0.8, 0.8],
            radius: 1.0,
            radius_scale: [1.0, 0.5, 0.25],
            scatter_anisotropy: 0.0,
        }
    }
}

/// Dielectric clear-coat layer with Beer-Lambert tinting, roughening, and internal darkening.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrCoat {
    /// Coat coverage weight in `[0, 1]`.
    pub weight: f32,
    /// Normal-incidence Beer-Lambert tint of underlying layers in `[0, 1]`.
    pub color: [f32; 3],
    /// Coat microfacet roughness in `[0, 1]`.
    pub roughness: f32,
    /// Coat microfacet anisotropy in `[0, 1]`.
    pub roughness_anisotropy: f32,
    /// Coat dielectric index of refraction (`>= 1e-5`, default `1.6`).
    pub ior: f32,
    /// Physical internal-reflection darkening factor in `[0, 1]` (default `1.0`).
    pub darkening: f32,
}

impl Default for OpenPbrCoat {
    fn default() -> Self {
        Self {
            weight: 0.0,
            color: [1.0, 1.0, 1.0],
            roughness: 0.0,
            roughness_anisotropy: 0.0,
            ior: 1.6,
            darkening: 1.0,
        }
    }
}

/// Topmost microfiber fuzz / sheen layer (Charlie distribution).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrFuzz {
    /// Fuzz coverage weight in `[0, 1]`.
    pub weight: f32,
    /// Linear RGB fuzz reflection tint in `[0, 1]`.
    pub color: [f32; 3],
    /// Microfiber roughness in `[0, 1]` (`0` = high sheen, `1` = dusty).
    pub roughness: f32,
}

impl Default for OpenPbrFuzz {
    fn default() -> Self {
        Self {
            weight: 0.0,
            color: [1.0, 1.0, 1.0],
            roughness: 0.5,
        }
    }
}

/// Thin-film Airy interference layer over the primary specular / metallic interface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrThinFilm {
    /// Thin-film coverage weight in `[0, 1]`.
    pub weight: f32,
    /// Film thickness in micrometers (`>= 0`, default `0.5`).
    pub thickness_um: f32,
    /// Film index of refraction (`>= 1e-5`, default `1.4`).
    pub ior: f32,
}

impl Default for OpenPbrThinFilm {
    fn default() -> Self {
        Self {
            weight: 0.0,
            thickness_um: 0.5,
            ior: 1.4,
        }
    }
}

/// Self-emission layer beneath coat and fuzz.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrEmission {
    /// Emitted luminance scale (`>= 0`).
    pub luminance: f32,
    /// Linear RGB emission color (`>= 0`).
    pub color: [f32; 3],
}

impl Default for OpenPbrEmission {
    fn default() -> Self {
        Self {
            luminance: 0.0,
            color: [1.0, 1.0, 1.0],
        }
    }
}

/// Geometric opacity and thin-walled slab mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenPbrGeometry {
    /// Surface opacity in `[0, 1]` (`0` = fully transparent, `1` = fully opaque).
    pub opacity: f32,
    /// Treats the surface as an infinitely thin double-sided slab when `true`.
    pub thin_walled: bool,
}

impl Default for OpenPbrGeometry {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            thin_walled: false,
        }
    }
}

/// Complete ASWF OpenPBR Surface v1.1 material definition.
///
/// ```
/// use martensite_tkservice::graphic3d::OpenPbrMaterial;
/// let mat = OpenPbrMaterial::default();
/// assert_eq!(mat.specular.ior, 1.5);
/// assert!(mat.validate().is_ok());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OpenPbrMaterial {
    /// Bulk dielectric/metallic base layer.
    pub base: OpenPbrBase,
    /// Primary specular reflection and microfacet roughness.
    pub specular: OpenPbrSpecular,
    /// Dielectric transmission, interior volume, and chromatic dispersion.
    pub transmission: OpenPbrTransmission,
    /// Subsurface scattering / thin-walled translucency.
    pub subsurface: OpenPbrSubsurface,
    /// Clear-coat dielectric layer.
    pub coat: OpenPbrCoat,
    /// Topmost Charlie microfiber fuzz layer.
    pub fuzz: OpenPbrFuzz,
    /// Airy thin-film interference layer.
    pub thin_film: OpenPbrThinFilm,
    /// Self-emission beneath coat and fuzz.
    pub emission: OpenPbrEmission,
    /// Surface opacity and thin-walled flag.
    pub geometry: OpenPbrGeometry,
}

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<OpenPbrMaterial>();
};

impl OpenPbrMaterial {
    /// Smallest accepted refractive index (`CLOSURE_EPS` in Blender Cycles).
    pub const MIN_IOR: f32 = 1e-5;
    /// Smallest accepted Abbe number for chromatic dispersion.
    pub const MIN_ABBE_NUMBER: f32 = 1.0;

    /// Builds a validated OpenPBR material from a standard metallic-roughness tuple
    /// (used by glTF/OBJ/XCAF workflows).
    ///
    /// Example: `OpenPbrMaterial::metallic_roughness(rgba, 1.0, 0.2, 1.5, Vec3::new(0.0, 0.0, 0.0))?`.
    pub fn metallic_roughness(
        base_color: ColorRgba,
        metalness: f32,
        roughness: f32,
        ior: f32,
        emission: Vec3<f32>,
    ) -> Result<Self, Failure> {
        let mut material = Self::default();
        material.set_base_color(*base_color.rgb());
        material.base.metalness = metalness;
        material.specular.roughness = roughness;
        material.specular.ior = ior;
        material.geometry.opacity = base_color.alpha();
        material.set_emission_radiance([emission.x, emission.y, emission.z])?;
        material.validate()?;
        Ok(material)
    }

    /// Validates every OpenPBR parameter against its physical domain.
    ///
    /// Example: `OpenPbrMaterial::default().validate()?`.
    pub fn validate(&self) -> Result<(), Failure> {
        validate_material(self)
    }

    /// Sets `base.color` from a linear `Quantity_Color`.
    ///
    /// Example: `material.set_base_color(Color::named(NamedColor::RED))`.
    pub fn set_base_color(&mut self, color: Color) {
        self.base.color = color.rgb();
    }

    /// Surface opacity in `[0, 1]`; `material.opacity()`.
    pub fn opacity(&self) -> f32 {
        self.geometry.opacity
    }

    /// Sets surface opacity in `[0, 1]`; `material.set_opacity(0.5)?`.
    pub fn set_opacity(&mut self, opacity: f32) -> Result<(), Failure> {
        super::openpbr_validate::check_unit(opacity, "geometry.opacity")?;
        self.geometry.opacity = opacity;
        Ok(())
    }

    /// Effective linear RGB emission radiance (`emission.color * emission.luminance`).
    ///
    /// Example: `material.emission_radiance()`.
    pub fn emission_radiance(&self) -> [f32; 3] {
        let scale = self.emission.luminance;
        [
            self.emission.color[0] * scale,
            self.emission.color[1] * scale,
            self.emission.color[2] * scale,
        ]
    }

    /// Sets `emission.luminance` and normalized `emission.color` from a non-negative RGB radiance.
    ///
    /// Example: `material.set_emission_radiance([2.0, 0.5, 0.0])?`.
    pub fn set_emission_radiance(&mut self, radiance: [f32; 3]) -> Result<(), Failure> {
        super::openpbr_validate::check_nonneg_rgb(radiance, "emission.radiance")?;
        let peak = radiance[0].max(radiance[1]).max(radiance[2]);
        if peak <= 0.0 {
            self.emission.luminance = 0.0;
            self.emission.color = [1.0, 1.0, 1.0];
            return Ok(());
        }
        self.emission.luminance = peak;
        self.emission.color = [radiance[0] / peak, radiance[1] / peak, radiance[2] / peak];
        Ok(())
    }

    /// Estimates PBR roughness from a legacy Phong specular color and shininess.
    ///
    /// Example: `OpenPbrMaterial::roughness_from_specular(white, 0.5)`.
    pub fn roughness_from_specular(specular: Color, shininess: f64) -> f32 {
        let mut roughness = (1.0 - shininess).clamp(0.0, 1.0);
        let intensity =
            specular.red() * 0.2125 + specular.green() * 0.7154 + specular.blue() * 0.0721;
        if intensity < 0.1 {
            roughness *= 1.0 - intensity;
        }
        roughness as f32
    }

    /// Estimates metalness from a legacy Phong specular color (largest linear channel).
    ///
    /// Example: `OpenPbrMaterial::metallic_from_specular(white) == 1.0`.
    pub fn metallic_from_specular(specular: Color) -> f32 {
        let [r, g, b] = specular.rgb();
        r.max(g).max(b).clamp(0.0, 1.0)
    }

    /// Whether the transmission slab is active.
    pub fn has_transmission(&self) -> bool {
        self.transmission.weight > 0.0 && self.base.metalness < 1.0
    }

    /// Whether the clear-coat slab is active.
    pub fn has_coat(&self) -> bool {
        self.coat.weight > 0.0
    }

    /// Whether the fuzz / sheen slab is active.
    pub fn has_fuzz(&self) -> bool {
        self.fuzz.weight > 0.0
    }

    /// Whether thin-film interference is active.
    pub fn has_thin_film(&self) -> bool {
        self.thin_film.weight > 0.0
    }

    /// Whether subsurface scattering is active.
    pub fn has_subsurface(&self) -> bool {
        self.subsurface.weight > 0.0 && self.base.metalness < 1.0 && self.transmission.weight < 1.0
    }

    /// Whether emission is active.
    pub fn has_emission(&self) -> bool {
        self.emission.luminance > 0.0
            && (self.emission.color[0] > 0.0
                || self.emission.color[1] > 0.0
                || self.emission.color[2] > 0.0)
    }

    /// Serializes all OpenPBR v1.1 parameter groups to a `DumpStream`.
    ///
    /// Example: `material.dump_json(&mut stream, -1)?`.
    pub fn dump_json(&self, stream: &mut DumpStream, _depth: i32) -> io::Result<()> {
        super::openpbr_validate::dump_openpbr_json(self, stream)
    }
}
