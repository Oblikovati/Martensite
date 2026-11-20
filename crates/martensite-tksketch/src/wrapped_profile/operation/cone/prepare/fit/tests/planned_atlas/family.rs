//! Real fitted and archived cone caps cover both disjoint and contained periodic regions.
use super::*;
use martensite_tkbo::profile_extrusion::{LoopUse, ProfileLoop, ProfileVertex};

#[test]
fn disjoint_closed_atlas_members_bind_one_family_certificate() {
    let mut request = strip_request();
    append_quad(
        &mut request,
        2,
        100,
        [[40., 0.], [41., 0.], [41., 1.], [40., 1.]],
    );
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
    let mut request = containment_request();
    let mut hole = sector_quad(request.support.semi_angle, [20.5, 23.5], [-1.2, -0.5]);
    hole.reverse();
    let hole = append_loop(&mut request, 50, hole);
    request.selections[0].holes.push(hole);
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
    super::super::super::super::selection::planar(&run, &inputs.originals, work, &mut || false)
        .unwrap();
    let [x, y] = inputs.originals[0].loops[0][0].poles[0];
    let reference = martensite_cmath::hypot(x, y);
    let prepared = inputs
        .originals
        .iter()
        .map(|source| prepare_member(&run, source, reference, work))
        .collect::<Vec<_>>();
    let stored = inputs
        .originals
        .iter()
        .zip(&prepared)
        .map(|(source, prepared)| freeze_member(request, source, prepared, work))
        .collect();
    (prepared, stored)
}

fn prepare_member(
    run: &ConeRun<'_>,
    source: &DevelopedRegionRequest,
    reference: f64,
    work: &mut Work,
) -> Prepared {
    let mut prepared = prepare_local(run, source, Some(reference), work);
    super::super::super::super::canonicalize::uv_endpoints(&mut prepared).unwrap();
    prepared
}

fn freeze_member(
    request: &WrappedConeNormalRequest,
    source: &DevelopedRegionRequest,
    prepared: &Prepared,
    work: &mut Work,
) -> storage::Stored {
    let body = decode_member(prepared, request.support);
    let resolution = crate::surface_sketch::wrapped_profile::pipeline::resolution(source.tolerance);
    let report = validate_body(&body.shape, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    storage::capture(source, prepared, body, work, &mut || false).unwrap()
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
