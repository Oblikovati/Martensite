//! Read the device facts behind a view's diagnostic dictionary: the Vulkan counterpart of the
//! strings GlContext::DiagnosticInformation queries from the GL and GLX implementations.
use crate::view_diagnostics::RenderDiagnosticFacts;
use ash::vk;
use std::ffi::CStr;

impl super::VulkanRenderer {
    /// Snapshot loader, device and attachment facts; `renderer.diagnostic_facts()`.
    pub(crate) fn diagnostic_facts(&self) -> RenderDiagnosticFacts {
        let runtime = &self.runtime;
        // The instance and physical device stay alive for the renderer's lifetime.
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let properties = unsafe {
            runtime
                .instance
                .get_physical_device_properties(runtime.physical)
        };
        let (color, _, _) =
            super::image::attachment_properties(runtime.depth.formats, false, false);
        let (depth_stencil, _, _) =
            super::image::attachment_properties(runtime.depth.formats, true, true);
        RenderDiagnosticFacts {
            instance_version: self.instance_version(),
            api_version: properties.api_version,
            validation: runtime.validation.is_enabled(),
            instance_extensions: self.instance_extensions(),
            device_extensions: self.device_extensions(),
            scene_formats: [color, depth_stencil].map(format_name),
            frame_prepared: runtime.image_memory.frame_target_bytes() > 0,
        }
    }
    fn instance_version(&self) -> u32 {
        // Vulkan 1.0 loaders lack the query; the instance then reports 1.0.
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe { self.runtime.entry.try_enumerate_instance_version() }
            .ok()
            .flatten()
            .unwrap_or(vk::API_VERSION_1_0)
    }
    fn instance_extensions(&self) -> Vec<String> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let available = unsafe {
            self.runtime
                .entry
                .enumerate_instance_extension_properties(None)
        };
        available
            .unwrap_or_default()
            .iter()
            .map(extension_name)
            .collect()
    }
    fn device_extensions(&self) -> Vec<String> {
        let runtime = &self.runtime;
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let available = unsafe {
            runtime
                .instance
                .enumerate_device_extension_properties(runtime.physical)
        };
        available
            .unwrap_or_default()
            .iter()
            .map(extension_name)
            .collect()
    }
}

fn extension_name(properties: &vk::ExtensionProperties) -> String {
    // Vulkan guarantees a null terminator inside the fixed-size name array.
    // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
    unsafe { CStr::from_ptr(properties.extension_name.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

/// Vulkan enumerant spelling, as `GlTextureFormat::FormatFormat` spells GL formats.
fn format_name(format: vk::Format) -> String {
    format!("VK_FORMAT_{format:?}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_names_use_the_vulkan_enumerant_spelling() {
        assert_eq!(
            super::format_name(ash::vk::Format::D32_SFLOAT_S8_UINT),
            "VK_FORMAT_D32_SFLOAT_S8_UINT"
        );
        assert_eq!(
            super::format_name(ash::vk::Format::R8G8B8A8_SRGB),
            "VK_FORMAT_R8G8B8A8_SRGB"
        );
    }
}
