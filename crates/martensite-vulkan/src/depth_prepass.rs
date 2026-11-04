//! Native depth pre-pass: GlLayerList::Render at renders each depth-clear bunch of
//! layers three times when Graphic3dRenderingParams::ToEnableDepthPrepass is set.
//!
//! Pass 0 draws the opaque content of the layers that take part in the pre-pass with colour writes
//! off (depth only, the ordinary LEQUAL test). Pass 1 draws the other layers normally. Pass 2 draws
//! every layer again with an EQUAL depth test and depth writes off. Blended content is deferred
//! as without a pre-pass; a layer outside the pre-pass queues its blended content in pass 1 and
//! again in pass 2, so the native renderer blends it twice.
use crate::RenderLayer;

/// The depth role of one layer copy; `RenderDepthPass::DepthOnly` writes no colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RenderDepthPass {
    /// Colour and depth as the layer settings ask (no pre-pass, or native pass 1).
    #[default]
    Shaded,
    /// Native pass 0: depth only, colour write mask cleared.
    DepthOnly,
    /// Native pass 2: colour where the depth equals the pre-pass depth, depth writes off.
    DepthEqual,
}

/// Expand the main and immediate layer lists of one frame; returns the layers and the new
/// immediate count. Example: `expand_view_layers(layers, 1, params.to_enable_depth_prepass)`.
pub(crate) fn expand_view_layers(
    mut layers: Vec<RenderLayer>,
    immediate: usize,
    enabled: bool,
) -> (Vec<RenderLayer>, usize) {
    if !enabled {
        return (layers, immediate);
    }
    // Each GlLayerList::Render call (main, then immediate) runs its own pass loop.
    let immediate_layers = layers.split_off(layers.len() - immediate);
    let mut expanded = expand_layers(layers);
    let immediate_expanded = expand_layers(immediate_layers);
    let count = immediate_expanded.len();
    expanded.extend(immediate_expanded);
    (expanded, count)
}

/// Expand one layer list bunch by bunch; a layer that clears depth starts a new bunch.
fn expand_layers(layers: Vec<RenderLayer>) -> Vec<RenderLayer> {
    let mut expanded = Vec::new();
    let mut bunch: Vec<RenderLayer> = Vec::new();
    for layer in layers {
        if layer.clear_depth && !bunch.is_empty() {
            expanded.extend(expand_bunch(std::mem::take(&mut bunch)));
        }
        bunch.push(layer);
    }
    expanded.extend(expand_bunch(bunch));
    expanded
}

/// Passes 0, 1 and 2 of one bunch; only the first emitted copy keeps the bunch's depth clear.
fn expand_bunch(bunch: Vec<RenderLayer>) -> Vec<RenderLayer> {
    let clear = bunch.first().is_some_and(|layer| layer.clear_depth);
    let depth_only = bunch
        .iter()
        .filter(|layer| layer.in_depth_prepass)
        .map(|layer| layer.opaque_copy(RenderDepthPass::DepthOnly));
    let outside = bunch
        .iter()
        .filter(|layer| !layer.in_depth_prepass)
        .map(|layer| layer.clone().with_depth_pass(RenderDepthPass::Shaded));
    let mut passes: Vec<RenderLayer> = depth_only.chain(outside).collect();
    passes.extend(
        bunch
            .into_iter()
            .map(|layer| layer.with_depth_pass(RenderDepthPass::DepthEqual)),
    );
    for (index, layer) in passes.iter_mut().enumerate() {
        layer.clear_depth = clear && index == 0;
    }
    passes
}

impl RenderLayer {
    /// The opaque part of this layer in `pass`; blended draws stay with the original layer.
    fn opaque_copy(&self, pass: RenderDepthPass) -> Self {
        crate::render_layers::split_blended_layer(self.clone())
            .0
            .with_depth_pass(pass)
    }

    /// Assign the depth role; the equal pass never writes depth (native DepthMask GL_FALSE).
    fn with_depth_pass(mut self, pass: RenderDepthPass) -> Self {
        self.depth_pass = pass;
        self.depth_write &= pass != RenderDepthPass::DepthEqual;
        self
    }
}

#[cfg(test)]
#[path = "depth_prepass_tests.rs"]
mod tests;
