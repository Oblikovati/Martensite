//! Derived CPU replay of line preparation; this does not observe GPU vertex output.
use martensite_vulkan::{PrimitiveDraw, PrimitiveTopology, RenderError, RenderFrame, RenderLayer};

/// Reuse renderer preparation for crossing-eye line cohorts; `prepare(draws, [409., 409.])?`.
pub(crate) fn prepare(
    draws: Vec<PrimitiveDraw>,
    size: [f64; 2],
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let extent = validated_extent(size)?;
    require_finite_clip(&draws)?;
    if !draws.iter().any(crosses_eye) {
        return Ok(draws);
    }
    if !draws.iter().all(opaque_line) {
        return Err(RenderError::rejected(
            "crossing-eye cohort containing nonopaque-line draws",
            "complete opaque line cohort for derived frame replay",
        ));
    }
    prepared_draws(draws, extent)
}

/// Replay complete opaque layers without discarding surface/order context; `prepare_layers(layers, [64.,64.])?`.
pub(crate) fn prepare_layers(
    layers: Vec<RenderLayer>,
    size: [f64; 2],
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let extent = validated_extent(size)?;
    let draws: Vec<_> = layers
        .iter()
        .flat_map(|layer| layer.draws().iter().cloned())
        .collect();
    require_finite_clip(&draws)?;
    if !draws.iter().any(crosses_eye) {
        return Ok(draws);
    }
    require_opaque_replay(&draws)?;
    let frame = RenderFrame::with_layers(extent[0], extent[1], [0.; 4], layers)?;
    checked_prepared_draws(frame)
}

fn require_opaque_replay(draws: &[PrimitiveDraw]) -> Result<(), RenderError> {
    if !draws.iter().all(opaque_replay_draw) {
        return Err(RenderError::rejected(
            "unsupported mixed crossing-eye cohort",
            "complete opaque surface and unit ordinary-line layers",
        ));
    }
    Ok(())
}

fn opaque_replay_draw(draw: &PrimitiveDraw) -> bool {
    if draw.topology() == PrimitiveTopology::Lines {
        return opaque_line(draw);
    }
    draw.topology() == PrimitiveTopology::Triangles
        && !draw.is_surface_blended()
        && draw.vertices().iter().all(|vertex| vertex.color[3] == 1.)
}

fn prepared_draws(
    draws: Vec<PrimitiveDraw>,
    extent: [u32; 2],
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let frame = RenderFrame::new(extent[0], extent[1], [0.; 4], draws)?;
    checked_prepared_draws(frame)
}

fn checked_prepared_draws(frame: RenderFrame) -> Result<Vec<PrimitiveDraw>, RenderError> {
    require_finite_clip(frame.draws())?;
    if frame.draws().iter().any(crosses_eye) {
        return Err(RenderError::rejected(
            "nonpositive clip W after derived preparation",
            "finite positive endpoint W or a completely rejected segment",
        ));
    }
    Ok(frame.draws().to_vec())
}

fn validated_extent(size: [f64; 2]) -> Result<[u32; 2], RenderError> {
    if size
        .iter()
        .any(|v| !v.is_finite() || *v < 1. || v.fract() != 0. || *v > u32::MAX as f64)
    {
        return Err(RenderError::rejected(
            size,
            "two positive integral u32 viewport dimensions",
        ));
    }
    Ok(size.map(|v| v as u32))
}

fn opaque_line(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Lines
        && draw.line_style().is_some_and(unit_opaque_style)
        && draw.vertices().iter().all(|vertex| vertex.color[3] == 1.)
        && !draw.is_surface_blended()
}

fn unit_opaque_style(style: martensite_vulkan::LineStyle) -> bool {
    style.parameters() == (1., u16::MAX, 1) && style.raster_width() == 1. && !style.alpha_blending()
}

fn crosses_eye(draw: &PrimitiveDraw) -> bool {
    if draw.topology() != PrimitiveTopology::Lines {
        return false;
    }
    draw.vertices().iter().any(|vertex| {
        let w = clip_weight(draw, vertex);
        w <= 0. || !w.is_finite()
    })
}

fn require_finite_clip(draws: &[PrimitiveDraw]) -> Result<(), RenderError> {
    let invalid = draws
        .iter()
        .filter(|draw| draw.topology() == PrimitiveTopology::Lines)
        .any(|draw| {
            draw.vertices()
                .iter()
                .any(|vertex| (0..4).any(|row| !clip_component(draw, vertex, row).is_finite()))
        });
    if invalid {
        return Err(RenderError::rejected(
            "nonfinite derived clip component",
            "four finite float32 clip components",
        ));
    }
    Ok(())
}

fn clip_component(
    draw: &PrimitiveDraw,
    vertex: &martensite_vulkan::RenderVertex,
    row: usize,
) -> f32 {
    let v = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    let m = draw.clip_transform();
    (0..4).map(|k| m[k * 4 + row] * v[k]).sum()
}

fn clip_weight(draw: &PrimitiveDraw, vertex: &martensite_vulkan::RenderVertex) -> f32 {
    clip_component(draw, vertex, 3)
}

#[cfg(test)]
#[path = "final_line_witness_controls.rs"]
mod controls;
