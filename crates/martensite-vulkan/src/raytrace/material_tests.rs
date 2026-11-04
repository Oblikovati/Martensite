use super::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{Graphic3dAspects, Graphic3dNameOfMaterial};

fn plastic_green() -> Handle<Graphic3dAspects> {
    let aspects = Graphic3dAspects::default();
    aspects.set_front_material(Graphic3dMaterialAspect::new(
        Graphic3dNameOfMaterial::Plastified,
    ));
    aspects.set_interior_color(Color::named(NamedColor::GREEN));
    Handle::new(aspects)
}

#[test]
fn aspect_materials_scale_ambient_diffuse_and_emission_by_the_interior_colour() {
    let aspects = plastic_green();
    let material = RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::Linear);
    let source = aspects.front_material();
    let green = Color::named(NamedColor::GREEN).rgb();
    let diffuse = source.diffuse_color().rgb();
    assert_eq!(material.records[DIFFUSE], scaled(diffuse, green, -1.));
    assert_eq!(material.records[SPECULAR][3], 128. * source.shininess());
    assert_eq!(material.records[TRANSPARENCY][2..], [1., 1.]);
    assert_eq!(material.records[TEXTURE_TRANSFORM + 3], [0., 0., 0., 1.]);
    assert_eq!(material.records[BSDF + 1][3], -1.);
}

#[test]
fn reflection_scales_the_specular_colour_to_three_quarters_of_the_brightest_channel() {
    let aspects = plastic_green();
    let material = RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::Linear);
    let [d, s] = [material.records[DIFFUSE], material.records[SPECULAR]];
    let brightest = (0..3).map(|i| d[i] + s[i]).fold(f32::MIN, f32::max);
    let expected = s[0] * (0.75 / brightest);
    assert_eq!(material.records[REFLECTION][0], expected);
}

#[test]
fn gamma_encoded_targets_convert_colours_but_not_the_reflection() {
    let aspects = plastic_green();
    let linear = RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::Linear);
    let encoded = RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::GammaEncoded);
    assert_eq!(
        encoded.records[DIFFUSE],
        ColorRgba::linear_to_srgb_vec4(linear.records[DIFFUSE])
    );
    assert_eq!(encoded.records[REFLECTION], linear.records[REFLECTION]);
}
