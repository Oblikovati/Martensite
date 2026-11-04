use super::*;

fn params(scale: f32) -> Graphic3dRenderingParams {
    let mut params = Graphic3dRenderingParams::new().unwrap();
    params.render_resolution_scale = scale;
    params.nb_msaa_samples = 4;
    params
}

fn capture(width: u32, height: u32, depth: Vec<f32>) -> FrameCapture {
    FrameCapture {
        width,
        height,
        rgba: vec![255; (width * height * 4) as usize],
        depth,
    }
}

#[test]
fn scaled_extent_rounds_in_float_like_native_redraw() {
    assert_eq!(scaled_extent([64, 64], 0.75).unwrap(), [48, 48]);
    assert_eq!(scaled_extent([409, 7], 0.5).unwrap(), [205, 4]);
    assert_eq!(scaled_extent([64, 33], 2.).unwrap(), [128, 66]);
    let error = scaled_extent([1, 1], 0.1).unwrap_err().to_string();
    assert!(error.contains("0.1"), "{error}");
}

#[test]
fn persistence_extent_truncates_the_raster_times_the_inverse_scale() {
    assert_eq!(persistence_extent([205, 4], &params(0.5)), [410, 8]);
    assert_eq!(persistence_extent([307, 48], &params(0.75)), [409, 64]);
    assert_eq!(persistence_extent([307, 48], &params(1.)), [307, 48]);
}

#[test]
fn scaled_rendering_scales_resolution_and_drops_msaa_only_when_the_width_changes() {
    let base = params(2.);
    let scaled = scaled_rendering(&base, [64, 64], [128, 128]);
    assert_eq!(scaled.resolution, 2 * base.resolution);
    assert_eq!(scaled.nb_msaa_samples, 0);
    let same_width = scaled_rendering(&params(1.01), [1, 64], [1, 65]);
    assert_eq!(same_width.nb_msaa_samples, 4);
    let unscaled = scaled_rendering(&params(1.), [64, 64], [64, 64]);
    assert_eq!(
        (unscaled.resolution, unscaled.nb_msaa_samples),
        (base.resolution, 4)
    );
    assert_eq!(zoomable_text_scale(&params(2.)), 0.5);
    assert_eq!(zoomable_text_scale(&params(1.)), 1.);
}

#[test]
fn blit_frame_filters_bilinear_only_between_different_sizes() {
    let frame = blit_frame(&capture(2, 2, vec![0.; 4]), [4, 4]).unwrap();
    let style = frame.draws()[0].surface_texture.as_ref().unwrap();
    assert_eq!(style.sampler.filter, Graphic3dTypeOfTextureFilter::Bilinear);
    assert!(!style.sampler.repeat);
    assert_eq!(style.image.extent, [2, 2]);
    assert_eq!(style.uv[0], [1., 0., 0., 1.]);
    assert_eq!(style.uv[2], [0., 0., 0., 1.]);
    let same = blit_frame(&capture(4, 4, vec![0.; 16]), [4, 4]).unwrap();
    let filter = same.draws()[0]
        .surface_texture
        .as_ref()
        .unwrap()
        .sampler
        .filter;
    assert_eq!(filter, Graphic3dTypeOfTextureFilter::Nearest);
}

#[test]
fn blit_texture_is_bottom_up_like_the_native_framebuffer() {
    let mut scaled = capture(1, 2, vec![0.; 2]);
    scaled.rgba = vec![1, 1, 1, 255, 2, 2, 2, 255];
    let frame = blit_frame(&scaled, [2, 4]).unwrap();
    let style = frame.draws()[0].surface_texture.as_ref().unwrap();
    assert_eq!(style.image.bytes.as_slice(), &[2, 2, 2, 255, 1, 1, 1, 255]);
    assert!(!style.image.top_down);
}

#[test]
fn blit_depth_interpolates_between_texel_centres_and_clamps_edges() {
    let scaled = capture(2, 1, vec![0., 1.]);
    assert_eq!(blit_depth(&scaled, [4, 1]), vec![0., 0.25, 0.75, 1.]);
    assert_eq!(
        blit_depth(&capture(4, 1, vec![0., 1., 0.5, 0.5]), [2, 1]),
        vec![0.5, 0.5]
    );
    assert_eq!(blit_depth(&scaled, [2, 1]), vec![0., 1.]);
}
