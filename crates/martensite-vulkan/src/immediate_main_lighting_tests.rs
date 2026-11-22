use super::*;
use crate::{
    RenderClipping, RenderLayer, RenderVertex, identity_clip, light_snapshot::PackedRenderLight,
};

use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectWindowMembers},
    graphic3d::{Graphic3dCView, Graphic3dCameraProjection, Graphic3dClipPlaneSequenceSource},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisInteractiveObject, AisShape},
    prs_mgr::PrsMgrDisplayStatus,
    v3d::{V3dViewSource, V3dViewer},
};

fn lit_draw() -> PrimitiveDraw {
    let points = [[-0.8, -0.8, 0.2], [0.8, -0.8, 0.2], [0., 0.8, 0.2]];
    let mut material = RenderMaterial::default();
    material.set_alpha(1.);
    material.common[1].ambient[1] = 0.;
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(RenderLightSet {
        ambient: [0.2, 0., 0., 1.],
        sources: vec![PackedRenderLight {
            kind: 1,
            cast_shadows: false,
            color: [1.; 4],
            position: [0.; 4],
            direction: [0., 0., 1., 0.],
            params: [0.; 4],
        }],
    });
    PrimitiveDraw::new(
        points
            .map(|point| RenderVertex::new(point, [1.; 4]))
            .to_vec(),
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap()
    .with_lighting(lighting)
    .unwrap()
}

fn fixture() -> RenderFrame {
    let left = lit_draw();
    let mut right = left.clone();
    right.lighting.as_mut().unwrap().world[12] = 0.25;
    let mut frame = RenderFrame::with_layers(
        32,
        32,
        [0.; 4],
        vec![RenderLayer::new(vec![left, right], true, true, true)],
    )
    .unwrap();
    frame.immediate_start = Some(1);
    frame
}

#[test]
fn shares_common_lights_and_materials_without_pinning_geometry() {
    let frame = fixture();
    let key = MainPassKey::from_frame(&frame, 1).unwrap();
    assert!(key.matches(&frame.clone(), 1));
    assert_eq!(key.lighting.materials.len(), 1);
    assert_eq!(key.lighting.view.as_ref().unwrap().lights.sources.len(), 1);
    assert_eq!(key.lighting.draws.len(), 2);
    for draw in &key.lighting.draws {
        assert_eq!(draw.normals.0.as_ref().unwrap().strong_count(), 2);
    }
    drop(frame);
    assert_eq!(
        key.lighting.draws[0]
            .normals
            .0
            .as_ref()
            .unwrap()
            .strong_count(),
        0
    );
}

#[test]
fn changed_lit_main_state_refuses_reuse_at_float_bit_precision() {
    let frame = fixture();
    let key = MainPassKey::from_frame(&frame, 1).unwrap();
    let cases: [fn(&mut RenderLighting); 11] = [
        |lit| lit.material.common[1].ambient[1] = -0.,
        |lit| lit.lights.ambient[1] = -0.,
        |lit| lit.lights.sources[0].position[0] = -0.,
        |lit| lit.lights.sources[0].cast_shadows = true,
        |lit| lit.world[12] = 0.5,
        |lit| lit.normal[0] = 2.,
        |lit| lit.view[1] = -0.,
        |lit| lit.eye[1] = -0.,
        |lit| lit.normals.make_mut()[0][0] = -0.,
        |lit| lit.shading = RenderShading::Gouraud,
        |lit| lit.front_vertex_colors_only = true,
    ];
    for (index, mutate) in cases.into_iter().enumerate() {
        let mut changed = frame.clone();
        mutate(changed.draws[1].lighting.as_mut().unwrap());
        assert!(
            !key.matches(&changed, 1),
            "lighting input {index} must invalidate"
        );
    }
}

#[test]
fn rejects_clipping_pbr_and_incompatible_view_lights() {
    let frame = fixture();
    let key = MainPassKey::from_frame(&frame, 1).unwrap();
    let cases: [fn(&mut PrimitiveDraw); 3] = [
        |draw| {
            draw.clipping =
                Some(RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], identity_clip()).unwrap())
        },
        |draw| draw.lighting.as_mut().unwrap().pbr = true,
        |draw| draw.lighting.as_mut().unwrap().lights.ambient[0] = 0.5,
    ];
    for mutate in cases {
        let mut changed = frame.clone();
        mutate(&mut changed.draws[1]);
        assert!(MainPassKey::from_frame(&changed, 1).is_none());
        assert!(!key.matches(&changed, 1));
    }
}

struct SceneExtractionOnly;

impl crate::RenderBackend for SceneExtractionOnly {
    fn render(&mut self, _frame: &RenderFrame) -> Result<crate::FrameCapture, crate::RenderError> {
        panic!("the AIS admission check only extracts scene frames");
    }
}

#[test]
fn live_shaded_ais_sphere_reuses_the_key_through_dynamic_highlight() {
    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 4096).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = lit_sphere_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let _object = display_sphere(&context);
    view.auto_z_fit().unwrap();
    let cview = view.get().unwrap().view();
    let clipping = Handle::<dyn Graphic3dClipPlaneSequenceSource>::default();
    let extract = || {
        crate::view_scene::frame_for_view(
            cview.get().unwrap(),
            [128, 128],
            &clipping,
            Default::default(),
        )
        .unwrap()
    };
    let first = extract();
    assert!(
        !first.draws.is_empty(),
        "displayed sphere must reach the scene"
    );
    let start = eligible_start(&first).unwrap();
    let key = MainPassKey::from_frame(&first, start).unwrap();
    assert!(!key.lighting.draws.is_empty());
    context.move_to(64, 64, &view, false).unwrap();
    assert!(context.has_detected());
    let hovered = extract();
    let hovered_start = eligible_start(&hovered).unwrap();
    assert!(
        hovered.layers[hovered_start..]
            .iter()
            .any(|layer| !layer.draw_range.is_empty())
    );
    assert!(key.matches(&hovered, hovered_start));
}

#[test]
fn shaded_ais_hover_capture_matches_full_vulkan_redraw() {
    let driver = crate::Graphic3dRenderDriver::new(crate::VulkanOptions {
        validation: true,
        device_index: None,
    })
    .unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = lit_sphere_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let _object = display_sphere(&context);
    view.auto_z_fit().unwrap();
    let rendered: Handle<crate::Graphic3dRenderView> = view.get().unwrap().view().cast();
    rendered.try_redraw().unwrap();
    let base = rendered.capture().unwrap();
    context.move_to(64, 64, &view, false).unwrap();
    assert!(context.has_detected());
    rendered.get().unwrap().redraw_immediate();
    rendered.get().unwrap().redraw_immediate();
    let restored = rendered.capture().unwrap();
    let immediate = rendered.last_frame_work();
    assert_ne!(restored.rgba, base.rgba);
    rendered.try_redraw().unwrap();
    let full = rendered.capture().unwrap();
    assert_eq!(restored.rgba, full.rgba);
    assert_eq!(restored.depth, full.depth);
    assert!(
        immediate.descriptor_sets_allocated < rendered.last_frame_work().descriptor_sets_allocated
    );
    assert!(driver.validation_errors().unwrap().is_empty());
}

fn lit_sphere_view(viewer: &V3dViewer) -> Handle<dyn V3dViewSource> {
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    let view = viewer.create_view().unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(128, 128);
    view.set_immediate_update(false);
    view.set_window(&window.cast(), 0).unwrap();
    let camera = view.camera();
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_center(&Point3::default()).unwrap();
    camera.set_eye(&Point3::new(20., 10., 10.)).unwrap();
    camera.set_scale(10.);
    view
}

fn display_sphere(context: &AisInteractiveContext) -> Handle<dyn AisInteractiveObject> {
    let mut sphere = BRepPrimAPIMakeSphere::new(1.).unwrap();
    let object = AisShape::new(sphere.shape().unwrap()).cast();
    context
        .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    object
}
