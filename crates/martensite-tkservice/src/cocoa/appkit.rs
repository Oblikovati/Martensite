//! The AppKit calls of the Cocoa window, behind one seam so the window logic is tested
//! with a fake on every platform and only `appkit_native` touches AppKit.
//!
//! Objects cross the seam as retained addresses (`usize`): `Cocoa_Window` stores
//! `NSWindow*`/`NSView*` in its fields and must stay `Send + Sync` like every
//! `AspectWindow`, while AppKit objects may only be used on the main thread. The native
//! implementation therefore answers queries only on the main thread (callers fall back to
//! their stored frame elsewhere) and sends mutations made elsewhere to the main queue
//! asynchronously, so a process without a running main run loop never deadlocks.
use std::sync::Arc;

/// A rectangle in AppKit coordinates (origin at the bottom left), in points or backing
/// pixels as each call states; `CocoaRect::new(0.0, 0.0, 640.0, 480.0)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CocoaRect {
    /// The left edge.
    pub x: f64,
    /// The bottom edge.
    pub y: f64,
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

impl CocoaRect {
    /// A rectangle from its origin and size; `CocoaRect::new(0.0, 0.0, 640.0, 480.0)`.
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// The new window and its retained content view, both as addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CocoaCreatedWindow {
    /// The retained `NSWindow*`.
    pub window: usize,
    /// The retained content `NSView*`.
    pub view: usize,
}

/// The AppKit operations `Cocoa_Window` and the Metal layer lease need.
///
/// Example: `calls.backing_bounds(view).map_or(0.0, |rect| rect.width)`.
pub trait CocoaAppKitCalls: Send + Sync {
    /// Whether the caller runs on the main (AppKit) thread; `calls.is_main_thread()`.
    fn is_main_thread(&self) -> bool;
    /// Whether the process has an `NSApplication` (`NSApp != nil`); `calls.has_application()`.
    fn has_application(&self) -> bool;
    /// The main screen frame height in points, `None` off the main thread or without a
    /// screen; `calls.main_screen_height()`.
    fn main_screen_height(&self) -> Option<f64>;
    /// A titled, closable, resizable sRGB window with the content rectangle `frame`
    /// (points, AppKit coordinates), kept alive when closed; `None` when AppKit refuses or
    /// off the main thread. Example: `calls.create_window("Viewer", frame)`.
    fn create_window(&self, title: &str, frame: CocoaRect) -> Option<CocoaCreatedWindow>;
    /// Closes the `NSWindow*` at `window` (`[window close]`) on the main thread; the
    /// reference stays with the caller. Example: `calls.close_window(window)`.
    fn close_window(&self, window: usize);
    /// Adds a reference to the object at `object` (any thread); `calls.retain_object(view)`.
    fn retain_object(&self, object: usize);
    /// Drops a reference taken by this seam; the last one is released on the main thread.
    /// Example: `calls.release_object(view)`.
    fn release_object(&self, object: usize);
    /// Whether the window of `view` is visible; `None` off the main thread.
    /// Example: `calls.is_window_visible(view)`.
    fn is_window_visible(&self, view: usize) -> Option<bool>;
    /// Orders the window of `view` front (`true`) or out (`false`); `calls.order_window(view, true)`.
    fn order_window(&self, view: usize, front: bool);
    /// The bounds of `view` in backing pixels; `None` off the main thread.
    /// Example: `calls.backing_bounds(view)`.
    fn backing_bounds(&self, view: usize) -> Option<CocoaRect>;
    /// The frame of the window of `view` in points; `None` off the main thread or for a
    /// view outside a window. Example: `calls.window_frame(view)`.
    fn window_frame(&self, view: usize) -> Option<CocoaRect>;
    /// Sets the title of the window of `view`; `calls.set_window_title(view, "Viewer")`.
    fn set_window_title(&self, view: usize, title: &str);
    /// Marks `view` as needing display (`setNeedsDisplay:YES`) on the main thread.
    /// Example: `calls.invalidate_view(view)`.
    fn invalidate_view(&self, view: usize);
    /// The retained `CAMetalLayer*` backing `view`, installing one sized to the backing
    /// scale when the view has none; `None` off the main thread when no Metal layer is
    /// installed yet. Example: `calls.metal_layer(view)`.
    fn metal_layer(&self, view: usize) -> Option<usize>;
}

/// The AppKit of this host, `None` on platforms without AppKit; `native_appkit()`.
pub fn native_appkit() -> Option<Arc<dyn CocoaAppKitCalls>> {
    #[cfg(target_os = "macos")]
    {
        Some(Arc::new(super::appkit_native::NativeAppKitCalls))
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}
