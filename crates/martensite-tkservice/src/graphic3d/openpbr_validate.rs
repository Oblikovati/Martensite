//! Validation and JSON dump helpers for [`OpenPbrMaterial`].
use super::{OpenPbrMaterial, material_definition_error};
use crate::json_dump::{class_begin, field_dumped, field_integer, field_real};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::dump::{DumpStream, dump_vector};
use std::io;

pub(super) fn check_unit(value: f32, name: &str) -> Result<(), Failure> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(material_definition_error(&format!(
            "OpenPBR '{name}' must be finite in [0.0, 1.0]; got {value}"
        )));
    }
    Ok(())
}

fn check_signed_unit(value: f32, name: &str) -> Result<(), Failure> {
    if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
        return Err(material_definition_error(&format!(
            "OpenPBR '{name}' must be finite in [-1.0, 1.0]; got {value}"
        )));
    }
    Ok(())
}

fn check_min(value: f32, min: f32, name: &str) -> Result<(), Failure> {
    if !value.is_finite() || value < min {
        return Err(material_definition_error(&format!(
            "OpenPBR '{name}' must be finite and >= {min}; got {value}"
        )));
    }
    Ok(())
}

fn check_unit_rgb(rgb: [f32; 3], name: &str) -> Result<(), Failure> {
    for (channel, value) in rgb.into_iter().enumerate() {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(material_definition_error(&format!(
                "OpenPBR '{name}[{channel}]' must be finite in [0.0, 1.0]; got {value}"
            )));
        }
    }
    Ok(())
}

pub(super) fn check_nonneg_rgb(rgb: [f32; 3], name: &str) -> Result<(), Failure> {
    for (channel, value) in rgb.into_iter().enumerate() {
        if !value.is_finite() || value < 0.0 {
            return Err(material_definition_error(&format!(
                "OpenPBR '{name}[{channel}]' must be finite and >= 0.0; got {value}"
            )));
        }
    }
    Ok(())
}

pub(super) fn validate_material(m: &OpenPbrMaterial) -> Result<(), Failure> {
    validate_base_and_specular(m)?;
    validate_transmission_and_sss(m)?;
    validate_coat_fuzz_film_emission(m)
}

fn validate_base_and_specular(m: &OpenPbrMaterial) -> Result<(), Failure> {
    check_unit(m.base.weight, "base.weight")?;
    check_unit_rgb(m.base.color, "base.color")?;
    check_unit(m.base.metalness, "base.metalness")?;
    check_unit(m.base.diffuse_roughness, "base.diffuse_roughness")?;
    check_min(m.specular.weight, 0.0, "specular.weight")?;
    check_unit_rgb(m.specular.color, "specular.color")?;
    check_unit(m.specular.roughness, "specular.roughness")?;
    check_unit(
        m.specular.roughness_anisotropy,
        "specular.roughness_anisotropy",
    )?;
    check_min(m.specular.ior, OpenPbrMaterial::MIN_IOR, "specular.ior")
}

fn validate_transmission_and_sss(m: &OpenPbrMaterial) -> Result<(), Failure> {
    check_unit(m.transmission.weight, "transmission.weight")?;
    check_unit_rgb(m.transmission.color, "transmission.color")?;
    check_min(m.transmission.depth, 0.0, "transmission.depth")?;
    check_nonneg_rgb(m.transmission.scatter, "transmission.scatter")?;
    check_signed_unit(
        m.transmission.scatter_anisotropy,
        "transmission.scatter_anisotropy",
    )?;
    check_unit(
        m.transmission.dispersion_scale,
        "transmission.dispersion_scale",
    )?;
    check_min(
        m.transmission.dispersion_abbe_number,
        OpenPbrMaterial::MIN_ABBE_NUMBER,
        "transmission.dispersion_abbe_number",
    )?;
    check_unit(m.subsurface.weight, "subsurface.weight")?;
    check_unit_rgb(m.subsurface.color, "subsurface.color")?;
    check_min(m.subsurface.radius, 0.0, "subsurface.radius")?;
    check_unit_rgb(m.subsurface.radius_scale, "subsurface.radius_scale")?;
    check_signed_unit(
        m.subsurface.scatter_anisotropy,
        "subsurface.scatter_anisotropy",
    )
}

fn validate_coat_fuzz_film_emission(m: &OpenPbrMaterial) -> Result<(), Failure> {
    check_unit(m.coat.weight, "coat.weight")?;
    check_unit_rgb(m.coat.color, "coat.color")?;
    check_unit(m.coat.roughness, "coat.roughness")?;
    check_unit(m.coat.roughness_anisotropy, "coat.roughness_anisotropy")?;
    check_min(m.coat.ior, OpenPbrMaterial::MIN_IOR, "coat.ior")?;
    check_unit(m.coat.darkening, "coat.darkening")?;
    check_unit(m.fuzz.weight, "fuzz.weight")?;
    check_unit_rgb(m.fuzz.color, "fuzz.color")?;
    check_unit(m.fuzz.roughness, "fuzz.roughness")?;
    check_unit(m.thin_film.weight, "thin_film.weight")?;
    check_min(m.thin_film.thickness_um, 0.0, "thin_film.thickness_um")?;
    check_min(m.thin_film.ior, OpenPbrMaterial::MIN_IOR, "thin_film.ior")?;
    check_min(m.emission.luminance, 0.0, "emission.luminance")?;
    check_nonneg_rgb(m.emission.color, "emission.color")?;
    check_unit(m.geometry.opacity, "geometry.opacity")
}

fn dump_rgb(stream: &mut DumpStream, key: &str, rgb: [f32; 3]) -> io::Result<()> {
    field_dumped(stream, -1, key, &|out, _| {
        dump_vector(
            out,
            "Vec3",
            &[f64::from(rgb[0]), f64::from(rgb[1]), f64::from(rgb[2])],
        )
    })
}

pub(super) fn dump_openpbr_json(m: &OpenPbrMaterial, stream: &mut DumpStream) -> io::Result<()> {
    class_begin(stream, "OpenPbrMaterial")?;
    field_real(stream, "baseWeight", f64::from(m.base.weight))?;
    dump_rgb(stream, "baseColor", m.base.color)?;
    field_real(stream, "baseMetalness", f64::from(m.base.metalness))?;
    field_real(
        stream,
        "baseDiffuseRoughness",
        f64::from(m.base.diffuse_roughness),
    )?;
    field_real(stream, "specularWeight", f64::from(m.specular.weight))?;
    dump_rgb(stream, "specularColor", m.specular.color)?;
    field_real(stream, "specularRoughness", f64::from(m.specular.roughness))?;
    field_real(
        stream,
        "specularRoughnessAnisotropy",
        f64::from(m.specular.roughness_anisotropy),
    )?;
    field_real(stream, "specularIor", f64::from(m.specular.ior))?;
    dump_transmission_and_sss(m, stream)?;
    dump_coat_fuzz_film_emission(m, stream)
}

fn dump_transmission_and_sss(m: &OpenPbrMaterial, stream: &mut DumpStream) -> io::Result<()> {
    field_real(
        stream,
        "transmissionWeight",
        f64::from(m.transmission.weight),
    )?;
    dump_rgb(stream, "transmissionColor", m.transmission.color)?;
    field_real(stream, "transmissionDepth", f64::from(m.transmission.depth))?;
    dump_rgb(stream, "transmissionScatter", m.transmission.scatter)?;
    field_real(
        stream,
        "transmissionScatterAnisotropy",
        f64::from(m.transmission.scatter_anisotropy),
    )?;
    field_real(
        stream,
        "transmissionDispersionScale",
        f64::from(m.transmission.dispersion_scale),
    )?;
    field_real(
        stream,
        "transmissionDispersionAbbeNumber",
        f64::from(m.transmission.dispersion_abbe_number),
    )?;
    field_real(stream, "subsurfaceWeight", f64::from(m.subsurface.weight))?;
    dump_rgb(stream, "subsurfaceColor", m.subsurface.color)?;
    field_real(stream, "subsurfaceRadius", f64::from(m.subsurface.radius))?;
    dump_rgb(stream, "subsurfaceRadiusScale", m.subsurface.radius_scale)?;
    field_real(
        stream,
        "subsurfaceScatterAnisotropy",
        f64::from(m.subsurface.scatter_anisotropy),
    )
}

fn dump_coat_fuzz_film_emission(m: &OpenPbrMaterial, stream: &mut DumpStream) -> io::Result<()> {
    field_real(stream, "coatWeight", f64::from(m.coat.weight))?;
    dump_rgb(stream, "coatColor", m.coat.color)?;
    field_real(stream, "coatRoughness", f64::from(m.coat.roughness))?;
    field_real(
        stream,
        "coatRoughnessAnisotropy",
        f64::from(m.coat.roughness_anisotropy),
    )?;
    field_real(stream, "coatIor", f64::from(m.coat.ior))?;
    field_real(stream, "coatDarkening", f64::from(m.coat.darkening))?;
    field_real(stream, "fuzzWeight", f64::from(m.fuzz.weight))?;
    dump_rgb(stream, "fuzzColor", m.fuzz.color)?;
    field_real(stream, "fuzzRoughness", f64::from(m.fuzz.roughness))?;
    field_real(stream, "thinFilmWeight", f64::from(m.thin_film.weight))?;
    field_real(
        stream,
        "thinFilmThicknessUm",
        f64::from(m.thin_film.thickness_um),
    )?;
    field_real(stream, "thinFilmIor", f64::from(m.thin_film.ior))?;
    field_real(stream, "emissionLuminance", f64::from(m.emission.luminance))?;
    dump_rgb(stream, "emissionColor", m.emission.color)?;
    field_real(stream, "geometryOpacity", f64::from(m.geometry.opacity))?;
    field_integer(
        stream,
        "geometryThinWalled",
        i32::from(m.geometry.thin_walled),
    )
}
