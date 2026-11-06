//! Diagnostic dictionary of a Vulkan view: the capability of GlView::DiagnosticInformation
//! with GlContext::DiagnosticInformation. Categories and their order follow the
//! native body. Values that keep their meaning under Vulkan keep the native key (limits,
//! viewport, window buffer, memory, resolution ratio, FBO buffer); GL and GLX strings become the
//! Vulkan loader, device and extension strings under `VK*` keys.
use crate::{RenderDeviceLimits, RenderDeviceMemoryReport, RenderDriverIdentity};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::{
    Graphic3dCView as _, Graphic3dCViewMembers as _, Graphic3dDiagnosticInfo as Info,
    Graphic3dFrameStatsDict,
};

impl crate::Graphic3dRenderView {
    /// Collect the device and window inputs of the dictionary; `view.diagnostics()`.
    pub(crate) fn diagnostics(&self) -> ViewDiagnostics {
        let device = self.device.read();
        let window = self.window();
        ViewDiagnostics {
            facts: device.diagnostic_facts(),
            identity: device.driver_identity(),
            limits: device.device_limits(),
            memory: device.memory_report().ok(),
            viewport: window.get().map(|window| window.size()).unwrap_or_default(),
            resolution_ratio: self.rendering_params().resolution_ratio(),
        }
    }
}

/// Loader, device and attachment facts of the selected Vulkan device.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct RenderDiagnosticFacts {
    /// `vkEnumerateInstanceVersion`.
    pub instance_version: u32,
    /// `VkPhysicalDeviceProperties::apiVersion`.
    pub api_version: u32,
    /// Whether the Khronos validation layer observes the device (native `GLdebug`).
    pub validation: bool,
    /// Instance extensions offered by the loader.
    pub instance_extensions: Vec<String>,
    /// Device extensions offered by the physical device.
    pub device_extensions: Vec<String>,
    /// Scene color and depth-stencil attachment formats.
    pub scene_formats: [String; 2],
    /// Whether a frame has created the scene attachments (native: the main FBO is valid).
    pub frame_prepared: bool,
}

/// Everything the dictionary reads; `ViewDiagnostics { facts: None, .. }` for injected devices.
pub(crate) struct ViewDiagnostics {
    pub facts: Option<RenderDiagnosticFacts>,
    pub identity: Option<RenderDriverIdentity>,
    pub limits: Option<RenderDeviceLimits>,
    pub memory: Option<RenderDeviceMemoryReport>,
    pub viewport: (i32, i32),
    pub resolution_ratio: f32,
}

impl ViewDiagnostics {
    /// Append the requested categories in native order; without a device natively the view
    /// returns before the context adds anything. Example: `diagnostics.fill(&mut dict, flags)`.
    pub fn fill(&self, dict: &mut Graphic3dFrameStatsDict, flags: Info) {
        let Some(facts) = &self.facts else {
            return;
        };
        let has = |category: Info| flags.0 & category.0 != 0;
        if has(Info::NATIVE_PLATFORM) {
            native_platform(dict, facts, has(Info::EXTENSIONS));
        }
        if has(Info::DEVICE) {
            self.device(dict, facts);
        }
        if has(Info::LIMITS) {
            self.limits(dict);
        }
        self.context_tail(dict, facts, &has);
    }
    /// Window, memory and extensions of the context body, then the view's own entries.
    fn context_tail(
        &self,
        dict: &mut Graphic3dFrameStatsDict,
        facts: &RenderDiagnosticFacts,
        has: &dyn Fn(Info) -> bool,
    ) {
        if has(Info::FRAME_BUFFER) {
            self.frame_buffer(dict);
        }
        if has(Info::MEMORY) {
            self.memory(dict);
        }
        if has(Info::EXTENSIONS) {
            add_info(dict, "VKextensions", &facts.device_extensions.join(" "));
        }
        if has(Info::FRAME_BUFFER) {
            self.scene_buffer(dict, facts);
        }
    }
    fn device(&self, dict: &mut Graphic3dFrameStatsDict, facts: &RenderDiagnosticFacts) {
        if let Some(identity) = &self.identity {
            add_info(dict, "VKvendor", &format!("{:#06x}", identity.vendor_id));
            add_info(dict, "VKdevice", &identity.device_name);
            let driver = format!("{} {}", identity.driver_name, identity.driver_info);
            add_info(dict, "VKdriver", driver.trim());
        }
        add_info(dict, "VKversion", &version_text(facts.api_version));
        if facts.validation {
            add_info(dict, "VKdebug", "ON");
        }
    }
    fn limits(&self, dict: &mut Graphic3dFrameStatsDict) {
        let Some(limits) = &self.limits else {
            return;
        };
        let texture = limits.max_texture_size().to_string();
        add_info(dict, "Max texture size", &texture);
        let dump = format!("{}x{}", limits.max_dump_size_x(), limits.max_dump_size_y());
        add_info(dict, "Max FBO dump size", &dump);
        let units = limits.max_combined_texture_units().to_string();
        add_info(dict, "Max combined texture units", &units);
        add_info(
            dict,
            "Max MSAA samples",
            &limits.max_msaa_samples().to_string(),
        );
    }
    fn frame_buffer(&self, dict: &mut Graphic3dFrameStatsDict) {
        let (width, height) = self.viewport;
        add_info(dict, "Viewport", &format!("{width}x{height}"));
        // Captures and swapchains are RGBA8/BGRA8 sRGB images without depth or stencil
        // (window_support::surface_format); depth lives only in the scene attachments.
        add_info(dict, "Window buffer", "RGB8 ALPHA8 DEPTH0 STENCIL0");
    }
    fn memory(&self, dict: &mut Graphic3dFrameStatsDict) {
        let Some(report) = &self.memory else {
            return;
        };
        let heaps = report.heaps();
        let local_free = free_bytes(heaps.iter().filter(|heap| heap.is_device_local()));
        let host_free = free_bytes(heaps.iter().filter(|heap| !heap.is_device_local()));
        if let Some(free) = local_free {
            add_info(dict, "GPU free memory", &mebibytes(free));
        }
        if let Some(free) = host_free {
            add_info(dict, "Free auxiliary memory", &mebibytes(free));
        }
        add_info(
            dict,
            "GPU memory",
            &mebibytes(report.device_local_capacity_bytes()),
        );
    }
    fn scene_buffer(&self, dict: &mut Graphic3dFrameStatsDict, facts: &RenderDiagnosticFacts) {
        add_info(dict, "ResolutionRatio", &self.resolution_ratio.to_string());
        if facts.frame_prepared {
            add_info(dict, "FBO buffer", &facts.scene_formats.join(" "));
        }
    }
}

/// The loader in place of GLX: its version, and its instance extensions on request.
fn native_platform(dict: &mut Graphic3dFrameStatsDict, facts: &RenderDiagnosticFacts, all: bool) {
    add_info(
        dict,
        "VKloaderVersion",
        &version_text(facts.instance_version),
    );
    if all {
        add_info(
            dict,
            "VKinstanceExtensions",
            &facts.instance_extensions.join(" "),
        );
    }
}

/// Sum of the remaining budget estimates; unknown without heaps or when one lacks a budget.
fn free_bytes<'a>(heaps: impl Iterator<Item = &'a crate::RenderDeviceMemoryHeap>) -> Option<u64> {
    let mut total = None;
    for heap in heaps {
        let remaining = heap.budget().remaining_estimate_bytes()?;
        total = Some(total.unwrap_or(0) + remaining);
    }
    total
}

/// Native `bytes / 1024 / 1024 + " MiB"` (truncating).
fn mebibytes(bytes: u64) -> String {
    format!("{} MiB", bytes / (1024 * 1024))
}

/// `major.minor.patch` of a packed Vulkan version.
fn version_text(version: u32) -> String {
    format!(
        "{}.{}.{}",
        ash::vk::api_version_major(version),
        ash::vk::api_version_minor(version),
        ash::vk::api_version_patch(version)
    )
}

/// Native `addInfo`: insert the key, or overwrite its value when it already exists.
fn add_info(dict: &mut Graphic3dFrameStatsDict, key: &str, value: &str) {
    let value = AsciiString::from(value);
    let index = dict.add(AsciiString::from(key), value.clone());
    if let Ok(slot) = dict.change_from_index(index) {
        *slot = value;
    }
}

#[cfg(test)]
#[path = "view_diagnostics_tests.rs"]
mod tests;
