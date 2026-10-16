//! Vector gamma conversions, the 2.2 approximation and the HLS helpers.
use super::Color;
use crate::standard::Failure;
impl Color {
    /// Gamma-encodes float linear RGB;
    /// `Color::linear_to_srgb_vec3_f32([0.0; 3]) == [0.0; 3]`.
    pub fn linear_to_srgb_vec3_f32(rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(Self::linear_to_srgb_f32)
    }
    /// Gamma-encodes double linear RGB;
    /// `Color::linear_to_srgb_vec3([1.0; 3])`.
    pub fn linear_to_srgb_vec3(rgb: [f64; 3]) -> [f64; 3] {
        rgb.map(Self::linear_to_srgb)
    }
    /// Decodes float sRGB to linear;
    /// `Color::srgb_to_linear_vec3_f32([0.0; 3]) == [0.0; 3]`.
    pub fn srgb_to_linear_vec3_f32(rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(Self::srgb_to_linear_f32)
    }
    /// Decodes double sRGB to linear;
    /// `Color::srgb_to_linear_vec3([0.5; 3])`.
    pub fn srgb_to_linear_vec3(rgb: [f64; 3]) -> [f64; 3] {
        rgb.map(Self::srgb_to_linear)
    }
    /// The 2.2 approximation. Kept on purpose: despite its name it
    /// raises to the power 2.2 (the decoding direction); `Color::linear_to_srgb_approx22(1.0)`.
    pub fn linear_to_srgb_approx22(value: f32) -> f32 {
        value.powf(2.2)
    }
    /// The inverse 2.2 approximation. Kept on purpose: it raises to 1/2.2
    /// (the encoding direction); `Color::srgb_to_linear_approx22(1.0) == 1.0`.
    pub fn srgb_to_linear_approx22(value: f32) -> f32 {
        value.powf(1.0 / 2.2)
    }
    /// Vector form of `linear_to_srgb_approx22`; `Color::linear_to_srgb_approx22_vec3([1.0; 3])`.
    pub fn linear_to_srgb_approx22_vec3(rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(Self::linear_to_srgb_approx22)
    }
    /// Vector form of `srgb_to_linear_approx22`; `Color::srgb_to_linear_approx22_vec3([1.0; 3])`.
    pub fn srgb_to_linear_approx22_vec3(rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(Self::srgb_to_linear_approx22)
    }
    /// Float gamma encoding, then HLS;
    /// `Color::linear_rgb_to_hls([1.0, 0.0, 0.0]) == [0.0, 1.0, 1.0]`.
    pub fn linear_rgb_to_hls(rgb: [f32; 3]) -> [f32; 3] {
        Self::srgb_to_hls(Self::linear_to_srgb_vec3_f32(rgb))
    }
    /// HLS to linear RGB; an invalid hue returns the "Color out" error;
    /// `Color::hls_to_linear_rgb([0.0, 1.0, 1.0])?`.
    pub fn hls_to_linear_rgb(hls: [f32; 3]) -> Result<[f32; 3], Failure> {
        Ok(Self::srgb_to_linear_vec3_f32(Self::hls_to_srgb(hls)?))
    }
    /// Double HLS narrowed to float, converted to sRGB and widened back. An invalid
    /// hue returns the "Color out" error;
    /// `Color::hls_rgb(0.0, 1.0, 1.0)? == [1.0, 0.0, 0.0]`.
    pub fn hls_rgb(hue: f64, light: f64, saturation: f64) -> Result<[f64; 3], Failure> {
        let hls = [hue as f32, light as f32, saturation as f32];
        Ok(Self::hls_to_srgb(hls)?.map(f64::from))
    }
    /// Double sRGB narrowed to float, converted to HLS and widened back;
    /// `Color::rgb_hls(1.0, 0.0, 0.0) == [0.0, 1.0, 1.0]`.
    pub fn rgb_hls(red: f64, green: f64, blue: f64) -> [f64; 3] {
        Self::srgb_to_hls([red as f32, green as f32, blue as f32]).map(f64::from)
    }
    /// Scales HLS light by `delta` percent; a result outside [0; 1]
    /// leaves the color unchanged; `color.change_intensity(-10.0)?`.
    pub fn change_intensity(&mut self, delta: f64) -> Result<(), Failure> {
        self.change_hls_component(1, delta)
    }
    /// Scales HLS saturation by `delta` percent; a result outside [0; 1]
    /// leaves the color unchanged; `color.change_contrast(10.0)?`.
    pub fn change_contrast(&mut self, delta: f64) -> Result<(), Failure> {
        self.change_hls_component(2, delta)
    }
    fn change_hls_component(&mut self, index: usize, delta: f64) -> Result<(), Failure> {
        let mut hls = Self::linear_rgb_to_hls(self.rgb());
        // Evaluation order: hls[i] += hls[i] * float(delta) / 100.0f, all in float.
        hls[index] += hls[index] * delta as f32 / 100.0;
        if hls[index] <= 1.0 && hls[index] >= 0.0 {
            self.set_rgb_unchecked(Self::hls_to_linear_rgb(hls)?);
        }
        Ok(())
    }
    /// Saturation and light differences `(DC, DI)` taken in float, then widened;
    /// `color.delta(other)`.
    pub fn delta(self, other: Self) -> (f64, f64) {
        let first = Self::linear_rgb_to_hls(self.rgb());
        let second = Self::linear_rgb_to_hls(other.rgb());
        (
            f64::from(first[2] - second[2]),
            f64::from(first[1] - second[1]),
        )
    }
}
