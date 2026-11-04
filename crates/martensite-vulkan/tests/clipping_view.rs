//! Live view/structure state is snapshotted at redraw, with native override and chain semantics.
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{ncollection::Vec4, standard::Handle};
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::*;
use recording_device::RecordingDevice;
fn setup() -> (scene::ViewerScene, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
    (scene::ViewerScene::new(&driver), device)
}
fn plane(equation: [f64; 4]) -> Handle<Graphic3dClipPlane> {
    Handle::new(
        Graphic3dClipPlane::from_equation(Vec4::new(
            equation[0],
            equation[1],
            equation[2],
            equation[3],
        ))
        .unwrap(),
    )
}
fn sequence(plane: &Handle<Graphic3dClipPlane>) -> Handle<Graphic3dSequenceOfHClipPlane> {
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    sequence.append(plane.clone());
    sequence
}
#[test]
fn view_clip_planes_return_the_set_sequence() {
    let (scene, _) = setup();
    assert!(
        scene
            .cview()
            .clip_planes()
            .get()
            .is_none_or(|planes| planes.is_empty())
    );
    let planes = sequence(&plane([1., 0., 0., 0.]));
    scene.cview().set_clip_planes(&planes);
    assert!(scene.cview().clip_planes() == planes);
    let reference = scene.cview().clip_planes_ref();
    assert!(
        reference
            .observe()
            .observe_as::<Graphic3dSequenceOfHClipPlane>()
            .clone()
            == planes
    );
}
/// `GlView::Lights`: the view returns the light set it was given.
#[test]
fn view_lights_return_the_set_light_set() {
    let (scene, _) = setup();
    let lights = Handle::new(Graphic3dLightSet::default());
    let ambient = Handle::new(Graphic3dCLight::new(Graphic3dTypeOfLightSource::Ambient));
    lights.add(&ambient).unwrap();
    scene.cview().set_lights(&lights);
    assert!(scene.cview().lights() == lights);
    assert_eq!(scene.cview().lights().get().unwrap().extent(), 1);
}
#[test]
fn view_planes_follow_live_equations_flags_and_chain_links_without_mutating_prior_frames() {
    let (scene, device) = setup();
    let first = plane([1., 0., 0., 0.]);
    let second = plane([0., 1., 0., 2.]);
    second.set_on(false).unwrap();
    Graphic3dClipPlane::set_chain_next_plane(&first, second).unwrap();
    scene.cview().set_clip_planes(&sequence(&first));
    scene.render_view().try_redraw().unwrap();
    let before = device.last_frame();
    assert_eq!(
        before.draws()[0].clipping().unwrap().chains(),
        &[vec![[1., 0., 0., 0.], [0., 1., 0., 2.]]]
    );
    first.set_equation(Vec4::new(1., 0., 0., 3.)).unwrap();
    scene.render_view().try_redraw().unwrap();
    assert_eq!(
        device.last_frame().draws()[0].clipping().unwrap().chains()[0][0][3],
        3.
    );
    assert_eq!(before.draws()[0].clipping().unwrap().chains()[0][0][3], 0.);
    first.set_on(false).unwrap();
    scene.render_view().try_redraw().unwrap();
    assert!(device.last_frame().draws()[0].clipping().is_none());
}
#[test]
fn local_sequences_intersect_global_chains_and_even_empty_override_replaces_globals() {
    let (scene, device) = setup();
    scene
        .cview()
        .set_clip_planes(&sequence(&plane([1., 0., 0., 0.])));
    let local = sequence(&plane([0., 1., 0., 0.]));
    scene.structure.set_clip_planes(&local);
    scene.render_view().try_redraw().unwrap();
    assert_eq!(
        device.last_frame().draws()[0]
            .clipping()
            .unwrap()
            .chains()
            .len(),
        2
    );
    local.set_override_global(true);
    scene.render_view().try_redraw().unwrap();
    assert_eq!(
        device.last_frame().draws()[0].clipping().unwrap().chains(),
        &[vec![[0., 1., 0., 0.]]]
    );
    local.clear();
    scene.render_view().try_redraw().unwrap();
    assert!(device.last_frame().draws()[0].clipping().is_none());
}
#[test]
fn world_plane_coordinates_include_structure_and_group_transforms() {
    let (scene, device) = setup();
    scene
        .cview()
        .set_clip_planes(&sequence(&plane([1., 0., 0., 0.])));
    scene
        .structure
        .set_transformation(&Handle::new(martensite_tkmath::toploc::Datum3::new(
            Transform3::translation_vector(Vector3::new(2., 0., 0.)),
        )));
    scene
        .group
        .set_transformation(&Transform3::translation_vector(Vector3::new(0., 3., 0.)));
    scene.render_view().try_redraw().unwrap();
    let frame = device.last_frame();
    let model = frame.draws()[0].clipping().unwrap().world_from_object();
    assert_eq!(&model[12..16], &[2., 3., 0., 1.]);
}
#[test]
fn invalid_planes_fail_redraw_and_sphere_generated_cap_textures_render() {
    let (scene, _) = setup();
    scene.render_view().try_redraw().unwrap();
    scene.cview().set_clip_planes(&sequence(&Handle::default()));
    assert!(scene.render_view().try_redraw().is_err());
    assert!(scene.render_view().capture().is_err());
    let cap = plane([1., 0., 0., 0.]);
    cap.set_capping(true);
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(Handle::new(
        martensite_tkservice::image::ImagePixMap::default(),
    )));
    texture.get_params().set_gen_mode(
        Graphic3dTypeOfTextureMode::Sphere,
        martensite_tkernel::ncollection::Vec4::new(1., 0., 0., 0.),
        martensite_tkernel::ncollection::Vec4::new(0., 1., 0., 0.),
    );
    cap.capping_aspect().set_texture_map(texture.cast());
    cap.capping_aspect().set_texture_map_on(true);
    scene.group.set_closed(true);
    scene.cview().set_clip_planes(&sequence(&cap));
    // The programmable pipeline ignores GenMode (the recorded sphere_texgen capture), so a
    // Sphere-generated cap texture renders like a manual one.
    scene.render_view().try_redraw().unwrap();
    cap.set_on(false).unwrap();
    scene.render_view().try_redraw().unwrap();
}

#[test]
fn persistent_structure_uses_global_planes_only_for_its_world_anchor() {
    let (scene, device) = setup();
    scene
        .view
        .set_clip_planes(&sequence(&plane([1., 0., 0., 0.])));
    for (x, visible) in [(1., true), (-1., false)] {
        scene.structure.set_transform_persistence(
            &Handle::new(
                Graphic3dTransformPers::with_point(
                    Graphic3dTransModeFlags::ZOOM_PERS,
                    &martensite_tkmath::gp::Point3::new(x, 0., 0.),
                )
                .unwrap(),
            )
            .cast(),
        );
        scene.render_view().try_redraw().unwrap();
        let frame = device.last_frame();
        assert_eq!(!frame.draws().is_empty(), visible);
        assert!(frame.draws().iter().all(|draw| draw.clipping().is_none()));
    }
}
#[test]
fn local_override_retains_fragment_clipping_for_a_persistent_structure() {
    let (scene, device) = setup();
    scene
        .view
        .set_clip_planes(&sequence(&plane([1., 0., 0., 0.])));
    scene.structure.set_transform_persistence(
        &Handle::new(
            Graphic3dTransformPers::with_point(
                Graphic3dTransModeFlags::ZOOM_PERS,
                &martensite_tkmath::gp::Point3::new(-1., 0., 0.),
            )
            .unwrap(),
        )
        .cast(),
    );
    let local = sequence(&plane([0., 1., 0., 0.]));
    local.set_override_global(true);
    scene.structure.set_clip_planes(&local);
    scene.render_view().try_redraw().unwrap();
    assert_eq!(
        device.last_frame().draws()[0].clipping().unwrap().chains(),
        &[vec![[0., 1., 0., 0.]]]
    );
}
#[test]
fn bounding_box_culls_the_entire_structure_and_its_check_can_be_disabled() {
    let (scene, device) = setup();
    scene.structure.calculate_bound_box();
    assert!(scene.structure.cstructure().bounding_box().is_valid());
    scene
        .view
        .set_clip_planes(&sequence(&plane([1., 0., 0., -20.])));
    scene.render_view().try_redraw().unwrap();
    assert!(device.last_frame().draws().is_empty());
    scene.structure.cstructure().set_bnd_box_clip_check(false);
    scene.render_view().try_redraw().unwrap();
    assert!(!device.last_frame().draws().is_empty());
}
#[test]
fn group_persistence_clears_model_translation_before_fragment_clipping() {
    let (scene, device) = setup();
    scene
        .view
        .set_clip_planes(&sequence(&plane([1., 0., 0., 0.])));
    scene
        .group
        .set_transformation(&Transform3::translation_vector(Vector3::new(2., 3., 0.)));
    scene.group.set_transform_persistence(
        &Handle::new(
            Graphic3dTransformPers::with_point(
                Graphic3dTransModeFlags::ZOOM_PERS,
                &martensite_tkmath::gp::Point3::new(1., 0., 0.),
            )
            .unwrap(),
        )
        .cast(),
    );
    scene.render_view().try_redraw().unwrap();
    assert_eq!(
        &device.last_frame().draws()[0]
            .clipping()
            .unwrap()
            .world_from_object()[12..16],
        &[0., 0., 0., 1.]
    );
}

#[test]
fn recalculated_empty_bounds_are_culled_even_without_a_clip_plane() {
    let (scene, device) = setup();
    scene.group.set_transform_persistence(
        &Handle::new(
            Graphic3dTransformPers::with_point(
                Graphic3dTransModeFlags::ZOOM_PERS,
                &martensite_tkmath::gp::Point3::new(1., 0., 0.),
            )
            .unwrap(),
        )
        .cast(),
    );
    scene.structure.calculate_bound_box();
    assert!(!scene.structure.cstructure().bounding_box().is_valid());
    scene.render_view().try_redraw().unwrap();
    assert!(device.last_frame().draws().is_empty());
    scene.view.change_rendering_params(&mut |params| {
        params.ray_tracing.frustum_culling_state = Graphic3dRenderingParamsFrustumCulling::Off
    });
    scene.render_view().try_redraw().unwrap();
    assert!(!device.last_frame().draws().is_empty());
}
