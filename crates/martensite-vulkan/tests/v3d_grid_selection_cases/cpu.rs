//! Actual source math and cursor mutation on the existing null graphics driver.
use super::*;
use crate::hatch_surface::scene::SourceViewOutput as _;
use martensite_tkservice::{graphic3d::Graphic3dCameraProjection, null_driver::NullGraphicDriver};

fn session(width: i32) -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, width, width).unwrap()
}

fn expect_point(actual: (f64, f64, f64), expected: [f64; 3]) {
    for (value, wanted) in [actual.0, actual.1, actual.2].into_iter().zip(expected) {
        assert!(
            value.is_finite() && (value - wanted).abs() <= 1e-7,
            "actual={actual:?}; expected {expected:?} with source absolute1e-7"
        );
    }
}

#[test]
fn translated_planes_preserve_actual_world_grid_coordinates() {
    let session = session(409);
    grid::prepare_shifted(&session).unwrap();
    expect_point(grid::world_grid(&session, 0.).unwrap(), [5., 5., 0.]);
    grid::plane(&session, [20., 20., 30.]).unwrap();
    expect_point(grid::world_grid(&session, 30.).unwrap(), [5., 5., 30.]);
}

#[test]
fn perspective_shifted_plane_keeps_actual_authored_grid_point() {
    let session = session(409);
    grid::prepare_shifted(&session).unwrap();
    grid::plane(&session, [20., 20., 30.]).unwrap();
    session
        .set_projection(Graphic3dCameraProjection::Perspective)
        .unwrap();
    grid::plane(&session, [10., 5., 10.]).unwrap();
    expect_point(grid::world_grid(&session, 10.).unwrap(), [5., 5., 10.]);
}

fn expect_pixel_pair(session: &ViewerSession, expected: [[f64; 3]; 2]) {
    for (top, point) in [false, true].into_iter().zip(expected) {
        expect_point(grid::pixel_orientation(session, top).unwrap(), point);
    }
}

#[test]
fn authored_four_hundred_pixel_projection_grid_results_are_computed() {
    let session = session(400);
    assert_eq!(session.source_window_size(), (400, 400));
    grid::prepare_pixel(&session).unwrap();
    expect_pixel_pair(&session, [[5., -5., 0.], [0., -5., 0.]]);
    grid::lift_pixel_grid(&session).unwrap();
    expect_pixel_pair(&session, [[15., -15., 10.], [0., -5., 10.]]);
}

#[test]
fn global_selection_reports_three_before_actual_cursor_removals() {
    let session = session(409);
    let report = selection::destructive_walk(session.context(), false).unwrap();
    assert_eq!(
        report,
        "\n No of currents = 3\n count is = 1\n count is = 2\n count is = 3"
    );
    selection::report_selection(&report, false);
    assert_eq!(selection::source_count(&report, false), Some(3));
    assert_eq!(session.context().nb_selected(), 0);
    assert!(!session.context().more_selected());
}

#[test]
fn local_selection_keeps_literal_percent_d_and_resets_actual_cursor() {
    let session = session(409);
    let report = selection::destructive_walk(session.context(), true).unwrap();
    assert_eq!(
        report,
        "\n No of selected = 3\n count is = %d1\n count is = %d2\n count is = %d3"
    );
    selection::report_selection(&report, true);
    assert_eq!(selection::source_count(&report, true), Some(3));
    assert_eq!(session.context().nb_selected(), 0);
    assert!(!session.context().more_selected());
}

#[test]
fn count_parser_requires_the_authored_header_and_number() {
    assert_eq!(
        selection::source_count("No of currents = 2", false),
        Some(2)
    );
    assert_eq!(selection::source_count("No of selected = 3", false), None);
    assert_eq!(
        selection::source_count("No of currents = unknown", false),
        None
    );
}

#[test]
fn numeric_labels_keep_source_coordinate_names() {
    assert_eq!(grid_check_name("world grid", "X", true), "world grid, X");
    assert_eq!(grid_check_name("pixel grid", "Y", false), "pixel grid");
    report_grid(
        "finite owned numeric report",
        (1., 2., 3.),
        [1., 2., 3.],
        false,
    );
}

#[allow(dead_code)] // The named recorder also exposes diagnostics unused by this control.
#[path = "../support/selection_frame_probe.rs"]
pub(super) mod final_line_probe;

fn final_line_scene() -> final_line_probe::SelectionFrameProbe {
    let probe = final_line_probe::SelectionFrameProbe::new();
    grid::prepare_shifted(&probe.session).unwrap();
    grid::plane(&probe.session, [20., 20., 30.]).unwrap();
    probe
        .session
        .set_projection(Graphic3dCameraProjection::Perspective)
        .unwrap();
    grid::plane(&probe.session, [10., 5., 10.]).unwrap();
    probe
}

fn require_final_line_witness(
    record: &crate::tie_edges::SubmittedDraws,
    frame: &martensite_vulkan::RenderFrame,
) {
    let derived =
        crate::tie_edges::final_line_witness::prepare(record.draws(), [409., 409.]).unwrap();
    assert_eq!(format!("{derived:?}"), format!("{:?}", frame.draws()));
    let lines = record.thin_lines(409., 409.).unwrap();
    let count: usize = frame
        .draws()
        .iter()
        .map(|draw| draw.indices().len() / 2)
        .sum();
    assert_eq!(lines.len(), count);
    assert!(!lines.is_empty());
}

#[test]
fn shifted_grid_derived_line_replay_matches_actual_injected_device_inputs() {
    let probe = final_line_scene();
    let view = probe
        .session
        .view()
        .get()
        .unwrap()
        .view()
        .cast::<martensite_vulkan::Graphic3dRenderView>();
    let record = crate::tie_edges::SubmittedDraws::install(&view);
    let (frame, _) = probe.snapshot();
    require_final_line_witness(&record, &frame);
}
