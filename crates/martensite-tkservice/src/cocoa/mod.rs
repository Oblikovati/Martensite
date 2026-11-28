//! Cocoa: the AppKit window and the autorelease pool of macOS.
//!
//! On macOS the window and the pool drive AppKit through objc2; every AppKit call goes
//! through [`CocoaAppKitCalls`], so the window logic also runs against a fake. Elsewhere
//! the native constructors raise `Standard_NotImplemented` (see [`crate::platform`]). The
//! key code table of the window is platform-neutral.
mod appkit;
#[cfg(target_os = "macos")]
mod appkit_native;
mod local_pool;
mod metal_layer;
mod resize;
mod virtual_keys;
mod window;

pub use appkit::{CocoaAppKitCalls, CocoaCreatedWindow, CocoaRect, native_appkit};
pub use local_pool::CocoaLocalPool;
pub use metal_layer::CocoaMetalLayerLease;
pub use window::{CocoaWindow, CocoaWindowFields};

/// Declares the opaque AppKit object types, held as addresses (`usize`).
macro_rules! appkit_objects {
    ($($(#[$doc:meta])* $name:ident;)+) => {
        $(
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum $name {}
        )+
    };
}

appkit_objects! {
    /// `NSView`: an AppKit view (an address in the window fields).
    CocoaNsView;
    /// `NSWindow`: an AppKit window (an address in the window fields).
    CocoaNsWindow;
    /// `NSAutoreleasePool`: an AppKit autorelease pool (an address in the pool).
    CocoaNsAutoreleasePool;
}

#[cfg(test)]
mod fake_appkit;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
