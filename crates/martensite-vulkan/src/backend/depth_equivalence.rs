//! OpenGL depth equivalence: the native view renders into `GL_DEPTH24_STENCIL8` and maps clip depth
//! with the OpenGL viewport transform. The Vulkan scene selects 24-bit depth where the device
//! offers it as an attachment (32-bit float otherwise) and, with 24-bit depth and
//! `VK_EXT_depth_clip_control`, lets the viewport transform apply OpenGL's `[-1, 1]` depth
//! mapping instead of remapping clip depth in the vertex shaders (`kRemapClipDepth`,
//! specialization constant 7).
use ash::{Instance, vk};

/// The depth attachment formats of a device; `formats.of(true)` carries stencil.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DepthFormats {
    pub plain: vk::Format,
    pub stencil: vk::Format,
}

impl Default for DepthFormats {
    fn default() -> Self {
        Self::FLOAT
    }
}

impl DepthFormats {
    /// The 32-bit float fallback.
    pub const FLOAT: Self = Self {
        plain: vk::Format::D32_SFLOAT,
        stencil: vk::Format::D32_SFLOAT_S8_UINT,
    };
    /// `GL_DEPTH24_STENCIL8`, and its stencil-less 24-bit layout.
    pub const UNORM24: Self = Self {
        plain: vk::Format::X8_D24_UNORM_PACK32,
        stencil: vk::Format::D24_UNORM_S8_UINT,
    };
    /// The attachment format with or without stencil; `formats.of(false)`.
    pub fn of(self, stencil: bool) -> vk::Format {
        if stencil { self.stencil } else { self.plain }
    }
}

/// Whether a depth format stores 24-bit normalized depth in 32-bit texels.
pub(crate) fn is_unorm24(format: vk::Format) -> bool {
    matches!(
        format,
        vk::Format::X8_D24_UNORM_PACK32 | vk::Format::D24_UNORM_S8_UINT
    )
}

/// Decode one depth texel copied from the depth aspect; `depth_value(format, word)`.
pub(crate) fn depth_value(format: vk::Format, word: [u8; 4]) -> f32 {
    if is_unorm24(format) {
        // Buffer copies of 24-bit depth place it in the low 24 bits of each 32-bit texel.
        let bits = u32::from_ne_bytes(word) & 0x00ff_ffff;
        return (f64::from(bits) / f64::from(0x00ff_ffff_u32)) as f32;
    }
    f32::from_ne_bytes(word)
}

/// The value a capture reads back for `window` depth stored in `format`; `stored_depth(f, 0.5)`.
pub(crate) fn stored_depth(format: vk::Format, window: f32) -> f32 {
    if is_unorm24(format) {
        let bits = (f64::from(window.clamp(0., 1.)) * f64::from(0x00ff_ffff_u32)).round() as u32;
        return depth_value(format, bits.to_ne_bytes());
    }
    window
}

/// The depth equivalence a device supports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DepthEquivalence {
    pub formats: DepthFormats,
    /// `VK_EXT_depth_clip_control` with `depthClipControl`, enabled at device creation.
    pub clip_control: bool,
}

impl DepthEquivalence {
    /// Probe 24-bit depth attachments and clip control; `DepthEquivalence::probe(&instance, physical)`.
    pub fn probe(instance: &Instance, physical: vk::PhysicalDevice) -> Self {
        let required =
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
        let supports = |format| {
            let properties =
                // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
                unsafe { instance.get_physical_device_format_properties(physical, format) };
            properties.optimal_tiling_features.contains(required)
        };
        let unorm = DepthFormats::UNORM24;
        let formats = if supports(unorm.plain) && supports(unorm.stencil) {
            unorm
        } else {
            DepthFormats::FLOAT
        };
        // OpenGL's depth mapping reproduces native depth only together with its 24-bit storage:
        // on a float-depth device the shader remap stays (measured on the shader-grid coplanar
        // ties: RADV with clip control and D32 differs from radeonsi on 6-7 pixels, with the
        // remap on 2-3; lavapipe with D24 and clip control improves from 15/12 to 14/7).
        let clip_control =
            formats == DepthFormats::UNORM24 && clip_control_supported(instance, physical);
        Self {
            formats,
            clip_control,
        }
    }
}

/// Whether `physical` lists the device extension `name`.
/// Example: `device_lists_extension(&instance, physical, ash::ext::depth_clip_control::NAME)`.
pub(super) fn device_lists_extension(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    name: &std::ffi::CStr,
) -> bool {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.enumerate_device_extension_properties(physical) }
        .unwrap_or_default()
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}

fn clip_control_supported(instance: &Instance, physical: vk::PhysicalDevice) -> bool {
    if !device_lists_extension(instance, physical, ash::ext::depth_clip_control::NAME) {
        return false;
    }
    let mut feature = vk::PhysicalDeviceDepthClipControlFeaturesEXT::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut feature);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    feature.depth_clip_control == vk::TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unorm24_texels_decode_like_opengl_depth_reads() {
        let format = vk::Format::D24_UNORM_S8_UINT;
        assert_eq!(depth_value(format, 0x00ff_ffff_u32.to_ne_bytes()), 1.0);
        assert_eq!(depth_value(format, 0xab00_0000_u32.to_ne_bytes()), 0.0);
        let half = depth_value(format, 0x0080_0000_u32.to_ne_bytes());
        assert_eq!(half, (8_388_608.0_f64 / 16_777_215.0) as f32);
        assert_eq!(
            depth_value(vk::Format::D32_SFLOAT, 0.25f32.to_ne_bytes()),
            0.25
        );
    }

    #[test]
    fn stored_depth_rounds_to_the_nearest_24_bit_step_and_keeps_float() {
        let unorm = DepthFormats::UNORM24.plain;
        assert_eq!(
            stored_depth(unorm, 0.5),
            (8_388_608.0_f64 / 16_777_215.0) as f32
        );
        assert_eq!(stored_depth(unorm, 1.0), 1.0);
        assert_eq!(stored_depth(unorm, 0.0), 0.0);
        assert_eq!(stored_depth(DepthFormats::FLOAT.plain, 0.5), 0.5);
    }

    #[test]
    fn formats_follow_the_stencil_request() {
        assert_eq!(
            DepthFormats::UNORM24.of(true),
            vk::Format::D24_UNORM_S8_UINT
        );
        assert_eq!(
            DepthFormats::UNORM24.of(false),
            vk::Format::X8_D24_UNORM_PACK32
        );
        assert_eq!(DepthFormats::FLOAT.of(true), vk::Format::D32_SFLOAT_S8_UINT);
        assert!(!is_unorm24(DepthFormats::FLOAT.plain));
    }
}
