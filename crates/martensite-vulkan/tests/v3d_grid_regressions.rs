//! Construction-grid regressions of the V3d grid group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_grid_cases;
use std::error::Error;
use v3d_grid_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bounded_circ() {
    run("bounded_circ", cases::bounded_circ);
}

#[test]
fn bounded_rect() {
    run("bounded_rect", cases::bounded_rect);
}

#[test]
fn circ_cpu() {
    run("circ_cpu", cases::circ_cpu);
}

#[test]
fn circ_gpu() {
    run("circ_gpu", cases::circ_gpu);
}

#[test]
fn gpu_options() {
    run("gpu_options", cases::gpu_options);
}

#[test]
fn gpu_pan() {
    run("gpu_pan", cases::gpu_pan);
}

#[test]
fn gpu_perspective_nomirror() {
    run("gpu_perspective_nomirror", cases::gpu_perspective_nomirror);
}

#[test]
fn gpu_plane() {
    run("gpu_plane", cases::gpu_plane);
}

#[test]
fn gpu_rotate() {
    run("gpu_rotate", cases::gpu_rotate);
}

#[test]
fn gpu_visibility() {
    run("gpu_visibility", cases::gpu_visibility);
}

#[test]
fn mode_switch() {
    run("mode_switch", cases::mode_switch);
}

#[test]
fn ortho() {
    run("ortho", cases::ortho);
}

#[test]
fn persp() {
    run("persp", cases::persp);
}

#[test]
fn rect_cpu() {
    run("rect_cpu", cases::rect_cpu);
}

#[test]
fn rect_gpu() {
    run("rect_gpu", cases::rect_gpu);
}

#[test]
fn rect_points() {
    run("rect_points", cases::rect_points);
}

#[test]
fn view_adaptive() {
    run("view_adaptive", cases::view_adaptive);
}
