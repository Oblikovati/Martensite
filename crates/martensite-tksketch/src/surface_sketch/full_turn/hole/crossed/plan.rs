// SPDX-License-Identifier: GPL-2.0-only
//! Prove disjoint axial apertures and name the complement of their seam crossings.
use super::*;
use placement::{CutLattice, SeamCrossings};
use std::collections::BTreeSet;

pub(in crate::surface_sketch::full_turn) struct CrossedPlan {
    pub band: FullTurnBand,
    pub stock: band_admission::BandSection,
    pub bounds: [f64; 2],
    pub holes: Vec<ApprovedHole>,
    pub rails: Vec<SeamRail>,
}

pub(in crate::surface_sketch::full_turn) struct ApprovedHole {
    pub source: tool::Contour,
    pub admitted: tool::ApertureOperand,
    pub crossings: Option<SeamCrossings>,
}

pub(in crate::surface_sketch::full_turn) struct SeamRail {
    pub ends: [PeriodicSeamEnd; 2],
    pub meridian: [f64; 2],
}

pub(super) struct NativeCut {
    pub proposal: images::Candidate,
    pub stock: Bindings,
    pub holes: Vec<Bindings>,
}

impl CrossedPlan {
    pub(super) fn admit(
        band: FullTurnBand,
        mut sources: Vec<tool::Contour>,
        depths: [f64; 2],
        options: NormalExtrusionOptions,
        scope: SeamScope,
    ) -> Checked<Self> {
        let (bounds, field) = match band.support {
            FullTurnSupport::Cylinder { axial, .. } => (axial, "aperture axial separation"),
            FullTurnSupport::Cone { slant, .. } => (slant, "aperture slant separation"),
        };
        if sources.is_empty() {
            return Err(incomplete("at least one crossed aperture", "0 apertures"));
        }
        // Native operand order follows the authored IDs, independent of the caller's list.
        sources.sort_unstable_by_key(|hole| hole.boundaries);
        distinct_contours(&sources)?;
        let stock = band_admission::admit(band, depths, options)?;
        let lattice = CutLattice::new(band.support, options)?;
        let holes = sources
            .into_iter()
            .map(|source| approve(&band, source, depths, options, &lattice, scope))
            .collect::<Checked<Vec<_>>>()?;
        let rails = approve_complement(band.rims, bounds, &holes, options, field)?;
        Ok(Self {
            band,
            stock,
            bounds,
            holes,
            rails,
        })
    }

    pub(super) fn subtract(&self) -> Result<NativeCut, SurfaceSketchFailure> {
        let stock = export_stock(&self.stock, self.band.rims)?;
        let holes = self
            .holes
            .iter()
            .map(|hole| hole.admitted.export())
            .collect::<Result<Vec<_>, _>>()?;
        let tools = holes.iter().map(|hole| hole.solid.clone());
        let proposal = images::subtract_all(&stock.solid, tools)
            .map_err(|cause| self.stock.sweep.failure(SurfaceStage::Construction, cause))?;
        Ok(NativeCut {
            proposal,
            stock: stock.entities,
            holes: holes.into_iter().map(|hole| hole.entities).collect(),
        })
    }

    pub(super) fn subtract_analytic_measures(&mut self) {
        for hole in &self.holes {
            hole.admitted.account(&mut self.stock.sweep);
        }
    }

    pub(in crate::surface_sketch::full_turn) fn crossing_count(&self) -> usize {
        self.holes
            .iter()
            .filter(|hole| hole.crossings.is_some())
            .count()
            * 2
    }
}

fn approve(
    band: &FullTurnBand,
    source: tool::Contour,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
    lattice: &CutLattice,
    scope: SeamScope,
) -> Checked<ApprovedHole> {
    let placement = lattice.classify(source)?;
    if scope == SeamScope::CrossingsOnly && placement.crossings.is_none() {
        return Err(placement::crossing_clearance(source.lift[0]));
    }
    let admitted = tool::ApertureOperand::admit(band, source, depths, options)?;
    placement.check_linear(
        admitted.minimum_radius(),
        effective_limits(options).linear_separation,
    )?;
    Ok(ApprovedHole {
        source,
        admitted,
        crossings: placement.crossings,
    })
}

fn distinct_contours(holes: &[tool::Contour]) -> Checked<()> {
    let mut curves = BTreeSet::new();
    let mut corners = BTreeSet::new();
    for hole in holes {
        for id in hole.boundaries {
            if !curves.insert(id) {
                return Err(SurfaceFailureCause::DuplicateIdentity {
                    kind: "aperture curve",
                    value: id.0,
                });
            }
        }
        for id in hole.corners {
            if !corners.insert(id) {
                return Err(SurfaceFailureCause::DuplicateIdentity {
                    kind: "aperture corner",
                    value: id.0,
                });
            }
        }
    }
    Ok(())
}

fn approve_complement(
    rims: [SurfaceCurveId; 2],
    bounds: [f64; 2],
    holes: &[ApprovedHole],
    options: NormalExtrusionOptions,
    field: &'static str,
) -> Checked<Vec<SeamRail>> {
    let mut by_meridian = holes.iter().collect::<Vec<_>>();
    by_meridian.sort_unstable_by(|a, b| a.source.meridian[0].total_cmp(&b.source.meridian[0]));
    check_separation(&by_meridian, bounds[0], options, field)?;
    let mut rails = Vec::with_capacity(holes.len() + 1);
    let mut front = (PeriodicSeamEnd::Rim(rims[0]), bounds[0]);
    for hole in by_meridian {
        let Some(crossings) = &hole.crossings else {
            continue;
        };
        let [lower, upper] = crossings.roots;
        let [first, last] = hole.source.meridian;
        rails.push(rail(front, (PeriodicSeamEnd::Crossing(lower.id), first)));
        front = (PeriodicSeamEnd::Crossing(upper.id), last);
    }
    rails.push(rail(front, (PeriodicSeamEnd::Rim(rims[1]), bounds[1])));
    Ok(rails)
}

fn check_separation(
    holes: &[&ApprovedHole],
    first: f64,
    options: NormalExtrusionOptions,
    field: &'static str,
) -> Checked<()> {
    let minimum = effective_limits(options).linear_separation / 2.;
    let mut front = first;
    for hole in holes {
        let [start, end] = hole.source.meridian;
        cylinder_admission::separated(field, (start - front).next_down(), minimum)?;
        front = end;
    }
    Ok(())
}

fn rail((a, z0): (PeriodicSeamEnd, f64), (b, z1): (PeriodicSeamEnd, f64)) -> SeamRail {
    SeamRail {
        ends: [a, b],
        meridian: [z0, z1],
    }
}
