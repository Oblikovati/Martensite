use super::fake_appkit::{FAKE_LAYER, FAKE_VIEW, FAKE_WINDOW, RecordingAppKit};
use super::*;
const WINDOW_DEFINITION_ERROR_TYPE: &str = "Aspect_WindowDefinitionError";
use crate::aspect::{
    AspectTypeOfResize, AspectVKey, AspectVKeyBasic as K, AspectWindow, AspectWindowMembers as _,
};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use std::sync::Arc;

#[path = "resize_tests.rs"]
mod resize_tests;

fn key(code: i32) -> AspectVKey {
    CocoaWindow::virtual_key_from_native(code)
}

fn created(appkit: &Arc<RecordingAppKit>) -> CocoaWindow {
    CocoaWindow::create_with(appkit.clone(), "Viewer", 10, 20, 640, 480)
        .expect("the scripted AppKit creates the window")
}

fn definition_message(appkit: RecordingAppKit, width: i32, height: i32) -> String {
    let Err(failure) = CocoaWindow::create_with(Arc::new(appkit), "Viewer", 0, 0, width, height)
    else {
        panic!("the window creation must be refused");
    };
    assert_eq!(failure.exception_type(), WINDOW_DEFINITION_ERROR_TYPE);
    failure.message().to_owned()
}

#[test]
fn carbon_key_codes_map_as_the_native_table() {
    assert_eq!(key(0x00), K::A as AspectVKey);
    assert_eq!(key(0x12), K::Digit1 as AspectVKey);
    assert_eq!(key(0x16), K::Digit6 as AspectVKey);
    assert_eq!(key(0x17), K::Digit5 as AspectVKey);
    assert_eq!(key(0x24), K::Enter as AspectVKey);
    assert_eq!(key(0x3B), K::Control as AspectVKey);
    assert_eq!(key(0x4C), K::Enter as AspectVKey);
    assert_eq!(key(0x7A), K::F1 as AspectVKey);
    assert_eq!(key(0x7E), K::Up as AspectVKey);
}

#[test]
fn unmapped_and_out_of_range_codes_are_unknown() {
    for code in [
        0x0A, 0x34, 0x36, 0x37, 0x3F, 0x40, 0x4B, 0x66, 0x7F, 0x80, -1, 1000,
    ] {
        assert_eq!(key(code), K::Unknown as AspectVKey, "code {code:#x}");
    }
}

#[cfg(not(target_os = "macos"))]
#[test]
fn appkit_constructors_raise_the_unsupported_platform_error_without_appkit() {
    use crate::platform::is_platform_unsupported;
    assert!(native_appkit().is_none());
    let Err(window) = CocoaWindow::new("Viewer", 0, 0, 640, 480) else {
        panic!("there is no AppKit on this platform");
    };
    assert!(is_platform_unsupported(&window));
    assert!(CocoaWindow::from_view(1).is_err_and(|failure| is_platform_unsupported(&failure)));
    assert!(CocoaLocalPool::new().is_err_and(|failure| is_platform_unsupported(&failure)));
}

#[test]
fn creation_flips_the_top_left_frame_into_appkit_coordinates() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    let fields = *window.fields.read();
    assert_eq!((fields.h_window, fields.h_view), (FAKE_WINDOW, FAKE_VIEW));
    assert_eq!(
        (fields.x_left, fields.y_top, fields.x_right, fields.y_bottom),
        (10, 20, 650, 500)
    );
    let expected = CocoaRect::new(10.0, 1080.0 - 20.0 - 480.0, 640.0, 480.0);
    assert_eq!(appkit.log(), [format!("create Viewer {expected:?}")]);
}

#[test]
fn creation_refusals_name_the_offending_precondition() {
    let message = definition_message(RecordingAppKit::on_main_thread(), 0, 480);
    assert!(message.contains("0x480"), "{message}");
    let off_main = RecordingAppKit::on_main_thread();
    off_main.with(|state| state.main_thread = false);
    assert!(definition_message(off_main, 640, 480).contains("main thread"));
    let no_app = RecordingAppKit::on_main_thread();
    no_app.with(|state| state.application = false);
    assert!(definition_message(no_app, 640, 480).contains("NSApp"));
    let refusing = RecordingAppKit::on_main_thread();
    refusing.with(|state| state.refuse_creation = true);
    assert!(definition_message(refusing, 640, 480).contains("Unable to create window"));
}

#[test]
fn dropping_a_created_window_closes_it_and_balances_every_reference() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    let lease = window
        .metal_layer()
        .expect("the main thread installs the layer");
    assert_eq!(lease.layer_address(), FAKE_LAYER);
    drop(window);
    assert!(appkit.log().contains(&format!("close {FAKE_WINDOW:#x}")));
    assert_eq!(
        appkit.references(FAKE_LAYER),
        1,
        "the lease still holds the layer"
    );
    drop(lease);
    for object in [FAKE_WINDOW, FAKE_VIEW, FAKE_LAYER] {
        assert_eq!(appkit.references(object), 0, "object {object:#x}");
    }
}

#[test]
fn a_wrapped_view_is_retained_sized_from_its_bounds_and_released_on_replacement() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = CocoaWindow::wrap_view_with(appkit.clone(), FAKE_VIEW);
    assert_eq!(appkit.references(FAKE_VIEW), 1);
    assert_eq!(window.size(), (640, 480));
    assert_eq!(
        window.position(),
        (0, 0, 640, 480),
        "no frame: the stored corners"
    );
    window
        .set_h_view(0)
        .expect("replacing the view never fails");
    assert_eq!(appkit.references(FAKE_VIEW), 0);
    assert_eq!(window.native_handle(), 0);
    drop(window);
    assert!(appkit.log().iter().all(|entry| !entry.starts_with("close")));
}

#[test]
fn queries_off_the_main_thread_answer_from_the_stored_frame() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    appkit.with(|state| state.main_thread = false);
    assert_eq!(window.do_resize(), AspectTypeOfResize::Unknown);
    assert_eq!(window.size(), (640, 480));
    assert_eq!(window.position(), (10, 20, 650, 500));
    assert!(!window.is_mapped());
    window.set_virtual(true);
    assert!(
        window.is_mapped(),
        "a virtual window always reads as mapped"
    );
}

#[test]
fn live_geometry_comes_from_backing_bounds_and_the_window_frame() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    appkit.with(|state| {
        state.bounds = Some(CocoaRect::new(0.0, 0.0, 1280.0, 960.0));
        state.frame = Some(CocoaRect::new(10.0, 580.0, 640.0, 480.0));
        state.visible = true;
    });
    assert_eq!(window.size(), (1280, 960));
    assert_eq!(window.ratio(), 1280.0 / 960.0);
    assert_eq!(window.position(), (10, 20, 650, 500));
    assert!(window.is_mapped());
}

#[test]
fn mutations_reach_the_view_and_skip_a_missing_one() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    window.map();
    window.unmap();
    window.set_title(&AsciiString::from("Part 1"));
    window.invalidate_content(&Handle::default());
    let view = format!("{FAKE_VIEW:#x}");
    let expected = [
        format!("order {view} front=true"),
        format!("order {view} front=false"),
        format!("title {view} Part 1"),
        format!("invalidate {view}"),
    ];
    assert_eq!(appkit.log()[1..], expected);
    window
        .set_h_view(0)
        .expect("replacing the view never fails");
    window.map();
    window.set_title(&AsciiString::from("ignored"));
    assert_eq!(appkit.log().len(), 5);
}

#[test]
fn the_metal_layer_is_installed_once_and_needs_the_main_thread_first() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    appkit.with(|state| state.main_thread = false);
    let refusal = window
        .metal_layer()
        .expect_err("no layer exists off the main thread");
    assert!(
        refusal.message().contains("main thread"),
        "{}",
        refusal.message()
    );
    appkit.with(|state| state.main_thread = true);
    let first = window
        .metal_layer()
        .expect("the main thread installs the layer");
    appkit.with(|state| state.main_thread = false);
    let second = window
        .metal_layer()
        .expect("an installed layer is shared on any thread");
    assert_eq!(first.layer_address(), second.layer_address());
    let installs = appkit
        .log()
        .iter()
        .filter(|entry| entry.starts_with("metal"))
        .count();
    assert_eq!(installs, 1);
    assert_eq!(
        appkit.references(FAKE_LAYER),
        3,
        "the window and two leases"
    );
}

#[test]
fn replacing_the_view_drops_the_layer_installed_on_the_old_one() {
    let appkit = Arc::new(RecordingAppKit::on_main_thread());
    let window = created(&appkit);
    drop(
        window
            .metal_layer()
            .expect("the main thread installs the layer"),
    );
    window
        .set_h_view(0)
        .expect("replacing the view never fails");
    assert_eq!(appkit.references(FAKE_LAYER), 0);
    let refusal = window.metal_layer().expect_err("no view, no layer");
    assert!(
        refusal.message().contains("no NSView"),
        "{}",
        refusal.message()
    );
}

#[cfg(target_os = "macos")]
#[test]
fn nested_local_pools_drain_once_even_when_dropped_out_of_order() {
    let outer = CocoaLocalPool::new().expect("the Objective-C runtime pushes a pool");
    let inner = CocoaLocalPool::new().expect("pools nest");
    assert_ne!(outer.pool_object(), inner.pool_object());
    // The outer drain also drains the inner pool; the inner drop must then do nothing.
    drop(outer);
    drop(inner);
    let again = CocoaLocalPool::new().expect("the thread keeps working after the drains");
    assert_ne!(again.pool_object(), 0);
}
