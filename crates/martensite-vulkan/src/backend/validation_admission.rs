//! Check the selected validation layer's catalog and version before instance creation (#388).
use super::{
    portability_admission::advertises,
    runtime::vk_error,
    validation_layer::{VERIFIED_VALIDATION_LAYER, ValidationLayerVersion},
    validation_settings,
};
use crate::{RenderError, RenderValidationAdmissionFailure};
use ash::vk;
use std::ffi::CStr;

/// Query one loader catalog; `source.extensions(Some(layer))?`.
pub(super) trait InstanceExtensionSource {
    /// The optional layer selects the global or named catalog; `source.extensions(None)?`.
    fn extensions(&self, layer: Option<&CStr>) -> Result<Vec<vk::ExtensionProperties>, vk::Result>;
    /// Read the selected layer's version from the loader's list; `source.active_layer()?`.
    fn active_layer(&self) -> Result<Option<ValidationLayerVersion>, vk::Result>;
}

/// Admit the configured validation layer; `admit_validation(source, &global, enabled)?`.
pub(super) fn admit_validation(
    source: &impl InstanceExtensionSource,
    global: &[vk::ExtensionProperties],
    enabled: bool,
) -> Result<(), RenderError> {
    if !enabled {
        return Ok(());
    }
    let layer = source
        .extensions(Some(validation_settings::VALIDATION_LAYER))
        .map_err(validation_query_failure)?;
    for required in [
        ash::ext::debug_utils::NAME,
        validation_settings::LAYER_SETTINGS_EXTENSION,
    ] {
        require_validation_extension(global, &layer, required)?;
    }
    let active = source
        .active_layer()
        .map_err(|error| vk_error("enumerate instance layers", error))?;
    require_verified_layer(active)
}

// An implicit layer can supply the required extensions for a layer too old to validate this
// renderer, so the catalog check alone does not admit the layer itself.
fn require_verified_layer(active: Option<ValidationLayerVersion>) -> Result<(), RenderError> {
    let layer = validation_layer_name();
    let failure = match active {
        Some(active) if active >= VERIFIED_VALIDATION_LAYER => return Ok(()),
        Some(active) => RenderValidationAdmissionFailure::UnverifiedLayer {
            layer,
            active,
            required: VERIFIED_VALIDATION_LAYER,
        },
        None => RenderValidationAdmissionFailure::MissingLayer { layer },
    };
    Err(failure.into())
}

fn validation_layer_name() -> String {
    validation_settings::VALIDATION_LAYER
        .to_string_lossy()
        .into_owned()
}

fn require_validation_extension(
    global: &[vk::ExtensionProperties],
    layer: &[vk::ExtensionProperties],
    required: &CStr,
) -> Result<(), RenderError> {
    if advertises(global, required) || advertises(layer, required) {
        return Ok(());
    }
    Err(RenderValidationAdmissionFailure::MissingExtension {
        layer: validation_layer_name(),
        extension: required.to_string_lossy().into_owned(),
    }
    .into())
}

fn validation_query_failure(error: vk::Result) -> RenderError {
    if error == vk::Result::ERROR_LAYER_NOT_PRESENT {
        return RenderValidationAdmissionFailure::MissingLayer {
            layer: validation_layer_name(),
        }
        .into();
    }
    vk_error(
        "enumerate VK_LAYER_KHRONOS_validation instance extensions",
        error,
    )
}

#[cfg(test)]
#[path = "validation_admission_tests.rs"]
mod tests;
