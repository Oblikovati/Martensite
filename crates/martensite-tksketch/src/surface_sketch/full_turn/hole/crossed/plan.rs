// SPDX-License-Identifier: GPL-2.0-only
//! Prove disjoint axial apertures and name the complement of their seam crossings.
use super::*;
use std::collections::BTreeSet;

pub(in crate::surface_sketch::full_turn) struct CrossedPlan {
    pub band: FullTurnBand,
    pub stock: band_admission::BandSection,
    pub holes: Vec<ApprovedHole>,
    pub rails: Vec<SeamRail>,
}

pub(in crate::surface_sketch::full_turn) struct ApprovedHole {
    pub source: CylinderBandHole,
    pub admitted: cylinder_admission::Admitted,
    pub crossings: [PeriodicCrossing; 2],
}

pub(in crate::surface_sketch::full_turn) struct SeamRail {
    pub ends: [PeriodicSeamEnd; 2],
    pub axial: [f64; 2],
}

pub(super) struct NativeCut {
    pub proposal: images::Candidate,
    pub stock: Bindings,
    pub holes: Vec<Bindings>,
}

impl CrossedPlan {
    pub(super) fn admit(
        band: FullTurnBand,
        holes: &[CylinderBandHole],
        depths: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<Self> {
        let (_, axial) = cylinder(band.support)?;
        if holes.is_empty() {
            return Err(incomplete(
                "at least one crossed cylindrical aperture",
                "0 apertures",
            ));
        }
        // Native operand order follows the authored IDs, independent of the caller's list.
        let mut sources = holes.to_vec();
        sources.sort_unstable_by_key(|hole| hole.boundaries);
        distinct_contours(&sources)?;
        let holes = sources
            .into_iter()
            .map(|source| approve(&band, source, depths, options))
            .collect::<Checked<Vec<_>>>()?;
        let rails = approve_complement(band.rims, axial, &holes, options)?;
        let stock = band_admission::admit(band, depths, options)?;
        Ok(Self {
            band,
            stock,
            holes,
            rails,
        })
    }

    pub(super) fn subtract(&self) -> Result<NativeCut, SurfaceSketchFailure> {
        let stock = export_stock(&self.stock, self.band.rims)?;
        let holes = self
            .holes
            .iter()
            .map(|hole| export_aperture(&hole.admitted))
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
            account_aperture(&mut self.stock.sweep, &hole.admitted);
        }
    }

    pub(in crate::surface_sketch::full_turn) fn crossing_count(&self) -> usize {
        self.holes.len() * 2
    }
}

fn approve(
    band: &FullTurnBand,
    source: CylinderBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<ApprovedHole> {
    let admitted = admit_aperture(band, &source, depths, options)?;
    require_authored_crossing(&admitted)?;
    let crossings = names::crossings(&source)?;
    Ok(ApprovedHole {
        source,
        admitted,
        crossings,
    })
}

fn distinct_contours(holes: &[CylinderBandHole]) -> Checked<()> {
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
) -> Checked<Vec<SeamRail>> {
    let mut by_axial = holes.iter().collect::<Vec<_>>();
    by_axial.sort_unstable_by(|a, b| a.source.axial[0].total_cmp(&b.source.axial[0]));
    let mut rails = Vec::with_capacity(holes.len() + 1);
    let mut front = (PeriodicSeamEnd::Rim(rims[0]), bounds[0]);
    let minimum = effective_limits(options).linear_separation / 2.;
    for hole in by_axial {
        let [lower, upper] = hole.crossings;
        let [start, last] = hole.source.axial;
        cylinder_admission::separated("aperture axial separation", start - front.1, minimum)?;
        let endpoint = (PeriodicSeamEnd::Crossing(lower.id), start);
        rails.push(rail(front, endpoint));
        front = (PeriodicSeamEnd::Crossing(upper.id), last);
    }
    rails.push(rail(front, (PeriodicSeamEnd::Rim(rims[1]), bounds[1])));
    Ok(rails)
}

fn rail((a, z0): (PeriodicSeamEnd, f64), (b, z1): (PeriodicSeamEnd, f64)) -> SeamRail {
    SeamRail {
        ends: [a, b],
        axial: [z0, z1],
    }
}
