//! Predefined materials catalogue (`RawMaterial`, `the_materials`).
use super::material_specs::{MATERIAL_SPECS, MaterialSpec, OpenPbrPreset};
use super::{Graphic3dNameOfMaterial, Graphic3dTypeOfMaterial, OpenPbrMaterial};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::Color;
use std::sync::OnceLock;

/// One predefined material (`RawMaterial`).
#[derive(Clone, Debug)]
pub(crate) struct RawMaterial {
    pub(crate) string_name: &'static str,
    pub(crate) openpbr: OpenPbrMaterial,
    /// Ambient, diffuse, specular and emission colors (`Colors[Graphic3d_TOR_*]`).
    pub(crate) colors: [Color; 4],
    pub(crate) transparency_coef: f32,
    pub(crate) refraction_index: f32,
    pub(crate) shininess: f32,
    /// Coefficients of `Graphic3dMaterialAspect::set_color`.
    pub(crate) ambient_coef: f32,
    pub(crate) diffuse_coef: f32,
    pub(crate) material_type: Graphic3dTypeOfMaterial,
}

/// The materials in `Graphic3dNameOfMaterial` order.
pub(crate) const MATERIAL_NAMES: [(Graphic3dNameOfMaterial, &str); 26] = {
    use Graphic3dNameOfMaterial as N;
    [
        (N::Brass, "Brass"),
        (N::Bronze, "Bronze"),
        (N::Copper, "Copper"),
        (N::Gold, "Gold"),
        (N::Pewter, "Pewter"),
        (N::Plastered, "Plastered"),
        (N::Plastified, "Plastified"),
        (N::Silver, "Silver"),
        (N::Steel, "Steel"),
        (N::Stone, "Stone"),
        (N::ShinyPlastified, "Shiny_plastified"),
        (N::Satin, "Satined"),
        (N::Metalized, "Metalized"),
        (N::Ionized, "Ionized"),
        (N::Chrome, "Chrome"),
        (N::Aluminum, "Aluminium"),
        (N::Obsidian, "Obsidian"),
        (N::Neon, "Neon"),
        (N::Jade, "Jade"),
        (N::Charcoal, "Charcoal"),
        (N::Water, "Water"),
        (N::Glass, "Glass"),
        (N::Diamond, "Diamond"),
        (N::Transparent, "Transparent"),
        (N::Default, "Default"),
        (N::UserDefined, "UserDefined"),
    ]
};

fn build_openpbr(preset: OpenPbrPreset, refraction_index: f32) -> OpenPbrMaterial {
    let mut m = OpenPbrMaterial::default();
    match preset {
        OpenPbrPreset::Dielectric {
            color,
            diffuse_roughness,
            roughness,
            ior,
            coat_weight,
            coat_roughness,
        } => {
            m.base.color = color;
            m.base.diffuse_roughness = diffuse_roughness;
            m.specular.roughness = roughness;
            m.specular.ior = ior;
            m.coat.weight = coat_weight;
            m.coat.roughness = coat_roughness;
        }
        OpenPbrPreset::Metal {
            f0,
            f82,
            roughness,
            anisotropy,
        } => {
            m.base.color = f0;
            m.base.metalness = 1.0;
            m.specular.color = f82;
            m.specular.roughness = roughness;
            m.specular.roughness_anisotropy = anisotropy;
        }
        OpenPbrPreset::Satin {
            color,
            roughness,
            anisotropy,
            fuzz_weight,
            fuzz_roughness,
        } => {
            m.base.color = color;
            m.specular.roughness = roughness;
            m.specular.roughness_anisotropy = anisotropy;
            m.fuzz.weight = fuzz_weight;
            m.fuzz.color = color;
            m.fuzz.roughness = fuzz_roughness;
        }
        OpenPbrPreset::Subsurface {
            color,
            sss_color,
            weight,
            radius,
            radius_scale,
            roughness,
            ior,
        } => {
            m.base.color = color;
            m.specular.roughness = roughness;
            m.specular.ior = ior;
            m.subsurface.weight = weight;
            m.subsurface.color = sss_color;
            m.subsurface.radius = radius;
            m.subsurface.radius_scale = radius_scale;
        }
        OpenPbrPreset::Emissive {
            base_color,
            emission_color,
            luminance,
            roughness,
        } => {
            m.base.color = base_color;
            m.specular.roughness = roughness;
            m.emission.color = emission_color;
            m.emission.luminance = luminance;
        }
        OpenPbrPreset::Glass {
            tint,
            depth,
            roughness,
            dispersion_scale,
            abbe_number,
        } => {
            m.base.color = tint;
            m.specular.ior = refraction_index;
            m.specular.roughness = roughness;
            m.transmission.weight = 1.0;
            m.transmission.color = tint;
            m.transmission.depth = depth;
            m.transmission.dispersion_scale = dispersion_scale;
            m.transmission.dispersion_abbe_number = abbe_number;
        }
        OpenPbrPreset::Transparent { color, opacity } => {
            m.base.color = color;
            m.specular.ior = 1.0;
            m.specular.roughness = 0.05;
            m.geometry.opacity = opacity;
            m.geometry.thin_walled = true;
        }
    }
    debug_assert!(m.validate().is_ok());
    m
}

fn color(rgb: [f32; 3]) -> Color {
    Color::from_vec3(Vec3::new(rgb[0], rgb[1], rgb[2]))
        .expect("predefined material colors are in [0, 1]")
}

fn raw_material(spec: &MaterialSpec, string_name: &'static str) -> RawMaterial {
    let (refraction_index, transparency_coef) = spec.optics.unwrap_or((1.0, 0.0));
    let (ambient_coef, diffuse_coef) = spec.coefs.unwrap_or((0.25, 1.0));
    let openpbr = build_openpbr(spec.openpbr, refraction_index);
    let material_type = if spec.physic {
        Graphic3dTypeOfMaterial::Physic
    } else {
        Graphic3dTypeOfMaterial::Aspect
    };
    RawMaterial {
        string_name,
        openpbr,
        colors: spec.colors.map(color),
        transparency_coef,
        refraction_index,
        shininess: spec.shininess,
        ambient_coef,
        diffuse_coef,
        material_type,
    }
}

/// Predefined material catalogue, built once.
pub(crate) fn the_materials() -> &'static [RawMaterial] {
    static MATERIALS: OnceLock<Vec<RawMaterial>> = OnceLock::new();
    MATERIALS.get_or_init(|| {
        MATERIAL_SPECS
            .iter()
            .zip(MATERIAL_NAMES)
            .map(|(spec, (_, text))| raw_material(spec, text))
            .collect()
    })
}
