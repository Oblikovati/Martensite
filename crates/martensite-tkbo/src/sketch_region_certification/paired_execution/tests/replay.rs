// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn compare_censuses(old: &BoundaryCensus, new: &BoundaryCensus) {
    assert_eq!(census_snapshot(old), census_snapshot(new));
    assert_eq!(old.current_obligations(), new.current_obligations());
    assert_eq!(old.bisections, new.bisections);
    let old_reuse = old.paired.as_ref().unwrap().reused_reports;
    assert_eq!(old_reuse, new.paired.as_ref().unwrap().reused_reports);
}
pub(super) fn result_snapshot(result: Result<BoundaryCensus, CensusCause>) -> Vec<String> {
    match result {
        Ok(census) => census_snapshot(&census),
        Err(CensusCause::Cancelled {
            completed,
            next_domain,
            during_domain,
        }) => {
            let mut snapshot = census_snapshot(&completed);
            snapshot.push(format!("cancel {next_domain:?} during={during_domain}"));
            snapshot
        }
        Err(_) => panic!("unexpected pre-enumeration failure"),
    }
}
pub(super) fn census_snapshot(census: &BoundaryCensus) -> Vec<String> {
    let paired = census.paired.as_ref().unwrap();
    let mut result = vec![format!(
        "work {} reuse {} policy {:?}",
        census.bisections, paired.reused_reports, paired.requested
    )];
    result.extend(domain_rows(census));
    for record in &paired.reports {
        result.extend(record_snapshot(record));
    }
    result
}
fn domain_rows(census: &BoundaryCensus) -> impl Iterator<Item = String> + '_ {
    census
        .domains
        .iter()
        .map(|domain| format!("{:?} {:?}", domain.occurrences, domain.evidence))
}
fn record_snapshot(record: &PairedStoredReport) -> Vec<String> {
    let mut result = vec![format!(
        "source {:?} mode {:?} effective {:?}",
        record.sources, record.mode, record.effective
    )];
    match &record.outcome {
        Ok(report) => result.extend(report_snapshot(report)),
        Err(cause) => result.push(format!("capture {cause:?}")),
    }
    result
}
fn report_snapshot(report: &BezierPairReport) -> Vec<String> {
    let mut rows = vec![format!(
        "{:?} {:?} {:?}",
        report.policy(),
        report.completion(),
        report.failure()
    )];
    rows.push(format!(
        "work {} {}",
        report.work().requested,
        report.work().completed
    ));
    rows.extend(report.source().captured_curves().map(curve_snapshot));
    if let Some(proof) = report.complete() {
        rows.extend(certificate_snapshot(&proof));
    }
    rows
}
fn curve_snapshot(curve: BezierPairCurve<'_>) -> String {
    let controls = curve
        .controls
        .iter()
        .map(|p| p.map(f64::to_bits))
        .collect::<Vec<_>>();
    let weights = curve
        .weights
        .map(|v| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    format!(
        "{controls:?} {weights:?} {:?}",
        curve.interval.map(f64::to_bits)
    )
}
fn certificate_snapshot(proof: &BezierPairCertificate<'_>) -> Vec<String> {
    let mut rows = vec![format!(
        "{:?} {:?}",
        proof.sign_stage(),
        proof.denominators()
    )];
    rows.extend(source_polynomials(proof));
    rows.extend((0..proof.contact_count()).map(|i| contact_snapshot(proof.contact(i).unwrap())));
    if let Some(graph) = proof.graph() {
        rows.extend(graph_snapshot(&graph));
    }
    rows
}
fn source_polynomials(proof: &BezierPairCertificate<'_>) -> Vec<String> {
    proof
        .homogeneous_sources()
        .into_iter()
        .flatten()
        .map(polynomial)
        .collect()
}
fn contact_snapshot(contact: CertifiedBezierPairContact<'_>) -> String {
    format!(
        "{} {:?} {:?} {:?} {:?} {}",
        contact.candidate_index,
        contact.parameter,
        contact.paired_parameter_bounds.map(f64::to_bits),
        contact.world_bounds.map(|p| p.map(f64::to_bits)),
        contact.point.map(f64::to_bits),
        contact.achieved_world_error.to_bits()
    )
}
fn graph_snapshot(graph: &BezierPairGraphCertificate<'_>) -> Vec<String> {
    let mut rows = projection_rows(graph);
    let (candidates, isolation) = graph.candidates();
    rows.push(polynomial(candidates));
    rows.extend(graph.leading_factors().map(polynomial));
    rows.push(format!(
        "{isolation:?} {:?} {:?} {:?}",
        graph.accepted_candidates(),
        graph.exceptional_fibers(),
        graph.effective_signs()
    ));
    rows.extend(graph.membership().map(membership_snapshot));
    rows
}
fn projection_rows(graph: &BezierPairGraphCertificate<'_>) -> Vec<String> {
    let projection = graph.projection();
    let mut rows = vec![polynomial(projection.alpha), polynomial(projection.beta)];
    rows.extend(projection.cleared_originals.map(polynomial));
    rows
}
fn membership_snapshot(membership: BezierPairMembership<'_>) -> String {
    format!(
        "{} {:?} {:?}",
        polynomial(membership.coefficients),
        membership.rule,
        membership.evaluation
    )
}
fn polynomial(polynomial: ExactPairPolynomial<'_>) -> String {
    let coefficients = (0..polynomial.coefficient_count())
        .map(|i| polynomial.signed_coefficient(i).unwrap().to_string())
        .collect::<Vec<_>>();
    coefficients.join(",")
}
