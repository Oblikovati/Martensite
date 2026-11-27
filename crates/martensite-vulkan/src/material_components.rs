//! Packed legacy Phong material record (`RenderMaterialCommon`); PBR materials use the
//! unified 256-byte [`crate::OpenPbrGpuMaterial`].

/// Phong coefficients in native vec4 order; `RenderMaterialCommon::default()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderMaterialCommon {
    /// Diffuse RGB coefficients and opacity.
    pub diffuse: [f32; 4],
    /// Emissive RGB coefficients and reserved component.
    pub emission: [f32; 4],
    /// Specular RGB coefficients and Phong exponent.
    pub specular_shininess: [f32; 4],
    /// Ambient RGB coefficients and reserved component.
    pub ambient: [f32; 4],
}

impl Default for RenderMaterialCommon {
    fn default() -> Self {
        Self {
            diffuse: [1.; 4],
            emission: [1.; 4],
            specular_shininess: [1., 1., 1., 0.],
            ambient: [1.; 4],
        }
    }
}

impl RenderMaterialCommon {
    /// Read the unnormalized Phong exponent; `material.shine()`.
    pub fn shine(&self) -> f32 {
        self.specular_shininess[3]
    }
    /// Mutate the exponent directly, without validation; `*material.change_shine() = 32.`.
    pub fn change_shine(&mut self) -> &mut f32 {
        &mut self.specular_shininess[3]
    }
    /// Highlight RGB without changing alpha, emission or gloss; `material.set_color([1.,0.,0.])`.
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.ambient[..3].copy_from_slice(&color.map(|channel| channel * 0.25));
        self.diffuse[..3].copy_from_slice(&color);
    }
    pub(crate) fn packed(&self) -> [[f32; 4]; 4] {
        [
            self.diffuse,
            self.emission,
            self.specular_shininess,
            self.ambient,
        ]
    }
}
