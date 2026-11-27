//! Layer ordering and depth policy stay explicit in immutable frame submissions.
use crate::PrimitiveDraw;
use crate::stencil_program::StructureDrawSpan;
use std::{ops::Range, sync::Arc};
/// One ordered pass; `RenderLayer::new(draws, true, true, false)`.
#[derive(Clone, Debug)]
pub struct RenderLayer {
    pub draws: Vec<PrimitiveDraw>,
    pub depth_test: bool,
    pub depth_write: bool,
    pub clear_depth: bool,
    pub polygon_offset: crate::RenderPolygonOffset,
    pub stencil_boundaries: Vec<crate::stencil_program::StencilBoundary>,
    pub placement_sources: Arc<[StructureDrawSpan]>,
    /// Native Graphic3dZLayerSettings::ToRenderInDepthPrepass (default true).
    pub in_depth_prepass: bool,
    pub depth_pass: crate::depth_prepass::RenderDepthPass,
    /// The Z layer's local origin (`Graphic3dZLayerSettings::SetOrigin`): its lights and
    /// clipping planes are taken relative to it while the layer renders.
    pub(crate) local_origin: Option<crate::layer_origin::LayerOrigin>,
    /// A ray-traced Z layer's raster pass: the ray-tracing pass follows the last such layer.
    pub raytraced: bool,
}
impl RenderLayer {
    /// Retain a layer's draws and depth policy; `RenderLayer::new(draws, true, true, false)`.
    pub fn new(
        mut draws: Vec<PrimitiveDraw>,
        depth_test: bool,
        depth_write: bool,
        clear_depth: bool,
    ) -> Self {
        crate::capping_order::order_layer_draws(&mut draws);
        let stencil_boundaries = crate::stencil_program::implicit_cap_restores(&draws);
        Self {
            draws,
            depth_test,
            depth_write,
            clear_depth,
            polygon_offset: crate::RenderPolygonOffset::default(),
            stencil_boundaries,
            placement_sources: Arc::from([]),
            in_depth_prepass: true,
            depth_pass: Default::default(),
            local_origin: None,
            raytraced: false,
        }
    }
}
impl RenderLayer {
    pub fn from_stencil_program(
        mut program: crate::stencil_program::StencilProgram,
        depth_test: bool,
        depth_write: bool,
        clear_depth: bool,
    ) -> Self {
        crate::capping_order::classify_capping_batches(&mut program.draws);
        Self {
            draws: program.draws,
            depth_test,
            depth_write,
            clear_depth,
            polygon_offset: Default::default(),
            stencil_boundaries: program.boundaries,
            placement_sources: program.sources.into(),
            in_depth_prepass: true,
            depth_pass: Default::default(),
            local_origin: None,
            raytraced: false,
        }
    }
    /// The layer's ordered draws, as submitted to the frame; `layer.draws().len()`.
    pub fn draws(&self) -> &[PrimitiveDraw] {
        &self.draws
    }
    /// Retain the layer's default polygon offset; `layer.with_polygon_offset(RenderPolygonOffset::new(1, 1., 1.)?)`.
    pub fn with_polygon_offset(mut self, offset: crate::RenderPolygonOffset) -> Self {
        self.polygon_offset = offset;
        self
    }
    /// Render around the layer's local origin; `layer.with_local_origin(LayerOrigin::of(&settings))`.
    pub(crate) fn with_local_origin(
        mut self,
        origin: Option<crate::layer_origin::LayerOrigin>,
    ) -> Self {
        self.local_origin = origin;
        self
    }
    /// Retain native ToRenderInDepthPrepass; `layer.with_depth_prepass_membership(false)`.
    pub fn with_depth_prepass_membership(mut self, member: bool) -> Self {
        self.in_depth_prepass = member;
        self
    }
}
/// Range of frame draws sharing one depth policy; `frame.layers()[0].draw_range()`.
#[derive(Clone, Debug)]
pub struct RenderLayerSpan {
    pub draw_range: Range<usize>,
    pub depth_test: bool,
    pub depth_write: bool,
    pub clear_depth: bool,
    pub polygon_offset: crate::RenderPolygonOffset,
    pub stencil_boundaries: Vec<crate::stencil_program::StencilBoundary>,
    pub placement_sources: Arc<[StructureDrawSpan]>,
}
impl RenderLayerSpan {
    /// Index into `frame.draws()`; `frame.layers()[0].draw_range()`.
    pub fn draw_range(&self) -> Range<usize> {
        self.draw_range.clone()
    }
    /// Read the retained layer default; `frame.layers()[0].polygon_offset()`.
    pub fn polygon_offset(&self) -> crate::RenderPolygonOffset {
        self.polygon_offset
    }
    /// Read `(test, write, clear)` flags; `frame.layers()[0].depth_policy()`.
    pub fn depth_policy(&self) -> (bool, bool, bool) {
        (self.depth_test, self.depth_write, self.clear_depth)
    }
}
/// Flatten layers into draw spans; a `boundary` layer index ends the main pass, whose
/// deferred transparency is flushed there (each GlLayerList::Render call flushes its
/// own). Returns the span index at the boundary, or the span count without one.
pub fn flatten_layers_at(
    layers: Vec<RenderLayer>,
    boundary: Option<usize>,
) -> (Vec<PrimitiveDraw>, Vec<RenderLayerSpan>, usize) {
    let mut draws = Vec::new();
    let mut spans = Vec::new();
    let mut deferred = Vec::new();
    let mut marker = None;
    for (index, layer) in layers.into_iter().enumerate() {
        if boundary == Some(index) {
            flush_blended_layers(&mut deferred, &mut draws, &mut spans);
            marker = Some(spans.len());
        }
        if layer.clear_depth {
            flush_blended_layers(&mut deferred, &mut draws, &mut spans);
        }
        let (opaque, blended) = split_blended_layer(layer);
        append_layer_span(opaque, &mut draws, &mut spans);
        deferred.extend(blended);
    }
    flush_blended_layers(&mut deferred, &mut draws, &mut spans);
    let marker = marker.unwrap_or(spans.len());
    (draws, spans, marker)
}
pub fn split_blended_layer(mut layer: RenderLayer) -> (RenderLayer, Option<RenderLayer>) {
    // Without deferred draws every stencil boundary already has its final ordinal.
    let Some(first_blended) = layer
        .draws
        .iter()
        .position(PrimitiveDraw::is_surface_blended)
    else {
        return (layer, None);
    };
    let deferred = if layer.stencil_boundaries.is_empty() {
        Vec::new()
    } else {
        blend_membership(&layer.draws, first_blended)
    };
    let (blended, opaque) = partition_blended(std::mem::take(&mut layer.draws), first_blended);
    let transparent =
        (!blended.is_empty()).then(|| layer.stencil_partition_branch(blended, &deferred, true));
    (
        layer.stencil_partition_branch(opaque, &deferred, false),
        transparent,
    )
}
/// The blended and the opaque draws, in order. A layer of only blended draws moves as one
/// vector; partitioning copied every 1.5 KB draw (#594).
fn partition_blended(
    draws: Vec<PrimitiveDraw>,
    first_blended: usize,
) -> (Vec<PrimitiveDraw>, Vec<PrimitiveDraw>) {
    if first_blended == 0 && draws.iter().all(PrimitiveDraw::is_surface_blended) {
        return (draws, Vec::new());
    }
    draws
        .into_iter()
        .partition(PrimitiveDraw::is_surface_blended)
}
fn blend_membership(draws: &[PrimitiveDraw], first_blended: usize) -> Vec<bool> {
    let mut deferred = Vec::with_capacity(draws.len());
    deferred.resize(first_blended, false);
    deferred.push(true);
    deferred.extend(
        draws[first_blended + 1..]
            .iter()
            .map(PrimitiveDraw::is_surface_blended),
    );
    deferred
}
impl RenderLayer {
    fn stencil_partition_branch(
        &self,
        draws: Vec<PrimitiveDraw>,
        deferred: &[bool],
        transparent: bool,
    ) -> Self {
        Self {
            draws,
            depth_test: self.depth_test,
            depth_write: self.depth_write && !transparent,
            clear_depth: self.clear_depth && !transparent,
            polygon_offset: self.polygon_offset,
            stencil_boundaries: crate::stencil_program::filtered_boundaries(
                &self.stencil_boundaries,
                deferred,
                transparent,
            ),
            placement_sources: Arc::from([]),
            in_depth_prepass: self.in_depth_prepass,
            local_origin: self.local_origin,
            raytraced: self.raytraced,
            // Deferred blended content keeps the ordinary depth test (native renderTransparent).
            depth_pass: if transparent {
                Default::default()
            } else {
                self.depth_pass
            },
        }
    }
}
fn flush_blended_layers(
    pending: &mut Vec<RenderLayer>,
    draws: &mut Vec<PrimitiveDraw>,
    spans: &mut Vec<RenderLayerSpan>,
) {
    // GlLayerList::Render at flushes transparent layers before the next depth clear.
    for layer in pending.drain(..) {
        append_layer_span(layer, draws, spans);
    }
}
fn append_layer_span(
    mut layer: RenderLayer,
    draws: &mut Vec<PrimitiveDraw>,
    spans: &mut Vec<RenderLayerSpan>,
) {
    let start = draws.len();
    let pass = layer.depth_pass;
    for draw in &mut layer.draws {
        // A dimension label rectangle stays colourless in every layer pass.
        if !draw.stencil_rect {
            draw.depth_pass = pass;
        }
    }
    if draws.capacity() == 0 {
        *draws = layer.draws;
    } else {
        draws.append(&mut layer.draws);
    }
    spans.push(RenderLayerSpan {
        draw_range: start..draws.len(),
        depth_test: layer.depth_test,
        depth_write: layer.depth_write,
        clear_depth: layer.clear_depth,
        polygon_offset: layer.polygon_offset,
        stencil_boundaries: layer.stencil_boundaries,
        placement_sources: layer.placement_sources,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RenderVertex;

    fn triangle() -> PrimitiveDraw {
        let vertex = |x: f32, y: f32| RenderVertex::new([x, y, 0.], [1.; 4]);
        let identity = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ];
        PrimitiveDraw::new(
            vec![vertex(0., 0.), vertex(1., 0.), vertex(0., 1.)],
            vec![0, 1, 2],
            identity,
        )
        .expect("a valid triangle draw")
    }

    #[test]
    fn an_all_blended_layer_moves_without_partitioning_and_a_mixed_one_keeps_order() {
        let blended = || {
            let mut draw = triangle();
            draw.surface_blend = true;
            draw
        };
        let draws = vec![blended(), blended()];
        let address = draws.as_ptr();
        let (moved, opaque) = partition_blended(draws, 0);
        assert_eq!((moved.as_ptr(), moved.len(), opaque.len()), (address, 2, 0));
        let mut mixed = vec![triangle(), blended(), triangle()];
        mixed[2].mirrored = true;
        let (blend, rest) = partition_blended(mixed, 1);
        assert_eq!((blend.len(), rest.len()), (1, 2));
        assert!(blend[0].surface_blend && !rest[0].mirrored && rest[1].mirrored);
    }

    #[test]
    fn layer_exposes_its_submitted_draws() {
        let draw = triangle();
        let layer = RenderLayer::new(vec![draw.clone()], true, true, false);
        assert_eq!(layer.draws(), &[draw]);
    }

    #[test]
    fn first_opaque_layer_keeps_its_draw_storage_and_stencil_ordinals() {
        use crate::depth_prepass::RenderDepthPass::{DepthEqual, DepthOnly};
        use crate::stencil_program::{StencilBoundary, StencilCommand};

        let mut rectangle = triangle();
        rectangle.stencil_rect = true;
        rectangle.depth_pass = DepthOnly;
        let draws = vec![triangle(), rectangle];
        let address = draws.as_ptr();
        let mut layer = RenderLayer::new(draws, true, false, true);
        layer.depth_pass = DepthEqual;
        let boundary = StencilBoundary {
            before_draw: 1,
            command: StencilCommand::DimensionMask,
        };
        layer.stencil_boundaries.push(boundary);
        let (draws, spans, main) = flatten_layers_at(vec![layer], None);
        assert_eq!(draws.as_ptr(), address);
        assert_eq!(
            [draws[0].depth_pass, draws[1].depth_pass],
            [DepthEqual, DepthOnly]
        );
        assert_eq!(spans[0].stencil_boundaries, [boundary]);
        assert_eq!(main, 1);
    }

    #[test]
    fn mixed_layer_maps_stencil_after_the_first_deferred_draw() {
        use crate::stencil_program::{StencilBoundary, StencilCommand, StencilProgram};

        let deferred = triangle().with_surface_blending(true).unwrap();
        let program = StencilProgram {
            draws: vec![triangle(), deferred, triangle()],
            boundaries: vec![StencilBoundary {
                before_draw: 2,
                command: StencilCommand::CappingRestore,
            }],
            sources: Vec::new(),
        };
        let layer = RenderLayer::from_stencil_program(program, true, true, true);
        let (opaque, blended) = split_blended_layer(layer);
        let blended = blended.unwrap();
        assert_eq!(opaque.draws.len(), 2);
        assert_eq!(blended.draws.len(), 1);
        assert_eq!(opaque.stencil_boundaries[0].before_draw, 1);
        assert_eq!(blended.stencil_boundaries[0].before_draw, 1);
        assert!(opaque.clear_depth && opaque.depth_write);
        assert!(!blended.clear_depth && !blended.depth_write);
    }
}
