//! A placement frame draw renders the bytes of its separate draws.
use super::*;
use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
use crate::draw_instances::{DrawInstancesRef, InstancePlacement};
use crate::{
    PrimitiveDraw, RenderBackend, RenderFrame, RenderLighting, RenderMaterial,
    RenderMaterialCommon, RenderShading, RenderVertex, identity_clip,
};
use martensite_tkernel::ncollection::Mat4;

const PLACEMENTS: usize = 12;

/// `projection * view` of the test: the strips of x in [0, 12] fill the frame.
fn projection_view() -> [f32; 16] {
    let mut matrix = identity_clip();
    matrix[0] = 2. / 12.;
    matrix[12] = -1.;
    matrix[10] = 0.5;
    matrix
}

fn world(index: usize) -> [f32; 16] {
    let mut matrix = identity_clip();
    matrix[12] = index as f32;
    matrix[14] = -0.05 * index as f32;
    matrix
}

fn template(index: usize) -> PrimitiveDraw {
    let vertices = [
        [0.1, -0.9, 0.],
        [0.9, -0.9, 0.],
        [0.9, 0.9, 0.],
        [0.1, 0.9, 0.],
    ]
    .map(|position| RenderVertex::new(position, [1.; 4]))
    .to_vec();
    let material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission: [0.3, 0.5, 0.8, 1.],
            ..RenderMaterialCommon::default()
        }; 2],
        ..Default::default()
    };
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        material,
        world(index),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    let clip = (Mat4::from_data(projection_view()) * Mat4::from_data(world(index))).data();
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], clip)
        .unwrap()
        .with_lighting(lighting)
        .unwrap()
}

/// The separate draws: one template body under every placement.
fn separate_draws() -> Vec<PrimitiveDraw> {
    let first = template(0);
    (0..PLACEMENTS)
        .map(|index| {
            let placed = template(index);
            let mut draw = first.clone();
            draw.clip_transform = placed.clip_transform;
            let lighting = draw.lighting.as_mut().unwrap();
            let source = placed.lighting.as_ref().unwrap();
            (lighting.world, lighting.normal) = (source.world, source.normal);
            draw
        })
        .collect()
}

fn placement_draw(draws: &[PrimitiveDraw]) -> PrimitiveDraw {
    let placements = draws
        .iter()
        .map(|draw| {
            let lighting = draw.lighting.as_ref().unwrap();
            InstancePlacement {
                template: 0,
                world: lighting.world,
                normal: lighting.normal,
            }
        })
        .collect();
    let instances = crate::draw_instances::DrawInstances {
        templates: vec![draws[0].clone()],
        placements,
        projection_view: projection_view(),
        visibility: None,
        hiz_history: None,
    };
    let mut head = draws[0].clone();
    head.instances = Some(DrawInstancesRef(std::sync::Arc::new(instances)));
    head
}

fn render(
    renderer: &mut crate::backend::VulkanRenderer,
    draws: Vec<PrimitiveDraw>,
) -> crate::FrameCapture {
    let frame = RenderFrame::new(48, 8, [0., 0., 0., 1.], draws).unwrap();
    renderer.render(&frame).unwrap()
}

#[test]
fn a_placement_draw_renders_the_bytes_of_its_separate_draws() {
    let mut renderer = create_resource_renderer();
    if !renderer.runtime.draw_tables {
        return;
    }
    let draws = separate_draws();
    let separate = render(&mut renderer, draws.clone());
    let placed = render(&mut renderer, vec![placement_draw(&draws)]);
    assert_ne!(
        separate.rgba,
        [0, 0, 0, 255].repeat(48 * 8),
        "the strips drew"
    );
    assert_eq!(placed.rgba, separate.rgba);
    assert_eq!(placed.depth, separate.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "lit-only-command-diagnostics"
)))]
fn mixed_placement_materials_preserve_separate_color_depth_and_light_segments() {
    let mut renderer = create_resource_renderer();
    if !renderer.runtime.draw_tables {
        return;
    }
    let mut draws = separate_draws();
    for draw in draws.iter_mut().skip(1).step_by(2) {
        for face in &mut draw.lighting.as_mut().unwrap().material.common {
            face.emission = [0.8, 0.2, 0.1, 1.];
        }
    }
    draws[6].lighting.as_mut().unwrap().lights.ambient = [0.4, 0.3, 0.2, 1.];
    let placement = placement_draw_with_templates(&draws);
    let separate = render(&mut renderer, draws);
    let placed = render(&mut renderer, vec![placement]);
    let (first, second) = ((4 * 48 + 2) * 4, (4 * 48 + 6) * 4);
    let light = (4 * 48 + 26) * 4;
    assert_ne!(
        separate.rgba[first..first + 4],
        separate.rgba[light..light + 4]
    );
    assert_ne!(
        separate.rgba[first..first + 4],
        separate.rgba[second..second + 4]
    );
    assert_eq!(placed.rgba, separate.rgba);
    assert!(
        placed
            .depth
            .iter()
            .map(|d| d.to_bits())
            .eq(separate.depth.iter().map(|d| d.to_bits()))
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "lit-only-command-diagnostics"
)))]
fn large_index_template() -> PrimitiveDraw {
    let source = template(1);
    let mut vertices = source.vertices.to_vec();
    let duplicate = vertices[1];
    vertices.resize(65_537, duplicate);
    let mut draw =
        source.with_replacement_geometry(vertices.into(), vec![0, 65_536, 2, 0, 2, 3].into());
    draw.lighting.as_mut().unwrap().normals = vec![[0., 0., 1.]; 65_537].into();
    draw
}

#[test]
#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "lit-only-command-diagnostics"
)))]
fn mixed_index_width_placements_match_separate_draws_after_warming() {
    use crate::backend::geometry_residency::IndexFormat;
    let mut renderer = create_resource_renderer();
    if !renderer.runtime.draw_tables {
        return;
    }
    let templates = vec![template(0), large_index_template(), template(2)];
    assert_eq!(
        templates
            .iter()
            .map(IndexFormat::of_draw)
            .collect::<Vec<_>>(),
        [IndexFormat::U16, IndexFormat::U32, IndexFormat::U16]
    );
    let placed = placement_draw_with_templates(&templates);
    let control = render(&mut renderer, (0..3).map(template).collect());
    let start = renderer.work_ledger().totals();
    let separate = render(&mut renderer, templates);
    let uploaded = renderer.work_ledger().totals().since(&start);
    let stride = u64::from(crate::backend::pipeline_state::LIT_VERTEX_STRIDE);
    let vertex_bytes = (2 * 4 + 65_537) * stride;
    assert_eq!(
        uploaded.geometry_uploaded_bytes,
        vertex_bytes + 2 * 6 * 2 + 6 * 4
    );

    let first = render(&mut renderer, vec![placed.clone()]);
    let before_warm = renderer.work_ledger().totals();
    let warm = render(&mut renderer, vec![placed]);
    let warmed = renderer.work_ledger().totals().since(&before_warm);
    assert_eq!(warmed.geometry_uploaded_bytes, 0);
    for mixed in [separate, first, warm] {
        assert_eq!(mixed.rgba, control.rgba);
        let actual = mixed.depth.iter().map(|depth| depth.to_bits());
        let expected = control.depth.iter().map(|depth| depth.to_bits());
        assert!(actual.eq(expected));
    }
    for column in [2, 6, 10] {
        let pixel = (4 * 48 + column) * 4;
        assert_ne!(control.rgba[pixel..pixel + 4], [0, 0, 0, 255]);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "lit-only-command-diagnostics"
)))]
fn placement_draw_with_templates(templates: &[PrimitiveDraw]) -> PrimitiveDraw {
    let placements = templates
        .iter()
        .enumerate()
        .map(|(index, draw)| {
            let lighting = draw.lighting.as_ref().unwrap();
            InstancePlacement {
                template: index as u32,
                world: lighting.world,
                normal: lighting.normal,
            }
        })
        .collect();
    let instances = DrawInstances {
        templates: templates.to_vec(),
        placements,
        projection_view: projection_view(),
        visibility: None,
        hiz_history: None,
    };
    let mut head = templates[0].clone();
    head.instances = Some(DrawInstancesRef(Arc::new(instances)));
    head
}

#[test]
fn the_table_holds_one_entry_per_placement() {
    let instances = crate::draw_instances::DrawInstances {
        templates: Vec::new(),
        placements: (0..4)
            .map(|index| InstancePlacement {
                template: index % 2,
                world: identity_clip(),
                normal: identity_clip(),
            })
            .collect(),
        projection_view: identity_clip(),
        visibility: None,
        hiz_history: None,
    };
    let entries = table_entries(&instances);
    assert_eq!(entries.len(), 4 * ENTRY_BYTES);
}

#[test]
fn placement_classes_keep_shared_lighting_and_index_materials_separately() {
    let first = template(0);
    let second = template(1);
    let mut colored = template(2);
    colored.lighting.as_mut().unwrap().material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission: [0.7, 0.1, 0.2, 1.],
            ..Default::default()
        }; 2],
        ..Default::default()
    };
    let mut changed_light = colored.clone();
    changed_light.lighting.as_mut().unwrap().eye[0] = 0.5;
    let draws = DrawInstances {
        templates: vec![first, second, colored, changed_light],
        placements: Vec::new().into(),
        projection_view: projection_view(),
        visibility: None,
        hiz_history: None,
    };
    assert_eq!(lighting_classes(&draws, false), [0, 0, 1, 2]);
    assert_eq!(lighting_classes(&draws, true), [0, 0, 0, 1]);
}

#[test]
fn indexed_material_words_are_deduplicated_and_follow_placement_order() {
    let first = template(0);
    let second = template(1);
    let mut colored = template(2);
    colored.lighting.as_mut().unwrap().material.common[0].emission = [-0., 0.7, 0.2, 1.];
    let instances = DrawInstances {
        templates: vec![first, second, colored],
        placements: [2, 1, 0, 2]
            .into_iter()
            .map(|template| InstancePlacement {
                template,
                world: identity_clip(),
                normal: identity_clip(),
            })
            .collect(),
        projection_view: projection_view(),
        visibility: None,
        hiz_history: None,
    };
    let table = placement_materials(&instances).unwrap();
    let words: Vec<_> = table
        .bytes()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_ne_bytes(*b))
        .collect();
    assert_eq!(words.len(), 4 + 2 * 32);
    assert_eq!(words[..4], [36, 4, 4, 36]);
    assert_eq!(words[8..12], [0.3f32, 0.5, 0.8, 1.].map(f32::to_bits));
    assert_eq!(words[40..44], [-0.0f32, 0.7, 0.2, 1.].map(f32::to_bits));
}

#[path = "placement_hiz_native_tests.rs"]
mod hiz;

#[test]
fn placement_admission_declines_mixed_dynamic_raster_state() {
    let renderer = create_resource_renderer();
    if !renderer.runtime.draw_tables {
        return;
    }
    let first = template(0);
    let mut mirrored = first.clone();
    mirrored.mirrored = true;
    let biased = first
        .clone()
        .with_polygon_offset(crate::RenderPolygonOffset::new(1, 1., 2.).unwrap());
    let culled = first
        .clone()
        .with_face_culling(crate::RenderFaceCulling::Back);
    assert!(
        renderer
            .supports_placements(std::slice::from_ref(&first), 1)
            .unwrap()
    );
    assert!(
        renderer
            .supports_placements(std::slice::from_ref(&mirrored), 1)
            .unwrap()
    );
    let too_many =
        renderer.runtime.limits.max_storage_buffer_range as usize / ENTRY_BYTES as usize + 1;
    assert!(
        !renderer
            .supports_placements(std::slice::from_ref(&first), too_many)
            .unwrap()
    );
    for changed in [mirrored, biased, culled] {
        assert!(
            !renderer
                .supports_placements(&[first.clone(), changed], 2)
                .unwrap()
        );
    }
}

#[test]
fn uniform_mirrored_placements_keep_their_winding_and_depth() {
    let mut renderer = create_resource_renderer();
    if !renderer.runtime.draw_tables {
        return;
    }
    let mut mirrored = separate_draws();
    for draw in &mut mirrored {
        draw.mirrored = true;
        draw.face_culling = crate::RenderFaceCulling::Back;
        let lighting = draw.lighting.as_mut().unwrap();
        lighting.world[0] = -1.;
        lighting.world[12] += 1.;
        lighting.normal[0] = -1.;
        draw.clip_transform =
            (Mat4::from_data(projection_view()) * Mat4::from_data(lighting.world)).data();
    }
    let placed = placement_draw(&mirrored);
    let expected = render(&mut renderer, mirrored);
    let actual = render(&mut renderer, vec![placed]);
    assert!(expected.depth.iter().any(|&depth| depth < 1.));
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}
