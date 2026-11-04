//! Setters called from the destructor of the retiring texture pixmap reach the application
//! override, which records the field before and after its base call exactly as natively
//! (`the recorded texture_retirement_override capture`).
#[path = "support/texture_retirement_override_fixture.rs"]
mod override_fixture;
use martensite_tkernel::{standard::*, tcollection::AsciiString};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::ais::*;
use override_fixture::*;
use std::sync::Arc;
use std::sync::atomic::Ordering;
const NATIVE: &str = include_str!("fixtures/ais_textured_shape/native-retirement-override.txt");

#[test]
fn texture_retirement_destructor_setters_route_to_the_derived_override() {
    let trace: RoutingTrace = Arc::new(SharedState::new(Vec::new()));
    for mode in ["none", "file", "pixmap"] {
        run_routing_case(mode, false, &trace);
        run_routing_case(mode, true, &trace);
    }
    let actual: Vec<String> = trace.read().clone();
    let expected: Vec<&str> = NATIVE.lines().collect();
    assert_eq!(
        actual, expected,
        "Rust override routing; expected the native records"
    );
}
#[test]
fn texture_retirement_unscoped_setter_from_a_dropping_pixmap_fails_typed() {
    // Native would reach the field during refcount-zero destruction; safe Rust refuses it
    // outside a retirement scope, the override still runs and the field stays retiring.
    let trace: RoutingTrace = Arc::new(SharedState::new(Vec::new()));
    let owner = RoutingShape::new(&trace);
    let nested = Handle::new(ImagePixMap::default());
    let mut old = routing_pixels(&owner, &nested, "unscoped");
    owner.set_texture_pix_map(&old);
    old.nullify();
    owner.set_texture_pix_map(&nested);
    let rows = trace.read().clone();
    let refused = rows
        .iter()
        .find(|row| row.starts_with("UNSCOPED "))
        .unwrap();
    assert!(
        refused.contains("expected idle slot"),
        "row={refused:?}; expected a typed refusal"
    );
    assert!(!owner.base.retirement.is_active());
    assert_eq!(
        owner.texture_pix_map().object_address(),
        nested.object_address()
    );
}
fn run_routing_case(mode: &'static str, outer_file: bool, trace: &RoutingTrace) {
    let owner = RoutingShape::new(trace);
    let next = Handle::new(ImagePixMap::default());
    let nested = Handle::new(ImagePixMap::default());
    trace
        .write()
        .push(format!("CASE {mode} {}", i32::from(outer_file)));
    let mut old = routing_pixels(&owner, &nested, mode);
    owner.set_texture_pix_map(&old);
    old.nullify();
    if outer_file {
        owner
            .set_texture_file_name(&AsciiString::from("outer.raw"))
            .unwrap();
    } else {
        owner.set_texture_pix_map(&next);
    }
    let slot = owner.texture_pix_map();
    let row = format!(
        "AFTER {mode} {} {} {} {} {} {} {} {}",
        i32::from(outer_file),
        owner.file_calls.load(Ordering::SeqCst),
        owner.pixel_calls.load(Ordering::SeqCst),
        i32::from(slot.is_null()),
        i32::from(slot.object_address() == next.object_address()),
        next.ref_count(),
        nested.ref_count(),
        routing_file(&owner)
    );
    trace.write().push(row);
}
