//! Precomputed OpenPBR Surface v1.1 multi-scattering and spectral lookup tables (pure safe Rust,
//! deterministic, IEEE-754 half-float `R16*` output for guaranteed linear filtering on all
//! Vulkan 1.3 devices).
use crate::openpbr_math::{f0_from_ior, fresnel_dielectric_cos};
use std::f32::consts::PI;
use std::sync::OnceLock;

/// Dimensions of the GGX multi-scatter directional & hemispherical albedo LUT (`RG16F`).
pub const GGX_LUT_SIZE: u32 = 64;
/// Dimensions of the 3D dielectric reflectance/transmittance LUT (`RG16F`, `cos_v × roughness × ior`).
pub const DIELECTRIC_LUT_SIZE: u32 = 32;
/// Dimensions of the Charlie microfiber sheen directional albedo LUT (`R16F`, `cos_v × roughness`).
pub const SHEEN_LUT_SIZE: u32 = 32;
/// Width of the 1D CIE XYZ Airy thin-film Fourier sensitivity LUT (`RGBA16F`).
pub const THIN_FILM_LUT_SIZE: u32 = 64;

/// Cached precomputed half-float LUT payloads ready for Vulkan staging upload.
pub struct OpenPbrLutTables {
    /// `64×64` `R16G16_SFLOAT`: `R = E(cos_v, roughness)`, `G = E_avg(roughness)`.
    pub ggx_rg16f: Vec<u8>,
    /// `32×32×32` `R16G16_SFLOAT`: `R = R_ms`, `G = T_ms` across `(cos_v, roughness, ior_u)`.
    pub dielectric_rg16f: Vec<u8>,
    /// `32×32` `R16_SFLOAT`: Charlie microfiber sheen directional albedo `E_sheen(cos_v, r)`.
    pub sheen_r16f: Vec<u8>,
    /// `64×1` `R16G16B16A16_SFLOAT`: CIE XYZ Fourier sensitivity `S(opd)` for Airy interference.
    pub thin_film_rgba16f: Vec<u8>,
}

/// Process-wide cached LUT tables (generated once deterministically in ~25 ms).
pub fn cached_openpbr_luts() -> &'static OpenPbrLutTables {
    static TABLES: OnceLock<OpenPbrLutTables> = OnceLock::new();
    TABLES.get_or_init(|| OpenPbrLutTables {
        ggx_rg16f: build_ggx_multiscatter_lut(),
        dielectric_rg16f: build_dielectric_lut(),
        sheen_r16f: build_sheen_lut(),
        thin_film_rgba16f: build_thin_film_sensitivity_lut(),
    })
}

fn radical_inverse_vdc(mut bits: u32) -> f32 {
    bits = bits.reverse_bits();
    (bits as f64 * 2.328_306_436_538_696_3e-10) as f32
}

fn hammersley(i: u32, n: u32) -> [f32; 2] {
    [i as f32 / n as f32, radical_inverse_vdc(i)]
}

/// Sample GGX VNDF half-vector in tangent space (`N = (0, 0, 1)`), Heitz (2018).
fn sample_ggx_vndf(v: [f32; 3], alpha: f32, u: [f32; 2]) -> [f32; 3] {
    let vh = normalize3([alpha * v[0], alpha * v[1], v[2].max(1e-5)]);
    let lensq = vh[0] * vh[0] + vh[1] * vh[1];
    let t1 = if lensq > 1e-12 {
        let inv = lensq.sqrt().recip();
        [-vh[1] * inv, vh[0] * inv, 0.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let t2 = [
        vh[1] * t1[2] - vh[2] * t1[1],
        vh[2] * t1[0] - vh[0] * t1[2],
        vh[0] * t1[1] - vh[1] * t1[0],
    ];
    let r = u[0].clamp(0.0, 0.999_999).sqrt();
    let phi = 2.0 * PI * u[1];
    let (sin_phi, cos_phi) = martensite_cmath::sincos(f64::from(phi));
    let t_1 = r * cos_phi as f32;
    let mut t_2 = r * sin_phi as f32;
    let s = 0.5 * (1.0 + vh[2]);
    t_2 = (1.0 - s) * (1.0 - t_1 * t_1).max(0.0).sqrt() + s * t_2;
    let nh_z = (1.0 - t_1 * t_1 - t_2 * t_2).max(0.0).sqrt();
    let nh = [
        t_1 * t1[0] + t_2 * t2[0] + nh_z * vh[0],
        t_1 * t1[1] + t_2 * t2[1] + nh_z * vh[1],
        t_1 * t1[2] + t_2 * t2[2] + nh_z * vh[2],
    ];
    normalize3([alpha * nh[0], alpha * nh[1], nh[2].max(0.0)])
}

fn ggx_g1(cos_theta: f32, alpha: f32) -> f32 {
    let a2 = alpha * alpha;
    let c2 = cos_theta * cos_theta;
    2.0 * cos_theta / (cos_theta + (a2 + (1.0 - a2) * c2).max(1e-8).sqrt())
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-12 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// Generates the `64×64` GGX directional albedo `E(cos_v, r)` and average albedo `E_avg(r)`.
fn build_ggx_multiscatter_lut() -> Vec<u8> {
    let n = GGX_LUT_SIZE as usize;
    let samples = 128u32;
    let mut e_grid = vec![0.0f32; n * n];
    let mut e_avg = vec![0.0f32; n];

    for y in 0..n {
        let roughness = (y as f32 + 0.5) / n as f32;
        let alpha = (roughness * roughness).max(0.002);
        let mut avg_acc = 0.0f32;
        for x in 0..n {
            let cos_v = ((x as f32 + 0.5) / n as f32).clamp(1e-3, 1.0);
            let sin_v = (1.0 - cos_v * cos_v).max(0.0).sqrt();
            let v = [sin_v, 0.0, cos_v];
            let mut sum = 0.0f32;
            for i in 0..samples {
                let h = sample_ggx_vndf(v, alpha, hammersley(i, samples));
                let vh = (v[0] * h[0] + v[1] * h[1] + v[2] * h[2]).max(0.0);
                let l_z = 2.0 * vh * h[2] - v[2];
                if l_z > 0.0 {
                    // VNDF weight = G2 / G1_v = G1_l / (1 + Lambda_v + Lambda_l) * (1 + Lambda_v)
                    // Exact separable Smith approximation: G1(l_z).
                    sum += ggx_g1(l_z, alpha);
                }
            }
            let e = (sum / samples as f32).clamp(0.01, 1.0);
            e_grid[y * n + x] = e;
            avg_acc += 2.0 * e * cos_v / n as f32;
        }
        e_avg[y] = avg_acc.clamp(0.01, 1.0);
    }

    let mut bytes = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            push_f16(&mut bytes, e_grid[y * n + x]);
            push_f16(&mut bytes, e_avg[y]);
        }
    }
    bytes
}

/// Maps normalized 3D LUT `z` coordinate in `[0, 1]` to refractive index `eta` (covering both
/// `eta < 1` internal/TIR and `eta >= 1` external dielectrics, matching Blender's F0 parameterization).
pub fn dielectric_lut_z_to_ior(z: f32) -> f32 {
    if z < 0.5 {
        // Internal (`eta < 1`): z in [0, 0.5) maps sqrt(F0) from 0.99 down to 0.
        let s = 1.0 - z * 2.0;
        let ior = (1.0 + s * 0.85) / (1.0 - s * 0.85).max(1e-3);
        1.0 / ior
    } else {
        // External (`eta >= 1`): z in [0.5, 1] maps sqrt(F0) from 0 to 0.85 (ior up to ~12.3).
        let s = (z - 0.5) * 2.0;
        (1.0 + s * 0.85) / (1.0 - s * 0.85).max(1e-3)
    }
}

/// Inverse mapping of [`dielectric_lut_z_to_ior`]: maps `ior` to 3D texture coordinate `z in [0, 1]`.
pub fn dielectric_ior_to_lut_z(ior: f32) -> f32 {
    let f0_sqrt = f0_from_ior(if ior < 1.0 { 1.0 / ior.max(1e-4) } else { ior })
        .sqrt()
        .clamp(0.0, 0.85);
    let s = f0_sqrt / 0.85;
    if ior < 1.0 {
        0.5 * (1.0 - s)
    } else {
        0.5 * (1.0 + s)
    }
}

/// Generates the `32×32×32` multi-scatter dielectric LUT `(R, T)` (`R16G16_SFLOAT`).
fn build_dielectric_lut() -> Vec<u8> {
    let n = DIELECTRIC_LUT_SIZE as usize;
    let samples = 64u32;
    let mut bytes = Vec::with_capacity(n * n * n * 4);
    for z in 0..n {
        let ior = dielectric_lut_z_to_ior((z as f32 + 0.5) / n as f32);
        for y in 0..n {
            let roughness = (y as f32 + 0.5) / n as f32;
            let alpha = (roughness * roughness).max(0.002);
            for x in 0..n {
                let cos_v = ((x as f32 + 0.5) / n as f32).clamp(1e-3, 1.0);
                let sin_v = (1.0 - cos_v * cos_v).max(0.0).sqrt();
                let v = [sin_v, 0.0, cos_v];
                let mut r_acc = 0.0f32;
                let mut t_acc = 0.0f32;
                for i in 0..samples {
                    let h = sample_ggx_vndf(v, alpha, hammersley(i, samples));
                    let vh = (v[0] * h[0] + v[1] * h[1] + v[2] * h[2]).max(1e-5);
                    let f = fresnel_dielectric_cos(vh, ior);
                    let l_z = 2.0 * vh * h[2] - v[2];
                    if l_z > 0.0 {
                        r_acc += f * ggx_g1(l_z, alpha);
                    }
                    let eta_inv = 1.0 / ior;
                    let k = 1.0 - eta_inv * eta_inv * (1.0 - vh * vh);
                    if k > 0.0 {
                        let t_z = (eta_inv * vh - k.sqrt()) * h[2] - eta_inv * v[2];
                        if t_z < 0.0 {
                            t_acc += (1.0 - f) * ggx_g1(-t_z, alpha);
                        }
                    }
                }
                let r_ss = (r_acc / samples as f32).clamp(0.0, 1.0);
                let t_ss = (t_acc / samples as f32).clamp(0.0, 1.0);
                // Multi-scattering energy compensation restores missing energy proportionally to R vs T.
                let missing = (1.0 - r_ss - t_ss).max(0.0);
                let denom = (r_ss + t_ss).max(1e-5);
                let r_ms = (r_ss + missing * (r_ss / denom)).clamp(0.0, 1.0);
                let t_ms = (t_ss + missing * (t_ss / denom)).clamp(0.0, 1.0);
                push_f16(&mut bytes, r_ms);
                push_f16(&mut bytes, t_ms);
            }
        }
    }
    bytes
}

/// Charlie microfiber NDF `D(sin_theta, alpha)` (Estevez & Kulla 2017).
fn charlie_d(cos_h: f32, roughness: f32) -> f32 {
    let alpha = (roughness * roughness).max(0.01);
    let inv_r = 1.0 / alpha;
    let sin2 = (1.0 - cos_h * cos_h).max(0.0);
    (2.0 + inv_r) * martensite_cmath::powf(sin2, 0.5 * inv_r) / (2.0 * PI)
}

fn charlie_l(x: f32, r: f32) -> f32 {
    let r = (1.0 - r) * (1.0 - r);
    let a = 25.3245 + (21.5473 - 25.3245) * r;
    let b = 3.32435 + (3.82987 - 3.32435) * r;
    let c = 0.16801 + (0.19823 - 0.16801) * r;
    let d = -1.27393 + (-1.97760 - -1.27393) * r;
    let e = -4.85967 + (-4.32054 - -4.85967) * r;
    a / (1.0 + b * martensite_cmath::powf(x.max(1e-4), c)) + d * x + e
}

fn charlie_lambda(cos_theta: f32, r: f32) -> f32 {
    if cos_theta < 0.5 {
        martensite_cmath::exp(f64::from(charlie_l(cos_theta, r))) as f32
    } else {
        martensite_cmath::exp(f64::from(
            2.0 * charlie_l(0.5, r) - charlie_l(1.0 - cos_theta, r),
        )) as f32
    }
}

/// Generates the `32×32` Charlie microfiber sheen directional albedo LUT (`R16_SFLOAT`).
fn build_sheen_lut() -> Vec<u8> {
    let n = SHEEN_LUT_SIZE as usize;
    let samples = 128u32;
    let mut bytes = Vec::with_capacity(n * n * 2);
    for y in 0..n {
        let r = ((y as f32 + 0.5) / n as f32).clamp(0.05, 1.0);
        for x in 0..n {
            let cos_v = ((x as f32 + 0.5) / n as f32).clamp(0.01, 1.0);
            let sin_v = (1.0 - cos_v * cos_v).max(0.0).sqrt();
            let v = [sin_v, 0.0, cos_v];
            let lambda_v = charlie_lambda(cos_v, r);
            let mut sum = 0.0f32;
            for i in 0..samples {
                let u = hammersley(i, samples);
                // Uniform hemisphere sampling (`pdf = 1 / (2*pi)`).
                let cos_l = u[0].clamp(1e-4, 1.0);
                let sin_l = (1.0 - cos_l * cos_l).max(0.0).sqrt();
                let phi = 2.0 * PI * u[1];
                let (sin_phi, cos_phi) = martensite_cmath::sincos(f64::from(phi));
                let l = [sin_l * cos_phi as f32, sin_l * sin_phi as f32, cos_l];
                let h = normalize3([v[0] + l[0], v[1] + l[1], v[2] + l[2]]);
                let d = charlie_d(h[2].max(0.0), r);
                let lambda_l = charlie_lambda(cos_l, r);
                let vis =
                    1.0 / (4.0 * (cos_l + cos_v - cos_l * cos_v) * (1.0 + lambda_v + lambda_l));
                let brdf = d * vis;
                sum += (brdf * cos_l * 2.0 * PI).min(1.0);
            }
            let albedo = (sum / samples as f32).clamp(0.0, 1.0);
            push_f16(&mut bytes, albedo);
        }
    }
    bytes
}

/// Generates the `64×1` spectral Airy thin-film Fourier sensitivity LUT (`RGBA16F`).
///
/// Integrates `CIE_XYZ(λ) * cos(2π * opd / λ)` over visible wavelengths `380..780 nm` for
/// `opd ∈ [0, 3200 nm]` (converted to linear Rec.709 RGB in `.rgb`, normalized envelope in `.a`).
fn build_thin_film_sensitivity_lut() -> Vec<u8> {
    let n = THIN_FILM_LUT_SIZE as usize;
    let mut bytes = Vec::with_capacity(n * 8);
    // Analytic Gaussian CIE 1931 2-degree color matching fit (Wyman et al. 2013).
    let cie_xyz = |wl: f32| -> [f32; 3] {
        let g = |x: f32, mu: f32, s1: f32, s2: f32| {
            let s = if x < mu { s1 } else { s2 };
            let t = (x - mu) * s;
            martensite_cmath::exp(f64::from(-0.5 * t * t)) as f32
        };
        let x = 1.056 * g(wl, 599.8, 0.0264, 0.0323) + 0.362 * g(wl, 442.0, 0.0624, 0.0374)
            - 0.065 * g(wl, 501.1, 0.0490, 0.0382);
        let y = 0.821 * g(wl, 568.8, 0.0213, 0.0247) + 0.286 * g(wl, 530.9, 0.0613, 0.0322);
        let z = 1.217 * g(wl, 437.0, 0.0845, 0.0278) + 0.681 * g(wl, 459.0, 0.0385, 0.0725);
        [x, y, z]
    };
    let steps = 80usize;
    let mut norm_xyz = [0.0f32; 3];
    for s in 0..steps {
        let wl = 380.0 + (s as f32 + 0.5) * (400.0 / steps as f32);
        let xyz = cie_xyz(wl);
        for c in 0..3 {
            norm_xyz[c] += xyz[c];
        }
    }
    for i in 0..n {
        let opd = (i as f32 / (n - 1) as f32) * 3200.0;
        let mut c_xyz = [0.0f32; 3];
        for s in 0..steps {
            let wl = 380.0 + (s as f32 + 0.5) * (400.0 / steps as f32);
            let phase = 2.0 * PI * opd / wl;
            let xyz = cie_xyz(wl);
            let cos_p = martensite_cmath::cos(f64::from(phase)) as f32;
            for c in 0..3 {
                c_xyz[c] += xyz[c] * cos_p;
            }
        }
        let xyz = [
            c_xyz[0] / norm_xyz[0],
            c_xyz[1] / norm_xyz[1],
            c_xyz[2] / norm_xyz[2],
        ];
        // XYZ (D65 white-balanced) -> linear Rec.709 RGB.
        let r = 3.240_454_2 * xyz[0] - 1.537_138_5 * xyz[1] - 0.498_531_4 * xyz[2];
        let g = -0.969_266 * xyz[0] + 1.876_010_8 * xyz[1] + 0.041_556 * xyz[2];
        let b = 0.055_643_4 * xyz[0] - 0.204_025_9 * xyz[1] + 1.057_225_2 * xyz[2];
        push_f16(&mut bytes, r);
        push_f16(&mut bytes, g);
        push_f16(&mut bytes, b);
        push_f16(&mut bytes, (r.abs() + g.abs() + b.abs()) / 3.0);
    }
    bytes
}

/// Converts a finite `f32` to IEEE-754 binary16 bits (`u16`).
pub fn f32_to_f16_bits(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32 - 127;
    let mant = bits & 0x007f_ffff;
    if exp > 15 {
        return sign | 0x7bff; // clamp to max finite half
    }
    if exp < -14 {
        let shift = (-14 - exp) as u32;
        if shift > 10 {
            return sign;
        }
        let sub = (0x0080_0000u32 | mant) >> (shift + 13);
        return sign | sub as u16;
    }
    sign | (((exp + 15) as u16) << 10) | ((mant >> 13) as u16)
}

fn push_f16(out: &mut Vec<u8>, value: f32) {
    out.extend_from_slice(&f32_to_f16_bits(value).to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luts_have_exact_byte_sizes_and_ior_mapping_round_trips() {
        let luts = cached_openpbr_luts();
        assert_eq!(
            luts.ggx_rg16f.len(),
            (GGX_LUT_SIZE * GGX_LUT_SIZE * 4) as usize
        );
        assert_eq!(
            luts.dielectric_rg16f.len(),
            (DIELECTRIC_LUT_SIZE * DIELECTRIC_LUT_SIZE * DIELECTRIC_LUT_SIZE * 4) as usize
        );
        assert_eq!(
            luts.sheen_r16f.len(),
            (SHEEN_LUT_SIZE * SHEEN_LUT_SIZE * 2) as usize
        );
        assert_eq!(
            luts.thin_film_rgba16f.len(),
            (THIN_FILM_LUT_SIZE * 8) as usize
        );
        for ior in [0.5, 0.75, 1.0, 1.33, 1.5, 2.42] {
            let z = dielectric_ior_to_lut_z(ior);
            let back = dielectric_lut_z_to_ior(z);
            assert!((back - ior).abs() < 1e-4, "ior={ior} z={z} back={back}");
        }
    }
}
