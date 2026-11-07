//! `RwObjMaterial`: a material of an MTL file.
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::tcollection::AsciiString;

/// A material definition; `RwObjMaterial::default()`.
#[derive(Clone, Debug)]
pub struct RwObjMaterial {
    /// The material name as defined in the MTL file.
    pub name: AsciiString,
    /// The path of the image defining the diffuse color.
    pub diffuse_texture: AsciiString,
    /// The path of the image defining the specular color.
    pub specular_texture: AsciiString,
    /// The path of the image defining the normal map.
    pub bump_texture: AsciiString,
    /// The ambient color.
    pub ambient_color: Color,
    /// The diffuse color.
    pub diffuse_color: Color,
    /// The specular color.
    pub specular_color: Color,
    /// The shininess, within 0 and 1.
    pub shininess: f32,
    /// The transparency, within 0 and 1.
    pub transparency: f32,
}

fn srgb_gray(value: f64) -> Color {
    Color::new(value, value, value, ColorSpace::Srgb).unwrap_or_default()
}

impl Default for RwObjMaterial {
    fn default() -> Self {
        Self {
            name: AsciiString::default(),
            diffuse_texture: AsciiString::default(),
            specular_texture: AsciiString::default(),
            bump_texture: AsciiString::default(),
            ambient_color: srgb_gray(0.1),
            diffuse_color: srgb_gray(0.8),
            specular_color: srgb_gray(0.2),
            shininess: 1.0,
            transparency: 0.0,
        }
    }
}
