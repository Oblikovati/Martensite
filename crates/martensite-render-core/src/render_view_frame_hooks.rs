//! Overridable frame steps of a Vulkan view: the counterparts of the protected virtual
//! `GlView::redraw`, `redrawImmediate`, `render`, `renderScene` and `renderStructs`. Each default calls the matching `*_base` body, which continues through
//! the installed hooks exactly as the native base implementation calls the next virtual.
use crate::{Graphic3dRenderView, RenderError, RenderLayer};
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use std::sync::Arc;

/// Layer builder of one step: normal (`false`) or immediate (`true`) layers.
pub(crate) type LayerBuilder<'a> = &'a dyn Fn(bool) -> Result<Vec<RenderLayer>, RenderError>;

/// The state of one frame step; `step.layers_mut().push(extra_layer)`.
pub struct RenderFrameStep<'a> {
    projection: Graphic3dCameraProjection,
    immediate: bool,
    partial: bool,
    layers: Vec<RenderLayer>,
    build: LayerBuilder<'a>,
}
impl RenderFrameStep<'_> {
    /// The camera projection being drawn (native `theProjection`); `step.projection()`.
    pub fn projection(&self) -> Graphic3dCameraProjection {
        self.projection
    }
    /// Whether immediate layers are drawn (native `theToDrawImmediate`); `step.to_draw_immediate()`.
    pub fn to_draw_immediate(&self) -> bool {
        self.immediate
    }
    /// Whether an immediate redraw only updates immediate layers (native `theIsPartialUpdate`).
    pub fn is_partial_update(&self) -> bool {
        self.partial
    }
    /// Layers this step has drawn so far; `step.layers().len()`.
    pub fn layers(&self) -> &[RenderLayer] {
        &self.layers
    }
    /// Add, remove or reorder this step's layers; `step.layers_mut().push(layer)`.
    pub fn layers_mut(&mut self) -> &mut Vec<RenderLayer> {
        &mut self.layers
    }
}

/// Subclass-style overrides of the view frame steps; `view.set_frame_hooks(Some(Arc::new(MyHooks)))`.
///
/// ```ignore
/// struct SkipImmediate;
/// impl RenderViewFrameHooks for SkipImmediate {
///     fn redraw_immediate(&self, _: &Graphic3dRenderView, _: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
///         Ok(())
///     }
/// }
/// ```
pub trait RenderViewFrameHooks: Send + Sync {
    /// Draw the main scene (`GlView::redraw`); the base calls `render` without immediate layers.
    fn redraw(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.redraw_base(step)
    }
    /// Draw immediate layers (`GlView::redrawImmediate`); the base calls `render` with them.
    fn redraw_immediate(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.redraw_immediate_base(step)
    }
    /// Draw one pass (`GlView::render`); the base calls `render_scene`.
    fn render(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.render_base(step)
    }
    /// Draw the scene content (`GlView::renderScene`); the base calls `render_structs`.
    fn render_scene(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.render_scene_base(step)
    }
    /// Draw the structures of the step's layers (`GlView::renderStructs`).
    fn render_structs(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.render_structs_base(step)
    }
}

/// The native base behaviour of every step.
pub(crate) struct BaseFrameHooks;
impl RenderViewFrameHooks for BaseFrameHooks {}

impl Graphic3dRenderView {
    /// Install or remove frame-step overrides; `view.set_frame_hooks(Some(Arc::new(MyHooks)))`.
    pub fn set_frame_hooks(&self, hooks: Option<Arc<dyn RenderViewFrameHooks>>) {
        *self.frame_hooks.write() = hooks;
    }
    fn frame_hooks(&self) -> Arc<dyn RenderViewFrameHooks> {
        // Clone the owner so nested steps and overrides never hold the slot lock.
        self.frame_hooks
            .read()
            .clone()
            .unwrap_or_else(|| Arc::new(BaseFrameHooks))
    }
    /// Native `redraw` body: render the main scene; `view.redraw_base(step)?`.
    pub fn redraw_base(&self, step: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
        step.immediate = false;
        self.frame_hooks().render(self, step)
    }
    /// Native `redrawImmediate` body: render the immediate layers; `view.redraw_immediate_base(step)?`.
    pub fn redraw_immediate_base(&self, step: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
        step.immediate = true;
        self.frame_hooks().render(self, step)
    }
    /// Native `render` body: continue with the scene; `view.render_base(step)?`.
    pub fn render_base(&self, step: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
        self.frame_hooks().render_scene(self, step)
    }
    /// Native `renderScene` body: continue with the structures; `view.render_scene_base(step)?`.
    pub fn render_scene_base(&self, step: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
        self.frame_hooks().render_structs(self, step)
    }
    /// Native `renderStructs` body: append the step's layer passes; `view.render_structs_base(step)?`.
    pub fn render_structs_base(&self, step: &mut RenderFrameStep<'_>) -> Result<(), RenderError> {
        let layers = (step.build)(step.immediate)?;
        step.layers.extend(layers);
        Ok(())
    }
    /// Run the native Redraw sequence: `redraw`, then `redrawImmediate` without partial update.
    /// Returns the main and immediate layers.
    pub(crate) fn run_frame_steps(
        &self,
        projection: Graphic3dCameraProjection,
        build: LayerBuilder<'_>,
    ) -> Result<(Vec<RenderLayer>, Vec<RenderLayer>), RenderError> {
        let step = |immediate| RenderFrameStep {
            projection,
            immediate,
            partial: false,
            layers: vec![],
            build,
        };
        let hooks = self.frame_hooks();
        let mut main = step(false);
        hooks.redraw(self, &mut main)?;
        let mut immediate = step(true);
        hooks.redraw_immediate(self, &mut immediate)?;
        Ok((main.layers, immediate.layers))
    }
}
