//! Graphic3d_PBRMaterial and Graphic3d_BSDF.
use super::common::{Reader, attempt, end, real};
use super::print::Output;
use martensite_tkernel::ncollection::{Vec3, Vec4};
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::graphic3d::{Graphic3dBsdf, Graphic3dFresnel, Graphic3dPbrMaterial};
use martensite_tkservice::image::{ImageColorRgf, ImageFormat, ImagePixMap};

pub fn vec3(output: &Output, v: Vec3<f32>) {
    [v.x, v.y, v.z]
        .into_iter()
        .for_each(|c| real(output, f64::from(c)));
}
pub fn vec4(output: &Output, v: Vec4<f32>) {
    [v.x, v.y, v.z, v.w]
        .into_iter()
        .for_each(|c| real(output, f64::from(c)));
}

/// `sv_pbr_lut SizeX SizeY Samples`.
pub fn lut_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (sx, sy, n) = (
        r.integer() as usize,
        r.integer() as usize,
        r.integer() as u32,
    );
    let lut = ImagePixMap::default();
    lut.init_zero(ImageFormat::Rgf, sx, sy, 0, 0);
    attempt(output, || {
        Graphic3dPbrMaterial::generate_env_lut(&lut, n)?;
        let state = lut.data();
        for y in 0..sy {
            for x in 0..sx {
                let texel = state.data.typed::<ImageColorRgf>(y, x).unwrap_or_default();
                real(output, f64::from(texel.v[0]));
                real(output, f64::from(texel.v[1]));
            }
        }
        Ok(())
    });
    end(output);
}

/// `sv_pbr_factor Probability Roughness`.
pub fn factor_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (p, rough) = (r.real() as f32, r.real() as f32);
    real(
        output,
        f64::from(Graphic3dPbrMaterial::spec_ibl_map_samples_factor(p, rough)),
    );
    real(output, f64::from(Graphic3dPbrMaterial::roughness_of(rough)));
    end(output);
}

pub fn print_bsdf(output: &Output, b: &Graphic3dBsdf) {
    vec4(output, b.kc);
    vec3(output, b.kd);
    vec4(output, b.ks);
    vec3(output, b.kt);
    vec3(output, b.le);
    vec4(output, b.absorption);
    vec4(output, b.fresnel_coat.serialize());
    vec4(output, b.fresnel_base.serialize());
}

/// `sv_pbr_bsdf R G B A Metallic Roughness IOR Er Eg Eb`.
pub fn bsdf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let v: Vec<f32> = (0..10).map(|_| r.real() as f32).collect();
    attempt(output, || {
        let mut pbr = Graphic3dPbrMaterial::default();
        pbr.set_color_rgba(&ColorRgba::new(v[0], v[1], v[2], v[3])?)?;
        pbr.set_metallic(v[4])?;
        pbr.set_roughness(v[5])?;
        pbr.set_ior(v[6])?;
        pbr.set_emission(Vec3::new(v[7], v[8], v[9]))?;
        let b = Graphic3dBsdf::create_metallic_roughness(&pbr);
        print_bsdf(output, &b);
        let mut normalized = b;
        normalized.normalize();
        vec3(output, normalized.kd);
        vec4(output, normalized.ks);
        vec3(output, normalized.kt);
        let mut back = Graphic3dPbrMaterial::default();
        back.set_bsdf(&b)?;
        vec4(output, back.color().to_vec4());
        [back.metallic(), back.normalized_roughness(), back.ior()]
            .into_iter()
            .for_each(|c| real(output, f64::from(c)));
        vec3(output, back.emission());
        Ok(())
    });
    end(output);
}

/// `sv_fresnel Nr Ng Nb Kr Kg Kb`.
pub fn fresnel_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let v: Vec<f32> = (0..6).map(|_| r.real() as f32).collect();
    let n = Vec3::new(v[0], v[1], v[2]);
    let k = Vec3::new(v[3], v[4], v[5]);
    vec4(
        output,
        Graphic3dFresnel::create_conductor_rgb(n, k).serialize(),
    );
    vec4(
        output,
        Graphic3dFresnel::create_conductor(v[0], v[3]).serialize(),
    );
    end(output);
}
