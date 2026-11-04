//! Declared endpoint classes select neighborhoods; no identity is inferred from distance.
mod inputs;
mod pairing;
mod work;
use super::*;
use crate::face_validation::closed_extrusion::resolved::*;
use inputs::{PairInputs, pair_inputs};
use martensite_tkmath::predicates::roots::sketch_incidence::vertex_neighborhood::*;
pub(super) fn certify<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    for cap_index in 0..carrier.strip.edges.len() {
        let Some(class) = cap_class(carrier.boundary.sides[cap_index]) else {
            continue;
        };
        cap_pairs(carrier, cap_index, class, report, budget)?;
    }
    pairing::disjoint(report, budget)
}
fn cap_pairs<F: FnMut() -> bool>(
    carrier: &Carrier<'_>,
    cap: usize,
    class: usize,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    for (seam, edge) in carrier.strip.edges.iter().enumerate() {
        if cap_class(carrier.boundary.sides[seam]).is_some() {
            continue;
        }
        let inputs = pair_inputs(carrier, &carrier.strip.edges[cap], edge, class)
            .map_err(ToleranceResolvedFaceCompletion::Face)?;
        record_pair(inputs, [cap, seam], class, report, budget)?;
    }
    Ok(())
}
fn record_pair<F: FnMut() -> bool>(
    inputs: PairInputs,
    address: [usize; 2],
    class: usize,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    let owner = inputs.owner;
    let proof = pairing::certify(inputs, report, budget, address[0], address[1])?;
    report.incidences.push(ToleranceResolvedIncidence {
        cap: address[0],
        seam: address[1],
        vertex_owner: owner,
        vertex_class: class,
        proof,
    });
    Ok(())
}

fn cap_class(side: Side) -> Option<usize> {
    match side {
        Side::VFirst => Some(0),
        Side::VLast => Some(1),
        _ => None,
    }
}
