// SPDX-License-Identifier: GPL-2.0-only
//! Checked line and cardinal circular-arc wire recipes.
mod circle;
mod materialize;
mod pairs;

pub use circle::{CardinalWireArc, WireArcScope};
pub use materialize::{
    DetachedMixedWire, MixedNativeEdgeBinding, NativeMixedTrim, NativeTangentBound,
};

use super::{
    DirectedLineUse, LineDirection, WireClosure, WireFailure, WirePhase, WireRecipeValidation,
    WireSegment,
    ordered::{check_chain, check_pairs},
    poll,
    recipe::resolve_line,
};
use crate::{CurveId, CurveRole, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use std::collections::BTreeSet;

/// A curve occurrence in the authored order or its reversal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedCurveUse {
    /// Original modelling curve.
    pub curve: CurveId,
    /// Traversal direction relative to the stored curve trim.
    pub direction: LineDirection,
}

/// Actual authored data and established carrier domain for one wire use.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MixedWireGeometry {
    /// Admitted straight segment with an exact affine carrier.
    Line(WireSegment),
    /// Exact cardinal quarter-circle with bounded native terminal discrepancy.
    Arc(CardinalWireArc),
}

/// One source-bound occurrence in a checked mixed wire.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MixedWireSegment {
    source: DirectedCurveUse,
    geometry: MixedWireGeometry,
}

impl MixedWireSegment {
    /// Identify the original curve and selected sense. Example: `segment.source()`.
    pub const fn source(&self) -> DirectedCurveUse {
        self.source
    }

    /// Read detached original geometry. Example: `segment.geometry()`.
    pub fn geometry(&self) -> &MixedWireGeometry {
        &self.geometry
    }

    /// Original point identities in wire order. Example: `segment.endpoints()`.
    pub fn endpoints(&self) -> [PointId; 2] {
        self.source.direction.endpoints(self.authored_endpoints())
    }

    fn authored_endpoints(&self) -> [PointId; 2] {
        match self.geometry {
            MixedWireGeometry::Line(line) => line.authored.line.endpoints,
            MixedWireGeometry::Arc(arc) => arc.authored().recipe().endpoints,
        }
    }

    fn points(&self) -> [Point3; 2] {
        match self.geometry {
            MixedWireGeometry::Line(line) => line.authored.points,
            MixedWireGeometry::Arc(arc) => arc.points(),
        }
    }
}

/// Independently owned mixed recipe with verified directed incidences.
#[derive(Clone, Debug, PartialEq)]
pub struct MixedWireRecipe {
    closure: WireClosure,
    segments: Vec<MixedWireSegment>,
    validation: WireRecipeValidation,
}

impl Snapshot {
    /// Certify a mixed line and circular-arc wire under a caller-supplied pair grant.
    /// Example: `snapshot.mixed_wire_recipe(&uses, WireClosure::Open, 16, || false)?`.
    ///
    /// Arc admission requires a center-defined cardinal quarter. Each neighboring
    /// line must continue the chosen tangent in wire direction. Nonincident pairs
    /// containing an arc require disjoint complete carrier boxes.
    pub fn mixed_wire_recipe(
        &self,
        uses: &[DirectedCurveUse],
        closure: WireClosure,
        max_pair_checks: usize,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<MixedWireRecipe, WireFailure> {
        poll(&mut cancelled, WirePhase::Resolve)?;
        let segments = resolve(self, uses, &mut cancelled)?;
        check_chain(
            &segments,
            closure,
            MixedWireSegment::endpoints,
            disconnected,
            &mut cancelled,
        )?;
        let pair_checks = check_pairs(
            &segments,
            closure,
            max_pair_checks,
            &mut cancelled,
            pairs::check,
            |pair| WireFailure::MixedPairLimit {
                curves: curves(pair),
                limit: max_pair_checks,
            },
        )?;
        Ok(MixedWireRecipe {
            closure,
            segments,
            validation: WireRecipeValidation {
                pair_checks,
                pair_limit: max_pair_checks,
            },
        })
    }
}

impl MixedWireRecipe {
    /// Read the admitted endpoint closure. Example: `recipe.closure()`.
    pub const fn closure(&self) -> WireClosure {
        self.closure
    }

    /// Read immutable occurrences in directed order. Example: `recipe.segments()`.
    pub fn segments(&self) -> &[MixedWireSegment] {
        &self.segments
    }

    /// Read the exact pair coverage. Example: `recipe.validation()`.
    pub const fn validation(&self) -> WireRecipeValidation {
        self.validation
    }
}

fn resolve(
    snapshot: &Snapshot,
    uses: &[DirectedCurveUse],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<MixedWireSegment>, WireFailure> {
    let mut seen = BTreeSet::new();
    let mut result = Vec::with_capacity(uses.len());
    for &source in uses {
        poll(cancelled, WirePhase::Resolve)?;
        if !seen.insert(source.curve) {
            return Err(WireFailure::RepeatedCurve(source.curve));
        }
        result.push(MixedWireSegment {
            source,
            geometry: resolve_geometry(snapshot, source)?,
        });
    }
    Ok(result)
}

fn resolve_geometry(
    snapshot: &Snapshot,
    source: DirectedCurveUse,
) -> Result<MixedWireGeometry, WireFailure> {
    let geometry = match source.curve {
        CurveId::Line(line) => {
            let segment = resolve_line(
                snapshot,
                DirectedLineUse {
                    line,
                    direction: source.direction,
                },
            )?;
            pairs::require_exact_line(&segment)?;
            MixedWireGeometry::Line(segment)
        }
        CurveId::Arc(arc) => {
            let authored = snapshot.arc(arc).ok_or(WireFailure::MissingArc(arc))?;
            if snapshot.curve_role(source.curve) != Some(CurveRole::Model) {
                return Err(WireFailure::ConstructionArc(arc));
            }
            MixedWireGeometry::Arc(CardinalWireArc::capture(authored)?)
        }
        _ => return Err(WireFailure::UnsupportedCurve(source.curve)),
    };
    Ok(geometry)
}

fn curves(pair: [&MixedWireSegment; 2]) -> [CurveId; 2] {
    pair.map(|segment| segment.source.curve)
}

fn disconnected(pair: [&MixedWireSegment; 2], points: [PointId; 2]) -> WireFailure {
    WireFailure::DisjointCurveUses {
        curves: curves(pair),
        points,
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(
        DirectedCurveUse,
        MixedWireGeometry,
        MixedWireSegment,
        MixedWireRecipe,
    )>();
};
