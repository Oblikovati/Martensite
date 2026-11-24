//! Draw tables: a run captures the bytes its draws capture one by one, and only plain lit
//! surfaces with equal recording state join a run.
use super::*;
use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
use crate::{
    RenderBackend, RenderFrame, RenderMaterial, RenderMaterialCommon, RenderShading, RenderVertex,
    identity_clip,
};

fn lighting(world_x: f32) -> RenderLighting {
    let emission = if (world_x as usize).is_multiple_of(2) {
        [0.2, 0.6, 0.9, 1.]
    } else {
        [0.9, 0.2, 0.6, 1.]
    };
    let material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission,
            ..RenderMaterialCommon::default()
        }; 2],
        ..Default::default()
    };
    let mut world = identity_clip();
    world[12] = world_x;
    RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        material,
        world,
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
}

/// A strip of the frame, placed by its clip matrix, with its own world matrix.
fn strip(index: usize) -> PrimitiveDraw {
    let vertices = [
        [-1., -1., 0.],
        [-0.85, -1., 0.],
        [-0.85, 1., 0.],
        [-1., 1., 0.],
    ]
    .map(|position| RenderVertex::new(position, [1.; 4]))
    .to_vec();
    let mut clip = identity_clip();
    clip[12] = 0.16 * index as f32;
    clip[14] = 0.01 * index as f32;
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], clip)
        .unwrap()
        .with_lighting(lighting(index as f32))
        .unwrap()
}

fn render(
    renderer: &mut crate::backend::VulkanRenderer,
    runs: bool,
    method: RenderShading,
) -> (crate::FrameCapture, usize) {
    let draws = (0..12)
        .map(|index| {
            let mut draw = strip(index);
            draw.lighting.as_mut().unwrap().shading = method;
            draw
        })
        .collect();
    let frame = RenderFrame::new(32, 8, [0., 0., 0., 1.], draws).unwrap();
    TEST_RUNS.set((runs, 0));
    let capture = renderer.render(&frame).unwrap();
    let formed = TEST_RUNS.get().1;
    TEST_RUNS.set((true, 0));
    (capture, formed)
}

#[test]
fn a_run_captures_the_bytes_its_draws_capture_one_by_one() {
    let mut renderer = create_resource_renderer();
    assert_material_run(&mut renderer, RenderShading::Phong);
    assert_material_run(&mut renderer, RenderShading::Gouraud);
}

fn assert_material_run(renderer: &mut crate::backend::VulkanRenderer, method: RenderShading) {
    let (single, _) = render(renderer, false, method);
    let (run, formed) = render(renderer, true, method);
    if renderer.runtime.draw_tables
        && super::super::draw_material_table::INDEX_MATERIALS
        && crate::backend::pipeline::indexed_storage_limits(&renderer.runtime.limits, false)
    {
        assert_eq!(formed, 1, "alternating materials still form one run");
    }
    assert_eq!(run.rgba, single.rgba);
    assert_eq!(run.depth, single.depth);
    assert_ne!(
        single.rgba,
        [0, 0, 0, 255].repeat(32 * 8),
        "the strips drew"
    );
    let (first, second) = ((4 * 32 + 1) * 4, (4 * 32 + 4) * 4);
    assert_ne!(
        single.rgba[first..first + 4],
        single.rgba[second..second + 4]
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn lighting_blocks_share_a_run_when_every_unindexed_word_matches() {
    let (near, far) = (lighting(0.), lighting(7.));
    assert!(shared_lighting_values(&near, &lighting(6.), false));
    assert!(!shared_lighting_values(&near, &far, false));
    assert!(shared_lighting_values(&near, &far, true));
    let mut other = lighting(0.);
    other.eye[0] = 0.5;
    assert!(
        !shared_lighting_values(&near, &other, true),
        "the eye is read by the fragments"
    );
}

#[test]
fn ordered_indirect_calls_preserve_ranges_above_the_device_count_limit() {
    assert_eq!(
        indirect_chunks(17, 100_000, 65_535).collect::<Vec<_>>(),
        [(17, 65_535), (65_552, 34_465)]
    );
    assert_eq!(
        indirect_chunks(7, 2, 1).collect::<Vec<_>>(),
        [(7, 1), (8, 1)]
    );
    assert_eq!(indirect_chunks(17, 0, 65_535).count(), 0);
}
