//! Cocoa: the AppKit window and the autorelease pool of macOS.
//!
//! The AppKit backend is not implemented yet: the window and pool constructors raise
//! `Standard_NotImplemented` on every platform (see [`crate::platform`]). The key code table
//! of the window is platform-neutral and implemented.
// TODO(platform): macOS — create and drive NSWindow/NSView and NSAutoreleasePool through
// AppKit in the constructors, Map/Unmap/DoResize/DoMapping, SetHView, SetTitle and
// InvalidateContent.
mod local_pool;
mod virtual_keys;
mod window;

pub use local_pool::CocoaLocalPool;
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
#[path = "tests.rs"]
mod tests;
