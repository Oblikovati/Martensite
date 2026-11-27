//! Snapshot the actual view parameters and its injected legacy lighting policy once per frame.
use crate::{
    Graphic3dRenderDriver, RenderError, RenderLightingPipeline, transparency::RenderRasterSettings,
};
use martensite_tkservice::graphic3d::{Graphic3dCView, Graphic3dRenderingParams};

pub(crate) fn view_raster_settings(
    view: &dyn Graphic3dCView,
    params: &Graphic3dRenderingParams,
) -> Result<RenderRasterSettings, RenderError> {
    let driver = view_driver(view);
    let driver = driver.as_ref().and_then(|driver| driver.get());
    let fixed = driver
        .is_some_and(|driver| driver.lighting_pipeline() == RenderLightingPipeline::FixedFunction);
    let mut settings = RenderRasterSettings::from_parameters(params, fixed)?;
    settings.alpha_writes = driver.is_some_and(|driver| !driver.options().buffers_opaque_alpha);
    Ok(settings)
}
/// The Vulkan driver that created `view`, if any.
fn view_driver(
    view: &dyn Graphic3dCView,
) -> Option<martensite_tkernel::standard::Handle<Graphic3dRenderDriver>> {
    let manager = view.cview_base().fields.read().structure_manager.clone();
    manager
        .get()
        .map(|manager| manager.graphic_driver().cast::<Graphic3dRenderDriver>())
}
/// The view's driver lights with the fixed-function pipeline (no sRGB targets, no PBR).
pub(crate) fn fixed_function_view(view: &dyn Graphic3dCView) -> bool {
    view_driver(view)
        .as_ref()
        .and_then(|driver| driver.get())
        .is_some_and(|driver| driver.lighting_pipeline() == RenderLightingPipeline::FixedFunction)
}
