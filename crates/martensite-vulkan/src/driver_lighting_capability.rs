//! Driver-scoped lighting changes leave retained geometry intact and obsolete earlier frames.
use crate::{Graphic3dRenderDriver, RenderLightingPipeline};
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;

pub(crate) type SharedLightingSelection = Arc<SharedState<LightingSelection>>;
#[derive(Clone, Debug)]
pub(crate) struct LightingSelection {
    pub policy: RenderLightingPipeline,
    pub revision: Arc<()>,
}
impl LightingSelection {
    pub fn new(policy: RenderLightingPipeline) -> Self {
        Self {
            policy,
            revision: Arc::new(()),
        }
    }
    pub(crate) fn require_current(&self, started: &Self) -> Result<(), crate::RenderError> {
        if Arc::ptr_eq(&self.revision, &started.revision) {
            return Ok(());
        }
        Err(crate::RenderError::lighting_changed(
            started.policy,
            self.policy,
        ))
    }
    fn replace(&mut self, policy: RenderLightingPipeline) -> bool {
        if self.policy == policy {
            return false;
        }
        *self = Self::new(policy);
        true
    }
}
impl Graphic3dRenderDriver {
    /// Inspect the current lighting family; `driver.lighting_pipeline()`.
    pub fn lighting_pipeline(&self) -> RenderLightingPipeline {
        self.lighting_selection.read().policy
    }
    /// Switch existing views on their next redraw; `driver.set_lighting_pipeline(RenderLightingPipeline::FixedFunction)?`.
    pub fn set_lighting_pipeline(
        &self,
        policy: RenderLightingPipeline,
    ) -> Result<(), crate::RenderError> {
        self.unlit_face_color_policy.validate(policy)?;
        if !self.lighting_selection.write().replace(policy) {
            return Ok(());
        }
        self.invalidate_created_lighting_views();
        Ok(())
    }
    fn invalidate_created_lighting_views(&self) {
        let views: Vec<_> = self
            .objects
            .views
            .read()
            .iter()
            .map(|v| v.upgrade())
            .collect();
        for view in views {
            if let Some(view) = view.cast::<crate::Graphic3dRenderView>().get() {
                view.invalidate_lighting_capture();
            }
        }
    }
}
