// SPDX-License-Identifier: GPL-2.0-only
//! Issue #449: one immutable validation view preserves every per-domain refusal.
use super::paired_binding::{DomainCertificates, domain_certificate, paired_work_exact};
use super::paired_tests::{curved_pair, paired};
use super::*;

#[test]
fn borrowed_validation_retains_analytic_contact_counts_for_every_alias_domain() {
    let mut shapes = curved_pair();
    shapes.push(shapes[0].reversed());
    let census = paired(&shapes);
    let shared = DomainCertificates::new(&census).unwrap();
    let actual = census
        .domains
        .iter()
        .map(|domain| {
            shared
                .certificate(domain)
                .map(|proof| proof.contact_count())
        })
        .collect::<Vec<_>>();
    // Both quadratics have x=t; t²=1/2-t² has one root t=1/2 in [0,1].
    assert_eq!(
        actual,
        vec![Some(0), Some(1), None, Some(0), Some(1), Some(0)]
    );
}

#[test]
fn borrowed_validation_rechecks_global_work_after_each_edit() {
    for mutation in 0..4 {
        let mut shapes = curved_pair();
        shapes.push(shapes[0].reversed());
        let mut census = paired(&shapes);
        assert!(DomainCertificates::new(&census).is_some());
        corrupt_work_ledger(&mut census, mutation);
        assert!(!paired_work_exact(&census));
        assert!(DomainCertificates::new(&census).is_none());
    }
}
fn corrupt_work_ledger(census: &mut BoundaryCensus, mutation: usize) {
    let paired = census.paired.as_mut().unwrap();
    match mutation {
        0 => paired.reused_reports += 1,
        1 => census.bisections += 1,
        2 => corrupt_first_report(census, usize::MAX, false),
        3 => corrupt_first_report(census, 0, true),
        _ => unreachable!(),
    }
}
fn corrupt_first_report(census: &mut BoundaryCensus, index: usize, reused_value: bool) {
    let evidence = census
        .domains
        .iter_mut()
        .find_map(|domain| match &mut domain.evidence {
            DomainEvidence::PairedReport { report, reused, .. } => Some((report, reused)),
            _ => None,
        })
        .unwrap();
    *evidence.0 = index;
    *evidence.1 = reused_value;
}

#[test]
fn borrowed_validation_keeps_domain_specific_binding_checks() {
    let census = paired(&curved_pair());
    let shared = DomainCertificates::new(&census).unwrap();
    let mut domain = BoundaryDomain {
        occurrences: census.domains[1].occurrences,
        evidence: census.domains[1].evidence,
    };
    assert!(shared.certificate(&domain).is_some());
    domain.occurrences.swap(0, 1);
    assert!(shared.certificate(&domain).is_none());
}

#[test]
fn borrowed_validation_refuses_native_geometry_mutated_through_an_alias() {
    assert_native_alias_refused(mutate_aliased_curve);
}
#[test]
fn borrowed_validation_refuses_native_pcurve_mutated_through_an_alias() {
    assert_native_alias_refused(mutate_aliased_pcurve);
}
#[test]
fn borrowed_validation_refuses_native_support_mutated_through_an_alias() {
    assert_native_alias_refused(mutate_aliased_support);
}
fn assert_native_alias_refused(mutate: fn(&BoundaryCensus)) {
    let census = paired(&curved_pair());
    let shared = DomainCertificates::new(&census).unwrap();
    let domain = &census.domains[1];
    assert!(shared.certificate(domain).is_some());
    mutate(&census);
    assert!(!census.source_representation_certified());
    assert!(domain_certificate(&census, domain).is_none());
    assert!(shared.certificate(domain).is_none());
    assert!(shared.certificate(&census.domains[2]).is_none());
}

fn mutate_aliased_curve(census: &BoundaryCensus) {
    use martensite_tkbrep::{brep::BRepTool, topods::edge};
    use martensite_tkg3d::geom::GeomBezierCurve;
    use martensite_tkmath::gp::Point3;
    let actual = edge(&census.inputs.segments[0].edge).unwrap();
    let (curve, _, _) = BRepTool::curve(&actual).unwrap();
    curve
        .downcast::<GeomBezierCurve>()
        .set_pole(2, Point3::new(8., 0., 1.))
        .unwrap();
}

fn mutate_aliased_pcurve(census: &BoundaryCensus) {
    use martensite_tkbrep::{brep::BRepTool, topods::edge};
    use martensite_tkg2d::geom2d::Geom2dBezierCurve;
    use martensite_tkmath::gp::Point2;
    let actual = edge(&census.inputs.segments[0].edge).unwrap();
    let stored = BRepTool::curve_on_face(&actual, &census.inputs.construction_support).unwrap();
    stored
        .pcurve
        .downcast::<Geom2dBezierCurve>()
        .set_pole(2, Point2::new(9., 8.))
        .unwrap();
}
fn mutate_aliased_support(census: &BoundaryCensus) {
    use martensite_tkbrep::brep::BRepTool;
    use martensite_tkg3d::geom::GeomPlane;
    use martensite_tkmath::gp::Point3;
    let (surface, _) =
        BRepTool::surface_with_location(&census.inputs.construction_support).unwrap();
    let plane = surface.downcast::<GeomPlane>();
    let mut changed = plane.pln();
    changed.set_location(Point3::new(0., 0., 1.));
    plane.set_pln(changed);
}
