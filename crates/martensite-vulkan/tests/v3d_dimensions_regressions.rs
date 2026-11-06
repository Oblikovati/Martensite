//! Dimension regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_dimensions_cases;
use std::error::Error;
use v3d_dimensions_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn angle180() {
    run("angle180", cases::straight_angle);
}

#[test]
fn bug25403() {
    run("bug25403", cases::valued_dimensions);
}

#[test]
fn bug26317() {
    run("bug26317", cases::face_pair_length);
}

#[test]
fn bug26035_1() {
    run("bug26035_1", cases::vertex_pair_length);
}

#[test]
fn bug26035_3() {
    run("bug26035_3", || {
        let operand = cases::EdgeOperand {
            vertex_shape: true,
            point_first: false,
        };
        cases::edge_vertex_length(
            operand,
            "CR26035: Drawing dimension between edge and vertex",
            "bug26035_3.png",
        )
    });
}

#[test]
fn bug26035_4() {
    run("bug26035_4", || {
        let operand = cases::EdgeOperand {
            vertex_shape: true,
            point_first: true,
        };
        cases::edge_vertex_length(operand, BUG26035_4, "bug26035_4.png")
    });
}

#[test]
fn bug26035_5() {
    run("bug26035_5", || {
        let operand = cases::EdgeOperand {
            vertex_shape: false,
            point_first: false,
        };
        cases::edge_vertex_length(operand, BUG26035_5, "bug26035_5.png")
    });
}

#[test]
fn bug26035_6() {
    run("bug26035_6", || {
        let operand = cases::EdgeOperand {
            vertex_shape: false,
            point_first: true,
        };
        cases::edge_vertex_length(operand, BUG26035_6, "bug26035_6.png")
    });
}

#[test]
fn bug24133_1() {
    run("bug24133_1", cases::highlighted_length);
}

#[test]
fn lengthparam() {
    run("lengthparam", || {
        let variant = cases::LengthDirections {
            reversed_points: false,
            reversed_flyout: false,
            reversed_directions: false,
        };
        cases::length_directions(variant, "lengthparam.png")
    });
}

#[test]
fn lengthparam_reversed() {
    run("lengthparam_reversed", || {
        let variant = cases::LengthDirections {
            reversed_points: true,
            reversed_flyout: false,
            reversed_directions: false,
        };
        cases::length_directions(variant, "lengthparam_reversed.png")
    });
}

#[test]
fn lengthparam_flyout_reversed() {
    run("lengthparam_flyout_reversed", || {
        let variant = cases::LengthDirections {
            reversed_points: false,
            reversed_flyout: true,
            reversed_directions: false,
        };
        cases::length_directions(variant, "lengthparam_flyout_reversed.png")
    });
}

#[test]
fn lengthparam_reversed_direction() {
    run("lengthparam_reversed_direction", || {
        let variant = cases::LengthDirections {
            reversed_points: false,
            reversed_flyout: false,
            reversed_directions: true,
        };
        cases::length_directions(variant, "lengthparam_reversed_direction.png")
    });
}

#[test]
fn lengthparam_reversed_flyout_reversed() {
    run("lengthparam_reversed_flyout_reversed", || {
        let variant = cases::LengthDirections {
            reversed_points: true,
            reversed_flyout: true,
            reversed_directions: false,
        };
        cases::length_directions(variant, "lengthparam_reversed_flyout_reversed.png")
    });
}

#[test]
fn bug24293() {
    run("bug24293", cases::highlighted_flyout);
}

#[test]
fn bug24374() {
    run("bug24374", cases::flipped_edge_highlight);
}

#[test]
fn bug25549_1() {
    run("bug25549_1", || {
        let title =
            "Tests case of two edges with the same line but different end points parameters";
        cases::collinear_angle(false, title, "bug25549_1.png")
    });
}

#[test]
fn bug25549_2() {
    run("bug25549_2", || {
        cases::collinear_angle(
            true,
            "Tests case of two edges with opposite lines directions",
            "bug25549_2.png",
        )
    });
}

#[test]
fn bug27692() {
    run("bug27692", cases::angle_arrows);
}

#[test]
fn bug32351() {
    run("bug32351", cases::circular_edge_lengths);
}

const BUG26035_4: &str = "CR26035: Drawing dimension between vertex and edge";
const BUG26035_5: &str = "CR26035: Drawing dimension between edge and point";
const BUG26035_6: &str = "CR26035: Drawing dimension between point and edge";
