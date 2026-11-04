//! Actual public presentation factories exercise the original unit-box Hatch4 scene.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "support/hatch_scene.rs"]
pub mod scene;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::*, graphic3d::*};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver};
use scene::HatchBoxScene;

#[test]
fn unit_box_hatch4_preserves_window_phase_solid_color_and_depth() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = HatchBoxScene::new(&driver);
    let solid = scene.capture("solid");
    scene.hatch(true);
    let hatch = scene.capture("hatch4");
    assert_window_mask(&solid, &hatch, grid_wide_pixel);
    scene.hatch(false);
    let restored = scene.capture("restored");
    assert_eq!(restored.rgba, solid.rgba);
    assert_eq!(restored.depth, solid.depth);
}

fn assert_window_mask(solid: &FrameCapture, hatch: &FrameCapture, kept: fn(usize, usize) -> bool) {
    assert_eq!([solid.width, solid.height], [409, 409]);
    let mut removed = 0;
    for (index, rgba) in hatch.rgba.as_chunks::<4>().0.iter().enumerate() {
        let x = index % 409;
        let y = 408 - index / 409;
        if kept(x, y) {
            assert_retained_pixel(solid, hatch, index);
            continue;
        }
        assert_eq!(&rgba[..3], &[0, 0, 0], "discarded pixel={index}");
        assert_eq!(hatch.depth[index], 1., "discarded pixel={index}");
        removed += usize::from(solid.rgba[index * 4..index * 4 + 3] != [0, 0, 0]);
    }
    assert!(
        removed > 1000,
        "removed={removed}; expected real box interior coverage"
    );
}

fn assert_retained_pixel(solid: &FrameCapture, hatch: &FrameCapture, index: usize) {
    let range = index * 4..index * 4 + 4;
    assert_eq!(
        &hatch.rgba[range.clone()],
        &solid.rgba[range],
        "pixel={index}"
    );
    // The masked pipeline may interpolate an outline pixel's depth slightly differently; it never
    // changes the colour there, and every interior pixel stays exact.
    if silhouette_pixel(solid, index) {
        let delta = (hatch.depth[index] - solid.depth[index]).abs();
        assert!(delta <= OUTLINE_DEPTH_BOUND, "pixel={index} delta={delta}");
    } else {
        assert_eq!(hatch.depth[index], solid.depth[index], "pixel={index}");
    }
}

const OUTLINE_DEPTH_BOUND: f32 = 0.001;

/// A covered pixel with an uncovered 8-neighbour; `silhouette_pixel(&capture, index)`.
fn silhouette_pixel(capture: &FrameCapture, index: usize) -> bool {
    let (x, y) = ((index % 409) as isize, (index / 409) as isize);
    (-1..=1).any(|dy| {
        (-1..=1).any(|dx| {
            let (nx, ny) = (x + dx, y + dy);
            (0..409).contains(&nx)
                && (0..409).contains(&ny)
                && capture.depth[(ny * 409 + nx) as usize] == 1.
        })
    })
}

#[test]
fn outline_depth_is_only_excused_beside_uncovered_pixels() {
    let mut capture = FrameCapture {
        width: 409,
        height: 409,
        rgba: vec![0; 409 * 409 * 4],
        depth: vec![0.5; 409 * 409],
    };
    assert!(!silhouette_pixel(&capture, 409 * 200 + 200));
    capture.depth[409 * 200 + 201] = 1.;
    assert!(silhouette_pixel(&capture, 409 * 200 + 200));
}

fn grid_wide_pixel(x: usize, y: usize) -> bool {
    // Independent analytic grid, rather than decoding the shader's packed bitmap.
    y.is_multiple_of(4) || x.is_multiple_of(8)
}

fn horizontal_pixel(_x: usize, y: usize) -> bool {
    // The native horizontal preset alternates whole rows in lower-left window coordinates.
    y.is_multiple_of(2)
}

#[test]
fn same_pipeline_receives_live_bitmap_replacements_and_preserves_prior_capture() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = HatchBoxScene::new(&driver);
    let solid = scene.capture("replacement-solid");
    scene.hatch(true);
    let first = scene.capture("replacement-grid");
    scene.pattern(AspectHatchStyle::Horizontal);
    let horizontal = scene.capture("replacement-horizontal");
    assert_window_mask(&solid, &horizontal, horizontal_pixel);
    scene.pattern(AspectHatchStyle::GridWide);
    let restored = scene.capture("replacement-restored");
    assert_eq!(first.rgba, restored.rgba);
    assert_eq!(first.depth, restored.depth);
}

#[test]
fn hatch_preserves_actual_clipping_planes_and_their_retirement() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = HatchBoxScene::new(&driver);
    set_hatch_clip(&scene, true);
    let solid = scene.capture("clipped-solid");
    scene.hatch(true);
    let hatch = scene.capture("clipped-hatch");
    assert_window_mask(&solid, &hatch, grid_wide_pixel);
    set_hatch_clip(&scene, false);
    let unclipped = scene.capture("clipped-retired");
    assert_ne!(hatch.rgba, unclipped.rgba);
}

fn set_hatch_clip(scene: &HatchBoxScene, enabled: bool) {
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    if enabled {
        let equation = martensite_tkernel::ncollection::Vec4::new(1., 0., 0., -0.5);
        sequence.append(Handle::new(
            Graphic3dClipPlane::from_equation(equation).unwrap(),
        ));
    }
    scene.view.get().unwrap().view().set_clip_planes(&sequence);
}

#[test]
fn translucent_fill_keeps_its_case_blending_outside_removed_fragments() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = HatchBoxScene::new(&driver);
    let aspect = scene.object.attributes().shading_aspect().aspect();
    let mut material = aspect.front_material();
    material.set_transparency(0.5).unwrap();
    aspect.set_front_material(material);
    aspect.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    let solid = scene.capture("alpha-solid");
    scene.hatch(true);
    let hatch = scene.capture("alpha-hatch");
    assert_window_mask(&solid, &hatch, grid_wide_pixel);
}

#[path = "support/hatch_source_aspect_controls.rs"]
mod source_aspect_controls;

#[path = "support/hatch_source_output_controls.rs"]
mod source_output_controls;

#[test]
fn coordinate_cells_keep_native_hatch_window_phase_and_depth() {
    crate::coordinate_corpus_cells::replay_cells(Default::default(), |driver| {
        let scene = HatchBoxScene::new(driver);
        let solid = scene.capture("coordinate-solid");
        scene.hatch(true);
        let hatch = scene.capture("coordinate-hatch");
        assert_window_mask(&solid, &hatch, grid_wide_pixel);
        scene.pattern(AspectHatchStyle::Horizontal);
        let horizontal = scene.capture("coordinate-horizontal");
        assert_window_mask(&solid, &horizontal, horizontal_pixel);
    });
}
