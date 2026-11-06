//! Freeze a Graphic3dCubeMap's six sides for upload, as GlTexture::InitCubeMap reads them:
//! `Reset`, then one `Value` per side with `Next` between, each side uploaded in its memory
//! row order with the first side's size and format.
use crate::environment_frame::CubeMapPixels;
use crate::{RenderError, error::rejected};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dCubeMap, Graphic3dCubeMapMembers as _, Graphic3dTextureRootMembers as _,
};
use martensite_tkservice::image::{ImageFormat, ImagePixMap, ImageSupportedFormats};
use std::sync::Arc;

/// Snapshot one cube map; `cube_map_pixels(&cube)?`. A compressed (DDS) first side selects the
/// compressed upload, as InitCubeMap does when the context lists its S3TC format; sides that
/// cannot be read are rejected with the offending side.
pub(crate) fn cube_map_pixels(cube: &dyn Graphic3dCubeMap) -> Result<CubeMapPixels, RenderError> {
    cube.reset();
    let compressed = crate::retained_texture_images::surface_supported_formats();
    if cube.compressed_value(&compressed).get().is_some() {
        return compressed_cube_map_pixels(cube, &compressed);
    }
    let supported = uncompressed_formats();
    cube.reset();
    let mut faces: [Vec<u8>; 6] = Default::default();
    let mut layout = None;
    let mut encoding = None;
    for (side, face) in faces.iter_mut().enumerate() {
        let image = cube.value(&supported);
        let image = image
            .get()
            .ok_or_else(|| rejected(side, "a readable cube map side"))?;
        let shape = side_layout(image)?;
        if *layout.get_or_insert(shape) != shape {
            return Err(rejected(
                (side, shape),
                "cube map sides of the first side's layout",
            ));
        }
        let (side_encoding, bytes) =
            crate::surface_texture_pixels::snapshot_surface_pixels(image, cube.is_color_map())?;
        encoding.get_or_insert(side_encoding);
        *face = bytes;
        cube.next();
    }
    let (size, _) = layout.expect("six cube map sides");
    let encoding = encoding.expect("six cube map sides");
    Ok(CubeMapPixels {
        id: cube.get_id().as_bytes().to_vec(),
        revision: cube.revision(),
        size,
        encoding,
        faces,
        levels: Vec::new(),
        top_down: cube.is_top_down(),
        z_inverted: cube.z_is_inverted(),
    })
}

/// InitCubeMap's compressed branch: `Reset()`, then one `CompressedValue()` per side with
/// `Next()` between; every side keeps the first side's extent, format and supplied mip lengths
/// (glCompressedTexImage2D of each level, no generated mipmaps).
fn compressed_cube_map_pixels(
    cube: &dyn Graphic3dCubeMap,
    supported: &Handle<ImageSupportedFormats>,
) -> Result<CubeMapPixels, RenderError> {
    cube.reset();
    let mut faces: [Vec<u8>; 6] = Default::default();
    let mut shape = None;
    let mut complete = None;
    for (side, face) in faces.iter_mut().enumerate() {
        let image = cube.compressed_value(supported);
        let image = image
            .get()
            .ok_or_else(|| rejected(side, "a readable compressed cube map side"))?;
        complete.get_or_insert(image.is_complete_mip_map_set());
        let source = crate::surface_texture_compressed::read_compressed_surface(image, cube)?;
        let side_shape = (source.extent, source.encoding, source.levels);
        if *shape.get_or_insert_with(|| side_shape.clone()) != side_shape {
            return Err(rejected(
                (side, side_shape),
                "compressed cube map sides of the first side's size, format and mip lengths",
            ));
        }
        *face = source.bytes;
        cube.next();
    }
    let (extent, encoding, levels) = shape.expect("six cube map sides");
    if extent[0] != extent[1] {
        return Err(rejected(extent, "a square compressed cube map side"));
    }
    if levels.len() > 1 && complete == Some(false) {
        return Ok(incomplete_cube_map(cube));
    }
    Ok(CubeMapPixels {
        id: cube.get_id().as_bytes().to_vec(),
        revision: cube.revision(),
        size: extent[0],
        encoding,
        faces,
        levels,
        top_down: cube.is_top_down(),
        z_inverted: cube.z_is_inverted(),
    })
}

/// A compressed cube map whose supplied mip set stops above 1x1, as GL samples it natively:
/// InitCubeMap keeps those levels and selects a mipmapped minification filter, but
/// GlSampler::applySamplerParams returns for GL_TEXTURE_CUBE_MAP before it limits
/// GL_TEXTURE_MAX_LEVEL, so the texture is incomplete and every lookup returns (0, 0, 0, 1).
/// One opaque black texel per side samples the same everywhere.
fn incomplete_cube_map(cube: &dyn Graphic3dCubeMap) -> CubeMapPixels {
    CubeMapPixels {
        id: cube.get_id().as_bytes().to_vec(),
        revision: cube.revision(),
        size: 1,
        encoding: crate::surface_texture::SurfaceImageEncoding::Linear8,
        faces: std::array::from_fn(|_| vec![0, 0, 0, 255]),
        levels: Vec::new(),
        top_down: cube.is_top_down(),
        z_inverted: cube.z_is_inverted(),
    }
}

/// The cached snapshot of `cube` when its id and revision still match, else a fresh one.
/// Example: `let pixels = cached_cube_map_pixels(&mut cache, &cube)?`.
pub(crate) fn cached_cube_map_pixels(
    cache: &mut Vec<Arc<CubeMapPixels>>,
    cube: &Handle<dyn Graphic3dCubeMap>,
) -> Result<Arc<CubeMapPixels>, RenderError> {
    let map = cube
        .get()
        .ok_or_else(|| rejected("null cube map", "a cube map"))?;
    let id = map.get_id().as_bytes().to_vec();
    let revision = map.revision();
    if let Some(pixels) = cache
        .iter()
        .find(|pixels| !id.is_empty() && pixels.id == id && pixels.revision == revision)
    {
        return Ok(pixels.clone());
    }
    let pixels = Arc::new(cube_map_pixels(map)?);
    cache.retain(|cached| cached.id != pixels.id);
    cache.push(pixels.clone());
    Ok(pixels)
}

fn side_layout(image: &ImagePixMap) -> Result<(u32, ImageFormat), RenderError> {
    let (width, height) = (image.width(), image.height());
    if width == 0 || width != height {
        return Err(rejected(
            (width, height),
            "a square non-empty cube map side",
        ));
    }
    let size = u32::try_from(width).map_err(|e| rejected(e, "a 32bit cube map side"))?;
    Ok((size, image.format()))
}

/// GlContext::SupportedTextureFormats without compressed formats.
fn uncompressed_formats() -> Handle<ImageSupportedFormats> {
    let formats = Handle::new(ImageSupportedFormats::default());
    for kind in [ImageFormat::Rgb, ImageFormat::Rgba] {
        formats.add(kind);
    }
    formats
}

#[cfg(test)]
#[path = "environment_cube_pixels_tests.rs"]
mod tests;
