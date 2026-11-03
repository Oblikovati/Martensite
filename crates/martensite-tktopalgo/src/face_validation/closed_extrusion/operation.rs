//! Private publication joins every required witness to the same immutable source.
use super::{boundary, budget::*, decode, incidence, metrics, report::ClosedFaceReport, types::*};
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
use martensite_tkg3d::geom::ClosedSplineExtrusionInput;
use martensite_tkmath::predicates::roots::closed_extrusion::*;

/// Certify the complete admitted represented face; `certify_closed_extrusion_face(input, ClosedFacePolicy::bounded(), ||false)`.
/// The report never validates a later-mutated handle or an entire enclosing solid.
pub fn certify_closed_extrusion_face(
    input: FaceInputSnapshot,
    policy: ClosedFacePolicy,
    mut cancelled: impl FnMut() -> bool,
) -> ClosedFaceReport {
    let mut report = initial_report(input, policy);
    let mut budget = Budget {
        policy,
        work: ClosedFaceWork::default(),
        cancel: &mut cancelled,
    };
    let completion = run(&mut report, &mut budget);
    report.work = budget.work;
    report.completion = match completion {
        Ok(()) => ClosedFaceCompletion::Certified,
        Err(cause) => cause,
    };
    report
}
fn run<F: FnMut() -> bool>(
    report: &mut ClosedFaceReport,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    validate_policy(budget.policy)?;
    let strip = decode::decode(&report.input, budget)?;
    let boundary = boundary::certify(&strip, budget)?;
    incidence::certify(&strip, &boundary, budget)?;
    report.coverage.topology = true;
    let support_input = admit_support(&strip, &boundary)?;
    metrics::certify(&strip, &boundary, budget, &mut report.errors)?;
    report.coverage.geometry = true;
    support(report, support_input, budget)?;
    boundary_identity(&report.errors, budget)?;
    report.coverage.self_exclusion = true;
    budget.poll()?;
    report.coverage.tolerances = true;
    Ok(())
}
fn admit_support(
    strip: &decode::Strip,
    boundary: &boundary::Boundary,
) -> CheckResult<ClosedQuadraticExtrusionInput> {
    let input = strip
        .basis
        .clone()
        .into_closed_extrusion_input(strip.direction, [boundary.bounds[2], boundary.bounds[3]])
        .map_err(|e| {
            unsupported(
                0,
                "support admission",
                format!("{e:?}"),
                "coherent exact closed quadratic input",
            )
        })?;
    require_captured(input)
}
fn require_captured(
    input: ClosedSplineExtrusionInput,
) -> CheckResult<ClosedQuadraticExtrusionInput> {
    match input {
        ClosedSplineExtrusionInput::Captured(input, _) => Ok(input),
        other => Err(unsupported(
            0,
            "support layout",
            format!("{other:?}"),
            "closed clamped quadratic without override",
        )),
    }
}

fn support<F: FnMut() -> bool>(
    report: &mut ClosedFaceReport,
    input: ClosedQuadraticExtrusionInput,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let proof = super::support_proof::prove(input, budget);
    let result = support_completion(proof.completion());
    report.support = Some(proof);
    result
}

fn initial_report(input: FaceInputSnapshot, policy: ClosedFacePolicy) -> ClosedFaceReport {
    ClosedFaceReport {
        input,
        policy,
        completion: ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Cancelled),
        coverage: ClosedFaceCoverage::default(),
        work: ClosedFaceWork::default(),
        errors: Vec::new(),
        support: None,
    }
}

fn validate_policy(policy: ClosedFacePolicy) -> CheckResult<()> {
    if policy.version != 1 {
        return Err(unsupported(
            0,
            "policy version",
            format!("{}", policy.version),
            "version 1",
        ));
    }
    if policy.max_bisections > u16::MAX as u64 {
        return Err(unsupported(
            0,
            "metric bisection policy",
            format!("{}", policy.max_bisections),
            "at most 65535 subdivisions",
        ));
    }
    Ok(())
}

fn support_completion(completion: &ClosedExtrusionCompletion) -> CheckResult<()> {
    match completion {
        ClosedExtrusionCompletion::Certified => Ok(()),
        ClosedExtrusionCompletion::Incomplete(cause) => Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::Support(cause.clone()),
        )),
        ClosedExtrusionCompletion::NotExcluded(contact) => Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::Projection(*contact),
        )),
    }
}

fn boundary_identity<F: FnMut() -> bool>(
    errors: &[ClosedFaceErrorBound],
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    for bound in errors {
        budget.query()?;
        if !matches!(
            bound.squared_upper.compare_f64(0.),
            Ok(std::cmp::Ordering::Equal)
        ) {
            return Err(ClosedFaceCompletion::Incomplete(
                ClosedFaceIncomplete::BoundaryIdentity(bound.clone()),
            ));
        }
    }
    Ok(())
}
