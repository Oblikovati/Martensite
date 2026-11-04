//! Native presentation surfaces are platform-specific; offscreen rendering never depends on them.
use std::fmt;

/// The native window system a Vulkan presentation surface is created from.
///
/// Example: `assert_eq!(RenderSurfacePlatform::host(), RenderSurfacePlatform::Xlib)` on Linux.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderSurfacePlatform {
    /// An X11 display connection and window XID through `VK_KHR_xlib_surface`.
    Xlib,
    /// A Win32 `HINSTANCE`/`HWND` pair through `VK_KHR_win32_surface`.
    Win32,
    /// A `CAMetalLayer` through `VK_EXT_metal_surface` (MoltenVK); never an `NSView`.
    Metal,
}

impl RenderSurfacePlatform {
    /// The window system this build presents through; `RenderSurfacePlatform::host()`.
    pub const fn host() -> Self {
        if cfg!(target_os = "windows") {
            Self::Win32
        } else if cfg!(target_vendor = "apple") {
            Self::Metal
        } else {
            Self::Xlib
        }
    }
    /// The native handle shape this platform's surface requires; `platform.native_target()`.
    pub const fn native_target(self) -> &'static str {
        match self {
            Self::Xlib => "a live Xlib display connection and window XID",
            Self::Win32 => "a live Win32 HINSTANCE and HWND",
            Self::Metal => "a live CAMetalLayer (an NSView is not a CAMetalLayer)",
        }
    }
}

/// A native presentation surface cannot be created; match `error.surface_platform_failure()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderSurfacePlatformFailure {
    /// The platform's surface interface is defined but not yet implemented.
    Unsupported {
        /// The window system that was asked to create the surface.
        platform: RenderSurfacePlatform,
    },
}

impl fmt::Display for RenderSurfacePlatformFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { platform } => write!(
                formatter,
                "received {platform:?} presentation surface request; expected {} on an implemented platform (Xlib)",
                platform.native_target()
            ),
        }
    }
}
impl std::error::Error for RenderSurfacePlatformFailure {}
