//! Native shader-record contracts for Phong common & OpenPBR v1.1 GPU materials.
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkservice::graphic3d::{
    Graphic3dMaterialAspect, Graphic3dNameOfMaterial, Graphic3dTypeOfMaterial,
};
use martensite_vulkan::{
    OpenPbrGpuMaterial, RenderMaterial, RenderMaterialColorSpace as Space, RenderMaterialCommon,
};

fn color(rgb: [f32; 3]) -> Color {
    Color::new(rgb[0].into(), rgb[1].into(), rgb[2].into(), ColorSpace::Rgb).unwrap()
}

fn aspect() -> Graphic3dMaterialAspect {
    let mut source = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    source.set_ambient_color(color([0.25, 0.5, 0.75]));
    source.set_diffuse_color(color([0.5, 0.25, 1.]));
    source.set_specular_color(color([0.125, 0.25, 0.5]));
    source.set_emissive_color(color([0.5, 1., 0.25]));
    source.set_material_type(Graphic3dTypeOfMaterial::Aspect);
    source.set_shininess(0.25).unwrap();
    source.set_alpha(0.75).unwrap();
    source
}

#[test]
fn native_defaults_and_mutable_coefficients() {
    let mut common = RenderMaterialCommon::default();
    assert_eq!(common.diffuse, [1.; 4]);
    assert_eq!(common.emission, [1.; 4]);
    assert_eq!(common.ambient, [1.; 4]);
    assert_eq!(common.shine(), 0.);
    *common.change_shine() = 67.;
    assert_eq!(common.shine(), 67.);
    common.ambient[3] = 0.5;
    common.set_color([0.5, 0.25, 1.]);
    assert_eq!(common.diffuse, [0.5, 0.25, 1., 1.]);
    assert_eq!(common.ambient, [0.125, 0.0625, 0.25, 0.5]);
    assert_eq!((common.emission, common.shine()), ([1.; 4], 67.));
    let mut pbr = OpenPbrGpuMaterial::default();
    assert_eq!(pbr.specular_ior(), 1.5);
    assert_eq!(pbr.opacity(), 1.0);
    pbr.base_spec_params[0] = 0.75;
    pbr.base_spec_params[3] = 0.125;
    assert_eq!((pbr.metalness(), pbr.roughness()), (0.75, 0.125));
}

#[test]
fn aspect_tints_ambient_diffuse_and_emission_but_never_specular() {
    let source = aspect();
    let mut material = RenderMaterial::default();
    material.init(
        Space::Linear,
        &source,
        color([0.5, 0.25, 1.]),
        &source,
        Color::default(),
    );
    let common = material.common[0];
    assert_eq!(common.ambient, [0.125, 0.125, 0.75, 1.]);
    assert_eq!(common.diffuse, [0.25, 0.0625, 1., 0.75]);
    assert_eq!(common.emission, [0.25, 0.25, 0.25, 1.]);
    assert_eq!(common.specular_shininess, [0.125, 0.25, 0.5, 32.]);
    assert_eq!(material.openpbr[0].base_color_weight[..3], [0.5, 0.25, 1.]);
    assert_eq!(material.openpbr[0].opacity(), 0.75);
}

#[test]
fn physical_material_ignores_interior_color() {
    let mut source = aspect();
    source.set_material_type(Graphic3dTypeOfMaterial::Physic);
    let mut red = RenderMaterial::default();
    let mut blue = red;
    red.init(
        Space::Linear,
        &source,
        color([1., 0., 0.]),
        &source,
        Color::default(),
    );
    blue.init(
        Space::Linear,
        &source,
        color([0., 0., 1.]),
        &source,
        Color::default(),
    );
    assert_eq!(red, blue);
    assert_eq!(red.common[0].diffuse, [0.5, 0.25, 1., 0.75]);
}

#[test]
fn aliasing_copies_front_instead_of_reinitializing_back() {
    let source = aspect();
    let copy = source.clone();
    let mut material = RenderMaterial::default();
    material.init(
        Space::Linear,
        &source,
        color([1., 0., 0.]),
        &source,
        color([0., 1., 0.]),
    );
    assert_eq!(material.common[0], material.common[1]);
    assert_eq!(material.openpbr[0], material.openpbr[1]);
    material.init(
        Space::Linear,
        &source,
        color([1., 0., 0.]),
        &copy,
        color([0., 1., 0.]),
    );
    assert_ne!(material.common[0], material.common[1]);
    assert_eq!(material.common[1].diffuse, [0., 0.25, 0., 0.75]);
}

#[test]
fn openpbr_init_packs_metallic_roughness_and_ior() {
    let mut source = aspect();
    let mut pbr = *source.openpbr_material();
    pbr.base.metalness = 0.75;
    pbr.specular.roughness = 0.125;
    pbr.specular.ior = 1.65;
    source.set_openpbr_material(pbr).unwrap();
    let mut material = RenderMaterial::default();
    material.init(
        Space::Linear,
        &source,
        Color::default(),
        &source,
        Color::default(),
    );
    assert_eq!(material.openpbr[0].metalness(), 0.75);
    assert_eq!(material.openpbr[0].roughness(), 0.125);
    assert_eq!(material.openpbr[0].specular_ior(), 1.65);
}

#[test]
fn gamma_conversion_preserves_fourth_channels_and_never_converts_pbr() {
    let source = aspect();
    let mut linear = RenderMaterial::default();
    linear.init(
        Space::Linear,
        &source,
        Color::default(),
        &source,
        Color::default(),
    );
    let mut gamma = RenderMaterial::default();
    gamma.init(
        Space::GammaEncoded,
        &source,
        Color::default(),
        &source,
        Color::default(),
    );
    for (encoded, original) in gamma
        .packed_common()
        .into_iter()
        .zip(linear.packed_common())
    {
        assert_eq!(encoded[3], original[3]);
        assert_eq!(
            encoded[..3],
            original[..3]
                .iter()
                .map(|&v| Color::linear_to_srgb_f32(v))
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(gamma.openpbr, linear.openpbr);
}

#[test]
fn highlight_preserves_alpha_and_non_color_properties_on_both_faces() {
    let source = aspect();
    let mut material = RenderMaterial::default();
    material.init(
        Space::Linear,
        &source,
        Color::default(),
        &source,
        Color::default(),
    );
    let previous = material;
    material.set_color([0.25, 0.5, 1.]);
    for face in 0..2 {
        assert_eq!(material.common[face].ambient, [0.0625, 0.125, 0.25, 1.]);
        assert_eq!(material.common[face].diffuse, [0.25, 0.5, 1., 0.75]);
        assert_eq!(
            material.common[face].emission,
            previous.common[face].emission
        );
        assert_eq!(
            material.common[face].specular_shininess,
            previous.common[face].specular_shininess
        );
        assert_eq!(
            material.openpbr[face].base_color_weight[..3],
            [0.25, 0.5, 1.]
        );
        assert_eq!(
            material.openpbr[face].opacity(),
            previous.openpbr[face].opacity()
        );
        assert_eq!(
            material.openpbr[face].base_spec_params,
            previous.openpbr[face].base_spec_params
        );
        assert_eq!(
            material.openpbr[face].emission_color_lum,
            previous.openpbr[face].emission_color_lum
        );
    }
}

#[test]
fn packed_order_matches_shader_abi() {
    let mut material = RenderMaterial::default();
    material.common[1].diffuse = [2.; 4];
    material.openpbr[1].base_spec_params = [0.2, 0.3, 0.8, 0.4];
    assert_eq!(
        RenderMaterial::nb_of_vec4_common(),
        material.packed_common().len()
    );
    assert_eq!(
        RenderMaterial::nb_of_vec4_openpbr(),
        material.packed_openpbr().len()
    );
    assert_eq!(RenderMaterial::nb_of_vec4_openpbr(), 32);
    assert_eq!(material.packed_common()[4], [2.; 4]);
    assert_eq!(material.packed_openpbr()[17], [0.2, 0.3, 0.8, 0.4]);
}

#[test]
fn equality_is_bitwise_including_signed_zero_and_nan_payloads() {
    let mut left = RenderMaterial::default();
    let mut right = left;
    right.common[0].specular_shininess[3] = -0.;
    assert_ne!(left, right);
    left.openpbr[1].base_spec_params[0] = f32::from_bits(0x7fc00001);
    right = left;
    assert!(left.is_equal(&right));
    assert_eq!(left, left);
    right.openpbr[1].base_spec_params[0] = f32::from_bits(0x7fc00002);
    assert_ne!(left, right);
}
