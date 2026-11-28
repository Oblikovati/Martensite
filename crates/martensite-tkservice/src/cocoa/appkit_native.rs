//! The AppKit implementation of [`CocoaAppKitCalls`] through objc2 (macOS only).
//!
//! Every address handed to these calls is a live object retained by the caller (a
//! `Cocoa_Window` field or a Metal layer lease), which is what makes the borrows below
//! sound. AppKit objects are only touched on the main thread: [`on_main`] runs a mutation
//! at once there and otherwise queues it on the main dispatch queue.
#![allow(unsafe_code)]
use super::appkit::{CocoaAppKitCalls, CocoaCreatedWindow, CocoaRect};
use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, Message};
use objc2_app_kit::{
    NSBackingStoreType, NSColorSpace, NSScreen, NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};
use objc2_quartz_core::CAMetalLayer;

/// The AppKit of the running process.
pub(super) struct NativeAppKitCalls;

unsafe extern "C" {
    /// AppKit's global application object, nil until `NSApplication` is instantiated.
    static NSApp: *mut AnyObject;
}

/// Runs `action` on the main thread: now when already there, otherwise queued.
fn on_main(action: impl FnOnce(MainThreadMarker) + Send + 'static) {
    if let Some(mtm) = MainThreadMarker::new() {
        action(mtm);
        return;
    }
    DispatchQueue::main().exec_async(move || {
        // SAFETY: blocks submitted to the main dispatch queue run on the main thread.
        action(unsafe { MainThreadMarker::new_unchecked() });
    });
}

/// Borrows the `NSView*` at `view` for `read`, `None` for a null address.
fn with_view<R>(view: usize, _mtm: MainThreadMarker, read: impl FnOnce(&NSView) -> R) -> Option<R> {
    // SAFETY: the caller retains the view for the duration of the call (module contract),
    // and the marker proves this runs on the main thread that NSView requires.
    unsafe { (view as *const NSView).as_ref() }.map(read)
}

/// Converts an AppKit rectangle to the seam's value.
fn to_rect(rect: NSRect) -> CocoaRect {
    CocoaRect::new(
        rect.origin.x,
        rect.origin.y,
        rect.size.width,
        rect.size.height,
    )
}

/// The address of a retained object, transferring its reference to the caller.
fn into_address<T: objc2::Message>(object: Retained<T>) -> usize {
    Retained::into_raw(object) as usize
}

/// A titled, closable, resizable window whose content rectangle is `frame`.
fn new_window(mtm: MainThreadMarker, frame: CocoaRect) -> Retained<NSWindow> {
    let rect = NSRect::new(
        NSPoint::new(frame.x, frame.y),
        NSSize::new(frame.width, frame.height),
    );
    let style =
        NSWindowStyleMask::Titled | NSWindowStyleMask::Closable | NSWindowStyleMask::Resizable;
    // SAFETY: the allocation comes from the main-thread marker and is consumed by this
    // designated initializer with a valid style mask and backing store type.
    unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            mtm.alloc(),
            rect,
            style,
            NSBackingStoreType::Buffered,
            false,
        )
    }
}

/// The existing `CAMetalLayer` of `view`, retained.
fn existing_metal_layer(view: &NSView) -> Option<usize> {
    let layer = view.layer()?;
    let any: &AnyObject = &layer;
    let metal = any.downcast_ref::<CAMetalLayer>()?;
    Some(into_address(metal.retain()))
}

/// Installs a new `CAMetalLayer` at the backing scale of the window of `view`, retained.
fn install_metal_layer(view: &NSView) -> usize {
    let layer = CAMetalLayer::new();
    if let Some(window) = view.window() {
        layer.setContentsScale(window.backingScaleFactor());
    }
    // A layer-hosting view takes its layer before wantsLayer, as AppKit documents.
    view.setLayer(Some(&layer));
    view.setWantsLayer(true);
    into_address(layer)
}

impl CocoaAppKitCalls for NativeAppKitCalls {
    fn is_main_thread(&self) -> bool {
        MainThreadMarker::new().is_some()
    }
    fn has_application(&self) -> bool {
        // SAFETY: NSApp is a pointer-sized AppKit global, only read here.
        !unsafe { NSApp }.is_null()
    }
    fn main_screen_height(&self) -> Option<f64> {
        let mtm = MainThreadMarker::new()?;
        NSScreen::mainScreen(mtm).map(|screen| screen.frame().size.height)
    }
    fn create_window(&self, title: &str, frame: CocoaRect) -> Option<CocoaCreatedWindow> {
        let window = new_window(MainThreadMarker::new()?, frame);
        // SAFETY: the window is owned by its retained address; AppKit must not release it on
        // close ("do not destroy NSWindow on close - we didn't handle it!", OCCT).
        unsafe { window.setReleasedWhenClosed(false) };
        window.setColorSpace(Some(&NSColorSpace::sRGBColorSpace()));
        window.setTitle(&NSString::from_str(title));
        let view = window.contentView()?;
        Some(CocoaCreatedWindow {
            window: into_address(window),
            view: into_address(view),
        })
    }
    fn close_window(&self, window: usize) {
        on_main(move |_mtm| {
            // SAFETY: the caller keeps its reference to the window until after this runs
            // (its release is queued behind this close), and this runs on the main thread.
            if let Some(window) = unsafe { (window as *const NSWindow).as_ref() } {
                window.close();
            }
        });
    }
    fn retain_object(&self, object: usize) {
        // SAFETY: the object is live (module contract); objc_retain is thread-safe.
        unsafe { objc2::ffi::objc_retain(object as *mut AnyObject) };
    }
    fn release_object(&self, object: usize) {
        // The last release deallocates an AppKit object, which must happen on the main thread.
        on_main(move |_mtm| {
            // SAFETY: this balances one reference the seam handed out for `object`.
            unsafe { objc2::ffi::objc_release(object as *mut AnyObject) };
        });
    }
    fn is_window_visible(&self, view: usize) -> Option<bool> {
        let mtm = MainThreadMarker::new()?;
        with_view(view, mtm, |view| {
            view.window().is_some_and(|window| window.isVisible())
        })
    }
    fn order_window(&self, view: usize, front: bool) {
        on_main(move |mtm| {
            let _ = with_view(view, mtm, |view| match (view.window(), front) {
                (Some(window), true) => window.orderFront(None),
                (Some(window), false) => window.orderOut(None),
                (None, _) => {}
            });
        });
    }
    fn backing_bounds(&self, view: usize) -> Option<CocoaRect> {
        let mtm = MainThreadMarker::new()?;
        with_view(view, mtm, |view| {
            to_rect(view.convertRectToBacking(view.bounds()))
        })
    }
    fn window_frame(&self, view: usize) -> Option<CocoaRect> {
        let mtm = MainThreadMarker::new()?;
        with_view(view, mtm, |view| {
            view.window().map(|window| to_rect(window.frame()))
        })?
    }
    fn set_window_title(&self, view: usize, title: &str) {
        let title = title.to_owned();
        on_main(move |mtm| {
            let _ = with_view(view, mtm, |view| {
                if let Some(window) = view.window() {
                    window.setTitle(&NSString::from_str(&title));
                }
            });
        });
    }
    fn invalidate_view(&self, view: usize) {
        on_main(move |mtm| {
            let _ = with_view(view, mtm, |view| view.setNeedsDisplay(true));
        });
    }
    fn metal_layer(&self, view: usize) -> Option<usize> {
        let mtm = MainThreadMarker::new()?;
        with_view(view, mtm, |view| {
            existing_metal_layer(view).unwrap_or_else(|| install_metal_layer(view))
        })
    }
}
