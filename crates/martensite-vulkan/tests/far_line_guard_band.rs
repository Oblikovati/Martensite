//! Lines far beyond the viewport draw their visible part on every driver (bug129_1, bug218).
//!
//! lavapipe hands unclipped lines to llvmpipe, which drops a line whose window span reaches
//! 2^15 px and fills stray pixels for some longer ones (a 36,000 px line lit 384 pixels of a
//! 64 x 64 frame). The renderer clips such lines to a guard band; the expected image is the
//! analytic one: the column x = 10 lit on every row, nothing else.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_vulkan::*;

/// A vertical line at window x = 10.5 from window y = `low` to `high` in a 64 px frame.
fn vertical_line(low: f32, high: f32) -> PrimitiveDraw {
    let ndc = |px: f32| px / 32. - 1.;
    let vertices = [low, high]
        .map(|y| RenderVertex::new([ndc(10.5), ndc(y), 0.5], [1., 1., 0., 1.]))
        .to_vec();
    let style = LineStyle::new(1., 0xffff, 1).unwrap();
    PrimitiveDraw::segments(vertices, vec![0, 1], identity_clip(), style).unwrap()
}

fn lit_pixels(renderer: &mut VulkanRenderer, draw: PrimitiveDraw) -> Vec<(usize, usize)> {
    let frame = RenderFrame::new(64, 64, [0., 0., 0., 1.], vec![draw]).unwrap();
    let capture = renderer.render(&frame).unwrap();
    let pixels = capture.rgba.as_chunks::<4>().0.iter().enumerate();
    pixels
        .filter(|(_, p)| p[0] > 0)
        .map(|(i, _)| (i % 64, i / 64))
        .collect()
}

#[test]
fn far_lines_draw_exactly_their_visible_column() {
    let mut renderer = VulkanRenderer::new(device_options::validation_options()).unwrap();
    let column: Vec<(usize, usize)> = (0..64).map(|row| (10, row)).collect();
    for half in [100., 16_400., 18_000., 500_000., 1.0e8] {
        let lit = lit_pixels(&mut renderer, vertical_line(32. - half, 32. + half));
        assert_eq!(lit, column, "line spanning {} px", 2. * half);
    }
    let off_screen = lit_pixels(&mut renderer, vertical_line(1.0e6, 2.0e6));
    assert!(off_screen.is_empty(), "off-screen line lit {off_screen:?}");
    assert_eq!(renderer.validation_errors().unwrap(), Vec::<String>::new());
}

/// Window ends of the far line that owned A1's differing pixels, as clip positions (409 px).
fn far_diagonal_ends() -> [[f32; 4]; 2] {
    let ndc = |px: f32| px / 204.5 - 1.;
    [
        [ndc(-8_363_180.8), ndc(4_828_826.1), 0.5, 1.],
        [ndc(8_363_526.), ndc(-4_828_343.7), 0.5, 1.],
    ]
}

fn line_through(ends: [[f32; 4]; 2]) -> PrimitiveDraw {
    let vertices = ends.map(|end| RenderVertex {
        position: [end[0], end[1], end[2]],
        weight: end[3],
        color: [1., 1., 0., 1.],
    });
    let style = LineStyle::new(1., 0xffff, 1).unwrap();
    PrimitiveDraw::segments(vertices.to_vec(), vec![0, 1], identity_clip(), style).unwrap()
}

/// `do_clip_line` of Mesa's `draw_pipe_clip.c` in binary32, against the four viewport planes.
fn driver_clipped_ends([a, b]: [[f32; 4]; 2]) -> [[f32; 4]; 2] {
    let planes = |c: [f32; 4]| [c[0] + c[3], c[3] - c[0], c[1] + c[3], c[3] - c[1]];
    let (mut low, mut high) = (0f32, 0f32);
    for (d0, d1) in planes(a).into_iter().zip(planes(b)) {
        if d1 < 0. {
            high = high.max(d1 / (d1 - d0));
        }
        if d0 < 0. {
            low = low.max(d0 / (d0 - d1));
        }
    }
    let toward = |out: [f32; 4], inside: [f32; 4], t: f32| -> [f32; 4] {
        std::array::from_fn(|i| out[i] + t * (inside[i] - out[i]))
    };
    [toward(a, b, low), toward(b, a, high)]
}

fn render_409(renderer: &mut VulkanRenderer, draw: PrimitiveDraw) -> Vec<u8> {
    let frame = RenderFrame::new(409, 409, [0., 0., 0., 1.], vec![draw]).unwrap();
    renderer.render(&frame).unwrap().rgba
}

fn selected_renderer(behavior: RenderLineClippingBehavior) -> VulkanRenderer {
    VulkanRenderer::new_with_line_clipping(
        device_options::validation_options(),
        Default::default(),
        Default::default(),
        behavior,
    )
    .unwrap()
}

/// The clipped ends must come from the source's clip coordinates; the guard band's prior
/// object-space rounding moved a line spanning 16.7 million pixels off its native slope.
#[test]
fn version_three_lets_the_driver_clip_the_source_segment() {
    let ends = far_diagonal_ends();
    let mut renderer = selected_renderer(RenderLineClippingBehavior::PlatformNativeV3);
    if !renderer.driver_identity().device_name.contains("llvmpipe") {
        return; // Other drivers keep the guard band; this contract is the software GL stage.
    }
    let expected = render_409(&mut renderer, line_through(driver_clipped_ends(ends)));
    assert!(expected.contains(&255));
    assert_eq!(render_409(&mut renderer, line_through(ends)), expected);
    assert_eq!(renderer.validation_errors().unwrap(), Vec::<String>::new());
    let mut band = selected_renderer(RenderLineClippingBehavior::PlatformNativeV2);
    assert_ne!(
        render_409(&mut band, line_through(ends)),
        expected,
        "the far line must discriminate the guard band from the driver clip"
    );
}

#[test]
fn version_three_keeps_every_far_line_guarantee() {
    let mut renderer = selected_renderer(RenderLineClippingBehavior::PlatformNativeV3);
    let column: Vec<(usize, usize)> = (0..64).map(|row| (10, row)).collect();
    for half in [100., 16_400., 18_000., 500_000., 1.0e8] {
        let lit = lit_pixels(&mut renderer, vertical_line(32. - half, 32. + half));
        assert_eq!(lit, column, "line spanning {} px", 2. * half);
    }
    let off_screen = lit_pixels(&mut renderer, vertical_line(1.0e6, 2.0e6));
    assert!(off_screen.is_empty(), "off-screen line lit {off_screen:?}");
    assert_eq!(renderer.validation_errors().unwrap(), Vec::<String>::new());
}
