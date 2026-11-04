//! Expand packed source channels without normalizing HDR floats or discarding native row orientation.
use crate::{RenderError, error::rejected, surface_texture::SurfaceImageEncoding};
use martensite_tkservice::image::{
    ImageFormat, ImagePixMap, image_convert_from_half_float, image_convert_to_half_float,
};
pub(crate) fn snapshot_surface_pixels(
    image: &ImagePixMap,
    color_map: bool,
) -> Result<(SurfaceImageEncoding, Vec<u8>), RenderError> {
    snapshot_slices(image, color_map, 1)
}
/// Every slice of a 3D image in memory order, as `glTexImage3D` uploads `ImagePixMap::Data`
/// (GlTexture::Init of a Graphic3dTypeOfTexture_3D map);
/// `snapshot_volume_pixels(&image, true)?`.
pub(crate) fn snapshot_volume_pixels(
    image: &ImagePixMap,
    color_map: bool,
) -> Result<(SurfaceImageEncoding, Vec<u8>), RenderError> {
    snapshot_slices(image, color_map, image.depth())
}
fn snapshot_slices(
    image: &ImagePixMap,
    color_map: bool,
    slices: usize,
) -> Result<(SurfaceImageEncoding, Vec<u8>), RenderError> {
    let state = image.data();
    let encoding = image_encoding(state.format, color_map)?;
    let data = &state.data;
    let mut bytes = Vec::new();
    for (slice, y, x) in memory_order(data.size_x, data.size_y, slices) {
        // Logical rows count from the top; memory row 0 of a bottom-up image is the last one.
        let row = if data.top_to_down == 1 {
            y
        } else {
            data.size_y - 1 - y
        };
        let raw = data
            .value_xyz(x, row, slice)
            .ok_or_else(|| rejected((slice, row, x), "allocated image pixel"))?;
        encode_pixel(&mut bytes, read_texture_pixel(raw, state.format)?, encoding);
    }
    Ok((encoding, bytes))
}
/// (slice, memory row, column) of every pixel in upload order.
fn memory_order(
    width: usize,
    height: usize,
    slices: usize,
) -> impl Iterator<Item = (usize, usize, usize)> {
    (0..slices).flat_map(move |slice| {
        (0..height).flat_map(move |y| (0..width).map(move |x| (slice, y, x)))
    })
}
fn image_encoding(format: ImageFormat, color: bool) -> Result<SurfaceImageEncoding, RenderError> {
    use ImageFormat::*;
    use SurfaceImageEncoding::*;
    Ok(match format {
        Rgb | Bgr | Rgb32 | Bgr32 | Rgba | Bgra if color => Srgb8,
        Gray | Alpha | Rgb | Bgr | Rgb32 | Bgr32 | Rgba | Bgra | GrayF | AlphaF => Linear8,
        Gray16 => Linear16,
        GrayFHalf | RgfHalf | RgbafHalf => Float16,
        Rgf | Rgbf | Bgrf | Rgbaf | Bgraf => Float32,
        _ => return Err(rejected(format, "native uncompressed 2D image format")),
    })
}
fn read_texture_pixel(raw: &[u8], format: ImageFormat) -> Result<[f32; 4], RenderError> {
    let size = component_size(format);
    let mut components = [0., 0., 0., 1.];
    for (i, component) in raw.chunks_exact(size).take(4).enumerate() {
        components[i] = decode_component(component, size, format);
    }
    if components.iter().any(|v| !v.is_finite()) {
        return Err(rejected(
            components,
            "finite surface image components (HDR permitted)",
        ));
    }
    Ok(channel_mapping(components, format))
}
fn component_size(format: ImageFormat) -> usize {
    use ImageFormat::*;
    match format {
        Gray16 | GrayFHalf | RgfHalf | RgbafHalf => 2,
        GrayF | AlphaF | Rgf | Rgbf | Bgrf | Rgbaf | Bgraf => 4,
        _ => 1,
    }
}
fn decode_component(bytes: &[u8], size: usize, format: ImageFormat) -> f32 {
    match size {
        1 => f32::from(bytes[0]) / 255.,
        2 if format == ImageFormat::Gray16 => {
            f32::from(u16::from_ne_bytes(bytes.try_into().unwrap())) / 65535.
        }
        2 => image_convert_from_half_float(u16::from_ne_bytes(bytes.try_into().unwrap())),
        _ => f32::from_ne_bytes(bytes.try_into().unwrap()),
    }
}
fn channel_mapping(mut rgba: [f32; 4], format: ImageFormat) -> [f32; 4] {
    use ImageFormat::*;
    if matches!(format, Bgr | Bgr32 | Bgra | Bgrf | Bgraf) {
        rgba.swap(0, 2);
    }
    if matches!(format, Rgb32 | Bgr32) {
        rgba[3] = 1.;
    }
    match format {
        Gray | GrayF | Gray16 => [rgba[0], rgba[0], rgba[0], 1.],
        Alpha | AlphaF => [0., 0., 0., rgba[0]],
        _ => rgba,
    }
}
fn encode_pixel(bytes: &mut Vec<u8>, rgba: [f32; 4], encoding: SurfaceImageEncoding) {
    for value in rgba {
        match encoding {
            SurfaceImageEncoding::Srgb8 | SurfaceImageEncoding::Linear8 => {
                bytes.push((value.clamp(0., 1.) * 255.).round() as u8)
            }
            SurfaceImageEncoding::Linear16 => {
                bytes.extend(((value.clamp(0., 1.) * 65535.).round() as u16).to_ne_bytes())
            }
            SurfaceImageEncoding::Float16 => {
                bytes.extend(image_convert_to_half_float(value).to_ne_bytes())
            }
            _ => bytes.extend(value.to_ne_bytes()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packed_bgr_padding_and_alpha_channels_follow_native_formats() {
        assert_eq!(
            read_texture_pixel(&[0, 127, 255, 17], ImageFormat::Bgr32).unwrap(),
            [1., 127. / 255., 0., 1.]
        );
        assert_eq!(
            read_texture_pixel(&[64], ImageFormat::Alpha).unwrap(),
            [0., 0., 0., 64. / 255.]
        );
        assert_eq!(
            image_encoding(ImageFormat::Gray, true).unwrap(),
            SurfaceImageEncoding::Linear8
        );
        assert_eq!(
            image_encoding(ImageFormat::Rgb, true).unwrap(),
            SurfaceImageEncoding::Srgb8
        );
    }
    #[test]
    fn floating_images_keep_hdr_and_gray_half_is_red_only() {
        let bytes = [2.5f32, -0.25, 0.5, 0.75]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        let rgba = read_texture_pixel(&bytes, ImageFormat::Rgbaf).unwrap();
        assert_eq!(rgba, [2.5, -0.25, 0.5, 0.75]);
        assert_eq!(
            read_texture_pixel(&0x3c00u16.to_ne_bytes(), ImageFormat::GrayFHalf).unwrap(),
            [1., 0., 0., 1.]
        );
        let mut encoded = Vec::new();
        encode_pixel(&mut encoded, rgba, SurfaceImageEncoding::Float32);
        assert_eq!(encoded, bytes);
    }
    #[test]
    fn physical_rows_are_preserved_with_the_source_orientation_flag() {
        let image = ImagePixMap::default();
        assert!(image.init_wrapper(
            ImageFormat::Rgba,
            Some(vec![1, 2, 3, 255, 4, 5, 6, 255]),
            1,
            2,
            4
        ));
        for top_down in [false, true] {
            image.set_top_down(top_down);
            assert_eq!(
                snapshot_surface_pixels(&image, false).unwrap().1,
                vec![1, 2, 3, 255, 4, 5, 6, 255]
            );
        }
    }
    #[test]
    fn volume_slices_upload_in_memory_order_for_both_orientations() {
        use martensite_tkernel::ncollection::Vec3;
        let image = ImagePixMap::default();
        assert!(image.init_trash_3d(ImageFormat::Rgba, Vec3::new(1, 2, 2), 0));
        let memory: Vec<u8> = (0..4).flat_map(|texel| [texel, 0, 0, 255]).collect();
        image
            .change_data()
            .data
            .change_data()
            .copy_from_slice(&memory);
        for top_down in [false, true] {
            image.set_top_down(top_down);
            assert_eq!(snapshot_volume_pixels(&image, false).unwrap().1, memory);
            assert_eq!(
                snapshot_surface_pixels(&image, false).unwrap().1,
                memory[..8]
            );
        }
    }
    #[test]
    fn compatibility_float_luminance_quantizes_but_rg_float_does_not() {
        let value = read_texture_pixel(&0.5f32.to_ne_bytes(), ImageFormat::GrayF).unwrap();
        let mut encoded = Vec::new();
        encode_pixel(
            &mut encoded,
            value,
            image_encoding(ImageFormat::GrayF, true).unwrap(),
        );
        assert_eq!(encoded, vec![128, 128, 128, 255]);
        assert_eq!(
            image_encoding(ImageFormat::Rgf, true).unwrap(),
            SurfaceImageEncoding::Float32
        );
        assert!(read_texture_pixel(&f32::NAN.to_ne_bytes(), ImageFormat::GrayF).is_err());
    }
}
