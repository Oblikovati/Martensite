use super::*;
use crate::{PrimitiveDraw, RenderLighting, RenderShading, RenderVertex, identity_clip};

fn lit_triangle(parts: &RecordCameraParts) -> PrimitiveDraw {
    let vertices = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]));
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        Default::default(),
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    let mut draw = PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip())
        .unwrap()
        .with_lighting(lighting)
        .unwrap();
    ReplayedFields::placed(parts, (identity_clip(), identity_clip()), false).patch(&mut draw);
    draw
}

#[test]
fn shared_geometry_keeps_first_template_order_and_each_placement() {
    let parts = RecordCameraParts::of(&Graphic3dCamera::default()).unwrap();
    let first = lit_triangle(&parts);
    let second = lit_triangle(&parts);
    let mut moved = first.clone();
    let mut world = identity_clip();
    world[12] = 2.;
    ReplayedFields::placed(&parts, (world, identity_clip()), false).patch(&mut moved);
    let split = split_placements(&[first, second, moved], &parts).unwrap();
    assert_eq!(split.first_placements, [0, 1]);
    assert_eq!(
        split
            .placements
            .iter()
            .map(|p| p.template)
            .collect::<Vec<_>>(),
        [0, 1, 0]
    );
    assert_eq!(split.placements[2].world, world);
}

#[test]
fn shared_geometry_with_different_recording_state_cannot_reuse_the_template() {
    let parts = RecordCameraParts::of(&Graphic3dCamera::default()).unwrap();
    let first = lit_triangle(&parts);
    let mut different = first.clone();
    different.lighting.as_mut().unwrap().normalize_fixed = true;
    assert!(split_placements(&[first, different], &parts).is_none());
}

#[test]
fn dormant_view_and_structure_planes_keep_placement_replay_ineligible() {
    for clipping in [ClipLocation::View, ClipLocation::Structure] {
        verify_dormant_clip_sequence(clipping);
    }
}

#[derive(Clone, Copy)]
enum ClipLocation {
    View,
    Structure,
}

fn verify_dormant_clip_sequence(location: ClipLocation) {
    use crate::immediate_redraw_fixture::{SceneExtractionOnly, display_box, lit_sphere_view};
    use martensite_tkv3d::{ais::AisInteractiveContext, v3d::V3dViewer};

    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 256).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = lit_sphere_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let _part = display_box(&context);
    view.auto_z_fit().unwrap();
    let rendered: Handle<Graphic3dRenderView> = view.get().unwrap().view().cast();
    assert!(placement_camera_policy(&rendered).is_some());
    let input = PlacementInputs::observe(&rendered).unwrap();
    let stamp = input.stamp(&rendered).unwrap();

    let plane = set_dormant_plane(&rendered, location);
    match location {
        ClipLocation::View => assert!(placement_camera_policy(&rendered).is_none()),
        ClipLocation::Structure => assert!(PlacementInputs::observe(&rendered).is_none()),
    }
    assert!(!stamp.current(&rendered));
    let before = rendered.scene_frame([128, 128]).unwrap();
    assert!(!before.draws.is_empty());
    assert!(before.draws.iter().all(|draw| draw.clipping().is_none()));
    let prior_seed = match location {
        ClipLocation::View => Some(single_draw_seed(&rendered)),
        ClipLocation::Structure => None,
    };

    plane.set_on(true).unwrap();
    if let Some((mut seed, stamp)) = prior_seed {
        assert!(
            stamp.current(&rendered),
            "plane toggle did not notify placement inputs"
        );
        assert!(
            seed.refresh(&rendered, [128, 128], &Some(stamp))
                .unwrap()
                .is_none()
        );
    }
    let after = rendered.scene_frame([128, 128]).unwrap();
    assert!(after.draws.iter().any(|draw| draw.clipping().is_some()));
    match location {
        ClipLocation::View => assert!(placement_camera_policy(&rendered).is_none()),
        ClipLocation::Structure => assert!(PlacementInputs::observe(&rendered).is_none()),
    }
}

// Model an already-admitted placement batch independently of a Vulkan device.
fn single_draw_seed(view: &Graphic3dRenderView) -> (PlacementBatch, PlacementStamp) {
    let stamp = PlacementInputs::observe(view).unwrap().stamp(view).unwrap();
    let (policy, camera) = BatchViewPolicy::capture_any_camera(view).unwrap();
    let parts = RecordCameraParts::of(camera.get().unwrap()).unwrap();
    let frame = crate::RenderFrame::new(128, 128, [0.; 4], vec![lit_triangle(&parts)]).unwrap();
    let split = split_placements(&frame.draws, &parts).unwrap();
    let seed = PlacementBatch::kept(
        view,
        &frame,
        split,
        (policy, parts.unit_axial_scale),
        stamp.clone(),
    );
    (seed, stamp)
}

fn set_dormant_plane(
    view: &Graphic3dRenderView,
    location: ClipLocation,
) -> Handle<Graphic3dClipPlane> {
    use martensite_tkernel::ncollection::Vec4;

    let plane = Handle::new(Graphic3dClipPlane::from_equation(Vec4::new(1., 0., 0., -2.)).unwrap());
    plane.set_on(false).unwrap();
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    assert!(sequence.append(plane.clone()));
    match location {
        ClipLocation::View => view.set_clip_planes(&sequence),
        ClipLocation::Structure => {
            let structures = view.structure_manager().displayed_structures();
            assert_eq!(structures.len(), 1);
            structures[0].set_clip_planes(&sequence);
        }
    }
    plane
}

#[test]
fn placement_replay_declines_an_inherited_alpha_cutoff() {
    use crate::immediate_redraw_fixture::{SceneExtractionOnly, lit_sphere_view};
    use crate::legacy_fragment_alpha::LegacyFragmentAlpha;
    use martensite_tkv3d::v3d::V3dViewer;

    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 256).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = lit_sphere_view(&viewer);
    let rendered: Handle<Graphic3dRenderView> = view.get().unwrap().view().cast();
    let (mut seed, stamp) = single_draw_seed(&rendered);
    assert!(
        seed.refresh(&rendered, [128, 128], &Some(stamp.clone()))
            .unwrap()
            .is_some()
    );
    let mut state = seed.seed.clone();
    state.attach_fragment_alpha(&rendered.fragment_alpha_feedback);
    state
        .fragment_alpha_context
        .unwrap()
        .commit(LegacyFragmentAlpha(Some(0.5)));
    assert!(stamp.current(&rendered));
    assert!(
        seed.refresh(&rendered, [128, 128], &Some(stamp))
            .unwrap()
            .is_none()
    );
}

#[test]
fn default_single_sample_raster_settings_admit_retained_placements() {
    let mut frame = crate::RenderFrame::new(128, 128, [0.; 4], Vec::new()).unwrap();
    let parameters = Graphic3dRenderingParams::new().unwrap();
    frame.raster =
        crate::transparency::RenderRasterSettings::from_parameters(&parameters, false).unwrap();
    assert!(frame.raster.alpha_to_coverage);
    assert!(placement_raster_compatible(&frame));
    frame.raster.requested_samples = 4;
    assert!(!placement_raster_compatible(&frame));
}
