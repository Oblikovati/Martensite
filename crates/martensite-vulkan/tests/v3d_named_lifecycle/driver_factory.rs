//! Driver construction keeps the established policy and uses real isolated CPU owners.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;
use martensite_vulkan::Graphic3dRenderDriver;

pub(super) struct BuiltDriver {
    pub core: Handle<dyn Graphic3dGraphicDriver>,
    pub renderer: Option<Handle<Graphic3dRenderDriver>>,
}

pub(super) trait SourceDriverFactory {
    fn create(&self) -> Outcome<BuiltDriver>;
}

pub(super) struct QualifiedVulkanFactory;
impl SourceDriverFactory for QualifiedVulkanFactory {
    fn create(&self) -> Outcome<BuiltDriver> {
        let renderer = crate::session_run::run_driver(Default::default())?;
        Ok(BuiltDriver {
            core: renderer.clone().cast(),
            renderer: Some(renderer),
        })
    }
}

pub(super) struct DriverOwner {
    pub core: Handle<dyn Graphic3dGraphicDriver>,
    pub session: ViewerSession,
}

impl DriverOwner {
    pub fn new(built: &BuiltDriver) -> Outcome<Self> {
        let session = match &built.renderer {
            Some(renderer) => ViewerSession::with_view_size(renderer, 409, 409)?,
            None => ViewerSession::with_graphic_driver(&built.core, 409, 409)?,
        };
        Ok(Self {
            core: built.core.clone(),
            session,
        })
    }
}
