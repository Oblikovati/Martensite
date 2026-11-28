//! `Cocoa_Window`: an AppKit window (NSWindow with its NSView) or a wrapper of an NSView.
use super::appkit::{CocoaAppKitCalls, native_appkit};
use super::metal_layer::CocoaMetalLayerLease;
use super::resize::appkit_frame;
use super::virtual_keys;
use crate::aspect::exceptions::{window_definition_error, window_error};
use crate::aspect::{AspectVKey, AspectWindow, AspectWindowBase};
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, SharedState};
use std::sync::Arc;

#[path = "window_aspect.rs"]
mod window_aspect;

/// The AppKit backend named by the unsupported errors.
const BACKEND: &str = "AppKit (macOS)";

/// The protected fields of a Cocoa window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CocoaWindowFields {
    /// `myHWindow`: the `NSWindow*` address, 0 for a wrapped view.
    pub h_window: usize,
    /// `myHView`: the `NSView*` address.
    pub h_view: usize,
    /// `myXLeft`.
    pub x_left: i32,
    /// `myYTop`.
    pub y_top: i32,
    /// `myXRight`.
    pub x_right: i32,
    /// `myYBottom`.
    pub y_bottom: i32,
}

/// An AppKit window; `CocoaWindow::new("Viewer", 0, 0, 640, 480)?` on the main thread.
///
/// AppKit objects live on the main thread: create the window there. Queries made on other
/// threads answer from the stored frame, and changes made there are queued to the main
/// thread (see [`CocoaAppKitCalls`]).
pub struct CocoaWindow {
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<CocoaWindowFields>,
    calls: Arc<dyn CocoaAppKitCalls>,
    /// The retained `CAMetalLayer*` installed for Vulkan presentation, 0 before the first request.
    metal_layer: SharedState<usize>,
}
martensite_tkernel::impl_transient!(CocoaWindow, "Cocoa_Window", dyn AspectWindow, interfaces: [dyn AspectWindow]);

/// The host AppKit, or `Standard_NotImplemented` where there is none.
fn host_appkit(interface: &str) -> Result<Arc<dyn CocoaAppKitCalls>, Failure> {
    native_appkit().ok_or_else(|| unsupported(interface, BACKEND))
}

impl CocoaWindow {
    /// `VirtualKeyFromNative`: the Aspect key of a Carbon virtual key code (unknown outside
    /// the table); `CocoaWindow::virtual_key_from_native(0x00)` is `A`.
    pub fn virtual_key_from_native(key: i32) -> AspectVKey {
        virtual_keys::virtual_key_from_native(key)
    }
    /// `Cocoa_Window(title, left, top, width, height)`: a new NSWindow with its NSView at
    /// the top-left based screen position; see [`CocoaWindow::create_with`] for the errors.
    /// Raises `Standard_NotImplemented` without AppKit.
    /// Example: `CocoaWindow::new("Viewer", 0, 0, 640, 480)`.
    pub fn new(title: &str, left: i32, top: i32, width: i32, height: i32) -> Result<Self, Failure> {
        Self::create_with(
            host_appkit("Cocoa_Window")?,
            title,
            left,
            top,
            width,
            height,
        )
    }
    /// The constructor of [`CocoaWindow::new`] over an explicit AppKit. A non-positive
    /// size, a call off the main thread, a missing `NSApplication` or a refused creation
    /// raise `Aspect_WindowDefinitionError`.
    /// Example: `CocoaWindow::create_with(native_appkit().unwrap(), "Viewer", 0, 0, 640, 480)`.
    pub fn create_with(
        calls: Arc<dyn CocoaAppKitCalls>,
        title: &str,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        check_creation(calls.as_ref(), width, height)?;
        let screen_height = calls.main_screen_height().unwrap_or(0.0);
        let frame = appkit_frame(left, top, width, height, screen_height);
        let created = calls.create_window(title, frame).ok_or_else(|| {
            window_definition_error(&format!(
                "Cocoa_Window, Unable to create window {title:?} at {frame:?}; expected AppKit to create an NSWindow"
            ))
        })?;
        let fields = CocoaWindowFields {
            h_window: created.window,
            h_view: created.view,
            x_left: left,
            y_top: top,
            x_right: left.wrapping_add(width),
            y_bottom: top.wrapping_add(height),
        };
        Ok(Self::with_fields(calls, fields))
    }
    /// `Cocoa_Window(NSView*)`: retains the NSView at `view` and reads its frame; raises
    /// `Standard_NotImplemented` without AppKit. Example: `CocoaWindow::from_view(view_address)`.
    pub fn from_view(view: usize) -> Result<Self, Failure> {
        Ok(Self::wrap_view_with(host_appkit("Cocoa_Window")?, view))
    }
    /// The constructor of [`CocoaWindow::from_view`] over an explicit AppKit; the frame
    /// starts at 512x512 until the view reports its bounds.
    /// Example: `CocoaWindow::wrap_view_with(native_appkit().unwrap(), view_address)`.
    pub fn wrap_view_with(calls: Arc<dyn CocoaAppKitCalls>, view: usize) -> Self {
        let fields = CocoaWindowFields {
            x_right: 512,
            y_bottom: 512,
            ..CocoaWindowFields::default()
        };
        let window = Self::with_fields(calls, fields);
        window.replace_view(view);
        let _ = window.do_resize();
        window
    }
    fn with_fields(calls: Arc<dyn CocoaAppKitCalls>, fields: CocoaWindowFields) -> Self {
        Self {
            base: AspectWindowBase::default(),
            fields: SharedState::new(fields),
            calls,
            metal_layer: SharedState::new(0),
        }
    }
    /// `HView`: the NSView address; `window.h_view()`.
    pub fn h_view(&self) -> usize {
        self.fields.read().h_view
    }
    /// `SetHView`: retains the NSView at `view` and releases the previous one; a Metal
    /// layer installed on the previous view is released too. Never fails; the `Result` is
    /// kept for the interface. Example: `window.set_h_view(view_address)?`.
    pub fn set_h_view(&self, view: usize) -> Result<(), Failure> {
        self.replace_view(view);
        Ok(())
    }
    fn replace_view(&self, view: usize) {
        if view != 0 {
            self.calls.retain_object(view);
        }
        let previous = std::mem::replace(&mut self.fields.write().h_view, view);
        self.release_if_set(previous);
        let layer = std::mem::take(&mut *self.metal_layer.write());
        self.release_if_set(layer);
    }
    fn release_if_set(&self, object: usize) {
        if object != 0 {
            self.calls.release_object(object);
        }
    }
    /// The `CAMetalLayer` that Vulkan presents to, installed on the view by the first
    /// request, which must run on the main thread; later requests may run on any thread.
    /// Raises `Aspect_WindowError` without a view or when the first request runs elsewhere.
    /// Example: `let lease = window.metal_layer()?; lease.layer_address()`.
    pub fn metal_layer(&self) -> Result<CocoaMetalLayerLease, Failure> {
        let mut cached = self.metal_layer.write();
        if *cached == 0 {
            *cached = self.install_metal_layer()?;
        }
        self.calls.retain_object(*cached);
        Ok(CocoaMetalLayerLease::adopt(self.calls.clone(), *cached))
    }
    fn install_metal_layer(&self) -> Result<usize, Failure> {
        let view = self.h_view();
        if view == 0 {
            return Err(window_error(
                "Cocoa_Window::MetalLayer: no NSView (address 0); expected a window with a view",
            ));
        }
        self.calls.metal_layer(view).ok_or_else(|| {
            window_error(&format!(
                "Cocoa_Window::MetalLayer: NSView {view:#x} has no CAMetalLayer yet; expected the first request on the main thread"
            ))
        })
    }
}

/// The `Cocoa_Window` constructor preconditions, in OCCT's order plus the main thread.
fn check_creation(calls: &dyn CocoaAppKitCalls, width: i32, height: i32) -> Result<(), Failure> {
    let refusal = if width <= 0 || height <= 0 {
        format!("Coordinate(s) out of range: {width}x{height}; expected a positive size")
    } else if !calls.is_main_thread() {
        "called off the main thread; expected AppKit windows to be created on the main thread"
            .to_owned()
    } else if !calls.has_application() {
        "Cocoa application should be instantiated before window; expected NSApp to exist".to_owned()
    } else {
        return Ok(());
    };
    Err(window_definition_error(&format!("Cocoa_Window, {refusal}")))
}

impl Drop for CocoaWindow {
    /// `~Cocoa_Window`: closes and releases a window created here, then releases the view
    /// and the Metal layer; AppKit runs the releases on the main thread.
    fn drop(&mut self) {
        let fields = *self.fields.read();
        if fields.h_window != 0 {
            self.calls.close_window(fields.h_window);
            self.calls.release_object(fields.h_window);
        }
        self.release_if_set(fields.h_view);
        self.release_if_set(*self.metal_layer.read());
    }
}
