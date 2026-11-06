# Validation instance admission

Validated Vulkan startup queries the global extension catalog, then the
`VK_LAYER_KHRONOS_validation` catalog. Both catalogs can supply
`VK_EXT_debug_utils` and `VK_EXT_layer_settings`. The loader reports a missing
layer through `VK_ERROR_LAYER_NOT_PRESENT`.

Admission then reads the layer's version from the loader and refuses anything
older than `VERIFIED_VALIDATION_LAYER`, 1.4.313. The catalogs do not cover this:
an implicit layer can supply `VK_EXT_layer_settings` in the global catalog, as
Mesa's device selection layer does, while the validation layer itself is too old
for the renderer. Ubuntu 24.04's 1.3.275 starts that way, then warns at device
creation that it does not know `VK_KHR_line_rasterization`, reports a false
depth-resolve hazard and can crash under parallel instances.
`scripts/install/vulkan-validation.sh` installs the verified layer on Ubuntu
24.04 x86_64. Elsewhere, install 1.4.313 or newer, or start with validation off.

`RenderError::validation_admission_failure()` returns the missing layer or
extension, or the too-old layer with both versions, as
`RenderValidationAdmissionFailure`. Other query failures retain the operation
and exact Vulkan result in `RenderDeviceFailure`. Admission runs before
`vkCreateInstance`; it does not change the enabled extension order, portability
flags or synchronization settings. A missing extension is named before the
version is read. Startup with validation disabled does not query the validation
layer.

The catalog contract follows
[`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html).
Named catalog fakes test missing names, global and layer providers, query errors,
layer versions and disabled validation. `tests/validation_layer_dependency.rs`
checks admission and refusal against the real loader. Existing portability tests check the exact extension
order and flags. Native rendering tests check successful instance and frame
creation with validation enabled.

This fixes the diagnostic part of [#388](https://github.com/Oblikovati/Martensite/issues/388).
Linux ARM64 still needs a qualified validation layer and software Vulkan driver.
This change does not provide that runtime or prove ARM64 frame acceptance.
