//! Bind numerical cross-contact domains to the single decoded publication owner.
use super::*;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::GeomBezierSurface;

mod budgets;
mod carrier;
mod collar;
mod continuation;
pub(super) mod interval;
mod localization;
mod pieces;
pub(super) mod prefix;
mod rail;
mod retraction;
mod windows;
pub(super) use budgets::LoadedTolerances;
pub(super) const COLLAR_GUARD: f64 = 3e-13;
use rail::{Rail, Source};

pub(super) struct SourceGraph {
    rails: BTreeMap<CylinderBodyRef, Rail>,
    owner_tolerances: BTreeMap<CylinderBodyRef, f64>,
    pub(super) limits: LoadedTolerances,
}

impl SourceGraph {
    pub(super) fn capture(
        attempt: Attempt,
        body: &DetachedCylinderFeature,
        limits: LoadedTolerances,
        retained: &retained::BoundSubset,
        welded: &[seams::ReboundSeam],
        incidence: &seams::FrozenIncidence,
    ) -> FeatureChecked<Self> {
        let index = body::Index::new(attempt, &body.body, CylinderFeatureStage::Encoding)?;
        let source = Source {
            attempt,
            body,
            index,
            retained,
        };
        let mut rails = BTreeMap::new();
        let mut owner_tolerances = BTreeMap::new();
        let mut seen_sides = std::collections::BTreeSet::new();
        for weld in welded {
            let rail = source.owned_rail(weld, incidence)?;
            owner_tolerances.insert(weld.face(), limits.entity(weld.face())?);
            if !seen_sides.insert(rail.side) || rails.insert(weld.edge(), rail).is_some() {
                return Err(refused(
                    attempt,
                    "unique cross-contact tool side and stock seam use",
                ));
            }
        }
        if rails.len() != retained.census()[3] || !limits.support_covers(&owner_tolerances) {
            return Err(refused(
                attempt,
                "complete cross-contact rail and support owner census",
            ));
        }
        Ok(Self {
            rails,
            owner_tolerances,
            limits,
        })
    }

    pub(super) fn qualify_remaining_domains(
        &self,
        attempt: Attempt,
        separation: &carrier::Separation,
        stock: &stock_binding::BoundStock,
        topology: &topology::Topology,
        subset: &retained::BoundSubset,
    ) -> FeatureChecked<()> {
        let broad = self.limits.class_maxima.into_iter().fold(0.0_f64, f64::max);
        let fuzzy = martensite_tkernel::precision::confusion();
        let upper = interval::I::p(broad)
            .add(interval::I::p(broad))
            .add(interval::I::p(fuzzy))
            .hi;
        if separation.proof.caps().iter().any(|cap| {
            cap.signed_normal_lower() <= upper
                || cap.rim_lower().iter().any(|&bound| bound <= upper)
        }) || separation
            .proof
            .sides()
            .iter()
            .any(|side| side.rim_lower().iter().any(|&bound| bound <= upper))
        {
            return Err(refused(
                attempt,
                "separation of far caps and retained disk domains",
            ));
        }
        let [f, e, v, _] = topology.counts();
        let [tool_f, tool_e, tool_v, seams] = subset.census();
        if f != tool_f + 2 + self.owner_tolerances.len()
            || e != tool_e + 3
            || v != tool_v + 2
            || !self.owner_tolerances.contains_key(&stock.band)
            || self.rails.len() != seams
        {
            return Err(refused(
                attempt,
                "complete stock and retained tool contact partition",
            ));
        }
        Ok(())
    }
}
