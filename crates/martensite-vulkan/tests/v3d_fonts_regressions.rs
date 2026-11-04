//! Font regressions of the V3d group (text as B-Rep shapes and labels), each in an isolated worker.
#![allow(non_snake_case)] // Test names follow the reference case names (B1..B7).
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_fonts_cases;
use std::error::Error;
use v3d_fonts_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn B1() {
    run("B1", cases::system_font_lines);
}

#[test]
fn B2() {
    run("B2", cases::composite_system_lines);
}

#[test]
fn B3() {
    run("B3", cases::alphabet_shapes);
}

#[test]
fn B4() {
    run("B4", cases::aligned_regular_text);
}

#[test]
fn B5() {
    run("B5", cases::aligned_bold_text);
}

#[test]
fn B6() {
    run("B6", cases::single_line_font_shapes);
}

#[test]
fn B7() {
    run("B7", cases::registered_font_lines);
}

#[test]
fn bug25852() {
    run("bug25852", cases::circled_symbol);
}
