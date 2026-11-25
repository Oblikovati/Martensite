//! Native presentation surfaces are platform-specific; offscreen rendering never depends on them.
use std::fmt;

/// The native window system a Vulkan presentation surface is created from.
///
/// On Linux both `Wayland` and `Xlib` are supported in parallel, with `Wayland` preferred
/// over XWayland whenever `WAYLAND_DISPLAY` is active.
///
/// Example: `assert_eq!(RenderSurfacePlatform::host(), RenderSurfacePlatform::Wayland)` on Linux.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderSurfacePlatform {
    /// A Wayland display connection and `wl_surface` through `VK_KHR_wayland_surface`.
    Wayland,
    /// An X11 display connection and window XID through `VK_KHR_xlib_surface`.
    Xlib,
    /// A Win32 `HINSTANCE`/`HWND` pair through `VK_KHR_win32_surface`.
    Win32,
    /// A `CAMetalLayer` through `VK_EXT_metal_surface` (MoltenVK); never an `NSView`.
    Metal,
}

impl RenderSurfacePlatform {
    /// The primary window system for this build; `RenderSurfacePlatform::host()`.
    pub const fn host() -> Self {
        if cfg!(target_os = "windows") {
            Self::Win32
        } else if cfg!(target_vendor = "apple") {
            Self::Metal
        } else {
            Self::Wayland
        }
    }
    /// All window systems supported in parallel by this build, in preference order.
    ///
    /// Example: `assert_eq!(RenderSurfacePlatform::host_supported()[0], RenderSurfacePlatform::Wayland)` on Linux.
    pub const fn host_supported() -> &'static [Self] {
        if cfg!(target_os = "windows") {
            &[Self::Win32]
        } else if cfg!(target_vendor = "apple") {
            &[Self::Metal]
        } else {
            &[Self::Wayland, Self::Xlib]
        }
    }
    /// The preferred window system for the current runtime environment.
    ///
    /// On Linux, `Wayland` is preferred over XWayland whenever `WAYLAND_DISPLAY` is non-empty;
    /// `Xlib` is chosen when only `DISPLAY` is set.
    ///
    /// Example: `let platform = RenderSurfacePlatform::preferred_host();`.
    pub fn preferred_host() -> Self {
        #[cfg(target_os = "linux")]
        {
            let wl = std::env::var("WAYLAND_DISPLAY").ok();
            let x11 = std::env::var("DISPLAY").ok();
            preferred_linux_platform(wl.as_deref(), x11.as_deref())
        }
        #[cfg(not(target_os = "linux"))]
        {
            Self::host()
        }
    }
    /// The native handle shape this platform's surface requires; `platform.native_target()`.
    pub const fn native_target(self) -> &'static str {
        match self {
            Self::Wayland => "a live Wayland display connection and wl_surface",
            Self::Xlib => "a live Xlib display connection and window XID",
            Self::Win32 => "a live Win32 HINSTANCE and HWND",
            Self::Metal => "a live CAMetalLayer (an NSView is not a CAMetalLayer)",
        }
    }
}

/// Pure Linux preference selector: Wayland always beats XWayland when `WAYLAND_DISPLAY` is set.
#[cfg(target_os = "linux")]
pub(crate) fn preferred_linux_platform(
    wayland_display: Option<&str>,
    x11_display: Option<&str>,
) -> RenderSurfacePlatform {
    if wayland_display.is_some_and(|v| !v.is_empty()) {
        return RenderSurfacePlatform::Wayland;
    }
    if x11_display.is_some_and(|v| !v.is_empty()) {
        return RenderSurfacePlatform::Xlib;
    }
    RenderSurfacePlatform::Wayland
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
                "received {platform:?} presentation surface request; expected {} on an implemented platform (Wayland, Xlib)",
                platform.native_target()
            ),
        }
    }
}
impl std::error::Error for RenderSurfacePlatformFailure {}
