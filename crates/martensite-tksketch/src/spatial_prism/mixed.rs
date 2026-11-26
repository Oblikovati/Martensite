// SPDX-License-Identifier: GPL-2.0-only
//! Checked normal sweep of the admitted line and cardinal circular-quarter source.
mod images;
mod native;
mod proof;
mod receipt;
mod source;

use super::{SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure, poll};
pub use receipt::{
    DetachedMixedSpatialPrism, MixedPrismArcTrim, MixedSpatialPrismHistory, MixedSpatialPrismImage,
    MixedSpatialPrismResult, MixedSpatialPrismSource,
};

use super::{SpatialPrismOptions, binding::bind_native_wire};
use martensite_tkbo::profile_extrusion::{
    ExtrudeOptions, PlanarRegion, PlanarRegionSet,
    build_planar_cardinal_quarter_regions_with_work_and_cancel, extrude,
};
use martensite_tkbrep::topods::Edge;
use martensite_tksketch3d::{
    CurveId,
    wire::{LineDirection, WireClosure, mixed::MixedWireRecipe},
};
use receipt::MixedEvidence;
use std::collections::BTreeMap;

/// Extrude a certified closed line loop with one cardinal circular quarter along its exact normal.
///
/// The input and translated cap coordinates must be exactly representable. Accepted results
/// bind the actual native trim, ordered validation and all cap, side and boundary history.
/// Example: `extrude_mixed_wire(&wire, [0., 0., 3.], options, || false)?`.
pub fn extrude_mixed_wire(
    wire: &MixedWireRecipe,
    direction: [f64; 3],
    options: SpatialPrismOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<MixedSpatialPrismResult, Failure> {
    poll(&mut cancelled, Stage::Plane)?;
    if wire.closure() != WireClosure::Closed {
        return Err(Failure::OpenWire);
    }
    let limit = options.source_trace_limit;
    if !limit.is_finite() || limit <= 0. {
        return Err(Failure::InvalidTraceLimit(limit));
    }
    let source = source::lower(wire, direction, &mut cancelled)?;
    let arc = arc_receipt(&source, limit)?;
    let planar = build_planar_cardinal_quarter_regions_with_work_and_cancel(
        &source.profile,
        options.resolution,
        options.planar_work,
        &mut cancelled,
    )
    .map_err(|cause| Failure::Region(Box::new(cause)))?;
    let region = region(&planar, &source, &mut cancelled)?;
    poll(&mut cancelled, Stage::Sweep)?;
    let swept = extrude(
        region,
        direction,
        &ExtrudeOptions {
            resolution: options.resolution,
        },
    );
    poll(&mut cancelled, Stage::Sweep)?;
    let swept = swept.map_err(|cause| Failure::Sweep(Box::new(cause)))?;
    let images = images::bind(&swept, &source, region.id(), &mut cancelled)?;
    let (area, volume) = proof::certify(&source, &images, &region.outer().uses, &mut cancelled)?;
    let evidence = MixedEvidence {
        area,
        volume,
        trace_limit: limit,
        arc,
        height: source.chart.height,
        work: planar.proof_work(),
    };
    let entries = images.entries;
    poll(&mut cancelled, Stage::History)?;
    Ok(MixedSpatialPrismResult::new(swept, entries, evidence))
}

fn arc_receipt(source: &source::Source, limit: f64) -> Result<MixedPrismArcTrim, Failure> {
    let arc = source.arc.authored().recipe().id;
    let curve = CurveId::Arc(arc);
    let bound = source.arc.terminal_error_upper();
    if bound > limit {
        return Err(Failure::MixedTrace {
            curve,
            bound,
            limit,
        });
    }
    let authored = source
        .curves
        .values()
        .find(|item| item.used.curve == curve)
        .ok_or_else(|| refusal(Stage::Plane, Some(curve), "its original source relation"))?;
    let direction = authored.used.direction;
    Ok(MixedPrismArcTrim {
        arc,
        center_control: source.arc.center(),
        endpoints: directed(authored.endpoints, direction),
        parameters: directed(source.arc.authored().range(), direction),
        terminal_error_upper: bound,
    })
}

fn directed<T>([first, last]: [T; 2], direction: LineDirection) -> [T; 2] {
    match direction {
        LineDirection::Forward => [first, last],
        LineDirection::Reversed => [last, first],
    }
}

fn region<'a>(
    planar: &'a PlanarRegionSet,
    source: &source::Source,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<&'a PlanarRegion, Failure> {
    let unbound = |expected| refusal(Stage::Binding, None, expected);
    let [region] = planar.regions() else {
        return Err(unbound("exactly one certified planar region"));
    };
    let count = source.curves.len();
    if !region.holes().is_empty()
        || region.vertices().len() != count
        || region.edges().len() != count
        || !source::loop_matches(&region.outer().uses, source, cancelled)?
    {
        return Err(unbound(
            "the complete authored directed loop, with one vertex per curve and no holes",
        ));
    }
    let edges: BTreeMap<_, &Edge> = region
        .edges()
        .iter()
        .map(|(id, edge)| (*id, edge))
        .collect();
    bind_native_wire(region, &edges, cancelled)?;
    Ok(region)
}

fn refusal(stage: Stage, curve: Option<CurveId>, expected: &'static str) -> Failure {
    Failure::MixedBinding {
        stage,
        curve,
        expected,
    }
}
