//! Source-layer culling defers clears until a later visible layer consumes them.
use crate::{RenderError, RenderLayer};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dLayer;

pub(crate) fn source_passes<'a>(
    layers: impl Iterator<Item = &'a Handle<Graphic3dLayer>>,
    build: impl Fn(&Graphic3dLayer) -> Result<RenderLayer, RenderError>,
) -> Result<Vec<RenderLayer>, RenderError> {
    let mut pending = false;
    let mut passes = Vec::new();
    for layer in layers {
        pending |= layer.layer_settings().to_clear_depth();
        if layer.is_culled() {
            continue;
        }
        let mut pass = build(layer)?;
        pass.clear_depth = std::mem::take(&mut pending);
        passes.push(pass);
    }
    Ok(passes)
}

pub(crate) fn initialize_normal_passes(passes: &mut Vec<RenderLayer>) {
    // Initialize before the shader grid or immediate content even when every source is culled.
    if passes.is_empty() {
        passes.push(RenderLayer::new(Vec::new(), true, true, false));
    }
}
