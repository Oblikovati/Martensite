//! 2D HDR Environment Importance-Sampling CDF / Alias Table for path-tracer Next Event
//! Estimation (NEE).
//!
//! Builds a Vose alias table over an equirectangular luminance grid weighted by solid angle
//! `sin(theta)`, enabling O(1) environment direction importance sampling in compute shaders.
use std::f32::consts::PI;

/// One GPU-aligned alias table entry (`std430`: 16 bytes = `vec4`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EnvAliasEntry {
    /// Threshold probability in `[0, 1]` for selecting the primary texel.
    pub prob: f32,
    /// Normalized PDF of this texel (`pdf_texel = lum * sin_theta / total_weight * (W*H)`).
    pub pdf: f32,
    /// Alias texel index (bitcast `u32` in `f32` or read as `uint`).
    pub alias: u32,
    /// Reserved alignment padding.
    pub pad: u32,
}

/// Complete 2D environment importance-sampling table.
#[derive(Clone, Debug)]
pub struct EnvImportanceTable {
    /// Width of the environment sampling grid.
    pub width: u32,
    /// Height of the environment sampling grid.
    pub height: u32,
    /// Total solid-angle-weighted luminance integral.
    pub total_weight: f32,
    /// Alias table entries (`width * height`).
    pub entries: Vec<EnvAliasEntry>,
}

impl EnvImportanceTable {
    /// Builds an O(1) Vose alias table from linear RGB texels (`width * height * 3` floats).
    ///
    /// Example: `EnvImportanceTable::from_rgb(4, 2, &vec![1.0; 24])`.
    pub fn from_rgb(width: u32, height: u32, rgb: &[f32]) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        if count == 0 || rgb.len() < count * 3 {
            return Self::uniform();
        }
        let mut weights = Vec::with_capacity(count);
        let mut total = 0.0f64;
        for y in 0..height as usize {
            let theta = PI * (y as f32 + 0.5) / height as f32;
            let sin_theta = (martensite_cmath::sin(f64::from(theta)) as f32).max(1e-5);
            for x in 0..width as usize {
                let base = (y * width as usize + x) * 3;
                let lum =
                    (0.2126 * rgb[base] + 0.7152 * rgb[base + 1] + 0.0722 * rgb[base + 2]).max(0.0);
                let w = f64::from(lum * sin_theta);
                weights.push(w);
                total += w;
            }
        }
        if total <= 1e-12 {
            return Self::uniform();
        }
        let entries = build_vose_alias(&weights, total);
        Self {
            width,
            height,
            total_weight: total as f32,
            entries,
        }
    }

    /// Minimal 1×1 uniform table for scenes without an HDR environment map.
    pub fn uniform() -> Self {
        Self {
            width: 1,
            height: 1,
            total_weight: 1.0,
            entries: vec![EnvAliasEntry {
                prob: 1.0,
                pdf: 1.0,
                alias: 0,
                pad: 0,
            }],
        }
    }

    /// Serializes alias entries as raw little-endian bytes for SSBO upload.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.entries.len() * 16);
        for e in &self.entries {
            bytes.extend_from_slice(&e.prob.to_ne_bytes());
            bytes.extend_from_slice(&e.pdf.to_ne_bytes());
            bytes.extend_from_slice(&e.alias.to_ne_bytes());
            bytes.extend_from_slice(&e.pad.to_ne_bytes());
        }
        bytes
    }
}

fn build_vose_alias(weights: &[f64], total: f64) -> Vec<EnvAliasEntry> {
    let n = weights.len();
    let scale = n as f64 / total;
    let mut scaled = Vec::with_capacity(n);
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut entries = vec![EnvAliasEntry::default(); n];
    for (idx, &w) in weights.iter().enumerate() {
        let p = w * scale;
        entries[idx].pdf = (w / total * n as f64) as f32;
        scaled.push(p);
        if p < 1.0 {
            small.push(idx);
        } else {
            large.push(idx);
        }
    }
    while let (Some(s), Some(l)) = (small.pop(), large.pop()) {
        entries[s].prob = scaled[s].clamp(0.0, 1.0) as f32;
        entries[s].alias = l as u32;
        scaled[l] = (scaled[l] + scaled[s]) - 1.0;
        if scaled[l] < 1.0 {
            small.push(l);
        } else {
            large.push(l);
        }
    }
    for idx in large.into_iter().chain(small) {
        entries[idx].prob = 1.0;
        entries[idx].alias = idx as u32;
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_table_pdf_sums_to_texel_count_and_concentrates_on_bright_sun() {
        let (w, h) = (8u32, 4u32);
        let mut rgb = vec![0.01f32; (w * h * 3) as usize];
        let sun_idx = w as usize + 3;
        let sun = sun_idx * 3;
        rgb[sun..sun + 3].copy_from_slice(&[100.0, 95.0, 90.0]);
        let table = EnvImportanceTable::from_rgb(w, h, &rgb);
        let pdf_sum: f32 = table.entries.iter().map(|e| e.pdf).sum();
        assert!((pdf_sum - (w * h) as f32).abs() < 1e-3, "pdf_sum={pdf_sum}");
        assert!(table.entries[sun_idx].pdf > 20.0);
    }
}
