use super::*;
use crate::{
    RenderBackend, RenderBehaviorConfiguration, RenderCoordinateConvention, RenderFaceCulling,
    RenderLightSet, RenderLighting, RenderMaterial, RenderMaterialCommon, RenderShading,
    VulkanOptions, VulkanRenderer, identity_clip,
};

#[test]
fn native_meshlets_match_ordered_color_and_depth_in_both_coordinates() {
    for coordinates in [
        RenderCoordinateConvention::LegacyBottomUpV1,
        RenderCoordinateConvention::TopLeftLogicalV2,
    ] {
        let mut selected = renderer(true, coordinates);
        if selected.runtime.meshlet_group_limit == 0 {
            return;
        }
        let mut control = renderer(false, coordinates);
        let frame = fixture(false);
        let expected = control.render(&frame).unwrap();
        assert!(
            expected
                .rgba
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[2] > 160)
        );
        for _ in 0..2 {
            let actual = selected.render(&frame).unwrap();
            assert!(expected.depth.iter().any(|&depth| depth < 1.));
            assert_eq!(actual.rgba, expected.rgba);
            assert_eq!(actual.depth, expected.depth);
        }
        assert_eq!(selected.pipeline.meshlets.0.lock().unwrap().len(), 1);
        let errors = selected.validation_errors().unwrap();
        assert!(errors.is_empty(), "{errors:?}");
    }
}

fn renderer(meshlets: bool, coordinates: RenderCoordinateConvention) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: None,
        },
        Default::default(),
        coordinates,
        Default::default(),
        RenderBehaviorConfiguration::default().with_meshlet_draws(meshlets),
    )
    .unwrap()
}

fn fixture(directional: bool) -> RenderFrame {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for (center, left_to_right) in [(-0.5, true), (0.5, false), (3., true)] {
        for _ in 0..192 {
            let first = u32::try_from(vertices.len()).unwrap();
            let positions = [
                [center - 0.4, -0.45, -0.2],
                [center + 0.4, -0.45, -0.2],
                [center, 0.45, -0.2],
            ];
            vertices.extend(positions.map(|point| RenderVertex::new(point, [1.; 4])));
            indices.extend(if left_to_right {
                [first, first + 1, first + 2]
            } else {
                [first, first + 2, first + 1]
            });
        }
    }
    if directional {
        let unused = RenderVertex::new([0.; 3], [1.; 4]);
        vertices.resize(directional_phong::MIN_VERTICES, unused);
    }
    let mesh = lit_mesh(vertices, indices).with_face_culling(RenderFaceCulling::Back);
    let small = [[-0.6, -0.15, -0.4], [-0.4, -0.15, -0.4], [-0.5, 0.15, -0.4]]
        .map(|position| RenderVertex::new(position, [0., 1., 0., 1.]));
    let small = PrimitiveDraw::new(small.into(), vec![0, 1, 2], identity_clip()).unwrap();
    RenderFrame::new(73, 57, [0., 0., 0., 1.], vec![mesh, small]).unwrap()
}

fn lit_mesh(vertices: Vec<RenderVertex>, indices: Vec<u32>) -> PrimitiveDraw {
    let material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission: [0.025, 0.05, 0.065, 1.],
            diffuse: [0.35, 0.45, 0.75, 1.],
            ..Default::default()
        }; 2],
        ..Default::default()
    };
    let mut world = identity_clip();
    world[2] = 0.35;
    world[10] = 1.15;
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; vertices.len()],
        material,
        world,
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(direct_light());
    PrimitiveDraw::new(vertices, indices, identity_clip())
        .unwrap()
        .with_lighting(lighting)
        .unwrap()
}

fn direct_light() -> RenderLightSet {
    use crate::light_snapshot::PackedRenderLight;
    use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;

    RenderLightSet {
        ambient: [0.07, 0.08, 0.1, 1.],
        sources: vec![PackedRenderLight {
            kind: Graphic3dTypeOfLightSource::Directional as u32,
            cast_shadows: false,
            color: [0.95, 0.85, 0.75, 1.],
            position: [0.1, 0.0, 0.994_987_4, 0.],
            direction: [0.; 4],
            params: [0.; 4],
        }]
        .into(),
    }
}

#[test]
fn meshlets_reuse_the_qualified_directional_fragment_without_changing_depth() {
    let mut selected = renderer(true, RenderCoordinateConvention::default());
    if selected.runtime.meshlet_group_limit == 0 {
        return;
    }
    let frame = fixture(true);
    let source = &frame.draws[0];
    assert!(directional_phong::admitted(&selected.runtime, source));
    let mut control = renderer(false, RenderCoordinateConvention::default());
    let expected = control.render(&frame).unwrap();
    let actual = selected.render(&frame).unwrap();
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
    assert_eq!(selected.pipeline.meshlets.0.lock().unwrap().len(), 1);
    let errors = selected.validation_errors().unwrap();
    assert!(errors.is_empty(), "{errors:?}");
}
