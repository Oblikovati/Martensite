//! CPU admission tests use catalogs, not the installed loader or GPU.
use super::super::portability_admission::fixture::PortabilityCatalogFake;
use super::*;
use std::cell::{Cell, RefCell};

const fn layer_version(major: u32, minor: u32, patch: u32) -> ValidationLayerVersion {
    ValidationLayerVersion {
        major,
        minor,
        patch,
    }
}
const UBUNTU_LAYER: ValidationLayerVersion = layer_version(1, 3, 275);

struct ValidationCatalogFake {
    reply: Result<Vec<vk::ExtensionProperties>, vk::Result>,
    queries: RefCell<Vec<String>>,
    active: Result<Option<ValidationLayerVersion>, vk::Result>,
    active_queries: Cell<usize>,
}
impl InstanceExtensionSource for ValidationCatalogFake {
    fn extensions(&self, layer: Option<&CStr>) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        self.queries
            .borrow_mut()
            .push(layer.unwrap().to_string_lossy().into_owned());
        self.reply.clone()
    }
    fn active_layer(&self) -> Result<Option<ValidationLayerVersion>, vk::Result> {
        self.active_queries.set(self.active_queries.get() + 1);
        self.active
    }
}
impl ValidationCatalogFake {
    fn named(names: &[&CStr]) -> Self {
        Self::replying(Ok(PortabilityCatalogFake::named(names).extensions))
    }
    fn refusing(error: vk::Result) -> Self {
        Self::replying(Err(error))
    }
    fn replying(reply: Result<Vec<vk::ExtensionProperties>, vk::Result>) -> Self {
        Self {
            reply,
            queries: RefCell::default(),
            active: Ok(Some(VERIFIED_VALIDATION_LAYER)),
            active_queries: Cell::default(),
        }
    }
    fn with_active(mut self, active: Result<Option<ValidationLayerVersion>, vk::Result>) -> Self {
        self.active = active;
        self
    }
}
fn global_catalog_with_both_extensions() -> Vec<vk::ExtensionProperties> {
    PortabilityCatalogFake::named(&[
        ash::ext::debug_utils::NAME,
        validation_settings::LAYER_SETTINGS_EXTENSION,
    ])
    .extensions
}
#[test]
fn missing_layer_settings_is_named_before_instance_creation() {
    let source = ValidationCatalogFake::named(&[ash::ext::debug_utils::NAME]);
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert_eq!(
        error.validation_admission_failure(),
        Some(&crate::RenderValidationAdmissionFailure::MissingExtension {
            layer: "VK_LAYER_KHRONOS_validation".to_owned(),
            extension: "VK_EXT_layer_settings".to_owned(),
        })
    );
    assert!(error.to_string().contains("VK_EXT_layer_settings"));
    assert!(error.to_string().contains("VK_LAYER_KHRONOS_validation"));
}
#[test]
fn missing_validation_layer_is_named() {
    let source = ValidationCatalogFake::refusing(vk::Result::ERROR_LAYER_NOT_PRESENT);
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert!(error.to_string().contains("VK_LAYER_KHRONOS_validation"));
}
#[test]
fn validation_disabled_does_not_query_or_require_the_layer() {
    let source = ValidationCatalogFake::named(&[]);
    admit_validation(&source, &[], false).unwrap();
    assert!(source.queries.borrow().is_empty());
    assert_eq!(source.active_queries.get(), 0);
}
#[test]
fn layer_and_global_catalogs_jointly_supply_required_extensions() {
    let source = ValidationCatalogFake::named(&[validation_settings::LAYER_SETTINGS_EXTENSION]);
    let global = PortabilityCatalogFake::named(&[ash::ext::debug_utils::NAME]);
    admit_validation(&source, &global.extensions, true).unwrap();
    assert_eq!(*source.queries.borrow(), ["VK_LAYER_KHRONOS_validation"]);
}
#[test]
fn validation_layer_can_supply_both_extensions_without_global_support() {
    let source = ValidationCatalogFake::named(&[
        ash::ext::debug_utils::NAME,
        validation_settings::LAYER_SETTINGS_EXTENSION,
    ]);
    admit_validation(&source, &[], true).unwrap();
    assert_eq!(source.queries.borrow().len(), 1);
}
#[test]
fn missing_debug_utils_is_named_even_if_layer_settings_is_available() {
    let source = ValidationCatalogFake::named(&[validation_settings::LAYER_SETTINGS_EXTENSION]);
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert!(error.to_string().contains("VK_EXT_debug_utils"));
}
#[test]
fn failed_layer_query_preserves_the_exact_backend_result() {
    let source = ValidationCatalogFake::refusing(vk::Result::ERROR_OUT_OF_HOST_MEMORY);
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
}

#[test]
fn global_extensions_do_not_hide_an_absent_enabled_layer() {
    let source = ValidationCatalogFake::refusing(vk::Result::ERROR_LAYER_NOT_PRESENT);
    let global = PortabilityCatalogFake::named(&[
        ash::ext::debug_utils::NAME,
        validation_settings::LAYER_SETTINGS_EXTENSION,
    ]);
    let error = admit_validation(&source, &global.extensions, true).unwrap_err();
    assert_eq!(
        error.validation_admission_failure(),
        Some(&crate::RenderValidationAdmissionFailure::MissingLayer {
            layer: "VK_LAYER_KHRONOS_validation".to_owned(),
        })
    );
}
#[test]
fn global_extensions_can_supply_both_required_names() {
    let source = ValidationCatalogFake::named(&[]);
    let global = PortabilityCatalogFake::named(&[
        ash::ext::debug_utils::NAME,
        validation_settings::LAYER_SETTINGS_EXTENSION,
    ]);
    admit_validation(&source, &global.extensions, true).unwrap();
    assert_eq!(source.queries.borrow().len(), 1);
}
// Mesa's implicit device selection layer can supply the settings extension globally, which is how
// the distribution's old layer reached device creation before its version was checked.
#[test]
fn old_layer_is_refused_with_both_versions_when_another_layer_supplies_the_extensions() {
    let source = ValidationCatalogFake::named(&[]).with_active(Ok(Some(UBUNTU_LAYER)));
    let error =
        admit_validation(&source, &global_catalog_with_both_extensions(), true).unwrap_err();
    assert_eq!(
        error.validation_admission_failure(),
        Some(&crate::RenderValidationAdmissionFailure::UnverifiedLayer {
            layer: "VK_LAYER_KHRONOS_validation".to_owned(),
            active: UBUNTU_LAYER,
            required: VERIFIED_VALIDATION_LAYER,
        })
    );
    assert!(error.to_string().contains("1.3.275"));
    assert!(error.to_string().contains("1.4.313"));
}
#[test]
fn the_layer_just_below_the_verified_version_is_refused() {
    let below = layer_version(1, 4, 312);
    let source = ValidationCatalogFake::named(&[]).with_active(Ok(Some(below)));
    let error =
        admit_validation(&source, &global_catalog_with_both_extensions(), true).unwrap_err();
    assert!(error.to_string().contains("1.4.312"));
}
#[test]
fn verified_and_newer_layers_are_admitted() {
    for active in [VERIFIED_VALIDATION_LAYER, layer_version(1, 4, 363)] {
        let source = ValidationCatalogFake::named(&[]).with_active(Ok(Some(active)));
        admit_validation(&source, &global_catalog_with_both_extensions(), true).unwrap();
        assert_eq!(source.active_queries.get(), 1);
    }
}
#[test]
fn a_missing_extension_is_named_before_the_layer_version_is_read() {
    let source = ValidationCatalogFake::named(&[]).with_active(Ok(Some(UBUNTU_LAYER)));
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert!(error.to_string().contains("VK_EXT_debug_utils"));
    assert_eq!(source.active_queries.get(), 0);
}
#[test]
fn a_layer_absent_from_the_list_is_named_as_missing() {
    let source = ValidationCatalogFake::named(&[]).with_active(Ok(None));
    let error =
        admit_validation(&source, &global_catalog_with_both_extensions(), true).unwrap_err();
    assert_eq!(
        error.validation_admission_failure(),
        Some(&crate::RenderValidationAdmissionFailure::MissingLayer {
            layer: "VK_LAYER_KHRONOS_validation".to_owned(),
        })
    );
}
#[test]
fn a_failed_layer_list_preserves_the_exact_backend_result() {
    let source =
        ValidationCatalogFake::named(&[]).with_active(Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY));
    let error =
        admit_validation(&source, &global_catalog_with_both_extensions(), true).unwrap_err();
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
}
