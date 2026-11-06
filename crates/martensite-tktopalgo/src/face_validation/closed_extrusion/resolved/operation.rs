//! A separate publication path preserves the exact face API and its replay results.
use super::super::{boundary, budget::*, carrier, decode, metrics, operation as exact};
use super::*;
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
/// Prove the explicitly tolerance-resolved represented-face contract on one coherent snapshot.
/// This cannot certify an exact raw-wire embedding or publish an enclosing solid.
/// Example: `certify_tolerance_resolved_extrusion_face(input, ToleranceResolvedFacePolicy::v4(), || false)`.
pub fn certify_tolerance_resolved_extrusion_face(
    input: FaceInputSnapshot,
    policy: ToleranceResolvedFacePolicy,
    mut cancelled: impl FnMut() -> bool,
) -> ToleranceResolvedFaceReport {
    let mut report = initial(input, policy);
    let mut budget = Budget {
        policy: policy.face,
        work: Default::default(),
        cancel: &mut cancelled,
    };
    let result = run(&mut report, &mut budget);
    report.work = budget.work;
    report.completion = result
        .err()
        .unwrap_or(ToleranceResolvedFaceCompletion::Certified);
    report
}
pub(super) fn initial(
    input: FaceInputSnapshot,
    policy: ToleranceResolvedFacePolicy,
) -> ToleranceResolvedFaceReport {
    ToleranceResolvedFaceReport {
        input,
        policy,
        completion: pending(),
        coverage: Default::default(),
        work: Default::default(),
        incidence_work: 0,
        errors: Vec::new(),
        support: None,
        carrier: None,
        incidences: Vec::new(),
        disjoint: None,
        transverse_coordinate: None,
    }
}
fn run<F: FnMut() -> bool>(
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    policy(report.policy)?;
    let (strip, boundary) = prepare(report, budget)?;
    let carrier = carrier::resolved::geometry(&strip, &boundary, report, budget)?;
    report.coverage.geometry = true;
    ideal(&strip, &boundary, report, budget)?;
    carrier::resolved::certify(&carrier, report, budget)?;
    finish(report, budget)
}
pub(super) fn prepare<F: FnMut() -> bool>(
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<(decode::Strip, boundary::Boundary)> {
    let mut source = exact::initial_report(report.input.clone(), report.policy.face);
    let (strip, boundary) =
        exact::prepare(&mut source, budget).map_err(ToleranceResolvedFaceCompletion::Face)?;
    report.coverage.topology = true;
    metrics::certify(&strip, &boundary, budget, &mut report.errors)
        .map_err(ToleranceResolvedFaceCompletion::Face)?;
    Ok((strip, boundary))
}
pub(super) fn finish<F: FnMut() -> bool>(
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    report.coverage.represented_noninterference = true;
    budget
        .poll()
        .map_err(ToleranceResolvedFaceCompletion::Face)?;
    report.coverage.tolerances = true;
    Ok(())
}
fn pending() -> ToleranceResolvedFaceCompletion {
    ToleranceResolvedFaceCompletion::Support {
        owner: 0,
        role: "not entered",
    }
}

fn policy(policy: ToleranceResolvedFacePolicy) -> ResolvedResult<()> {
    if policy.version == 4 && policy.face.version == 3 {
        return Ok(());
    }
    Err(ToleranceResolvedFaceCompletion::Face(unsupported(
        0,
        "resolved incidence policy",
        format!("version={}, face={}", policy.version, policy.face.version),
        "behavior four with relative-frame settings three",
    )))
}
pub(super) fn ideal<F: FnMut() -> bool>(
    strip: &decode::Strip,
    boundary: &boundary::Boundary,
    report: &mut ToleranceResolvedFaceReport,
    budget: &mut Budget<'_, F>,
) -> ResolvedResult<()> {
    let proof = super::support::prove(
        &strip.basis,
        strip.direction,
        [boundary.bounds[2], boundary.bounds[3]],
        0,
        budget,
    )?;
    super::support::retain(&mut report.support, proof, 0, "ideal support")?;
    report.coverage.support = true;
    Ok(())
}
