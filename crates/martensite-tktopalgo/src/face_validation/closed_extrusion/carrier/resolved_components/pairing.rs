//! Complete exact contacts may be absent; neither absence nor a bound alone proves a face.
use super::*;
pub(super) fn certify<F: FnMut() -> bool>(
    inputs: PairInputs,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
    cap: usize,
    seam: usize,
) -> ResolvedResult<GivenVertexContactsReport> {
    budget
        .poll()
        .map_err(ToleranceResolvedFaceCompletion::Face)?;
    let limit = work::remaining(report)?;
    let first = neighborhood(inputs.cap, cap, report, budget)?;
    let second = neighborhood(inputs.seam, seam, report, budget)?;
    classify_pair([&first, &second], limit, [cap, seam], report, budget)
}
fn classify_pair<F: FnMut() -> bool>(
    pair: [&GivenVertexNeighborhoodReport; 2],
    limit: u32,
    address: [usize; 2],
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<GivenVertexContactsReport> {
    let proof = certify_given_vertex_contacts(
        pair[0],
        pair[1],
        VertexNeighborhoodPolicy::v1(limit),
        &mut *budget.cancel,
    );
    let prior = u64::from(pair[0].work()) + u64::from(pair[1].work());
    work::record(report, u64::from(proof.work()).saturating_sub(prior) as u32)?;
    require_contacts(proof, address)
}
fn require_contacts(
    proof: GivenVertexContactsReport,
    address: [usize; 2],
) -> ResolvedResult<GivenVertexContactsReport> {
    if proof.contacts().is_none() {
        return Err(ToleranceResolvedFaceCompletion::Contacts {
            cap: address[0],
            seam: address[1],
            report: Box::new(proof),
        });
    }
    Ok(proof)
}

fn neighborhood<F: FnMut() -> bool>(
    input: GivenVertexNeighborhoodInput,
    edge: usize,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<GivenVertexNeighborhoodReport> {
    let limit = work::remaining(report)?;
    let proof = certify_given_vertex_neighborhood(
        input,
        VertexNeighborhoodPolicy::v1(limit),
        &mut *budget.cancel,
    );
    work::record(report, proof.work())?;
    require_neighborhood(proof, edge)
}
fn require_neighborhood(
    proof: GivenVertexNeighborhoodReport,
    edge: usize,
) -> ResolvedResult<GivenVertexNeighborhoodReport> {
    if proof.certificate().is_none() {
        return Err(ToleranceResolvedFaceCompletion::Neighborhood {
            edge,
            report: Box::new(proof),
        });
    }
    Ok(proof)
}

pub(super) fn disjoint<F: FnMut() -> bool>(
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    let first = endpoint(report, 0)?;
    let second = endpoint(report, 1)?;
    let [prior, limit] = work::pair_allowance(report, [first, second])?;
    let proof = certify_disjoint_vertex_neighborhoods(
        first,
        second,
        VertexNeighborhoodPolicy::v1(limit),
        &mut *budget.cancel,
    );
    work::record(report, proof.work().saturating_sub(prior))?;
    retain_disjoint(proof, report)
}
fn retain_disjoint(
    proof: DisjointVertexNeighborhoodReport,
    report: &mut ToleranceResolvedFaceReport,
) -> ResolvedResult<()> {
    if !proof.separated() {
        return Err(ToleranceResolvedFaceCompletion::Disjoint(Box::new(proof)));
    }
    report.disjoint = Some(proof);
    Ok(())
}

fn endpoint(
    report: &ToleranceResolvedFaceReport,
    class: usize,
) -> ResolvedResult<&GivenVertexNeighborhoodReport> {
    report
        .incidences
        .iter()
        .find(|p| p.vertex_class == class)
        .map(|p| p.proof.source())
        .ok_or_else(|| {
            ToleranceResolvedFaceCompletion::Face(invalid(
                0,
                "declared endpoint class",
                format!("missing class {class}"),
            ))
        })
}
