//! Real fitted and archived cone caps cover both disjoint and contained periodic regions.
use super::*;
use crate::surface_sketch::wrapped_profile::family::Control;
use martensite_tkbo::profile_extrusion::{LoopUse, ProfileLoop, ProfileVertex};

#[test]
fn public_connected_atlas_reports_the_periodic_overlap_owners() {
    let mut request = containment_request();
    select_uv_program(&mut request);
    let Err(error) = execute_atlas(&request) else {
        panic!("two wrapped volumes overlap");
    };
    assert_eq!(error.class(), WrappedFailureClass::InvalidInput, "{error}");
    assert_eq!(
        error.entities(),
        &[
            crate::surface_sketch::HistorySource::Region(SurfaceRegionId(1)),
            crate::surface_sketch::HistorySource::Region(SurfaceRegionId(2)),
        ]
    );
}

#[test]
fn public_connected_atlas_publishes_regions_inside_a_periodic_hole() {
    let mut request = holed_request();
    select_uv_program(&mut request);
    let result = execute_atlas(&request).unwrap();
    assert_eq!(result.reports().len(), 2);
    assert!(
        result
            .reports()
            .values()
            .all(|report| report.validation.is_valid())
    );
    let proof = result.reference_body_receipt();
    assert_eq!(proof.decoded_caps().unwrap().len(), 2);
    assert!(
        proof
            .decoded_solid_separation()
            .unwrap()
            .compare_f64(0.)
            .unwrap()
            .is_gt()
    );
    let first = result.planar_integral_receipt(SurfaceRegionId(1)).unwrap();
    let second = result.planar_integral_receipt(SurfaceRegionId(2)).unwrap();
    assert!(first.shares_issuance(second));
    let members = &result.family.family.members;
    let first = &members[&SurfaceRegionId(1)].geometry.joins;
    let second = &members[&SurfaceRegionId(2)].geometry.joins;
    assert!(proof.shares_decoded_join_issuance([first, second]));
    assert!(!proof.shares_decoded_join_issuance([second, first]));
    assert!(!proof.shares_decoded_join_issuance([first]));
}

#[test]
fn disjoint_closed_atlas_members_bind_one_family_certificate() {
    let mut request = strip_request();
    append_quad(
        &mut request,
        2,
        100,
        [[40., 0.], [41., 0.], [41., 1.], [40., 1.]],
    );
    request.selections.reverse();
    let mut work = Work::new(request.proof.arithmetic);
    let (prepared, stored) = freeze_family(&request, &mut work);
    let projections = projections(&prepared, &stored);
    assert_key_stability(&prepared, &stored, &projections);
    let planar = periodic::planar(&projections, &request, &mut work);
    let boundary = periodic::clearance(&projections, &mut work);
    let material = periodic::classify(&projections, &planar, &boundary, &mut work).unwrap();
    assert_both_directions(&material);
    let proofs = homotopies(&stored, &boundary);
    assert_bound_family(&stored, &boundary, &proofs);
    assert_member_measures(&projections, proofs, &mut work);
}

#[test]
fn a_periodic_image_inside_another_region_refuses_despite_boundary_clearance() {
    let request = containment_request();
    let mut work = Work::new(request.proof.arithmetic);
    let (prepared, stored) = freeze_family(&request, &mut work);
    let projections = projections(&prepared, &stored);
    let planar = periodic::planar(&projections, &request, &mut work);
    let boundary = periodic::clearance(&projections, &mut work);
    assert_eq!(boundary.periodic.images().len(), 1);
    let proofs = homotopies(&stored, &boundary);
    assert_eq!(proofs.len(), 2);
    let before = work.consumed;
    let error = periodic::classify(&projections, &planar, &boundary, &mut work).unwrap_err();
    assert!(work.consumed > before);
    assert!(
        matches!(
            error,
            periodic_selection::MaterialFailure::Overlap {
                source: SurfaceRegionId(2), target: SurfaceRegionId(1), probe, ..
            } if probe.turn == 1
        ),
        "{error:?}"
    );
}

#[test]
fn a_periodic_image_inside_a_certified_hole_keeps_both_regions() {
    use periodic_selection::material::OutsideReason;
    let request = holed_request();
    let mut work = Work::new(request.proof.arithmetic);
    let (prepared, stored) = freeze_family(&request, &mut work);
    let projections = projections(&prepared, &stored);
    let planar = periodic::planar(&projections, &request, &mut work);
    let boundary = periodic::clearance(&projections, &mut work);
    let material = periodic::classify(&projections, &planar, &boundary, &mut work).unwrap();
    assert!(material.relations.iter().any(|relation| {
        relation.source == SurfaceRegionId(2)
            && relation.target == SurfaceRegionId(1)
            && matches!(relation.reason, OutsideReason::InsideHole(_))
    }));
    assert_member_measures(&projections, homotopies(&stored, &boundary), &mut work);
}

fn holed_request() -> WrappedConeNormalRequest {
    let mut request = containment_request();
    let mut hole = sector_quad(request.support.semi_angle, [20.5, 23.5], [-1.2, -0.5]);
    hole.reverse();
    let hole = append_loop(&mut request, 50, hole);
    request.selections[0].holes.push(hole);
    request
}

fn containment_request() -> WrappedConeNormalRequest {
    let mut request = strip_request();
    request.profile.vertices.clear();
    request.profile.curves.clear();
    request.selections.clear();
    let angle = request.support.semi_angle;
    let turn = std::f64::consts::TAU;
    append_quad(
        &mut request,
        1,
        1,
        sector_quad(angle, [20., 24.], [-1.4, -0.3]),
    );
    append_quad(
        &mut request,
        2,
        100,
        sector_quad(angle, [21., 22.], [turn - 1., turn - 0.8]),
    );
    request
}

fn sector_quad(alpha: f64, [r0, r1]: [f64; 2], [u0, u1]: [f64; 2]) -> [[f64; 2]; 4] {
    let sine = martensite_cmath::sin(alpha);
    [(r0, u0), (r1, u0), (r1, u1), (r0, u1)].map(|(slant, angle)| {
        let developed = sine * angle;
        [
            slant * martensite_cmath::cos(developed),
            slant * martensite_cmath::sin(developed),
        ]
    })
}

fn append_quad(
    request: &mut WrappedConeNormalRequest,
    region: u64,
    first: u64,
    points: [[f64; 2]; 4],
) {
    let outer = append_loop(request, first, points);
    request.selections.push(WrappedRegionSelection {
        region: SurfaceRegionId(region),
        outer,
        holes: Vec::new(),
    });
}

fn append_loop(
    request: &mut WrappedConeNormalRequest,
    first: u64,
    points: [[f64; 2]; 4],
) -> ProfileLoop {
    let mut uses = Vec::new();
    for (i, point) in points.into_iter().enumerate() {
        let id = first + i as u64;
        request.profile.vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point,
        });
        request.profile.curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(first + (i as u64 + 1) % 4),
        });
        uses.push(LoopUse {
            curve: ProfileCurveId(id),
            reversed: false,
        });
    }
    ProfileLoop { uses }
}

fn freeze_family(
    request: &WrappedConeNormalRequest,
    work: &mut Work,
) -> (Vec<Prepared>, Vec<storage::Stored>) {
    let run = ConeRun::from(request);
    let inputs = input::capture(request, &mut || false).unwrap();
    super::super::super::super::selection::planar(
        &run,
        &inputs.originals,
        atlas_selection(),
        work,
        &mut || false,
    )
    .unwrap();
    let owner = inputs
        .originals
        .iter()
        .min_by_key(|source| source.region)
        .unwrap();
    let reference = super::super::super::super::reference(owner).unwrap();
    let prepared = inputs
        .originals
        .iter()
        .map(|source| prepare_local(&run, source, Some(reference), work))
        .collect::<Vec<_>>();
    let stored = capture_family(&run, &inputs.originals, &prepared, work);
    (prepared, stored)
}

fn capture_family(
    run: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    prepared: &[Prepared],
    work: &mut Work,
) -> Vec<storage::Stored> {
    let mut continue_operation = |_| false;
    let mut control = Control::new(&mut continue_operation);
    let stage = stages::ConeStages {
        request: run,
        execution_selection: prepared[0].executed_selection,
    };
    let (_, keys, root, names) =
        operation::frozen::construct(&stage, prepared, &mut control).unwrap();
    let (coverage, members) = coverage::certify_developed_family(prepared, &root, &names).unwrap();
    let history = history::certify_developed(originals, prepared, &keys).unwrap();
    assert_eq!(coverage.members, prepared.len());
    assert_eq!(coverage.bindings, history.records.len());
    let mut stored =
        operation::members::store(&stage, originals, prepared, members, work, &mut control)
            .unwrap();
    originals
        .iter()
        .zip(prepared)
        .map(|(source, prepared)| {
            let member = stored.remove(&source.region).unwrap();
            validate_member(source, prepared, &member);
            member
        })
        .collect()
}

fn validate_member(source: &DevelopedRegionRequest, prepared: &Prepared, stored: &storage::Stored) {
    let resolution = crate::surface_sketch::wrapped_profile::pipeline::resolution(source.tolerance);
    let report = validate_body(&stored.shape, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    storage::bind_cap_joins(source, prepared, stored).unwrap();
}

fn projections(prepared: &[Prepared], stored: &[storage::Stored]) -> Vec<Request> {
    let members = prepared
        .iter()
        .zip(stored)
        .map(|(prepared, stored)| (prepared, stored.spans.as_slice()))
        .collect::<Vec<_>>();
    periodic::project(&members)
}

fn assert_key_stability(prepared: &[Prepared], stored: &[storage::Stored], original: &[Request]) {
    let mut members = prepared
        .iter()
        .zip(stored)
        .map(|(prepared, stored)| (prepared, stored.spans.as_slice()))
        .collect::<Vec<_>>();
    members.reverse();
    let reversed = periodic::project(&members);
    for (first, second) in original.iter().zip(reversed.iter().rev()) {
        assert_eq!(first.region, second.region);
        assert!(
            first
                .loops
                .iter()
                .flatten()
                .zip(second.loops.iter().flatten())
                .all(|(a, b)| a.span == b.span && a.start == b.start && a.end == b.end)
        );
    }
}

fn assert_both_directions(material: &periodic_selection::PeriodicMaterialEvidence) {
    let cross: Vec<_> = material
        .relations
        .iter()
        .filter(|relation| relation.source != relation.target)
        .collect();
    assert_eq!(cross.len(), 4);
    for region in [SurfaceRegionId(1), SurfaceRegionId(2)] {
        assert_eq!(
            cross
                .iter()
                .filter(|relation| relation.source == region)
                .count(),
            2
        );
    }
}

fn homotopies<'a>(
    stored: &'a [storage::Stored],
    boundary: &planar_clearance::OriginalClearance,
) -> Vec<ConicalCapHomotopyEvidence<'a>> {
    let joins = stored
        .iter()
        .map(|stored| &stored.joins)
        .collect::<Vec<_>>();
    certify_conical_cap_family_homotopy(&joins, &boundary.certificate, &boundary.periodic).unwrap()
}

fn assert_bound_family(
    stored: &[storage::Stored],
    boundary: &planar_clearance::OriginalClearance,
    proofs: &[ConicalCapHomotopyEvidence<'_>],
) {
    assert_eq!(proofs.len(), 2);
    assert_eq!(
        proofs[0].separation_lower_bounds(),
        proofs[1].separation_lower_bounds()
    );
    let swapped = [&stored[1].joins, &stored[0].joins];
    let error =
        certify_conical_cap_family_homotopy(&swapped, &boundary.certificate, &boundary.periodic)
            .err()
            .unwrap();
    assert!(matches!(
        error.cause,
        ConicalCapHomotopyFailure::Binding(ConicalCapClearanceRelation::Nonadjacent)
    ));
    let only = [&stored[0].joins];
    assert!(
        certify_conical_cap_family_homotopy(&only, &boundary.certificate, &boundary.periodic)
            .is_err()
    );
}

fn assert_member_measures(
    projections: &[Request],
    proofs: Vec<ConicalCapHomotopyEvidence<'_>>,
    work: &mut Work,
) {
    for (profile, proof) in projections.iter().zip(proofs) {
        let decoded = proof
            .certify_decoded_reference(profile.tolerance, work.remaining(), || false)
            .unwrap();
        work.charge(profile.region, None, decoded.operations())
            .unwrap();
        assert_decoded_boundary(&decoded, profile.tolerance);
        assert!(
            decoded.normal().volume()[0]
                .compare_f64(0.)
                .unwrap()
                .is_gt()
        );
    }
}
