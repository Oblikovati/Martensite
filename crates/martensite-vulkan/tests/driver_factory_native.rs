//! The Vulkan driver factory as viewers use it: native GlGraphicDriverFactory
//! creates each driver from its default options (`myDefaultCaps`), and DefaultOptions /
//! SetDefaultOptions read and replace them for drivers created later.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{Graphic3dGraphicDriverFactory, Graphic3dTypeOfLimit};
use martensite_vulkan::{Graphic3dRenderDriver, VulkanDriverFactory};

#[test]
fn factory_creates_a_device_driver_from_its_default_options() {
    let factory = VulkanDriverFactory::new();
    factory.set_default_options(device_options::validation_options());
    let driver = factory.create_driver(&Handle::default()).unwrap();
    let vulkan: Handle<Graphic3dRenderDriver> = driver.cast();
    assert!(!vulkan.is_null());
    // A device-backed driver answers device limits; the validation option reached the device.
    assert!(vulkan.device_limits().is_some());
    assert!(driver.inquire_limit(Graphic3dTypeOfLimit::MaxTextureSize) > 0);
    assert!(vulkan.validation_errors().unwrap().is_empty());
}
