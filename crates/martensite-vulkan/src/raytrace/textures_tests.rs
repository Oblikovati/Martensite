use super::*;
use crate::RenderMaterialColorSpace;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};

fn pixels(red: u8) -> Handle<ImagePixMap> {
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgba, Some(vec![red, 0, 0, 255]), 1, 1, 4));
    image
}

fn texture(red: u8, unit: Graphic3dTextureUnit) -> Handle<dyn Graphic3dTextureMap> {
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(pixels(red)));
    texture.get_params().set_texture_unit(unit);
    texture.cast()
}

fn textured_aspects(textures: &[Handle<dyn Graphic3dTextureMap>]) -> Handle<Graphic3dAspects> {
    let aspects = Handle::new(Graphic3dAspects::default());
    let set = Handle::new(Graphic3dTextureSet::with_size(textures.len() as i32).unwrap());
    for (index, texture) in textures.iter().enumerate() {
        set.set_value(index as i32, texture.clone()).unwrap();
    }
    aspects.set_texture_set(set);
    aspects.set_texture_map_on(true);
    aspects
}

fn sampled(images: &RetainedTextureImages, red: u8) -> RaytraceTexture {
    let texture = texture(red, Graphic3dTextureUnit::BASE_COLOR);
    let image = images.snapshot(&texture).unwrap().unwrap();
    let sampler = images
        .samplers
        .policy(&image, &texture.get_params())
        .unwrap();
    RaytraceTexture { image, sampler }
}

#[test]
fn add_texture_reuses_held_textures_and_refuses_beyond_the_array_size() {
    let images = RetainedTextureImages::default();
    let mut textures = RaytraceTextures::default();
    assert!(!textures.has_textures());
    let first = sampled(&images, 1);
    assert_eq!(textures.add(first.clone()), 0);
    assert_eq!(textures.add(sampled(&images, 2)), 1);
    assert_eq!(textures.add(first), 0);
    for red in 3..=32 {
        assert_eq!(textures.add(sampled(&images, red)), i32::from(red) - 1);
    }
    assert_eq!(textures.add(sampled(&images, 33)), -1);
    assert_eq!(textures.textures.len(), MAX_TEX_NUMBER);
    assert!(textures.has_textures());
}

#[test]
fn the_texture_transform_is_the_column_major_scale_translation_rotation() {
    let params = Graphic3dTextureParams::default();
    let identity = texture_transform(&params);
    assert_eq!(identity[0], [1., 0., 0., 0.]);
    assert_eq!(identity[3], [0., 0., 0., 1.]);
    params.set_scale(Vec2::new(2., 3.));
    params.set_translation(Vec2::new(0.5, -0.25));
    params.set_rotation(90.);
    let columns = texture_transform(&params);
    let angle = -90f32 * (std::f64::consts::PI / 180.0) as f32;
    // Column 0 holds the first matrix column: (sx cos, sy sin, 0, 0).
    assert_eq!(
        columns[0],
        [
            2. * (martensite_cmath::cos(f64::from(angle)) as f32),
            3. * (martensite_cmath::sin(f64::from(angle)) as f32),
            0.,
            0.
        ]
    );
    assert_eq!(
        columns[1],
        [
            -(2. * (martensite_cmath::sin(f64::from(angle)) as f32)),
            3. * (martensite_cmath::cos(f64::from(angle)) as f32),
            0.,
            0.
        ]
    );
    assert_eq!(columns[3], [2. * -0.5, 3. * 0.25, 0., 1.]);
}

#[test]
fn material_textures_index_each_unit_record_and_skip_untextured_aspects() {
    let images = RetainedTextureImages::default();
    let aspects = textured_aspects(&[
        texture(9, Graphic3dTextureUnit::BASE_COLOR),
        texture(8, Graphic3dTextureUnit::METALLIC_ROUGHNESS),
        texture(7, Graphic3dTextureUnit::OCCLUSION),
        texture(6, Graphic3dTextureUnit::NORMAL),
    ]);
    let space = RenderMaterialColorSpace::Linear;
    let mut material = RaytraceMaterial::of_aspects(&*aspects, space);
    let mut textures = RaytraceTextures::default();
    apply_material_textures(&mut material, &*aspects, &images, &mut textures).unwrap();
    assert_eq!(textures.textures.len(), 3);
    let w = |record: usize| material.records[record][3];
    assert_eq!((w(1), w(12)), (0., 0.)); // Diffuse and BSDF Kd
    assert_eq!(w(14), 1.); // BSDF Kt
    assert_eq!(w(15), -1.); // BSDF Le: no emissive map
    assert_eq!(w(17), 2.); // BSDF FresnelBase
    aspects.set_texture_map_on(false);
    let mut plain = RaytraceMaterial::of_aspects(&*aspects, space);
    let mut none = RaytraceTextures::default();
    apply_material_textures(&mut plain, &*aspects, &images, &mut none).unwrap();
    assert!(!none.has_textures());
    assert_eq!(plain.records[1][3], -1.);
}

#[test]
fn units_outside_the_ray_traced_records_are_ignored() {
    assert_eq!(
        TexturedRecord::of_unit(Graphic3dTextureUnit::EMISSIVE),
        Some(TexturedRecord::Emissive)
    );
    assert_eq!(
        TexturedRecord::of_unit(Graphic3dTextureUnit::OCCLUSION),
        None
    );
    assert_eq!(TexturedRecord::of_unit(Graphic3dTextureUnit::Unit5), None);
}
