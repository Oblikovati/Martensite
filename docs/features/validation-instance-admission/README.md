# Validation instance admission

Validated Vulkan startup queries the global extension catalog, then the
`VK_LAYER_KHRONOS_validation` catalog. Both catalogs can supply
`VK_EXT_debug_utils` and `VK_EXT_layer_settings`. The loader reports a missing
layer through `VK_ERROR_LAYER_NOT_PRESENT`.

`RenderError::validation_admission_failure()` returns the missing layer or
extension as `RenderValidationAdmissionFailure`. Other query failures retain
the operation and exact Vulkan result in `RenderDeviceFailure`. Admission runs
before `vkCreateInstance`; it does not change the enabled extension order,
portability flags, synchronization settings, or validation policy. Startup with
validation disabled does not query the validation layer.

The catalog contract follows
[`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html).
Named catalog fakes test missing names, global and layer providers, query errors,
and disabled validation. Existing portability tests check the exact extension
order and flags. Native rendering tests check successful instance and frame
creation with validation enabled.

This fixes the diagnostic part of [#388](https://github.com/Oblikovati/Martensite/issues/388).
Linux ARM64 still needs a qualified validation layer and software Vulkan driver.
This change does not provide that runtime or prove ARM64 frame acceptance.
