// SPDX-License-Identifier: GPL-2.0-only
//! An owned snapshot expires at each batch; ordinary views remain live afterwards.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods::edge};
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::gp::Point3;
#[test]
fn owned_batches_recapture_curve_edits_and_preserve_existing_live_view_guards() {
    let prepared = super::super::super::pipeline::prepare_owned_profile(request(), &mut Continue)
        .unwrap_or_else(|_| panic!("owned fixture preparation"));
    let count = std::cell::Cell::new(0);
    let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &count);
    let census = &prepared.bridge.census;
    let domain = &census.domains[0];
    let live = crate::sketch_region_certification::DomainCertificates::new(census).unwrap();
    assert!(
        SourceAccess::Owned(&access)
            .certificates(census)
            .unwrap()
            .certificate(domain)
            .is_some()
    );
    check_mutated_batch(census, domain, &access, &count, &live);
}
fn check_mutated_batch(
    census: &BoundaryCensus,
    domain: &crate::sketch_region_certification::BoundaryDomain,
    access: &OwnedGraphAccess<'_>,
    count: &std::cell::Cell<usize>,
    live: &crate::sketch_region_certification::DomainCertificates<'_>,
) {
    let before = count.get();
    change_curve(census);
    assert!(
        SourceAccess::Owned(access)
            .certificates(census)
            .unwrap()
            .certificate(domain)
            .is_none()
    );
    assert!(count.get() > before);
    assert!(live.certificate(domain).is_none());
    assert!(!census.source_representation_certified());
}
pub(super) fn change_curve(census: &BoundaryCensus) {
    let native = edge(&census.inputs.segments[0].edge).unwrap();
    let (curve, _, _) = BRepTool::curve(&native).unwrap();
    curve
        .downcast::<GeomBezierCurve>()
        .set_pole(2, Point3::new(0.5, -0.25, 0.))
        .unwrap();
}
#[test]
fn owned_manifest_rejects_wrong_source_order_coefficients_and_declared_policy() {
    let prepared = super::super::super::pipeline::prepare_owned_profile(request(), &mut Continue)
        .unwrap_or_else(|_| panic!("owned fixture preparation"));
    let count = std::cell::Cell::new(0);
    for mutation in 0..4 {
        let mut changed = prepared.request.clone();
        match mutation {
            0 => changed.edges.swap(0, 1),
            1 => changed.edges[0].source = CallerBoundaryId(55),
            2 => changed.edges[0].world_poles[1][0] = 0.75,
            3 => changed.policy.source.construction_limit = 1e-4,
            _ => panic!("mutation={mutation}"),
        }
        let access = OwnedGraphAccess::fresh(&changed, &prepared.edges, &count);
        assert!(!access.binds(&prepared.bridge.census));
    }
}
