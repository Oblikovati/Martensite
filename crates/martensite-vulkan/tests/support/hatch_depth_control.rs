//! Certified constant depth makes placeholder or disabled depth readback fail visibly.
use crate::hatch_surface::scene::capture_artifacts;
use crate::{
    FrameCapture, PrimitiveDraw, RenderBackend, RenderFrame, RenderVertex, VulkanOptions,
    VulkanRenderer,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::*, graphic3d::*};

#[test]
fn gpu_hatch_masks_real_constant_depth_before_depth_write() {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    let mut renderer = VulkanRenderer::new(options).unwrap();
    capture_artifacts::record_identity(&renderer.driver_identity());
    let mut draws = vec![constant_depth_quad()];
    let unmasked = renderer.render(&depth_frame(draws.clone())).unwrap();
    let half = renderer.stored_depth(0.5);
    assert!(unmasked.depth.iter().all(|value| *value == half));
    let style: Handle<dyn Graphic3dAspectsSource> = Handle::new(Graphic3dAspects::default()).cast();
    style.set_interior_style(AspectInteriorStyle::Hatch);
    style.set_hatch_style_type(AspectHatchStyle::GridWide);
    super::apply_hatch(&mut draws, &style).unwrap();
    let masked = renderer.render(&depth_frame(draws)).unwrap();
    capture_artifacts::record("constant-depth-unmasked", &unmasked);
    capture_artifacts::record("constant-depth-masked", &masked);
    assert_real_mask_depth(&unmasked, &masked, half);
}

fn constant_depth_quad() -> PrimitiveDraw {
    let vertices = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]]
        .into_iter()
        .map(|point| RenderVertex::new(point, [1., 0., 0., 1.]))
        .collect();
    // NDC z=0 maps to window depth one half (stored as the attachment format holds it).
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], crate::identity_clip()).unwrap()
}

fn depth_frame(draws: Vec<PrimitiveDraw>) -> RenderFrame {
    // RenderFrame::new explicitly enables both depth test and depth write.
    RenderFrame::new(65, 67, [0., 0., 0., 1.], draws).unwrap()
}

fn assert_real_mask_depth(unmasked: &FrameCapture, masked: &FrameCapture, half: f32) {
    for index in 0usize..65 * 67 {
        let (x, y) = (index % 65, 66 - index / 65);
        let kept = y.is_multiple_of(4) || x.is_multiple_of(8);
        assert_eq!(
            masked.depth[index],
            if kept { half } else { 1. },
            "pixel={index}"
        );
        let expected = mask_pixel_color(kept);
        assert_eq!(
            &masked.rgba[index * 4..index * 4 + 4],
            &expected,
            "pixel={index}"
        );
        assert_eq!(unmasked.depth[index], half, "unmasked pixel={index}");
    }
}

fn mask_pixel_color(kept: bool) -> [u8; 4] {
    // Both values are exact endpoints in linear and sRGB encodings.
    if kept {
        [255, 0, 0, 255]
    } else {
        [0, 0, 0, 255]
    }
}

#[test]
fn gpu_custom_bitmap_preserves_byte_order_and_releases_the_source_image() {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    let mut renderer = VulkanRenderer::new(options).unwrap();
    capture_artifacts::record_identity(&renderer.driver_identity());
    let style: Handle<dyn Graphic3dAspectsSource> = Handle::new(Graphic3dAspects::default()).cast();
    style.set_interior_style(AspectInteriorStyle::Hatch);
    style.set_hatch_style(asymmetric_bitmap());
    let mut draws = vec![constant_depth_quad()];
    super::apply_hatch(&mut draws, &style).unwrap();
    drop(style);
    let captured = renderer.render(&depth_frame(draws)).unwrap();
    capture_artifacts::record("asymmetric-mask", &captured);
    assert_asymmetric_pixels(&captured, renderer.stored_depth(0.5));
}

fn asymmetric_bitmap() -> Handle<Graphic3dHatchStyle> {
    use martensite_tkservice::image::{ImageFormat, ImagePixMap};
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_zero(ImageFormat::Gray, 32, 32, 0, 0));
    for y in 0usize..32 {
        let x = (y * 7 + 3) % 32;
        image.change_data().data.change_data()[y * 4 + x / 8] = 1 << (7 - x % 8);
    }
    let bitmap = Graphic3dHatchStyle::from_pattern(&image).unwrap();
    image.change_data().data.change_data().fill(0);
    Handle::new(bitmap)
}

fn assert_asymmetric_pixels(capture: &FrameCapture, half: f32) {
    for index in 0usize..65 * 67 {
        let (x, y) = (index % 65, 66 - index / 65);
        let kept = x % 32 == ((y % 32) * 7 + 3) % 32;
        assert_eq!(
            capture.depth[index],
            if kept { half } else { 1. },
            "pixel={index}"
        );
        assert_eq!(
            &capture.rgba[index * 4..index * 4 + 4],
            &mask_pixel_color(kept),
            "pixel={index}"
        );
    }
}
