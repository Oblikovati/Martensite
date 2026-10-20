use super::*;
use crate::aspect::{AspectVKey, AspectVKeyBasic as K};
use crate::platform::is_platform_unsupported;

fn key(code: i32) -> AspectVKey {
    CocoaWindow::virtual_key_from_native(code)
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

#[test]
fn appkit_constructors_raise_the_unsupported_platform_error() {
    let Err(window) = CocoaWindow::new("Viewer", 0, 0, 640, 480) else {
        panic!("the AppKit backend is not implemented");
    };
    assert!(is_platform_unsupported(&window));
    assert!(CocoaWindow::from_view(1).is_err_and(|failure| is_platform_unsupported(&failure)));
    assert!(CocoaLocalPool::new().is_err_and(|failure| is_platform_unsupported(&failure)));
}
