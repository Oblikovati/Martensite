//! Actual retained coefficients and source-keyed joins are independent required controls.
use super::*;
fn owner() -> (SurfaceRegionId, CurveSpan) {
    (
        SurfaceRegionId(9),
        CurveSpan {
            curve: SurfaceCurveId(81),
            level: 1,
            index: 0,
        },
    )
}
#[test]
fn exact_uv_closure_is_required_even_for_sub_tolerance_gaps() {
    let (region, span) = owner();
    let cap = vec![[0., 0.], [0.1, 1.]];
    let pcurves = [cap.clone(), cap];
    let next = [[0.1, 1.], [0., 0.]];
    assert!(domain::validate_uv(region, span, &pcurves, &next, true).is_ok());
    let changed = [[0.1_f64.next_up(), 1.], [0., 0.]];
    let error = domain::validate_uv(region, span, &pcurves, &changed, true).unwrap_err();
    assert_eq!(error.span, Some(span));
    assert_eq!(error.region, Some(region));
}
#[test]
fn identical_endpoints_do_not_replace_source_join_identity() {
    let (region, span) = owner();
    let cap = vec![[0., 0.], [0.1, 1.]];
    let pcurves = [cap.clone(), cap];
    assert!(domain::validate_uv(region, span, &pcurves, &[[0.1, 1.]], false).is_err());
}
#[test]
fn second_cap_cannot_change_the_filled_uv_domain() {
    let (region, span) = owner();
    let cap = vec![[0., 0.], [0.1, 1.]];
    let mut pcurves = [cap.clone(), cap];
    pcurves[1][0][0] = 0.01;
    assert!(domain::validate_uv(region, span, &pcurves, &[[0.1, 1.]], true).is_err());
}

#[test]
fn radial_uv_cubic_self_domain_completes_for_owned_cone_coefficients() {
    use martensite_tkmath::predicates::roots::bezier_pair::*;
    let controls = owned_radial_uv();
    assert_radial_uv_bits(&controls);
    let query = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
    let policy = BezierPairExclusionPolicy {
        version: 1,
        pair_policy: radial_self_policy(),
    };
    let report = query.certify_bernstein_exclusion(policy, || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Excluded);
    let proof = report
        .complete()
        .expect("source-bound exclusion of the complete self domain");
    assert_eq!(proof.coordinate_axis(), 0);
    assert!(proof.divided_self_diagonal());
}
#[test]
fn legacy_radial_uv_self_query_retains_documented_non_linear_fibre_refusal() {
    use martensite_tkmath::predicates::roots::bezier_pair::*;
    let controls = owned_radial_uv();
    assert_radial_uv_bits(&controls);
    let query = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
    let report = query.certify(radial_self_policy(), || false);
    assert!(report.complete().is_none());
    assert!(
        matches!(report.failure(), Some(BezierPairFailure::Operation(cause))
        if matches!(cause.original_cause(), BezierPairOperationFailure::NonLinearFiber))
    );
}
#[test]
fn uv_proof_version_binding_refuses_a_different_actual_engine() {
    let region = SurfaceRegionId(9);
    let requested = PlanarProofVersion::BernsteinV3;
    assert_eq!(
        selected_version(region, requested, requested).unwrap(),
        requested
    );
    let error = selected_version(region, requested, PlanarProofVersion::CarrierV2).unwrap_err();
    assert_eq!(error.region, Some(region));
}

fn owned_radial_uv() -> [[f64; 2]; 4] {
    let request = super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = storage::Work::new(request.proof.arithmetic);
    let selection = prepare::select(
        &ConeRun::from(&request),
        &inputs.originals,
        &mut work,
        &mut || false,
    )
    .unwrap();
    let prepared = &selection.prepared[0];
    let piece = &prepared.source.loops[0][0];
    assert_eq!(
        piece.span,
        CurveSpan {
            curve: SurfaceCurveId(1),
            level: 0,
            index: 0
        }
    );
    prepared.fits[&piece.span].uv.map(|[u, v]| [v, u])
}
fn assert_radial_uv_bits(controls: &[[f64; 2]; 4]) {
    assert_eq!(
        controls.map(|point| point.map(f64::to_bits)),
        [
            [0, 0],
            [4_599_676_419_421_066_576, 0],
            [4_604_180_019_048_437_080, 0],
            [4_607_182_418_800_017_408, 0],
        ]
    );
    // This is a strict derivative-hull proof of injectivity, not a sampled trace.
    assert!(controls.windows(2).all(|pair| pair[0][0] < pair[1][0]));
}
fn radial_self_policy() -> martensite_tkmath::predicates::roots::bezier_pair::BezierPairPolicy {
    use martensite_tkmath::predicates::roots::{RootIsolationOptions, bezier_pair::*};
    let mut roots = RootIsolationOptions::new(0.001);
    roots.max_bisections = 4034;
    BezierPairPolicy {
        version: 1,
        roots,
        signs: BezierPairSignOptions::derived(),
        requested_world_error: 1e-4,
    }
}
