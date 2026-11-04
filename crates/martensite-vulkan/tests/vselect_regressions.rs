//! Selection regressions of the selection group, each in an isolated worker under its policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_cases;
use std::error::Error;
use vselect_cases as cases;

/// The selection group runs every regression under a 300 s CPU limit.
fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case_within(identity, 300, body);
}

#[test]
fn vertex_a2() {
    run("vertex_a2", cases::vertex::vertex_a2);
}

#[test]
fn vertex_a3() {
    run("vertex_a3", cases::vertex::vertex_a3);
}

#[test]
fn vertex_a4() {
    run("vertex_a4", cases::vertex::vertex_a4);
}

#[test]
fn vertex_a5() {
    run("vertex_a5", cases::vertex::vertex_a5);
}

#[test]
fn vertex_a6() {
    run("vertex_a6", cases::vertex::vertex_a6);
}

#[test]
fn vertex_a8() {
    run("vertex_a8", cases::vertex::vertex_a8);
}

#[test]
fn vertex_b1() {
    run("vertex_b1", cases::vertex::vertex_b1);
}

#[test]
fn vertex_b2() {
    run("vertex_b2", cases::vertex::vertex_b2);
}

#[test]
fn vertex_b4() {
    run("vertex_b4", cases::vertex::vertex_b4);
}

#[test]
fn vertex_b5() {
    run("vertex_b5", cases::vertex::vertex_b5);
}

#[test]
fn vertex_b8() {
    run("vertex_b8", cases::vertex::vertex_b8);
}

#[test]
fn vertex_c1() {
    run("vertex_c1", cases::vertex::vertex_c1);
}

#[test]
fn vertex_f2() {
    run("vertex_f2", cases::vertex::vertex_f2);
}

#[test]
fn vertex_edge_b1() {
    run("vertex_edge_b1", cases::vertex_edge::vertex_edge_b1);
}

#[test]
fn vertex_edge_f2() {
    run("vertex_edge_f2", cases::vertex_edge::vertex_edge_f2);
}

#[test]
fn vertex_edge_g3() {
    run("vertex_edge_g3", cases::vertex_edge::vertex_edge_g3);
}

#[test]
fn vertex_edge_k4() {
    run("vertex_edge_k4", cases::vertex_edge::vertex_edge_k4);
}

#[test]
fn vertex_face_b1() {
    run("vertex_face_b1", cases::vertex_face::vertex_face_b1);
}

#[test]
fn vertex_face_f2() {
    run("vertex_face_f2", cases::vertex_face::vertex_face_f2);
}

#[test]
fn vertex_face_g3() {
    run("vertex_face_g3", cases::vertex_face::vertex_face_g3);
}

#[test]
fn vertex_face_k4() {
    run("vertex_face_k4", cases::vertex_face::vertex_face_k4);
}

#[test]
fn vertex_solid_b1() {
    run("vertex_solid_b1", cases::vertex_solid::vertex_solid_b1);
}

#[test]
fn vertex_solid_f2() {
    run("vertex_solid_f2", cases::vertex_solid::vertex_solid_f2);
}

#[test]
fn vertex_solid_g3() {
    run("vertex_solid_g3", cases::vertex_solid::vertex_solid_g3);
}

#[test]
fn vertex_solid_k4() {
    run("vertex_solid_k4", cases::vertex_solid::vertex_solid_k4);
}

#[test]
fn vertex_wire_a1() {
    run("vertex_wire_a1", cases::vertex_wire::vertex_wire_a1);
}

#[test]
fn vertex_wire_b1() {
    run("vertex_wire_b1", cases::vertex_wire::vertex_wire_b1);
}

#[test]
fn vertex_wire_f2() {
    run("vertex_wire_f2", cases::vertex_wire::vertex_wire_f2);
}

#[test]
fn vertex_wire_g3() {
    run("vertex_wire_g3", cases::vertex_wire::vertex_wire_g3);
}

#[test]
fn vertex_wire_k4() {
    run("vertex_wire_k4", cases::vertex_wire::vertex_wire_k4);
}

#[test]
fn wire_b1() {
    run("wire_b1", cases::wire::wire_b1);
}

#[test]
fn wire_f2() {
    run("wire_f2", cases::wire::wire_f2);
}

#[test]
fn wire_solid_b1() {
    run("wire_solid_b1", cases::wire_solid::wire_solid_b1);
}

#[test]
fn wire_solid_f2() {
    run("wire_solid_f2", cases::wire_solid::wire_solid_f2);
}

#[test]
fn wire_solid_g3() {
    run("wire_solid_g3", cases::wire_solid::wire_solid_g3);
}

#[test]
fn wire_solid_k4() {
    run("wire_solid_k4", cases::wire_solid::wire_solid_k4);
}

#[test]
fn edge_b1() {
    run("edge_b1", cases::edge::edge_b1);
}

#[test]
fn edge_f2() {
    run("edge_f2", cases::edge::edge_f2);
}

#[test]
fn edge_face_b1() {
    run("edge_face_b1", cases::edge_face::edge_face_b1);
}

#[test]
fn edge_face_f2() {
    run("edge_face_f2", cases::edge_face::edge_face_f2);
}

#[test]
fn edge_face_g3() {
    run("edge_face_g3", cases::edge_face::edge_face_g3);
}

#[test]
fn edge_face_k4() {
    run("edge_face_k4", cases::edge_face_more::edge_face_k4);
}

#[test]
fn edge_face_l5() {
    run("edge_face_l5", cases::edge_face_more::edge_face_l5);
}

#[test]
fn edge_face_p6() {
    run("edge_face_p6", cases::edge_face_more::edge_face_p6);
}

#[test]
fn edge_solid_b1() {
    run("edge_solid_b1", cases::edge_solid::edge_solid_b1);
}

#[test]
fn edge_solid_f2() {
    run("edge_solid_f2", cases::edge_solid::edge_solid_f2);
}

#[test]
fn edge_solid_g3() {
    run("edge_solid_g3", cases::edge_solid::edge_solid_g3);
}

#[test]
fn edge_solid_k4() {
    run("edge_solid_k4", cases::edge_solid_more::edge_solid_k4);
}

#[test]
fn edge_solid_l5() {
    run("edge_solid_l5", cases::edge_solid_more::edge_solid_l5);
}

#[test]
fn edge_solid_p6() {
    run("edge_solid_p6", cases::edge_solid_more::edge_solid_p6);
}

#[test]
fn face_b1() {
    run("face_b1", cases::face::face_b1);
}

#[test]
fn face_f2() {
    run("face_f2", cases::face::face_f2);
}
#[test]
fn edge_a10() {
    run("edge_a10", cases::checked::edge_a10);
}

#[test]
fn face_f3() {
    run("face_f3", cases::checked::face_f3);
}

#[test]
fn wire_solid_l1() {
    run("wire_solid_l1", cases::checked::wire_solid_l1);
}

#[test]
fn face_g1() {
    run("face_g1", cases::infinite_face::infinite_face_isolines);
}
