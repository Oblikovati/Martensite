//! Native AppKit checks of `Cocoa_Window` and `Cocoa_LocalPool` on the process main thread.
//!
//! Run in a macOS login session with
//! `cargo test -p martensite-tkservice --features appkit-integration-tests --test appkit_window`.
//! A small window appears briefly. Each check panics with the observed value on failure.

#[cfg(target_os = "macos")]
fn main() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;
    let mtm = MainThreadMarker::new().expect("a harness-less test binary runs on the main thread");
    let _application = NSApplication::sharedApplication(mtm);
    checks::run_all();
    println!("appkit_window: all checks passed");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("appkit_window: AppKit exists only on macOS; nothing to check");
}

#[cfg(target_os = "macos")]
mod checks {
    use martensite_tkernel::tcollection::AsciiString;
    use martensite_tkservice::aspect::{AspectTypeOfResize, AspectWindow};
    use martensite_tkservice::cocoa::{CocoaLocalPool, CocoaWindow};

    pub(super) fn run_all() {
        let _pool = CocoaLocalPool::new().expect("the Objective-C runtime pushes a pool");
        created_window_maps_resizes_and_unmaps();
        metal_layer_is_installed_once_and_shared();
        wrapped_view_reads_its_backing_size();
    }

    fn created_window() -> CocoaWindow {
        CocoaWindow::new("Martensite AppKit check", 120, 120, 320, 240)
            .expect("the main thread of an application creates windows")
    }

    fn created_window_maps_resizes_and_unmaps() {
        let window = created_window();
        assert_ne!(window.native_handle(), 0, "the window exposes its NSView");
        let (width, height) = window.size();
        assert!(
            width >= 320 && height >= 240,
            "backing size {width}x{height}"
        );
        assert_eq!(window.ratio(), f64::from(width) / f64::from(height));
        window.map();
        assert!(window.is_mapped(), "orderFront: makes the window visible");
        // As in OCCT, the constructor stores screen corners while DoResize compares view
        // bounds, so the first call moves every edge (Unknown) and syncs the corners.
        let first = window.do_resize();
        assert_eq!(
            first,
            AspectTypeOfResize::Unknown,
            "first resize syncs the corners"
        );
        let second = window.do_resize();
        assert_eq!(
            second,
            AspectTypeOfResize::NoBorder,
            "an unchanged view has no moved border"
        );
        window.set_title(&AsciiString::from("Martensite AppKit check (renamed)"));
        window.unmap();
        assert!(!window.is_mapped(), "orderOut: hides the window");
    }

    fn metal_layer_is_installed_once_and_shared() {
        let window = created_window();
        let first = window
            .metal_layer()
            .expect("the main thread installs a CAMetalLayer");
        let second = window.metal_layer().expect("the installed layer is shared");
        assert_ne!(first.layer_address(), 0);
        assert_eq!(first.layer_address(), second.layer_address());
        drop(window);
        assert_ne!(first.layer_address(), 0, "the lease outlives the window");
    }

    fn wrapped_view_reads_its_backing_size() {
        let owner = created_window();
        let wrapper = CocoaWindow::from_view(owner.h_view()).expect("an NSView is wrapped");
        assert_eq!(wrapper.size(), owner.size());
        assert_eq!(
            wrapper.fields.read().h_window,
            0,
            "a wrapper owns no window"
        );
        drop(owner);
        assert_ne!(
            wrapper.native_handle(),
            0,
            "the wrapper keeps the view alive"
        );
    }
}
