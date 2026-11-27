//! Loader-selected validation-layer provenance keeps renderer gates independent of host packages.
use super::validation_settings::VALIDATION_LAYER;
use crate::error::{RenderError, rejected};
use ash::vk;
pub use martensite_render_core::{VERIFIED_VALIDATION_LAYER, ValidationLayerVersion};

/// Read the Khronos validation layer the loader would enable, if any.
///
/// Example: `assert!(active_validation_layer()? >= Some(VERIFIED_VALIDATION_LAYER))`.
pub fn active_validation_layer() -> Result<Option<ValidationLayerVersion>, RenderError> {
    let entry =
        // SAFETY: the Vulkan loader library is the system's, and loading it runs only its own initializers.
        unsafe { ash::Entry::load() }.map_err(|e| rejected(e, "a loadable Vulkan 1.3 runtime"))?;
    loader_validation_layer(&entry).map_err(|e| rejected(e, "enumerable Vulkan instance layers"))
}

/// Read the layer from a loaded runtime; `loader_validation_layer(&entry)?`.
pub(super) fn loader_validation_layer(
    entry: &ash::Entry,
) -> Result<Option<ValidationLayerVersion>, vk::Result> {
    // SAFETY: this Entry keeps the loader alive, and the call only reads through out-parameters it owns.
    let layers = unsafe { entry.enumerate_instance_layer_properties() }?;
    Ok(layers
        .iter()
        .find(|layer| layer.layer_name_as_c_str() == Ok(VALIDATION_LAYER))
        .map(|layer| ValidationLayerVersion::from_spec(layer.spec_version)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spec_versions_order_the_legacy_layer_below_the_verified_dependency() {
        let legacy = ValidationLayerVersion::from_spec(vk::make_api_version(0, 1, 3, 275));
        let pinned = ValidationLayerVersion::from_spec(vk::make_api_version(0, 1, 4, 313));
        assert_eq!((legacy.major, legacy.minor, legacy.patch), (1, 3, 275));
        assert_eq!(pinned, VERIFIED_VALIDATION_LAYER);
        assert!(legacy < VERIFIED_VALIDATION_LAYER);
    }
}
