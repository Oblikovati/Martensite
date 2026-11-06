//! CPU admission tests use catalogs, not the installed loader or GPU.
use super::super::portability_admission::fixture::PortabilityCatalogFake;
use super::*;
use std::cell::RefCell;

struct ValidationCatalogFake {
    reply: Result<Vec<vk::ExtensionProperties>, vk::Result>,
    queries: RefCell<Vec<String>>,
}
impl InstanceExtensionSource for ValidationCatalogFake {
    fn extensions(&self, layer: Option<&CStr>) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        self.queries
            .borrow_mut()
            .push(layer.unwrap().to_string_lossy().into_owned());
        self.reply.clone()
    }
}
impl ValidationCatalogFake {
    fn named(names: &[&CStr]) -> Self {
        Self {
            reply: Ok(PortabilityCatalogFake::named(names).extensions),
            queries: RefCell::default(),
        }
    }
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
    let source = ValidationCatalogFake {
        reply: Err(vk::Result::ERROR_LAYER_NOT_PRESENT),
        queries: RefCell::default(),
    };
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert!(error.to_string().contains("VK_LAYER_KHRONOS_validation"));
}
#[test]
fn validation_disabled_does_not_query_or_require_the_layer() {
    let source = ValidationCatalogFake::named(&[]);
    admit_validation(&source, &[], false).unwrap();
    assert!(source.queries.borrow().is_empty());
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
    let source = ValidationCatalogFake {
        reply: Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY),
        queries: RefCell::default(),
    };
    let error = admit_validation(&source, &[], true).unwrap_err();
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
}

#[test]
fn global_extensions_do_not_hide_an_absent_enabled_layer() {
    let source = ValidationCatalogFake {
        reply: Err(vk::Result::ERROR_LAYER_NOT_PRESENT),
        queries: RefCell::default(),
    };
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
