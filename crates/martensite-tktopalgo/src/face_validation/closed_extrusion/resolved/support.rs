//! Open and closed proofs share the original global support work account.
use super::super::{budget::*, support_proof};
use super::types::*;
use martensite_tkg3d::geom::StoredSplineInput;
use martensite_tkmath::predicates::roots::{closed_extrusion::*, open_extrusion::*};
pub(in crate::face_validation::closed_extrusion) fn prove<F: FnMut() -> bool>(
    source: &StoredSplineInput,
    direction: [f64; 3],
    interval: [f64; 2],
    owner: usize,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<ToleranceResolvedSupportProof> {
    if source.poles().first() == source.poles().last() {
        return closed(source, direction, interval, owner, budget);
    }
    open(source, direction, interval, owner, budget)
}
fn closed<F: FnMut() -> bool>(
    source: &StoredSplineInput,
    direction: [f64; 3],
    interval: [f64; 2],
    owner: usize,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<ToleranceResolvedSupportProof> {
    let captured = source
        .clone()
        .into_closed_extrusion_input(direction, interval)
        .map_err(|e| admission(owner, &e))?;
    let input = super::super::operation::require_captured(captured)
        .map_err(ToleranceResolvedFaceCompletion::Face)?;
    Ok(ToleranceResolvedSupportProof::Closed(support_proof::prove(
        input, budget,
    )))
}
fn open<F: FnMut() -> bool>(
    source: &StoredSplineInput,
    direction: [f64; 3],
    interval: [f64; 2],
    owner: usize,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<ToleranceResolvedSupportProof> {
    let captured = source
        .clone()
        .into_open_extrusion_input(direction, interval)
        .map_err(|e| admission(owner, &e))?;
    let proof = certify_open_quadratic_extrusion(
        captured.numeric().clone(),
        open_policy(budget),
        &mut *budget.cancel,
    );
    support_proof::record(proof.work(), budget);
    Ok(ToleranceResolvedSupportProof::Open(proof))
}

fn open_policy<F: FnMut() -> bool>(budget: &Budget<'_, F>) -> OpenExtrusionPolicy {
    let p = support_proof::remaining(budget);
    OpenExtrusionPolicy {
        version: p.version,
        max_spans: p.max_spans,
        max_obligations: p.max_obligations,
        max_proof_steps: p.max_proof_steps,
        max_coefficient_bits: p.max_coefficient_bits,
    }
}
fn admission(owner: usize, cause: &impl std::fmt::Debug) -> ToleranceResolvedFaceCompletion {
    ToleranceResolvedFaceCompletion::Face(unsupported(
        owner,
        "support admission",
        format!("{cause:?}"),
        "coherent clamped quadratic support without evaluation override",
    ))
}

pub(in crate::face_validation::closed_extrusion) fn retain(
    slot: &mut Option<ToleranceResolvedSupportProof>,
    proof: ToleranceResolvedSupportProof,
    owner: usize,
    role: &'static str,
) -> ResolvedResult<()> {
    let complete = proof.complete();
    *slot = Some(proof);
    if !complete {
        return Err(ToleranceResolvedFaceCompletion::Support { owner, role });
    }
    Ok(())
}
