//! Dimension text and relation regressions of the V3d dimensions group, each in an isolated
//! worker: highlighted 2d and 3d text, flipped text, label layouts, moved text, custom values
//! and relations between selected sub-shapes.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_dimension_text_cases;
use std::error::Error;
use v3d_dimension_text_cases::DimensionKind;
use v3d_dimension_text_cases::{self as cases, FlipAxis};

fn run(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug24133_2() {
    run("bug24133_2", cases::angle_text_highlight);
}

#[test]
fn bug24133_3() {
    run("bug24133_3", || {
        cases::circle_text_highlight(DimensionKind::Radius)
    });
}

#[test]
fn bug24133_4() {
    run("bug24133_4", || {
        cases::circle_text_highlight(DimensionKind::Diameter)
    });
}

#[test]
fn bug24288_1() {
    run("bug24288_1", || cases::flipped_radius(FlipAxis::X));
}

#[test]
fn bug24288_2() {
    run("bug24288_2", || cases::flipped_radius(FlipAxis::Y));
}

#[test]
fn bug24288_3() {
    run("bug24288_3", || cases::flipped_radius(FlipAxis::Z));
}

#[test]
fn bug24351_1() {
    run("bug24351_1", cases::length_layouts);
}

#[test]
fn bug24351_2() {
    run("bug24351_2", || {
        cases::circle_layouts(DimensionKind::Diameter)
    });
}

#[test]
fn bug24351_3() {
    run("bug24351_3", || {
        cases::circle_layouts(DimensionKind::Radius)
    });
}

#[test]
fn bug24351_4() {
    run("bug24351_4", cases::angle_layouts);
}

#[test]
fn bug24389() {
    run("bug24389", cases::local_part_highlight);
}

#[test]
fn bug24412_1() {
    run("bug24412_1", cases::moved_diameter_text);
}

#[test]
fn bug24412_2() {
    run("bug24412_2", cases::moved_angle_text);
}

#[test]
fn bug26035_2() {
    // REQUIRED All: the refused dimension's message is expected output.
    let required =
        ["Error: dimension geometry is invalid, -length dimension can't be built on input shapes"];
    case_todo::run_case_requiring("bug26035_2", 1000, &required, cases::face_point_length);
}

#[test]
fn bug26056() {
    run("bug26056", cases::face_edge_lengths);
}

#[test]
fn bug27688() {
    run("bug27688", cases::custom_text_values);
}

#[test]
fn bug27919() {
    run("bug27919", cases::fixed_size_arrows);
}

#[test]
fn concentric() {
    run("concentric", cases::concentric_circles);
}

#[test]
fn equaldistance() {
    run("equaldistance", cases::equal_distances);
}

#[test]
fn equalradius() {
    run("equalradius", cases::equal_radii);
}

#[test]
fn fix() {
    run("fix", cases::fixed_edge);
}

#[test]
fn identic() {
    run("identic", cases::identic_shapes);
}

#[test]
fn offset() {
    run("offset", cases::offset_faces);
}

#[test]
fn parallel() {
    run("parallel", cases::parallel_edges);
}

#[test]
fn perpendicular() {
    run("perpendicular", cases::perpendicular_edges);
}

#[test]
fn symmetric() {
    run("symmetric", cases::symmetric_shapes);
}
