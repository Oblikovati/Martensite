//! CPU reference implementation of ASWF OpenPBR Surface v1.1 analytical equations and HDR display
//! transforms (AgX, ACES, Filmic), mirroring `shaders/openpbr/openpbr_common.glsl`.
//!
//! Used both for LUT generation and as closed-form mathematical oracles in verification tests.
use std::f32::consts::PI;

/// Normal-incidence dielectric Fresnel reflectance `F0 = ((ior - 1) / (ior + 1))^2`.
pub fn f0_from_ior(ior: f32) -> f32 {
    let r = (ior - 1.0) / (ior + 1.0);
    r * r
}

/// Inverts `f0_from_ior` for `ior >= 1`.
pub fn ior_from_f0(f0: f32) -> f32 {
    let f = f0.clamp(0.0, 0.9999).sqrt();
    (1.0 + f) / (1.0 - f)
}

/// Exact unpolarized dielectric Fresnel reflectance (`fresnel_dielectric_cos`).
pub fn fresnel_dielectric_cos(cos_i: f32, eta: f32) -> f32 {
    let c = cos_i.abs();
    let g_sq = eta * eta - 1.0 + c * c;
    if g_sq <= 0.0 {
        return 1.0;
    }
    let g = g_sq.sqrt();
    let a = (g - c) / (g + c);
    let b = (c * (g + c) - 1.0) / (c * (g - c) + 1.0);
    0.5 * a * a * (1.0 + b * b)
}

/// d'Eon (v0.3.2 Eq. 11.15 / 11.12) analytical hemispherical Fresnel albedo.
pub fn hemispherical_albedo(eta: f32) -> f32 {
    let eta_ = if eta < 1.0 { 1.0 / eta } else { eta };
    let num = 10893.0 * eta_ - 1438.2;
    let den = -774.4 * eta_ * eta_ + 10212.0 * eta_ + 1.0;
    let mut albedo = martensite_cmath::log(f64::from((num / den).max(1e-7))) as f32;
    if eta < 1.0 {
        albedo = 1.0 - eta * eta * (1.0 - albedo);
    }
    albedo.clamp(0.0, 1.0)
}

/// OpenPBR v1.1 Eq. (25-26): modulates specular IOR so `F0` scales by `weight`.
pub fn openpbr_modulate_ior(ior: f32, weight: f32) -> f32 {
    if (weight - 1.0).abs() <= 1e-6 {
        return ior;
    }
    let modulated = ior_from_f0(weight * f0_from_ior(ior));
    if ior < 1.0 {
        1.0 / modulated
    } else {
        modulated
    }
}

/// OpenPBR v1.1 Eq. (60, 74): effective specular IOR beneath a dielectric coat with TIR fix.
pub fn openpbr_effective_specular_ior(
    specular_ior: f32,
    specular_weight: f32,
    coat_ior: f32,
    coat_weight: f32,
) -> f32 {
    let mut ior = specular_ior.max(1e-5);
    if coat_weight > 0.0 {
        let coat_ior = coat_ior.max(1e-5);
        let tir_ratio = if coat_ior > ior {
            coat_ior / ior
        } else {
            ior / coat_ior
        };
        ior = ior + (tir_ratio - ior) * coat_weight;
    }
    openpbr_modulate_ior(ior, specular_weight)
}

/// OpenPBR v1.1 Eq. (73): roughening of base specular layer beneath a rough coat.
pub fn openpbr_coat_roughening(
    specular_roughness: f32,
    coat_roughness: f32,
    coat_weight: f32,
) -> f32 {
    if coat_weight <= 0.0 || coat_roughness <= 0.0 {
        return specular_roughness;
    }
    let r2 = specular_roughness * specular_roughness;
    let c2 = coat_roughness * coat_roughness;
    let coated = martensite_cmath::powf(r2 * r2 + 2.0 * c2 * c2, 0.25).min(1.0);
    specular_roughness + (coated - specular_roughness) * coat_weight
}

/// OpenPBR v1.1 anisotropic microfacet roughness `(alpha_x, alpha_y)` from `(roughness, anisotropy)`.
pub fn openpbr_anisotropic_roughness(roughness: f32, anisotropy: f32) -> [f32; 2] {
    let r2 = roughness * roughness;
    let factor = (1.0 - 0.9 * anisotropy.clamp(0.0, 1.0)).sqrt();
    [(r2 / factor).min(1.0), r2 * factor]
}

/// OpenPBR v1.1 Eq. (64-70): coat internal-reflection darkening factor `Delta`.
#[allow(clippy::too_many_arguments)]
pub fn openpbr_coat_darkening(
    cos_i: f32,
    base_color: [f32; 3],
    metalness: f32,
    specular_color: [f32; 3],
    specular_ior: f32,
    specular_roughness: f32,
    transmission_weight: f32,
    transmission_color: [f32; 3],
    subsurface_weight: f32,
    subsurface_color: [f32; 3],
    coat_weight: f32,
    coat_ior: f32,
    coat_darkening: f32,
) -> [f32; 3] {
    if coat_weight <= 0.0 || coat_darkening <= 0.0 {
        return [1.0; 3];
    }
    let f0 = f0_from_ior(specular_ior);
    let e_f = hemispherical_albedo(coat_ior);
    let k_r = 1.0 - (1.0 - e_f) / (coat_ior * coat_ior);
    let k_s = fresnel_dielectric_cos(cos_i, coat_ior);
    let r_d = 1.0 + (specular_roughness - 1.0) * f0;
    let r_b = r_d + (specular_roughness - r_d) * metalness;
    let k = k_s + (k_r - k_s) * r_b;
    let mix_factor = coat_weight * coat_darkening;
    std::array::from_fn(|c| {
        let e_glossy = f0 * specular_color[c];
        let e_gd = e_glossy + (1.0 - e_glossy) * base_color[c];
        let e_opaque = e_gd + (subsurface_color[c] - e_gd) * subsurface_weight;
        let e_trans = e_glossy + (1.0 - f0) * transmission_color[c];
        let e_diel = e_opaque + (e_trans - e_opaque) * transmission_weight;
        let e_b = e_diel + (base_color[c] - e_diel) * metalness;
        if (e_b - 1.0).abs() <= 1e-6 {
            return 1.0;
        }
        let delta = (1.0 - k) / (1.0 - e_b * k).max(1e-6);
        1.0 + (delta - 1.0) * mix_factor
    })
}

/// F82-tint conductor Fresnel reflectance (OpenPBR v1.1 / Kutz et al.).
pub fn fresnel_f82_tint(cos_i: f32, f0: [f32; 3], tint: [f32; 3]) -> [f32; 3] {
    let s = (1.0 - cos_i).clamp(0.0, 1.0);
    let s5 = s * s * s * s * s;
    const F: f32 = 6.0 / 7.0;
    const F5: f32 = F * F * F * F * F;
    const INV_SCALE: f32 = 7.0 / (F5 * F);
    std::array::from_fn(|c| {
        let f_schlick_82 = f0[c] + (1.0 - f0[c]) * F5;
        let b = f_schlick_82 * INV_SCALE * (1.0 - tint[c]);
        let f_schlick = f0[c] + (1.0 - f0[c]) * s5;
        (f_schlick - b * cos_i.max(0.0) * s5 * s).clamp(0.0, 1.0)
    })
}

/// Beer-Lambert slab transmittance at angle `cos_theta_i` through a layer of refractive index `ior`.
pub fn slab_transmittance_at_angle(color: [f32; 3], cos_theta_i: f32, ior: f32) -> [f32; 3] {
    let sin2 = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
    let inv_cos_t = ior / (ior * ior - sin2).max(1e-5).sqrt();
    color.map(|c| martensite_cmath::powf(c.clamp(1e-5, 1.0), inv_cos_t))
}

/// Thin-walled glass transmission roughness remap (OpenPBR v1.1 / Cycles).
pub fn thin_glass_transmission_roughness(roughness: f32, ior: f32) -> f32 {
    let eta = ior.max(1.0);
    let em = eta - 0.5;
    let scale = (3.4 * (eta - 1.0) * em * em / (eta * eta * eta))
        .max(0.0)
        .sqrt();
    (roughness * scale).clamp(0.0, 1.0)
}

/// Energy-preserving Oren-Nayar (EON) BRDF evaluation `f(wi, wo)` (without `cos_l` factor).
pub fn oren_nayar_eon(
    albedo: [f32; 3],
    roughness: f32,
    n_dot_v: f32,
    n_dot_l: f32,
    v_dot_l: f32,
) -> [f32; 3] {
    let sigma = roughness.clamp(0.0, 1.0);
    if sigma < 1e-5 {
        return albedo.map(|c| c / PI);
    }
    let a = 1.0 / (PI + sigma * (PI * 0.5 - 2.0 / 3.0));
    let b = sigma * a;
    let nv = n_dot_v.max(0.0);
    let nl = n_dot_l.max(0.0);
    let mut t = v_dot_l - nl * nv;
    if t > 0.0 {
        t /= nl.max(nv).max(1e-7);
    }
    let single = a + b * t;
    let e_avg = a * PI + ((2.0 * PI - 5.6) / 3.0) * b;
    let ev = a * PI + b * oren_nayar_g(nv);
    let el = a * PI + b * oren_nayar_g(nl);
    std::array::from_fn(|c| {
        let rho = albedo[c].clamp(0.0, 1.0);
        let e_ms = (rho * rho / PI) * (e_avg / (1.0 - e_avg).max(1e-6))
            / (1.0 - rho * (1.0 - e_avg)).max(1e-6);
        rho * single + e_ms * (1.0 - ev) * (1.0 - el)
    })
}

fn oren_nayar_g(cos_theta: f32) -> f32 {
    if cos_theta < 1e-6 {
        return (PI * 0.5 - 2.0 / 3.0) - cos_theta;
    }
    let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
    let theta = martensite_cmath::acos(f64::from(cos_theta.clamp(-1.0, 1.0))) as f32;
    sin_theta * (theta - 2.0 / 3.0 - sin_theta * cos_theta)
        + (2.0 / 3.0) * (sin_theta / cos_theta) * (1.0 - sin_theta * sin_theta * sin_theta)
}

/// AgX display transform (Blender AgX inset + 6th-order polynomial sigmoid + outset).
pub fn tonemap_agx(linear_rgb: [f32; 3]) -> [f32; 3] {
    let x = linear_rgb.map(|c| c.max(0.0));
    // Inset matrix (sRGB -> AgX working space).
    let m_in = [
        [0.842_479_05, 0.078_433_6, 0.079_223_75],
        [0.042_328_24, 0.878_468_63, 0.079_166_13],
        [0.042_375_65, 0.078_433_6, 0.879_142_94],
    ];
    let v = mat3_mul(m_in, x);
    let min_ev = -12.473_93f32;
    let max_ev = 4.026_069f32;
    let log_v = v.map(|c| {
        let l = (martensite_cmath::log2(f64::from(c.max(1e-10))) as f32).clamp(min_ev, max_ev);
        (l - min_ev) / (max_ev - min_ev)
    });
    let sig = log_v.map(|u| {
        let u2 = u * u;
        let u4 = u2 * u2;
        15.5 * u4 * u2 - 40.14 * u4 * u + 31.96 * u4 - 6.868 * u2 * u + 0.4298 * u2 + 0.1191 * u
            - 0.00232
    });
    // Outset matrix (AgX working space -> sRGB).
    let m_out = [
        [1.196_879_4, -0.098_020_88, -0.099_029_75],
        [-0.052_896_85, 1.151_903_1, -0.098_961_18],
        [-0.052_971_63, -0.098_043_45, 1.151_073_7],
    ];
    mat3_mul(m_out, sig).map(|c| c.clamp(0.0, 1.0))
}

/// Fitted ACES RRT + ODT display transform (Stephen Hill sRGB <-> AP1 fit).
pub fn tonemap_aces(linear_rgb: [f32; 3]) -> [f32; 3] {
    let x = linear_rgb.map(|c| c.max(0.0));
    let m_in = [
        [0.59719, 0.35458, 0.04823],
        [0.07600, 0.90834, 0.01566],
        [0.02840, 0.13383, 0.83777],
    ];
    let m_out = [
        [1.60475, -0.53108, -0.07367],
        [-0.10208, 1.10813, -0.00605],
        [-0.00327, -0.07276, 1.07602],
    ];
    let v = mat3_mul(m_in, x).map(|c| {
        let num = c * (c + 0.0245786) - 0.000090537;
        let den = c * (0.983_729 * c + 0.432_951) + 0.238_081;
        num / den
    });
    mat3_mul(m_out, v).map(|c| c.clamp(0.0, 1.0))
}

fn mat3_mul(m: [[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openpbr_ior_modulation_and_coat_roughening_match_spec() {
        assert!((f0_from_ior(1.5) - 0.04).abs() < 1e-6);
        assert!((ior_from_f0(0.04) - 1.5).abs() < 1e-4);
        assert!((openpbr_modulate_ior(1.5, 0.0) - 1.0).abs() < 1e-5);
        assert!((openpbr_modulate_ior(1.5, 1.0) - 1.5).abs() < 1e-6);
        // Eq. (73): r=0, coat_r=0.5, coat_w=1 => (2 * 0.5^4)^0.25 = 2^0.25 * 0.5
        let expected = martensite_cmath::powf(2.0, 0.25) * 0.5;
        assert!((openpbr_coat_roughening(0.0, 0.5, 1.0) - expected).abs() < 1e-6);
    }

    #[test]
    fn coat_darkening_is_unit_for_white_base_and_darkens_saturated_base() {
        let white = openpbr_coat_darkening(
            1.0, [1.0; 3], 0.0, [1.0; 3], 1.5, 0.3, 0.0, [1.0; 3], 0.0, [0.8; 3], 1.0, 1.6, 1.0,
        );
        assert!(white.iter().all(|&c| (c - 1.0).abs() < 1e-5));
        let red = openpbr_coat_darkening(
            1.0,
            [0.8, 0.1, 0.1],
            0.0,
            [1.0; 3],
            1.5,
            0.3,
            0.0,
            [1.0; 3],
            0.0,
            [0.8; 3],
            1.0,
            1.6,
            1.0,
        );
        assert!(red[0] < 1.0 && red[1] < red[0], "red={red:?}");
    }

    #[test]
    fn f82_tint_hits_f0_at_normal_and_one_at_grazing() {
        let f0 = [0.944, 0.776, 0.373];
        let tint = [0.998, 0.981, 0.751];
        let at_normal = fresnel_f82_tint(1.0, f0, tint);
        let at_grazing = fresnel_f82_tint(0.0, f0, tint);
        for c in 0..3 {
            assert!((at_normal[c] - f0[c]).abs() < 1e-5);
            assert!((at_grazing[c] - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn agx_and_aces_are_monotonic_and_bounded_in_unit_interval() {
        for tm in [tonemap_agx, tonemap_aces] {
            let mut prev = -1.0f32;
            for step in 0..=20 {
                let s = step as f32 * 0.25;
                let v = s * s;
                let out = tm([v, v, v]);
                assert!(
                    out.iter()
                        .all(|&c| c.is_finite() && (0.0..=1.0).contains(&c))
                );
                assert!(out[0] >= prev - 1e-5);
                prev = out[0];
            }
        }
    }
}
