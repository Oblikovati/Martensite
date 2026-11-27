//! Unit tests for `OpenPbrMaterial`, presets, tone mapping, texture slots, and ray tracing params.
use super::*;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::dump::DumpStream;

#[test]
fn defaults_match_openpbr_v1_1_specification() {
    let m = OpenPbrMaterial::default();
    assert!(m.validate().is_ok());
    assert_eq!(m.base.weight, 1.0);
    assert_eq!(m.base.color, [0.8, 0.8, 0.8]);
    assert_eq!(m.base.metalness, 0.0);
    assert_eq!(m.base.diffuse_roughness, 0.0);
    assert_eq!(m.specular.weight, 1.0);
    assert_eq!(m.specular.color, [1.0, 1.0, 1.0]);
    assert_eq!(m.specular.roughness, 0.3);
    assert_eq!(m.specular.roughness_anisotropy, 0.0);
    assert_eq!(m.specular.ior, 1.5);
    assert_eq!(m.transmission.dispersion_abbe_number, 20.0);
    assert_eq!(m.subsurface.radius, 1.0);
    assert_eq!(m.subsurface.radius_scale, [1.0, 0.5, 0.25]);
    assert_eq!(m.coat.ior, 1.6);
    assert_eq!(m.coat.darkening, 1.0);
    assert_eq!(m.fuzz.roughness, 0.5);
    assert_eq!(m.thin_film.thickness_um, 0.5);
    assert_eq!(m.thin_film.ior, 1.4);
    assert_eq!(m.geometry.opacity, 1.0);
    assert!(!m.geometry.thin_walled);
}

#[test]
fn validation_rejects_out_of_range_and_non_finite_parameters() {
    let mut bad = OpenPbrMaterial::default();
    bad.base.metalness = 1.2;
    let err = bad.validate().unwrap_err();
    assert_eq!(err.exception_type(), "Graphic3d_MaterialDefinitionError");
    assert!(err.message().contains("base.metalness") && err.message().contains("1.2"));

    let mut bad_ior = OpenPbrMaterial::default();
    bad_ior.specular.ior = 0.0;
    assert!(bad_ior.validate().is_err());

    let mut bad_nan = OpenPbrMaterial::default();
    bad_nan.coat.roughness = f32::NAN;
    assert!(bad_nan.validate().is_err());

    let mut bad_abbe = OpenPbrMaterial::default();
    bad_abbe.transmission.dispersion_abbe_number = 0.5;
    assert!(bad_abbe.validate().is_err());

    let mut bad_scatter = OpenPbrMaterial::default();
    bad_scatter.transmission.scatter = [-0.1, 0.0, 0.0];
    assert!(bad_scatter.validate().is_err());
}

#[test]
fn metallic_roughness_constructor_packs_emission_and_opacity() {
    let rgba = ColorRgba::new(0.9, 0.6, 0.2, 0.75).unwrap();
    let mat = OpenPbrMaterial::metallic_roughness(rgba, 1.0, 0.15, 1.5, Vec3::new(4.0, 2.0, 0.0))
        .unwrap();
    assert_eq!(mat.base.metalness, 1.0);
    assert_eq!(mat.specular.roughness, 0.15);
    assert_eq!(mat.geometry.opacity, 0.75);
    assert_eq!(mat.emission.luminance, 4.0);
    assert_eq!(mat.emission.color, [1.0, 0.5, 0.0]);
    assert_eq!(mat.emission_radiance(), [4.0, 2.0, 0.0]);
    assert!(mat.has_emission());
}

#[test]
fn all_predefined_materials_have_valid_physical_openpbr_presets() {
    for (name, _) in super::material_table::MATERIAL_NAMES {
        let aspect = Graphic3dMaterialAspect::new(name);
        let pbr = aspect.openpbr_material();
        assert!(
            pbr.validate().is_ok(),
            "invalid OpenPBR preset for {name:?}"
        );
    }
    let gold = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
    assert_eq!(gold.openpbr_material().base.metalness, 1.0);
    let diamond = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Diamond);
    assert!(diamond.openpbr_material().has_transmission());
    assert!(diamond.openpbr_material().transmission.dispersion_scale > 0.0);
    let jade = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Jade);
    assert!(jade.openpbr_material().has_subsurface());
    let satin = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Satin);
    assert!(satin.openpbr_material().has_fuzz());
    let neon = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Neon);
    assert!(neon.openpbr_material().has_emission());
}

#[test]
fn extended_texture_units_tone_mapping_and_ray_tracing_params() {
    assert_eq!(Graphic3dTextureUnit::SPECULAR_COLOR as i32, 5);
    assert_eq!(Graphic3dTextureUnit::ANISOTROPY_TANGENT as i32, 12);
    assert_eq!(Graphic3dTextureSetBits::THIN_FILM.0, 1 << 11);
    assert_eq!(Graphic3dToneMappingMethod::AgX as i32, 2);
    assert_eq!(Graphic3dToneMappingMethod::Aces as i32, 3);
    let params = Graphic3dRenderingParams::new().unwrap();
    assert_eq!(
        params.ray_tracing.backend,
        Graphic3dRayTracingBackend::AutoPreferHardware
    );
    assert!(params.ray_tracing.denoiser_enabled);
    assert_eq!(params.ray_tracing.denoiser_atrous_iterations, 4);
    let mut stream = DumpStream::default();
    OpenPbrMaterial::default()
        .dump_json(&mut stream, -1)
        .unwrap();
    assert!(stream.text().contains("\"className\": \"OpenPbrMaterial\""));
    assert!(stream.text().contains("\"specularIor\": 1.5"));
    let _ = Color::named(NamedColor::WHITE);
}
