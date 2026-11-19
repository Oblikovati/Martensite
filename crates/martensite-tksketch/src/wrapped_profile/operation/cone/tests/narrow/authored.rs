//! Original O. and dot sources are measured in isolated conditional authored-volume leaves.
use super::*;
#[test]
fn eight_original_o_curves_conditioned_volume_fits_actual_remaining_budget() {
    check_original_region(SurfaceRegionId(1), 8);
}
#[test]
fn four_original_dot_lines_conditioned_volume_has_isolated_cost_and_width() {
    check_original_region(SurfaceRegionId(2), 4);
}
fn check_original_region(region: SurfaceRegionId, curves: usize) {
    let request = fixture::request(NormalSense::Outward, fixture::Quadrant::PositiveX);
    let inputs = input::capture(&request, &mut || false).unwrap();
    let original = inputs
        .originals
        .iter()
        .find(|source| source.region == region)
        .unwrap();
    let loops = original_loops(original);
    complete_original_leaf(&request, original, &loops, curves);
}
fn original_loops(original: &DevelopedRegionRequest) -> Vec<Vec<&[[f64; 2]]>> {
    original
        .loops
        .iter()
        .map(|contour| contour.iter().map(|piece| piece.poles.as_slice()).collect())
        .collect()
}
fn complete_original_leaf(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    loops: &[Vec<&[[f64; 2]]>],
    curves: usize,
) {
    let contours: Vec<_> = loops
        .iter()
        .map(|spans| ConicalAuthoredContour {
            spans,
            reverse: false,
        })
        .collect();
    assert_original_curve_count(&contours, curves);
    let query = leaf_query(request, original, &contours);
    let proof = execute_leaf(query);
    check_policy(&proof, query);
    check_independent_volume(&proof, original.region);
    check_source_mutation(&proof, query);
    report_charged(&proof, original.region, curves);
}
fn assert_original_curve_count(contours: &[ConicalAuthoredContour<'_>], expected: usize) {
    assert_eq!(
        contours
            .iter()
            .map(|contour| contour.spans.len())
            .sum::<usize>(),
        expected
    );
}
fn execute_leaf(query: ConicalAuthoredFluxQuery<'_>) -> ConicalAuthoredFlux {
    certify_conical_authored_flux_with_method(
        query,
        ConicalAuthoredFluxMethod::ConditionedSimpsonV2,
        || false,
    )
    .unwrap_or_else(|cause| panic!("isolated original authored leaf: {cause:?}"))
}

fn leaf_query<'a>(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    contours: &'a [ConicalAuthoredContour<'a>],
) -> ConicalAuthoredFluxQuery<'a> {
    let options = request.proof.authored_volume;
    ConicalAuthoredFluxQuery {
        contours,
        semi_angle: request.support.semi_angle,
        depths: original.depths,
        outward: true,
        level: options.subdivision_level,
        clearance_level: options.clearance_level,
        volume_width: options.enclosure_width,
        // No native body is constructed: this placeholder supplies no represented-flux acceptance claim.
        flux: [0., 0.],
        arithmetic: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 36_558,
        },
    }
}
fn check_policy(proof: &ConicalAuthoredFlux, query: ConicalAuthoredFluxQuery<'_>) {
    assert_options(query);
    assert_eq!(
        proof.method(),
        ConicalAuthoredFluxMethod::ConditionedSimpsonV2
    );
    assert!(proof.matches_with_method(query, ConicalAuthoredFluxMethod::ConditionedSimpsonV2));
    assert!(!proof.matches(query));
    let mut altered = query;
    altered.arithmetic.max_operations -= 1;
    assert!(!proof.matches_with_method(altered, proof.method()));
}
fn volume_interval(proof: &ConicalAuthoredFlux) -> [f64; 2] {
    let bounds = proof.volume();
    [
        bounds[0].binary64_bounds().unwrap()[0],
        bounds[1].binary64_bounds().unwrap()[1],
    ]
}
fn check_independent_volume(proof: &ConicalAuthoredFlux, region: SurfaceRegionId) {
    let measured = volume_interval(proof);
    let independent = super::volume::bounds(region, NormalSense::Outward);
    assert!(measured[0] >= independent[0] && measured[1] <= independent[1]);
    assert!((measured[1] - measured[0]).next_up() <= 1e-7);
}
fn check_source_mutation(proof: &ConicalAuthoredFlux, query: ConicalAuthoredFluxQuery<'_>) {
    let mut controls = query.contours[0].spans[0].to_vec();
    controls[1][0] = controls[1][0].next_up();
    let mut spans = query.contours[0].spans.to_vec();
    spans[0] = &controls;
    let mut contours = query.contours.to_vec();
    contours[0].spans = &spans;
    let mut altered = query;
    altered.contours = &contours;
    assert!(!proof.matches_with_method(altered, proof.method()));
}
fn report_charged(proof: &ConicalAuthoredFlux, region: SurfaceRegionId, curves: usize) {
    let mut work = storage::Work::new(ConeClearanceOptions {
        fractional_bits: 128,
        max_operations: 36_558,
    });
    work.charge(region, None, proof.operations()).unwrap();
    assert_eq!(work.consumed, proof.operations());
    assert!(work.consumed > 0 && work.consumed <= 36_558);
    let [lower, upper] = volume_interval(proof);
    eprintln!(
        r#"{{"event":"isolated_original_authored_leaf","region":{},"curves":{curves},"isolated_leaf_allowance":36558,"method":"{:?}","operations":{},"isolated_remaining":{},"lower":{lower},"upper":{upper},"width":{}}}"#,
        region.0,
        proof.method(),
        work.consumed,
        work.remaining().max_operations,
        (upper - lower).next_up()
    );
}

fn assert_options(query: ConicalAuthoredFluxQuery<'_>) {
    assert_eq!(
        (query.level, query.clearance_level, query.volume_width),
        (6, 0, 1e-7)
    );
    assert_eq!(
        (
            query.arithmetic.fractional_bits,
            query.arithmetic.max_operations
        ),
        (128, 36_558)
    );
}
