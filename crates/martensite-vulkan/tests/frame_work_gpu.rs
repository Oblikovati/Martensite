//! Device work counters of redraws on a real Vulkan device with validation: what a frame
//! created, moved and waited for (phase 0 of docs/features/vulkan-occt-parity/PLAN.md).
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)] // Each caller uses a subset of the shared scene support.
#[path = "support/frame_statistics_scene.rs"]
mod statistics_scene;
#[allow(dead_code)] // This regression uses the ordinary texture case from the shared fixture.
#[path = "support/surface_texture_scene.rs"]
mod surface_texture_scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{Graphic3dRenderDriver, RenderDeviceWork, RenderWorkTimes};
use statistics_scene::StatisticsScene;
use std::time::Duration;

/// The 64x64 test window: colour and depth are read back with four bytes per pixel each.
const READBACK_BYTES: u64 = 2 * 64 * 64 * 4;

fn device_scene() -> (StatisticsScene, Handle<Graphic3dRenderDriver>) {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    (StatisticsScene::mixed(&driver), driver)
}

fn assert_clean_validation(driver: &Graphic3dRenderDriver) {
    let messages = driver.validation_errors().unwrap();
    assert!(messages.is_empty(), "validation messages {messages:?}");
}

#[test]
fn alternating_live_view_extents_reuse_their_frame_attachments() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let square = scene::ViewerScene::new(&driver);
    let wide = scene::ViewerScene::new(&driver);
    wide.window.set_size(92, 72);
    wide.view.must_be_resized();
    for (scene, extent) in [(&square, [64, 64]), (&wide, [92, 72])] {
        let view = scene.render_view();
        view.try_redraw().unwrap();
        let capture = view.capture().unwrap();
        assert_eq!([capture.width, capture.height], extent);
    }
    for view in [
        square.render_view(),
        wide.render_view(),
        square.render_view(),
    ] {
        view.try_redraw().unwrap();
        assert_eq!(view.last_frame_work().images_created, 0);
    }
    assert_clean_validation(&driver);
}

/// The counts of `work` without its times, which differ on every redraw.
fn counts(work: RenderDeviceWork) -> RenderDeviceWork {
    RenderDeviceWork {
        times: RenderWorkTimes::default(),
        ..work
    }
}

#[test]
fn a_redraw_reports_the_resources_and_bytes_it_used() {
    let (scene, driver) = device_scene();
    scene.render_view().try_redraw().unwrap();
    let work = scene.render_view().last_frame_work();
    assert_eq!(work.redraws, 1, "{work:?}");
    assert!(work.buffers_created > 0, "{work:?}");
    assert!(work.uploaded_bytes > 0, "{work:?}");
    assert!(work.submissions >= 1, "{work:?}");
    // The frame stays on the device until a capture asks for it (#594).
    assert_eq!(work.read_back_bytes, 0, "{work:?}");
    assert_clean_validation(&driver);
}

#[test]
fn a_redraw_returns_before_its_frame_finishes() {
    let (scene, driver) = device_scene();
    let view = scene.render_view();
    view.try_redraw().unwrap();
    driver.finish_frames().unwrap();
    view.try_redraw().unwrap();
    let work = view.last_frame_work();
    // An empty flight has room, so the redraw submits and waits for no fence (#594).
    assert_eq!(work.times.waiting, Duration::ZERO, "{work:?}");
    assert_eq!(work.submissions, 1, "{work:?}");
    let before = driver.device_work();
    driver.finish_frames().unwrap();
    let finished = driver.device_work().since(&before);
    assert!(finished.times.waiting > Duration::ZERO, "{finished:?}");
    assert_clean_validation(&driver);
}

#[test]
fn the_first_capture_reads_the_frame_back_once() {
    let (scene, driver) = device_scene();
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let before = driver.device_work();
    let first = view.capture().unwrap();
    let read = driver.device_work().since(&before);
    assert!(read.read_back_bytes >= READBACK_BYTES, "{read:?}");
    assert_eq!(read.redraws, 0, "{read:?}");
    let after_first = driver.device_work();
    let again = view.capture().unwrap();
    let repeated = driver.device_work().since(&after_first);
    assert_eq!(repeated.read_back_bytes, 0, "{repeated:?}");
    assert_eq!(repeated.submissions, 0, "{repeated:?}");
    assert_eq!(again.rgba, first.rgba);
    assert_eq!(again.depth, first.depth);
    assert_clean_validation(&driver);
}

#[test]
fn the_view_share_is_the_device_difference_over_the_redraw() {
    let (scene, driver) = device_scene();
    let before = driver.device_work();
    scene.render_view().try_redraw().unwrap();
    let device = driver.device_work().since(&before);
    assert_eq!(device, scene.render_view().last_frame_work());
}

#[test]
fn phase_times_fit_inside_the_redraw() {
    let (scene, _driver) = device_scene();
    scene.render_view().try_redraw().unwrap();
    let times = scene.render_view().last_frame_work().times;
    let phases = times.scene
        + times.upload
        + times.recording
        + times.submission
        + times.waiting
        + times.readback;
    assert!(phases <= times.redraw, "{times:?}");
    assert!(times.scene > Duration::ZERO, "{times:?}");
    assert!(times.upload > Duration::ZERO, "{times:?}");
}

#[test]
fn counts_belong_to_one_redraw_and_do_not_accumulate() {
    let (scene, _driver) = device_scene();
    let view = scene.render_view();
    // The first redraw also creates the pipelines and glyph images later frames reuse; the
    // second creates the attachments and pages of the second frame in flight (#594).
    view.try_redraw().unwrap();
    view.try_redraw().unwrap();
    view.try_redraw().unwrap();
    let third = counts(view.last_frame_work());
    view.try_redraw().unwrap();
    assert_eq!(counts(view.last_frame_work()), third);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn orbiting_a_live_triangle_keeps_geometry_on_the_device() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let first = view.capture().unwrap();
    let uploaded = view.last_frame_work();
    assert!(uploaded.geometry_uploaded_bytes > 0, "{uploaded:?}");
    assert!(uploaded.geometry_buffers_created > 0, "{uploaded:?}");

    // The second frame in flight creates its own attachments and upload page once (#594).
    view.try_redraw().unwrap();
    scene
        .view
        .camera()
        .set_eye(&martensite_tkmath::gp::Point3::new(7., 0., 19.))
        .unwrap();
    view.try_redraw().unwrap();
    assert_ne!(view.capture().unwrap().rgba, first.rgba);
    let orbit = view.last_frame_work();
    assert_eq!(orbit.geometry_uploaded_bytes, 0, "{orbit:?}");
    assert_eq!(orbit.geometry_buffers_created, 0, "{orbit:?}");
    // Phase 1 acceptance: a static orbit creates no buffer at all and reads nothing back.
    assert_eq!(orbit.buffers_created, 0, "{orbit:?}");
    assert_eq!(orbit.read_back_bytes, 0, "{orbit:?}");
    assert_clean_validation(&driver);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn ordinary_textured_orbit_retains_geometry_until_uv_transform_changes() {
    use martensite_tkernel::ncollection::Vec2;
    use martensite_tkservice::graphic3d::{
        Graphic3dTextureRootMembers, Graphic3dTypeOfTextureFilter,
    };
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let fixture = surface_texture_scene::SurfaceTextureScene::with_mode(&driver, "nearest");
    let view = fixture.scene.render_view();
    view.try_redraw().unwrap();
    assert!(view.last_frame_work().geometry_uploaded_bytes > 0);
    let first = view.capture().unwrap();
    fixture
        .scene
        .view
        .camera()
        .set_eye(&martensite_tkmath::gp::Point3::new(7., 0., 19.))
        .unwrap();
    view.try_redraw().unwrap();
    let orbit = view.capture().unwrap();
    assert_ne!(orbit.rgba, first.rgba);
    assert_no_geometry_work(view.last_frame_work());
    let parameters = fixture.texture.get_params();
    parameters.set_filter(Graphic3dTypeOfTextureFilter::Bilinear);
    view.try_redraw().unwrap();
    let filtered = view.capture().unwrap();
    assert_ne!(filtered.rgba, orbit.rgba);
    assert_no_geometry_work(view.last_frame_work());
    parameters.set_translation(Vec2::new(0.25, 0.));
    view.try_redraw().unwrap();
    assert!(view.last_frame_work().geometry_uploaded_bytes > 0);
    assert_ne!(view.capture().unwrap().rgba, filtered.rgba);
    assert_clean_validation(&driver);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
fn assert_no_geometry_work(work: RenderDeviceWork) {
    assert_eq!(work.geometry_uploaded_bytes, 0, "{work:?}");
    assert_eq!(work.geometry_buffers_created, 0, "{work:?}");
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn a_mutable_triangle_transfers_only_the_edited_vertex_record() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let array = add_mutable_triangle(&scene);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let first = view.capture().unwrap();

    array.set_vertice(3, 0., 2., 0.).unwrap();
    view.try_redraw().unwrap();
    let update = view.last_frame_work();
    assert_ne!(view.capture().unwrap().rgba, first.rgba);
    assert_eq!(update.geometry_uploaded_bytes, 32, "{update:?}");
    assert_eq!(update.geometry_buffers_created, 1, "{update:?}");
    assert_clean_validation(&driver);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
fn add_mutable_triangle(
    scene: &scene::ViewerScene,
) -> Handle<martensite_tkservice::graphic3d::Graphic3dArrayOfPrimitives> {
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, Graphic3dArrayOfPrimitives, Graphic3dGroupMembers,
        Graphic3dTypeOfPrimitiveArray,
    };
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(
            Graphic3dTypeOfPrimitiveArray::Triangles,
            3,
            0,
            3,
            GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE,
        )
        .unwrap(),
    );
    for (index, [x, y]) in (1..=3).zip([[-8., -8.], [8., -8.], [0., 8.]]) {
        array.set_vertice(index, x, y, 0.).unwrap();
    }
    array.add_edges3(1, 2, 3).unwrap();
    scene.group.clear(true);
    scene
        .group
        .set_group_primitives_aspect(&scene::unlit_style([1., 0., 0., 1.]));
    scene.group.add_primitive_array(&array.upcast(), true);
    array
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn a_mutable_index_transfers_one_device_word_and_matches_a_fresh_upload() {
    use martensite_tkservice::graphic3d::{
        Graphic3dIndexBufferMembers, Graphic3dMutableIndexBuffer,
    };
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let array = add_indexed_triangle(&scene, 2, 8., false);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let first = view.capture().unwrap();
    let indices = array.indices().cast::<Graphic3dMutableIndexBuffer>();
    assert!(indices.set_index(1, 3));
    indices.invalidate_indices(1, 1).unwrap();
    view.try_redraw().unwrap();
    let update = view.last_frame_work();
    let edited = view.capture().unwrap();
    assert_ne!(edited.rgba, first.rgba);
    assert_eq!(update.geometry_uploaded_bytes, 4, "{update:?}");
    assert_eq!(update.geometry_buffers_created, 1, "{update:?}");
    assert_matches_fresh_indexed_triangle(&scene, edited, 8.);
    assert_clean_validation(&driver);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn a_mutable_array_transfers_only_one_vertex_and_one_index_together() {
    use martensite_tkservice::graphic3d::Graphic3dIndexBufferMembers;
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let array = add_indexed_triangle(&scene, 2, 8., true);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let first = view.capture().unwrap();
    array.set_vertice(3, -8., 2., 0.).unwrap();
    assert!(array.indices().set_index(1, 3));
    view.try_redraw().unwrap();
    let update = view.last_frame_work();
    let edited = view.capture().unwrap();
    assert_ne!(edited.rgba, first.rgba);
    assert_eq!(update.geometry_uploaded_bytes, 36, "{update:?}");
    assert_eq!(update.geometry_buffers_created, 2, "{update:?}");
    assert_matches_fresh_indexed_triangle(&scene, edited, 2.);
    assert_clean_validation(&driver);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
fn assert_matches_fresh_indexed_triangle(
    scene: &scene::ViewerScene,
    edited: martensite_vulkan::FrameCapture,
    third_y: f32,
) {
    let _array = add_indexed_triangle(scene, 4, third_y, false);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    assert!(view.last_frame_work().geometry_uploaded_bytes > 4);
    let fresh = view.capture().unwrap();
    assert_eq!(fresh.rgba, edited.rgba);
    assert_eq!(fresh.depth, edited.depth);
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
fn add_indexed_triangle(
    scene: &scene::ViewerScene,
    middle_vertex: i32,
    third_y: f32,
    mutable_vertices: bool,
) -> Handle<martensite_tkservice::graphic3d::Graphic3dArrayOfPrimitives> {
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE,
        Graphic3dArrayOfPrimitives, Graphic3dGroupMembers, Graphic3dTypeOfPrimitiveArray as Kind,
    };
    let mut flags = GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE;
    if mutable_vertices {
        flags |= GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE;
    }
    let array =
        Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 4, 0, 3, flags).unwrap());
    let positions = [[-8., -8.], [8., -8.], [-8., third_y], [8., 8.]];
    for (rank, [x, y]) in (1..).zip(positions) {
        array.set_vertice(rank, x, y, 0.).unwrap();
    }
    array.add_edges3(1, middle_vertex, 3).unwrap();
    scene.group.clear(true);
    scene
        .group
        .set_group_primitives_aspect(&scene::unlit_style([1., 0., 0., 1.]));
    scene.group.add_primitive_array(&array.upcast(), true);
    array
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics"
)))]
#[test]
fn live_immediate_redraw_restores_main_color_and_depth_then_reuses_its_target_pair() {
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ZLAYER_ID_TOPMOST, Graphic3dGroupMembers, Graphic3dStructure,
        Graphic3dStructureMembers, Graphic3dStructureSource,
    };

    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let base = view.capture().unwrap();
    scene.cview().redraw_immediate();
    assert_eq!(view.capture().unwrap().rgba, base.rgba);

    let overlay =
        Graphic3dStructure::new(&scene.viewer.structure_manager(), &Handle::default()).unwrap();
    let group = overlay.new_group().unwrap();
    group.set_group_primitives_aspect(&scene::unlit_style([0., 1., 0., 1.]));
    group.add_primitive_array(&scene.vertices.upcast(), true);
    overlay.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOPMOST);
    overlay.display();

    scene.cview().redraw_immediate();
    let restored = view.capture().unwrap();
    assert_ne!(restored.rgba, base.rgba);
    // The first restore separates the transfer destination from the retained main pair.
    assert_eq!(view.last_frame_work().images_created, 2);
    // A restore while the previous one is in flight gets its own destination pair once (#594).
    scene.cview().redraw_immediate();
    scene.cview().redraw_immediate();
    assert_eq!(view.last_frame_work().images_created, 0);

    view.try_redraw().unwrap();
    let full = view.capture().unwrap();
    assert_eq!(restored.rgba, full.rgba);
    assert_eq!(restored.depth, full.depth);
    assert_clean_validation(&driver);
}
