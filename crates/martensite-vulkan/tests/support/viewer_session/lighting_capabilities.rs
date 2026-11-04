//! Live capability changes use the current context's actual driver, without a redraw.
use super::*;
use martensite_vulkan::{RenderError, RenderLightingPipeline};
impl ViewerSession {
    /// Switch the fixed-function option live; `session.set_fixed_function_lighting(true)?`.
    pub fn set_fixed_function_lighting(&self, enabled: bool) -> Result<(), RenderError> {
        let driver = self.current_lighting_driver()?;
        let policy = if enabled {
            RenderLightingPipeline::FixedFunction
        } else {
            RenderLightingPipeline::Programmable
        };
        driver.set_lighting_pipeline(policy)
    }
    fn current_lighting_driver(&self) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
        let viewer = self.context.current_viewer();
        let viewer = viewer.get().ok_or_else(|| {
            RenderError::rejected(
                "null current viewer",
                "a current viewer with a renderer driver",
            )
        })?;
        let driver = viewer.driver().cast::<Graphic3dRenderDriver>();
        if driver.is_null() {
            return Err(RenderError::rejected(
                "non-renderer current driver",
                "the concrete driver supporting mutable lighting",
            ));
        }
        Ok(driver)
    }
}
#[cfg(test)]
#[path = "lighting_capabilities_tests.rs"]
mod tests;
