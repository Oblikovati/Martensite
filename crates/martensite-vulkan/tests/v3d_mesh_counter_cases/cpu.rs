//! Owned state fixtures pin the existing statistics query before its structural extraction.
use super::*;
use crate::primitive_shapes::sphere;
use martensite_tkservice::{
    graphic3d::Graphic3dRenderingParamsPerfCounters as Perf, null_driver::NullGraphicDriver,
};
use martensite_vulkan::Graphic3dRenderDriver;
#[path = "../support/recording_device.rs"]
pub(super) mod recording_device;

fn null_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}

#[test]
fn actual_auto_mesh_replaces_the_coarse_source_mesh() {
    let mut session = null_session();
    mesh::prepare_auto_mesh(&mut session).unwrap();
    let shape = sphere(0.5).unwrap();
    mesh::show_sphere(&mut session, &shape).unwrap();
    session.clear().unwrap();
    let coarse = mesh::rebuild_source_mesh(&shape).unwrap();
    let displayed = mesh::show_sphere(&mut session, &shape).unwrap();
    assert_ne!(displayed, coarse);
    println!("{{\"automatic\":true,\"coarse\":{coarse},\"displayed\":{displayed}}}");
}

#[test]
fn disabling_auto_mesh_preserves_actual_stored_triangle_count() {
    let mut session = null_session();
    mesh::prepare_auto_mesh(&mut session).unwrap();
    session.set_auto_triangulation(false);
    let shape = sphere(0.5).unwrap();
    let coarse = mesh::rebuild_source_mesh(&shape).unwrap();
    let displayed = mesh::show_sphere(&mut session, &shape).unwrap();
    assert_eq!(displayed, coarse);
    println!("{{\"automatic\":false,\"coarse\":{coarse},\"displayed\":{displayed}}}");
}

pub(super) fn recorded_session_with_device() -> (ViewerSession, recording_device::RecordingDevice) {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 2048).unwrap();
    (
        ViewerSession::with_view_size(&driver, 800, 400).unwrap(),
        backend,
    )
}

fn recorded_session() -> ViewerSession {
    recorded_session_with_device().0
}

#[test]
fn old_structure_counter_query_preserves_nondefault_flags_and_interval() {
    let mut session = recorded_session();
    counter::prepare_counter_scene(&mut session).unwrap();
    let view = session.view().view_base();
    view.change_rendering_params(&mut |params| {
        params.stats.collected_stats = Perf::POINTS;
        params.stats.stats_update_interval = 0.25;
    });
    assert_eq!(session.rendered_structures().unwrap(), "4");
    let actual = view.rendering_params().stats;
    assert_eq!(actual.collected_stats.0, Perf::POINTS.0);
    assert_eq!(actual.stats_update_interval.to_bits(), 0.25f32.to_bits());
}

fn assert_counter_settings(session: &ViewerSession) {
    let actual = session.view().view_base().rendering_params().stats;
    assert_eq!(actual.collected_stats.0, Perf::STRUCTURES.0);
    assert_eq!(actual.stats_update_interval.to_bits(), 0.125f32.to_bits());
}

type CounterQuery = fn(&ViewerSession) -> Result<String, Failure>;
const SOURCE_QUERIES: [(CounterQuery, &str); 2] = [
    (ViewerSession::rendered_points, "3"),
    (ViewerSession::rendered_triangles, "2"),
];

#[test]
fn actual_culled_point_and_triangle_queries_restore_each_source_setting() {
    let (mut session, backend) = recorded_session_with_device();
    counter::prepare_counter_scene(&mut session).unwrap();
    session
        .view()
        .view_base()
        .change_rendering_params(&mut |params| {
            params.stats.collected_stats = Perf::STRUCTURES;
            params.stats.stats_update_interval = 0.125;
        });
    for (query, expected) in SOURCE_QUERIES {
        let before = backend.frames.read().len();
        assert_eq!(query(&session).unwrap(), expected);
        assert_eq!(backend.frames.read().len(), before + 1);
        assert_counter_settings(&session);
    }
    assert_eq!(backend.last_frame().extent(), [800, 400]);
}

#[test]
fn profiler_parse_keeps_integer_text_and_refuses_missing_or_malformed_counts() {
    assert_eq!(counter::profiler_integer("3 ", "points").unwrap(), 3);
    assert_eq!(counter::profiler_integer("2 ", "triangles").unwrap(), 2);
    assert!(counter::profiler_integer("", "points").is_err());
    let error = counter::profiler_integer("bad_count", "triangles").unwrap_err();
    assert!(error.to_string().contains("bad_count"));
}
