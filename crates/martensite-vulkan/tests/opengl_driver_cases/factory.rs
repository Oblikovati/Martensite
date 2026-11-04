//! Dedicated driver requests inspect the real registry instead of relabelling Vulkan.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dGraphicDriverFactory, graphic3d_graphic_driver_factories,
};

#[derive(Debug)]
pub(super) struct MissingFactory {
    pub requested: &'static str,
    pub available: Vec<String>,
}
impl std::fmt::Display for MissingFactory {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "driver factory {:?} unavailable; expected exact backend, registered {:?}",
            self.requested, self.available
        )
    }
}
impl std::error::Error for MissingFactory {}

pub(super) fn requested_factory(
    name: &'static str,
) -> Result<Handle<dyn Graphic3dGraphicDriverFactory>, MissingFactory> {
    find_factory(name, &graphic3d_graphic_driver_factories())
}
fn find_factory(
    name: &'static str,
    factories: &[Handle<dyn Graphic3dGraphicDriverFactory>],
) -> Result<Handle<dyn Graphic3dGraphicDriverFactory>, MissingFactory> {
    let available = factories.iter().map(factory_name).collect();
    factories
        .iter()
        .find(|factory| factory.name().is_same_string(name, false))
        .cloned()
        .ok_or(MissingFactory {
            requested: name,
            available,
        })
}
fn factory_name(factory: &Handle<dyn Graphic3dGraphicDriverFactory>) -> String {
    String::from_utf8_lossy(factory.name().as_bytes()).into_owned()
}

#[test]
fn dedicated_factories_never_alias_the_actual_vulkan_factory() {
    let factories = [Handle::new(martensite_vulkan::VulkanDriverFactory::new()).cast()];
    assert!(find_factory("TKVulkan", &factories).is_ok());
    for request in ["Renderer", "RendererGles", "TKD3DHost"] {
        let failure = find_factory(request, &factories).err().unwrap();
        assert_eq!(failure.requested, request);
        assert_eq!(failure.available, ["TKVulkan"]);
    }
}
