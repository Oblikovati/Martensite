//! Validation-layer configuration through VK_EXT_layer_settings, which deprecates
//! VK_EXT_validation_features.
use ash::vk;
use std::ffi::CStr;

/// The layer-provided settings extension; enabled only with the validation layer.
pub(super) const LAYER_SETTINGS_EXTENSION: &CStr = c"VK_EXT_layer_settings";
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
/// One VkBool32 `true`, referenced by the setting for the instance lifetime.
static ENABLED: vk::Bool32 = vk::TRUE;

/// Enable synchronization validation; `LayerSettingsCreateInfoEXT::default().settings(&settings)`.
pub(super) fn synchronization_settings() -> [vk::LayerSettingEXT<'static>; 1] {
    let mut sync = vk::LayerSettingEXT::default()
        .layer_name(VALIDATION_LAYER)
        .setting_name(c"validate_sync")
        .ty(vk::LayerSettingTypeEXT::BOOL32);
    // ash's `values` counts bytes; a BOOL32 setting counts VkBool32 elements.
    sync.value_count = 1;
    sync.p_values = std::ptr::from_ref(&ENABLED).cast();
    [sync]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sync_validation_is_one_true_boolean_for_the_khronos_layer() {
        let [setting] = synchronization_settings();
        assert_eq!(
            // SAFETY: the layer setting holds NUL-terminated names that outlive this read.
            unsafe { setting.layer_name_as_c_str() },
            Some(VALIDATION_LAYER)
        );
        assert_eq!(
            // SAFETY: the layer setting holds NUL-terminated names that outlive this read.
            unsafe { setting.setting_name_as_c_str() },
            Some(c"validate_sync")
        );
        assert_eq!(
            (setting.ty, setting.value_count),
            (vk::LayerSettingTypeEXT::BOOL32, 1)
        );
        // SAFETY: the setting declares one boolean value at this live pointer.
        assert_eq!(unsafe { *setting.p_values.cast::<vk::Bool32>() }, vk::TRUE);
    }
}
