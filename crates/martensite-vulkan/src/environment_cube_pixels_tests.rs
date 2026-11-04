use super::*;
use crate::surface_texture::SurfaceImageEncoding;

fn side(value: u8, top_down: bool) -> Handle<ImagePixMap> {
    let image = Handle::new(ImagePixMap::default());
    let mut texels = vec![value; 2 * 2 * 3];
    texels[0] = 200; // first physical texel marks the memory row order
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(texels), 2, 2, 6));
    image.set_top_down(top_down);
    image
}

fn cube(top_down: bool) -> Handle<dyn Graphic3dCubeMap> {
    let sides: Vec<_> = (0..6).map(|index| side(10 * index, top_down)).collect();
    Handle::new(
        martensite_tkservice::graphic3d::Graphic3dCubeMapSeparate::from_images(&sides).unwrap(),
    )
    .cast()
}

#[test]
fn sides_keep_their_order_memory_rows_and_colour_encoding() {
    let pixels = cube_map_pixels(cube(true).get().unwrap()).unwrap();
    assert_eq!(pixels.size, 2);
    assert_eq!(pixels.encoding, SurfaceImageEncoding::Srgb8);
    assert!(pixels.top_down);
    assert_eq!(pixels.faces[3].len(), 16);
    assert_eq!(&pixels.faces[3][..8], &[200, 30, 30, 255, 30, 30, 30, 255]);
}

#[test]
fn bottom_up_sides_upload_their_memory_rows_unchanged() {
    let pixels = cube_map_pixels(cube(false).get().unwrap()).unwrap();
    assert!(!pixels.top_down);
    assert_eq!(
        pixels.faces[0][0], 200,
        "physical row 0 first, as glTexImage2D reads it"
    );
    assert_eq!(pixels.coefficients(), [-1, 1]);
}

#[test]
fn snapshots_are_reused_until_the_revision_changes() {
    let cube = cube(true);
    let mut cache = Vec::new();
    let first = cached_cube_map_pixels(&mut cache, &cube).unwrap();
    let again = cached_cube_map_pixels(&mut cache, &cube).unwrap();
    assert!(Arc::ptr_eq(&first, &again));
    cube.get().unwrap().update_revision();
    let changed = cached_cube_map_pixels(&mut cache, &cube).unwrap();
    assert!(!Arc::ptr_eq(&first, &changed));
    assert_eq!(cache.len(), 1);
}

#[test]
fn a_null_cube_map_is_rejected() {
    let mut cache = Vec::new();
    assert!(cached_cube_map_pixels(&mut cache, &Handle::default()).is_err());
}

fn packed_dds(name: &str) -> Handle<dyn Graphic3dCubeMap> {
    let path = format!(
        "{}/tests/fixtures/dds_cubemap/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let order = martensite_tkservice::graphic3d::Graphic3dCubeMapOrder::default_order();
    let path = martensite_tkernel::tcollection::AsciiString::from(path.as_str());
    let cube = martensite_tkservice::graphic3d::Graphic3dCubeMapPacked::from_file(&path, &order);
    Handle::new(cube).cast()
}

#[test]
fn an_incomplete_compressed_mip_set_samples_opaque_black() {
    // cube.dds: 16x16 BC1 sides with 3 of 5 levels (the native GL texture is incomplete).
    let pixels = cube_map_pixels(packed_dds("cube.dds").get().unwrap()).unwrap();
    assert_eq!(
        (pixels.size, pixels.encoding),
        (1, SurfaceImageEncoding::Linear8)
    );
    assert!(pixels.levels.is_empty());
    assert!(pixels.faces.iter().all(|face| face == &[0, 0, 0, 255]));
}

#[test]
fn a_complete_compressed_mip_set_keeps_every_supplied_level() {
    let pixels = cube_map_pixels(packed_dds("cube-complete.dds").get().unwrap()).unwrap();
    assert_eq!((pixels.size, pixels.levels.len()), (16, 5));
}
