//! Context requests retain unsupported profile constraints instead of silently dropping them.
use super::*;
use martensite_tkservice::graphic3d::{Graphic3dGraphicDriver, Graphic3dGraphicDriverFactory};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct ContextRequest {
    pub core: Option<bool>,
    pub maximum_version: Option<[i32; 2]>,
    pub software: Option<bool>,
    pub fixed_function: Option<bool>,
    pub debug: Option<bool>,
}
#[derive(Debug)]
pub(super) struct UnsupportedContext {
    factory: String,
    request: ContextRequest,
    actual_options: String,
}
impl std::fmt::Display for UnsupportedContext {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "factory {:?} options {}; expected configurable OpenGL context {:?}",
            self.factory, self.actual_options, self.request
        )
    }
}
impl std::error::Error for UnsupportedContext {}

pub(super) fn create_context(
    name: &'static str,
    request: ContextRequest,
) -> Result<(Handle<dyn Graphic3dGraphicDriver>, ViewerSession), Box<dyn std::error::Error>> {
    let factory = factory::requested_factory(name)?;
    require_context_options(name, &factory, request)?;
    let driver = factory.create_driver(&Handle::default())?;
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409)?;
    Ok((driver, session))
}
fn require_context_options(
    name: &str,
    factory: &Handle<dyn Graphic3dGraphicDriverFactory>,
    request: ContextRequest,
) -> Outcome {
    if request == ContextRequest::default() {
        return Ok(());
    }
    Err(Box::new(UnsupportedContext {
        factory: name.into(),
        request,
        actual_options: actual_options(factory),
    }))
}
fn actual_options(factory: &Handle<dyn Graphic3dGraphicDriverFactory>) -> String {
    let vulkan = factory.cast::<martensite_vulkan::VulkanDriverFactory>();
    if let Some(vulkan) = vulkan.get() {
        return format!("{:?}", vulkan.default_options());
    }
    let kind = factory
        .dynamic_type()
        .map_or("null factory", |kind| kind.name());
    format!("{kind} has no context configuration interface")
}

#[test]
fn explicit_false_and_unbounded_resets_are_not_absent_context_requests() {
    let cases = requests();
    assert_ne!(cases[0], ContextRequest::default());
    assert_eq!(cases[0], reset_request());
    assert_eq!(cases[0].core, Some(false));
    assert_eq!(cases[0].software, Some(false));
    assert_eq!(cases[0].debug, Some(false));
    assert_eq!(cases[0].maximum_version, Some([-1, -1]));
    assert_eq!(cases[1].core, Some(true));
    assert_eq!(cases[2].maximum_version, Some([2, 1]));
    assert_eq!(cases[3].software, Some(true));
    assert_eq!(cases[4].debug, Some(true));
}
pub(super) fn requests() -> [ContextRequest; 5] {
    [
        reset_request(),
        core_request(),
        version_request(),
        software_request(),
        debug_request(),
    ]
}
pub(super) fn reset_request() -> ContextRequest {
    ContextRequest {
        core: Some(false),
        software: Some(false),
        debug: Some(false),
        maximum_version: Some([-1, -1]),
        fixed_function: None,
    }
}
pub(super) fn reset_context() -> Outcome {
    let factory = factory::requested_factory("Renderer")?;
    require_context_options("Renderer", &factory, reset_request())
}
fn core_request() -> ContextRequest {
    ContextRequest {
        core: Some(true),
        ..reset_request()
    }
}
fn version_request() -> ContextRequest {
    ContextRequest {
        maximum_version: Some([2, 1]),
        ..reset_request()
    }
}
fn software_request() -> ContextRequest {
    ContextRequest {
        software: Some(true),
        ..reset_request()
    }
}
fn debug_request() -> ContextRequest {
    ContextRequest {
        debug: Some(true),
        ..reset_request()
    }
}

#[test]
fn unavailable_context_configuration_refuses_before_creating_a_device_or_view() {
    let factory = Handle::new(martensite_vulkan::VulkanDriverFactory::new()).cast();
    for request in requests() {
        let refusal = require_context_options("TKVulkan", &factory, request).unwrap_err();
        let typed = refusal.downcast_ref::<UnsupportedContext>().unwrap();
        assert_eq!(typed.request, request);
        assert!(typed.actual_options.contains("device_index"));
    }
}

#[test]
fn genuinely_absent_context_options_do_not_invent_a_configuration_command() {
    let factory = Handle::new(martensite_vulkan::VulkanDriverFactory::new()).cast();
    assert!(require_context_options("TKVulkan", &factory, ContextRequest::default()).is_ok());
}
