//! Optional line support must not exclude devices capable of the original triangle path.
use crate::{LineStyle, RenderError, error::rejected};
use ash::{Instance, vk};
use std::ffi::CStr;
#[derive(Clone, Copy, Default)]
pub(super) struct LineCapabilities {
    pub bresenham: bool,
    pub stippled: bool,
    pub rectangular: bool,
    pub stippled_rectangular: bool,
    pub wide: bool,
    /// The advertised line extension: KHR when present, else the deprecated EXT it promotes.
    pub extension: Option<&'static CStr>,
}
/// Prefer VK_KHR_line_rasterization; both share feature structures and enum values.
fn line_extension(extensions: &[vk::ExtensionProperties]) -> Option<&'static CStr> {
    let advertised = |name: &CStr| {
        extensions
            .iter()
            .any(|extension| extension.extension_name_as_c_str() == Ok(name))
    };
    [
        ash::khr::line_rasterization::NAME,
        ash::ext::line_rasterization::NAME,
    ]
    .into_iter()
    .find(|name| advertised(name))
}
/// The loaded `vkCmdSetLineStipple*` entry of the enabled line extension.
pub(super) enum LineStippleDevice {
    Khr(ash::khr::line_rasterization::Device),
    Ext(ash::ext::line_rasterization::Device),
}
impl LineStippleDevice {
    /// Load the entry of `extension`; `LineStippleDevice::new(instance, device, name)`.
    pub fn new(instance: &Instance, device: &ash::Device, extension: &CStr) -> Self {
        if extension == ash::khr::line_rasterization::NAME {
            return Self::Khr(ash::khr::line_rasterization::Device::new(instance, device));
        }
        Self::Ext(ash::ext::line_rasterization::Device::new(instance, device))
    }
    /// Record a stipple; the caller validated the stippled feature.
    pub unsafe fn set(&self, command: vk::CommandBuffer, factor: u32, pattern: u16) {
        match self {
            // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
            Self::Khr(device) => unsafe { device.cmd_set_line_stipple(command, factor, pattern) },
            // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
            Self::Ext(device) => unsafe {
                (device.fp().cmd_set_line_stipple_ext)(command, factor, pattern)
            },
        }
    }
}
impl LineCapabilities {
    pub fn query(instance: &Instance, physical: vk::PhysicalDevice) -> Result<Self, RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let extensions = unsafe { instance.enumerate_device_extension_properties(physical) }
            .map_err(|error| {
                super::runtime::vk_error("enumerate line rasterization support", error)
            })?;
        let extension = line_extension(&extensions);
        let wide =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { instance.get_physical_device_features(physical) }.wide_lines == vk::TRUE;
        if extension.is_none() {
            return Ok(Self {
                bresenham: false,
                stippled: false,
                rectangular: false,
                stippled_rectangular: false,
                wide,
                extension,
            });
        }
        let mut lines = vk::PhysicalDeviceLineRasterizationFeaturesKHR::default();
        let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut lines);
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe { instance.get_physical_device_features2(physical, &mut features) };
        Ok(Self {
            bresenham: lines.bresenham_lines == vk::TRUE,
            stippled: lines.stippled_bresenham_lines == vk::TRUE,
            rectangular: lines.rectangular_lines == vk::TRUE,
            stippled_rectangular: lines.stippled_rectangular_lines == vk::TRUE,
            wide,
            extension,
        })
    }
    pub fn validate(&self, style: LineStyle, range: [f32; 2]) -> Result<(), RenderError> {
        if !self.bresenham
            || (!self.wide && style.raster_width() != 1.)
            || !(range[0]..=range[1]).contains(&style.raster_width())
        {
            return Err(rejected(
                (style.width, self.bresenham, self.wide, range),
                "Bresenham line support and width within the enabled device range",
            ));
        }
        Ok(())
    }
    pub fn validate_multisampling(
        &self,
        samples: vk::SampleCountFlags,
        stippled: bool,
    ) -> Result<(), RenderError> {
        if samples == vk::SampleCountFlags::TYPE_1
            || (self.rectangular && (!stippled || self.stippled_rectangular))
        {
            return Ok(());
        }
        let observed = (
            samples,
            stippled,
            self.rectangular,
            self.stippled_rectangular,
        );
        Err(rejected(
            observed,
            "rectangular line support and rectangular stippling for multisample lines",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_sample_lines_keep_the_existing_capability_contract() {
        let absent = LineCapabilities::default();
        assert!(
            absent
                .validate_multisampling(vk::SampleCountFlags::TYPE_1, true)
                .is_ok()
        );
        assert!(
            absent
                .validate_multisampling(vk::SampleCountFlags::TYPE_8, false)
                .is_err()
        );
    }
    #[test]
    fn multisample_lines_require_explicit_rectangular_capabilities() {
        let rectangular = LineCapabilities {
            rectangular: true,
            ..Default::default()
        };
        assert!(
            rectangular
                .validate_multisampling(vk::SampleCountFlags::TYPE_8, false)
                .is_ok()
        );
        assert!(
            rectangular
                .validate_multisampling(vk::SampleCountFlags::TYPE_8, true)
                .is_err()
        );
    }
    #[test]
    fn multisample_stipple_requires_rectangular_stipple_support() {
        let stippled = LineCapabilities {
            rectangular: true,
            stippled_rectangular: true,
            ..Default::default()
        };
        assert!(
            stippled
                .validate_multisampling(vk::SampleCountFlags::TYPE_8, true)
                .is_ok()
        );
    }
    #[test]
    fn absent_line_and_wide_capabilities_reject_only_unsupported_requests() {
        let narrow = LineCapabilities {
            bresenham: true,
            stippled: false,
            wide: false,
            extension: None,
            ..Default::default()
        };
        assert!(
            narrow
                .validate(LineStyle::new(1., 0xffff, 1).unwrap(), [1., 8.])
                .is_ok()
        );
        assert!(
            narrow
                .validate(LineStyle::new(2., 0xffff, 1).unwrap(), [1., 8.])
                .is_err()
        );
        assert!(
            LineCapabilities::default()
                .validate(LineStyle::new(1., 0xffff, 1).unwrap(), [1., 8.])
                .is_err()
        );
    }
    #[test]
    fn subpixel_native_lines_use_one_pixel_without_requiring_wide_lines() {
        let narrow = LineCapabilities {
            bresenham: true,
            stippled: false,
            wide: false,
            extension: None,
            ..Default::default()
        };
        let style = LineStyle::new(0.5, 0xffff, 1).unwrap();
        assert_eq!(style.parameters().0, 0.5);
        assert_eq!(style.raster_width(), 1.);
        assert!(narrow.validate(style, [1., 8.]).is_ok());
    }
    #[test]
    fn promoted_khr_line_extension_is_preferred_over_deprecated_ext() {
        let named = |name: &CStr| {
            vk::ExtensionProperties::default()
                .extension_name(name)
                .unwrap()
        };
        let khr = ash::khr::line_rasterization::NAME;
        let ext = ash::ext::line_rasterization::NAME;
        assert_eq!(line_extension(&[named(ext), named(khr)]), Some(khr));
        assert_eq!(line_extension(&[named(ext)]), Some(ext));
        assert_eq!(line_extension(&[]), None);
    }
    #[test]
    fn widths_must_fit_the_selected_device_range() {
        let wide = LineCapabilities {
            bresenham: true,
            stippled: false,
            wide: true,
            extension: None,
            ..Default::default()
        };
        assert!(
            wide.validate(LineStyle::new(3., 0xffff, 1).unwrap(), [1., 8.])
                .is_ok()
        );
        assert!(
            wide.validate(LineStyle::new(9., 0xffff, 1).unwrap(), [1., 8.])
                .is_err()
        );
    }
}

#[cfg(test)]
mod forced_blend_tests {
    use super::*;
    #[test]
    fn forced_blending_requires_the_same_enabled_line_capabilities() {
        let style = LineStyle::new(1., 0xffff, 1)
            .unwrap()
            .with_alpha_blending(true);
        let error = LineCapabilities::default()
            .validate(style, [1., 8.])
            .unwrap_err();
        assert!(error.to_string().contains("Bresenham line support"));
        let narrow = LineCapabilities {
            bresenham: true,
            stippled: false,
            wide: false,
            extension: None,
            ..Default::default()
        };
        assert!(narrow.validate(style, [1., 8.]).is_ok());
        let wide_style = LineStyle::new(3., 0xffff, 1)
            .unwrap()
            .with_alpha_blending(true);
        assert!(narrow.validate(wide_style, [1., 8.]).is_err());
        assert!(
            LineCapabilities {
                bresenham: true,
                stippled: false,
                wide: true,
                extension: None,
                ..Default::default()
            }
            .validate(wide_style, [1., 2.])
            .is_err()
        );
    }
}
