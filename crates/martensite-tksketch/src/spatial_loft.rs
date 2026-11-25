// SPDX-License-Identifier: GPL-2.0-only
//! Ordinary two-section ruled solids with explicit authored correspondence.
mod admission;
mod binding;
mod failure;
mod result;

pub use failure::{LoftFailure, LoftStage};
pub use result::{DetachedLoft, LoftHistory, LoftImage, LoftLine, LoftPoint, LoftResult};

use martensite_tkbo::{
    body_check::validate_body,
    profile_extrusion::{Resolution, ToleranceCap},
};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIThruSections;
use martensite_tksketch3d::wire::WireRecipe;
use std::num::NonZeroU64;

/// Stable caller identity of a selected section, independent of local point and line IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoftSectionId(pub NonZeroU64);

/// One directed, ordered, closed line section. The first occurrence is the correspondence anchor.
#[derive(Clone, Copy, Debug)]
pub struct LoftSection<'a> {
    /// Stable section identity, distinct from all other section identities.
    pub id: LoftSectionId,
    /// Authored ordered wire.
    pub wire: &'a WireRecipe,
}

/// Separate native modelling resolution and authored boundary acceptance limit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoftOptions {
    /// Requested modelling resolution and tolerance policy.
    pub resolution: Resolution,
    /// Maximum bounded world trace error of the retained section and ruled boundaries.
    pub source_trace_limit: f64,
}

/// Loft two exact planar line recipes under their explicit positive correspondence.
/// The native builder may run to completion before cancellation is observed.
/// Example: `loft_ruled_pair([first, last], options, || false)?`.
pub fn loft_ruled_pair(
    sections: [LoftSection<'_>; 2],
    options: LoftOptions,
    mut cancelled: impl FnMut() -> bool,
) -> Result<LoftResult, LoftFailure> {
    let pair = admission::admit(sections, options, &mut cancelled)?;
    let wires = admission::materialize(sections, options.source_trace_limit, &mut cancelled)?;
    poll(&mut cancelled, LoftStage::Construction)?;
    let built = BRepOffsetAPIThruSections::corresponding_ruled_pair(
        [&wires[0].wire, &wires[1].wire],
        options.resolution.sew,
    );
    poll(&mut cancelled, LoftStage::Construction)?;
    let native = built.map_err(|source| LoftFailure::NativeBuild {
        sections: sections.map(|section| section.id),
        source,
    })?;
    poll(&mut cancelled, LoftStage::Validation)?;
    let mut validation_resolution = options.resolution;
    if matches!(validation_resolution.cap, ToleranceCap::Report(_)) {
        validation_resolution.cap = ToleranceCap::NoLimit;
    }
    let validation =
        validate_body(&native.solid, &validation_resolution).map_err(LoftFailure::Resolution)?;
    poll(&mut cancelled, LoftStage::Validation)?;
    if !validation.is_valid() {
        return Err(LoftFailure::Validation(Box::new(validation)));
    }
    let evidence = binding::bind(&native, &wires, &pair, options, &mut cancelled)?;
    poll(&mut cancelled, LoftStage::Publication)?;
    Ok(LoftResult::new(
        native.solid,
        validation,
        pair.station_sign,
        options,
        evidence,
    ))
}

fn poll(cancelled: &mut impl FnMut() -> bool, stage: LoftStage) -> Result<(), LoftFailure> {
    if cancelled() {
        return Err(LoftFailure::Cancelled(stage));
    }
    Ok(())
}
