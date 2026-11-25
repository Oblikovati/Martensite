// SPDX-License-Identifier: GPL-2.0-only
//! Bind exact authored spatial lines to the ordinary planar-extrusion pipeline.
mod binding;
mod failure;
mod plane;
mod result;

pub use failure::{SpatialPrismFailure, SpatialPrismStage};
pub use result::{
    DetachedSpatialPrism, SpatialPrismHistory, SpatialPrismImage, SpatialPrismResult,
    SpatialPrismSource,
};

use martensite_tkbo::profile_extrusion::{
    ExtrudeOptions, PlanarProofWorkLimit, Resolution,
    build_planar_line_regions_with_work_and_cancel, extrude,
};
use martensite_tksketch3d::wire::{WireClosure, WireRecipe};

/// Separate source trace tolerance and the ordinary planar proof settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialPrismOptions {
    /// Ordinary native region and sweep resolution.
    pub resolution: Resolution,
    /// Finite bounded planar certification work.
    pub planar_work: PlanarProofWorkLimit,
    /// Allowed certified deviation from original XYZ for actual planar edges.
    pub source_trace_limit: f64,
}

/// Extrude one certified closed line loop along a transverse model-space vector.
/// Example: `extrude_wire(&wire, [0., 0., 3.], options, || false)?`.
pub fn extrude_wire(
    wire: &WireRecipe,
    direction: [f64; 3],
    options: SpatialPrismOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<SpatialPrismResult, SpatialPrismFailure> {
    poll(&mut cancelled, SpatialPrismStage::Plane)?;
    if wire.closure() != WireClosure::Closed {
        return Err(SpatialPrismFailure::OpenWire);
    }
    let limit = options.source_trace_limit;
    if !limit.is_finite() || limit <= 0.0 {
        return Err(SpatialPrismFailure::InvalidTraceLimit(limit));
    }
    let lowered = plane::lower(wire, direction, &mut cancelled)?;
    poll(&mut cancelled, SpatialPrismStage::Region)?;
    let planar = build_planar_line_regions_with_work_and_cancel(
        &lowered.profile,
        options.resolution,
        options.planar_work,
        &mut cancelled,
    )
    .map_err(|cause| SpatialPrismFailure::Region(Box::new(cause)))?;
    let (region, trace_bound) = binding::bind(wire, &planar, limit, &mut cancelled)?;
    let volume = binding::placed_volume(region, direction, lowered.sweep_sign)?;
    poll(&mut cancelled, SpatialPrismStage::Sweep)?;
    let swept = extrude(
        region,
        direction,
        &ExtrudeOptions {
            resolution: options.resolution,
        },
    );
    poll(&mut cancelled, SpatialPrismStage::Sweep)?;
    let swept = swept.map_err(|cause| SpatialPrismFailure::Sweep(Box::new(cause)))?;
    result::new(
        wire,
        lowered,
        region.id(),
        planar.proof_work(),
        result::SweepEvidence {
            trace_bound,
            trace_limit: limit,
            volume,
        },
        swept,
        &mut cancelled,
    )
}

fn poll(
    cancelled: &mut impl FnMut() -> bool,
    stage: SpatialPrismStage,
) -> Result<(), SpatialPrismFailure> {
    if cancelled() {
        return Err(SpatialPrismFailure::Cancelled(stage));
    }
    Ok(())
}
