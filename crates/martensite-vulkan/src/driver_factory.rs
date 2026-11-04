//! The graphic driver factory of the Vulkan driver: the counterpart of the native
//! GlGraphicDriverFactory. `Graphic3dGraphicDriverFactory::DefaultDriverFactory` hands
//! it to viewers, and each created driver starts from the factory's default options.
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::AspectDisplayConnection;
use martensite_tkservice::graphic3d::{
    Graphic3dGraphicDriver, Graphic3dGraphicDriverFactory, Graphic3dGraphicDriverFactoryBase,
};

use crate::{Graphic3dRenderDriver, VulkanOptions};

/// The name of the factory, as `Renderer` names the native OpenGL one.
pub const VULKAN_DRIVER_NAME: &str = "TKVulkan";

/// Creates Vulkan drivers from shared default options; `VulkanDriverFactory::new()`.
pub struct VulkanDriverFactory {
    transient: StandardTransient,
    base: Graphic3dGraphicDriverFactoryBase,
    default_options: SharedState<VulkanOptions>,
}
martensite_tkernel::impl_transient!(VulkanDriverFactory, "Martensite_VulkanDriverFactory",
    StandardTransient, transient, interfaces:[dyn Graphic3dGraphicDriverFactory]);

impl VulkanDriverFactory {
    /// A factory named [`VULKAN_DRIVER_NAME`] with default options; `VulkanDriverFactory::new()`.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: Graphic3dGraphicDriverFactoryBase::new(&AsciiString::from(VULKAN_DRIVER_NAME)),
            default_options: SharedState::new(VulkanOptions::default()),
        }
    }

    /// The options each new driver starts from (native DefaultOptions);
    /// `factory.default_options().validation`.
    pub fn default_options(&self) -> VulkanOptions {
        *self.default_options.read()
    }

    /// Replace the options of drivers created later (native SetDefaultOptions);
    /// `factory.set_default_options(VulkanOptions { validation: true, ..Default::default() })`.
    pub fn set_default_options(&self, options: VulkanOptions) {
        *self.default_options.write() = options;
    }
}

impl Default for VulkanDriverFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Graphic3dGraphicDriverFactory for VulkanDriverFactory {
    fn graphic_driver_factory_base(&self) -> &Graphic3dGraphicDriverFactoryBase {
        &self.base
    }

    /// The Vulkan driver presents through window surfaces, not through the display connection.
    fn create_driver(
        &self,
        _display: &Handle<AspectDisplayConnection>,
    ) -> Result<Handle<dyn Graphic3dGraphicDriver>, Failure> {
        let driver = Graphic3dRenderDriver::new(self.default_options()).map_err(|error| {
            Failure::with_message(
                "Standard_ProgramError",
                &format!("Error: the Vulkan graphic driver cannot be created: {error}"),
            )
        })?;
        Ok(driver.cast())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_start_native_and_follow_the_setter() {
        let factory = VulkanDriverFactory::new();
        assert_eq!(factory.name(), AsciiString::from(VULKAN_DRIVER_NAME));
        assert!(!factory.default_options().validation);
        factory.set_default_options(VulkanOptions {
            validation: true,
            device_index: Some(3),
        });
        let options = factory.default_options();
        assert!(options.validation);
        assert_eq!(options.device_index, Some(3));
    }

    #[test]
    fn creation_failure_names_the_offending_options() {
        let factory = VulkanDriverFactory::new();
        factory.set_default_options(VulkanOptions {
            validation: false,
            device_index: Some(usize::MAX),
        });
        let Err(error) = factory.create_driver(&Handle::default()) else {
            panic!("no device has index usize::MAX");
        };
        assert!(error.message().contains("cannot be created"), "{error:?}");
    }
}
