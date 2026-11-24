//! Compare current depth witnesses and the final render with independent direct draws.
use super::*;
use crate::backend::{HiZHistory, placement_hiz_history::HiZView};
use crate::draw_instances::{InstanceBounds, InstanceVisibility};

const EXTENT: [u32; 2] = [95, 79];

#[test]
fn warm_hiz_compacts_covered_placements_and_preserves_ties_holes_and_near_plane() {
    let mut renderer = hiz_renderer(true, true);
    if !renderer.supports_hiz() || cfg!(feature = "command-state-diagnostics") {
        return;
    }
    let history = Arc::new(HiZHistory::default());
    let camera = identity_clip();
    let initial = separate_hiz_draws(camera);
    let expected = capture_hiz(&mut renderer, initial.clone());
    let frame = hiz_draw(&initial, camera, history.clone());
    for mask in [[1, 1, 1, 1, 1], [1, 0, 1, 1, 1], [1, 0, 1, 1, 1]] {
        let actual = capture_hiz(&mut renderer, vec![frame.clone()]);
        assert_eq!(actual.rgba, expected.rgba);
        assert_eq!(actual.depth, expected.depth);
        assert_eq!(visibility_mask(&renderer, &history, camera), mask);
    }

    let before_move = hiz_frame(vec![frame]);
    let mut old_completed = renderer.completed_owned_frame(&before_move).unwrap();
    let mut moved = camera;
    moved[12] += 0.31;
    let updated = separate_hiz_draws(moved);
    let expected = capture_hiz(&mut renderer, updated.clone());
    let actual = capture_hiz(
        &mut renderer,
        vec![hiz_draw(&updated, moved, history.clone())],
    );
    crate::backend::draw_table::publish_hiz(&mut old_completed.draws);
    assert_eq!(visibility_mask(&renderer, &history, moved), [1; 5]);
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
    let other = Arc::new(HiZHistory::default());
    capture_hiz(
        &mut renderer,
        vec![hiz_draw(&updated, moved, other.clone())],
    );
    assert_eq!(visibility_mask(&renderer, &other, moved), [1; 5]);
    check_clear_hole(&mut renderer, &initial, camera);
    let validation = renderer.validation_errors().unwrap();
    assert!(validation.is_empty(), "{validation:#?}");
}

fn check_clear_hole(
    renderer: &mut crate::backend::VulkanRenderer,
    templates: &[PrimitiveDraw],
    camera: [f32; 16],
) {
    let history = Arc::new(HiZHistory::default());
    let mut seed = hiz_draw(templates, camera, history.clone());
    let instances = Arc::get_mut(&mut seed.instances.as_mut().unwrap().0).unwrap();
    instances.visibility.as_mut().unwrap().visible[3] = false;
    capture_hiz(renderer, vec![seed]);
    assert_eq!(visibility_mask(renderer, &history, camera), [1, 1, 1, 0, 1]);

    let expected = capture_hiz(renderer, templates.to_vec());
    let actual = capture_hiz(renderer, vec![hiz_draw(templates, camera, history.clone())]);
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
    assert_eq!(visibility_mask(renderer, &history, camera), [1, 0, 1, 1, 1]);
}

fn hiz_renderer(enabled: bool, validation: bool) -> crate::backend::VulkanRenderer {
    let options = crate::VulkanOptions {
        validation,
        device_index: Some(0),
    };
    let behavior = crate::RenderBehaviorConfiguration::default().with_placement_occlusion(enabled);
    crate::backend::VulkanRenderer::new_with_render_behavior(
        options,
        Default::default(),
        Default::default(),
        Default::default(),
        behavior,
    )
    .unwrap()
}

fn separate_hiz_draws(camera: [f32; 16]) -> Vec<PrimitiveDraw> {
    [
        ([-0.98, -0.93], [-0.1, 0.93], [-0.2; 2], [0.2, 0.3, 0.7]),
        ([-0.57, -0.08], [-0.46, 0.08], [0.6; 2], [1., 0., 0.]),
        ([-0.57, -0.08], [-0.46, 0.08], [-0.2; 2], [0., 1., 0.]),
        ([-0.095, -0.02], [-0.045, 0.02], [0.6; 2], [0., 0., 1.]),
        ([-0.45, 0.12], [-0.33, 0.28], [-1.2, -0.8], [1., 1., 0.]),
    ]
    .map(|(min, max, depths, color)| hiz_quad(camera, min, max, depths, color))
    .into()
}

fn hiz_quad(
    camera: [f32; 16],
    [left, bottom]: [f32; 2],
    [right, top]: [f32; 2],
    [near, far]: [f32; 2],
    [r, g, b]: [f32; 3],
) -> PrimitiveDraw {
    let vertices = [
        [left, bottom, near],
        [right, bottom, near],
        [right, top, far],
        [left, top, far],
    ]
    .map(|position| RenderVertex::new(position, [1.; 4]));
    let material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission: [r, g, b, 1.],
            ..RenderMaterialCommon::default()
        }; 2],
        ..Default::default()
    };
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    PrimitiveDraw::new(vertices.into(), vec![0, 1, 2, 0, 2, 3], camera)
        .unwrap()
        .with_lighting(lighting)
        .unwrap()
}

fn hiz_draw(
    templates: &[PrimitiveDraw],
    camera: [f32; 16],
    history: Arc<HiZHistory>,
) -> PrimitiveDraw {
    let placements = (0..templates.len())
        .map(|template| InstancePlacement {
            template: template as u32,
            world: identity_clip(),
            normal: identity_clip(),
        })
        .collect();
    let mut head = templates[0].clone();
    head.instances = Some(DrawInstancesRef(Arc::new(DrawInstances {
        templates: templates.to_vec(),
        placements,
        projection_view: camera,
        visibility: Some(InstanceVisibility {
            visible: vec![true; templates.len()],
            bounds: templates.iter().map(InstanceBounds::of).collect(),
        }),
        hiz_history: Some(history),
    })));
    head
}

fn capture_hiz(
    renderer: &mut crate::backend::VulkanRenderer,
    draws: Vec<PrimitiveDraw>,
) -> crate::FrameCapture {
    renderer.render(&hiz_frame(draws)).unwrap()
}

fn hiz_frame(draws: Vec<PrimitiveDraw>) -> RenderFrame {
    RenderFrame::new(EXTENT[0], EXTENT[1], [0., 0., 0., 1.], draws).unwrap()
}

fn visibility_mask(
    renderer: &crate::backend::VulkanRenderer,
    history: &HiZHistory,
    camera: [f32; 16],
) -> [u32; 5] {
    read_visibility_mask(renderer, history, camera, 5)
        .try_into()
        .unwrap()
}

fn read_visibility_mask(
    renderer: &crate::backend::VulkanRenderer,
    history: &HiZHistory,
    camera: [f32; 16],
    count: usize,
) -> Vec<u32> {
    let extent = vk::Extent2D {
        width: EXTENT[0],
        height: EXTENT[1],
    };
    let view = HiZView::new(&renderer.runtime, extent, camera);
    let witness = history
        .witness(&view)
        .expect("this frame qualified for Hi-Z");
    let offset = (witness.output.offset + u64::from(witness.first_word) * 4) as usize;
    let bytes = witness.output.buffer.read_at(offset, count * 4).unwrap();
    let (words, _) = bytes.as_chunks::<4>();
    words.iter().copied().map(u32::from_ne_bytes).collect()
}

#[test]
#[ignore = "measures both native GPU paths; run explicitly on an idle device"]
fn dense_hiz_native_ab() {
    let mut culling = hiz_renderer(true, false);
    if !culling.supports_hiz() {
        return;
    }
    let mut control = hiz_renderer(false, false);
    for covered in [1024, 4096] {
        dense_case(&mut control, &mut culling, covered);
    }
}

fn dense_case(
    control: &mut crate::backend::VulkanRenderer,
    culling: &mut crate::backend::VulkanRenderer,
    covered: usize,
) {
    let camera = identity_clip();
    let templates = [
        separate_hiz_draws(camera).remove(0),
        dense_rear_template(camera),
    ];
    let history = Arc::new(HiZHistory::default());
    let draw = repeated_hiz_draw(&templates, covered, camera, history.clone());
    let baseline = capture_hiz(control, vec![draw.clone()]);
    for _ in 0..3 {
        let frame = capture_hiz(culling, vec![draw.clone()]);
        assert_eq!(frame.rgba, baseline.rgba);
        assert_eq!(frame.depth, baseline.depth);
    }
    let mask = read_visibility_mask(culling, &history, camera, covered + 1);
    assert_eq!(mask[0], 1);
    assert!(mask[1..].iter().all(|&keep| keep == 0));
    let (mut plain, mut culled) = (Vec::new(), Vec::new());
    for _ in 0..7 {
        plain.push(measure_capture(control, &draw, &baseline));
        culled.push(measure_capture(culling, &draw, &baseline));
    }
    plain.sort_unstable();
    culled.sort_unstable();
    let triangles = templates[1].indices.len() / 3 * covered;
    println!(
        "{{\"benchmark\":\"dense-hiz\",\"device\":{:?},\"extent\":{:?},\"discardedPlacements\":{covered},\"discardedTriangles\":{triangles},\"baselineCaptureMedianUs\":{},\"hizCaptureMedianUs\":{}}}",
        culling.device_name(),
        EXTENT,
        plain[3],
        culled[3],
    );
}

fn measure_capture(
    renderer: &mut crate::backend::VulkanRenderer,
    draw: &PrimitiveDraw,
    baseline: &crate::FrameCapture,
) -> u128 {
    let started = std::time::Instant::now();
    let frame = capture_hiz(renderer, vec![draw.clone()]);
    let micros = started.elapsed().as_micros();
    assert_eq!(frame.rgba, baseline.rgba);
    assert_eq!(frame.depth, baseline.depth);
    micros
}

fn repeated_hiz_draw(
    templates: &[PrimitiveDraw],
    repeats: usize,
    camera: [f32; 16],
    history: Arc<HiZHistory>,
) -> PrimitiveDraw {
    let mut draw = hiz_draw(templates, camera, history);
    let instances = Arc::get_mut(&mut draw.instances.as_mut().unwrap().0).unwrap();
    let front = instances.placements[0];
    let rear = instances.placements[1];
    instances.placements = std::iter::once(front)
        .chain(std::iter::repeat_n(rear, repeats))
        .collect();
    instances.visibility.as_mut().unwrap().visible = vec![true; repeats + 1];
    draw
}

fn dense_rear_template(camera: [f32; 16]) -> PrimitiveDraw {
    const QUADS: u32 = 32;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for y in 0..QUADS {
        for x in 0..QUADS {
            add_rear_quad(&mut vertices, &mut indices, [x, y], QUADS as f32);
        }
    }
    let mut draw = separate_hiz_draws(camera)
        .remove(1)
        .with_replacement_geometry(vertices.into(), indices.into());
    draw.lighting.as_mut().unwrap().normals = vec![[0., 0., 1.]; draw.vertices.len()].into();
    draw
}

fn add_rear_quad(
    vertices: &mut Vec<RenderVertex>,
    indices: &mut Vec<u32>,
    [x, y]: [u32; 2],
    side: f32,
) {
    let base = vertices.len() as u32;
    indices.extend([0, 1, 2, 0, 2, 3].map(|index| index + base));
    for [dx, dy] in [[0., 0.], [1., 0.], [1., 1.], [0., 1.]] {
        let px = -0.57 + (x as f32 + dx) * 0.11 / side;
        let py = -0.08 + (y as f32 + dy) * 0.16 / side;
        vertices.push(RenderVertex::new([px, py, 0.6], [1.; 4]));
    }
}
