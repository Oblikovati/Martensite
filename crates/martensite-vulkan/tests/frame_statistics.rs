//! Frame statistics of the Vulkan view against native OpenGL counters of the same scenes.
//! The expected values are the recorded counters for
//! `scene_mixed`: nine primitive-array objects and one text label, with every performance
//! counter on. See
//! the recorded controls
#[allow(dead_code)] // Each caller uses a subset of the shared scene support.
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/frame_statistics_scene.rs"]
mod statistics_scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsCounter as Counter, Graphic3dFrameStatsDict,
    Graphic3dFrameStatsMembers as _, Graphic3dRenderingParamsPerfCounters as Perf,
};
use recording_device::RecordingDevice;
use statistics_scene::{StatisticsScene, collect_every_frame};

fn text(value: &martensite_tkernel::tcollection::AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn recorded_scene() -> (StatisticsScene, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver =
        martensite_vulkan::Graphic3dRenderDriver::with_backend(device.clone(), 512).unwrap();
    (StatisticsScene::mixed(&driver), device)
}

/// Recorded counter values of scene_mixed (lines has no dictionary key in the recording).
const NATIVE_MIXED: [(Counter, usize); 14] = [
    (Counter::NbLayers, 5),
    (Counter::NbLayersNotCulled, 1),
    (Counter::NbStructs, 10),
    (Counter::NbStructsNotCulled, 10),
    (Counter::NbGroupsNotCulled, 10),
    (Counter::NbElemsNotCulled, 10),
    (Counter::NbElemsFillNotCulled, 6),
    (Counter::NbElemsLineNotCulled, 2),
    (Counter::NbElemsPointNotCulled, 1),
    (Counter::NbElemsTextNotCulled, 1),
    (Counter::NbTrianglesNotCulled, 30),
    (Counter::NbLinesNotCulled, 5),
    (Counter::NbPointsNotCulled, 5),
    (Counter::EstimatedBytesGeom, 1272),
];

#[test]
fn mixed_scene_counters_equal_native_opengl() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    for (counter, expected) in NATIVE_MIXED {
        assert_eq!(stats.counter_value(counter), expected, "{counter:?}");
    }
}

#[test]
fn statistic_dictionary_reports_the_vulkan_counters() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let mut dict = Graphic3dFrameStatsDict::default();
    scene.view.statistic_information_dict(&mut dict);
    let value = |key: &str| {
        dict.items()
            .find(|(name, _)| text(name) == key)
            .map(|(_, value)| text(value))
    };
    assert_eq!(value("Rendered triangles").as_deref(), Some("30"));
    assert_eq!(value("Rendered [text] arrays").as_deref(), Some("1"));
    let lines = text(&scene.view.statistic_information());
    assert!(
        lines.contains(" Triangles:    30"),
        "statistics text {lines:?}"
    );
}

#[test]
fn counters_follow_the_requested_collection_bits() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::STRUCTURES);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::NbStructsNotCulled), 10);
    // Native toCountGroups needs the groups bit or an element collection.
    assert_eq!(stats.counter_value(Counter::NbGroupsNotCulled), 0);
    assert_eq!(stats.counter_value(Counter::NbElemsNotCulled), 0);
    assert_eq!(stats.counter_value(Counter::EstimatedBytesGeom), 0);
}

#[test]
fn group_arrays_without_detail_skip_primitive_counts() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::GROUP_ARRAYS);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::NbElemsNotCulled), 10);
    assert_eq!(stats.counter_value(Counter::NbTrianglesNotCulled), 0);
}

/// Recorded scene_hidden: the label is erased from the view after it was rendered.
const NATIVE_HIDDEN: [(Counter, usize); 13] = [
    (Counter::NbLayers, 5),
    (Counter::NbLayersNotCulled, 1),
    (Counter::NbStructs, 10),
    (Counter::NbStructsNotCulled, 10),
    (Counter::NbGroupsNotCulled, 9),
    (Counter::NbElemsNotCulled, 9),
    (Counter::NbElemsFillNotCulled, 6),
    (Counter::NbElemsLineNotCulled, 2),
    (Counter::NbElemsPointNotCulled, 1),
    (Counter::NbElemsTextNotCulled, 0),
    (Counter::NbTrianglesNotCulled, 16),
    (Counter::NbPointsNotCulled, 5),
    // GlFrameStats::updateStructures keeps the hidden text's built buffers.
    (Counter::EstimatedBytesGeom, 1272),
];

#[test]
fn hidden_rendered_text_keeps_its_memory_like_native() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    scene.hide_text();
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    for (counter, expected) in NATIVE_HIDDEN {
        assert_eq!(stats.counter_value(counter), expected, "{counter:?}");
    }
}

#[test]
fn never_rendered_text_has_no_resident_buffers_like_native() {
    let (scene, _) = recorded_scene();
    scene.hide_text();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    // Recorded scene_neverrendered: a text label without an update, then erased from the view.
    assert_eq!(stats.counter_value(Counter::NbGroupsNotCulled), 9);
    assert_eq!(stats.counter_value(Counter::EstimatedBytesGeom), 600);
}

#[test]
fn update_interval_keeps_the_previous_data_frame() {
    let (scene, _) = recorded_scene();
    scene.view.change_rendering_params(&mut |params| {
        params.stats.collected_stats = Perf::ALL;
        params.stats.stats_update_interval = 3600.0;
    });
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::NbStructs), 0);
}

#[test]
fn failed_redraw_still_closes_the_frame() {
    let (scene, device) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    device
        .fail
        .store(true, std::sync::atomic::Ordering::Relaxed);
    assert!(scene.render_view().try_redraw().is_err());
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::NbStructs), 10);
}

#[test]
fn injected_backends_report_no_device_images() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::EstimatedBytesFbos), 0);
    assert_eq!(stats.counter_value(Counter::EstimatedBytesTextures), 0);
}

#[test]
fn frame_rate_and_timers_are_measured_per_slice() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    for _ in 0..3 {
        scene.render_view().try_redraw().unwrap();
    }
    let stats = scene.render_view().frame_stats();
    let frame = stats.last_data_frame();
    assert!(
        frame.frame_rate() > 0.0,
        "frame rate {}",
        frame.frame_rate()
    );
    assert!(stats.frame_duration() >= 0.0);
    use martensite_tkservice::graphic3d::Graphic3dFrameStatsTimer as Timer;
    assert!(frame[Timer::ElapsedFrame] > 0.0);
    assert!(frame[Timer::CpuCulling] >= 0.0);
}

#[test]
fn replaced_statistics_receive_later_frames() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    let replacement = martensite_vulkan::RenderFrameStats::new();
    scene.render_view().set_frame_stats(&replacement);
    scene.render_view().try_redraw().unwrap();
    assert_eq!(replacement.counter_value(Counter::NbStructs), 10);
}

#[test]
fn frame_update_detection_copies_the_last_frame() {
    let (scene, _) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    let mut previous = martensite_tkernel::standard::Handle::default();
    assert!(stats.is_frame_updated(&mut previous));
    assert!(!stats.is_frame_updated(&mut previous));
    scene.hide_text();
    scene.render_view().try_redraw().unwrap();
    assert!(stats.is_frame_updated(&mut previous));
}

#[test]
fn shown_statistics_append_the_overlay_after_the_scene() {
    let (scene, device) = recorded_scene();
    collect_every_frame(&scene.view, Perf::LAYERS);
    scene.render_view().try_redraw().unwrap();
    let plain = device.last_frame().draws().len();
    scene
        .view
        .change_rendering_params(&mut |params| params.stats.to_show_stats = true);
    scene.render_view().try_redraw().unwrap();
    let frame = device.last_frame();
    assert!(
        frame.draws().len() > plain,
        "{} <= {plain}",
        frame.draws().len()
    );
    let last = frame.layers().last().unwrap();
    assert_eq!(last.depth_policy(), (true, false, false));
}

#[test]
fn shown_statistics_do_not_enter_the_view_bounding_box() {
    // The statistics overlay draws outside the layer list, so MinMaxValues (and a view fit)
    // never see the overlay.
    let (scene, _device) = recorded_scene();
    collect_every_frame(&scene.view, Perf::ALL);
    scene.render_view().try_redraw().unwrap();
    let cview = scene.scene.cview();
    let plain = [false, true].map(|aux| cview.min_max_values(aux).unwrap());
    scene
        .view
        .change_rendering_params(&mut |params| params.stats.to_show_stats = true);
    scene.render_view().try_redraw().unwrap();
    scene.render_view().try_redraw().unwrap();
    let shown = [false, true].map(|aux| cview.min_max_values(aux).unwrap());
    for (before, after) in plain.iter().zip(&shown) {
        assert_eq!(format!("{before:?}"), format!("{after:?}"));
    }
}
