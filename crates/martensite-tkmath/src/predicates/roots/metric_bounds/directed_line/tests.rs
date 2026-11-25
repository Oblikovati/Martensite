// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::cmp::Ordering;

const X: [[f64; 3]; 2] = [[0.; 3], [1., 0., 0.]];

fn transverse(first: [[f64; 3]; 2], second: [[f64; 3]; 2]) -> Box<DirectedLineHit> {
    let DirectedLineContact::Transverse(hit) = directed_line_contact(first, second).unwrap() else {
        panic!("expected an exact transverse hit");
    };
    hit
}

#[test]
fn endpoints_and_reversed_authored_parameters_stay_distinct() {
    let boundary = [[1., 0., 0.], [1., 1., 0.]];
    let hit = transverse(X, boundary);
    assert_eq!(hit.parameters()[0].location(), LineParameterLocation::End);
    assert_eq!(hit.parameters()[1].location(), LineParameterLocation::Start);
    assert_eq!(hit.placement().unwrap().parameters, [[1.; 2], [0.; 2]]);
    let result = transverse([X[1], X[0]], [boundary[1], boundary[0]]);
    assert_eq!(result.placement().unwrap().parameters, [[0.; 2], [1.; 2]]);
    assert_eq!(result.placement().unwrap().position, [1., 0., 0.]);
}

#[test]
fn exact_order_can_distinguish_hits_with_identical_rounded_positions() {
    let next = 2_f64.next_up();
    let near = transverse(X, [[2., -1., 0.], [next, 7., 0.]]);
    let far = transverse(X, [[2., -1., 0.], [next, 3., 0.]]);
    let first = |hit: &DirectedLineHit| hit.parameters()[0].clone();
    assert_eq!(first(&near).location(), LineParameterLocation::AfterEnd);
    assert_eq!(first(&near).compare_authored(2.), Ok(Ordering::Greater));
    assert_eq!(first(&near).cmp(&first(&far)), Ordering::Less);
    for hit in [near, far] {
        let evidence = hit.placement().unwrap();
        assert_eq!(evidence.position, [2., 0., 0.]);
        assert_eq!(evidence.parameters[0], [2., next]);
    }
}

#[test]
fn wide_source_can_cross_without_binary64_displacement_or_dyadic_parameter() {
    let first = [[-f64::MAX, 0., 0.], [f64::MAX, 0., 0.]];
    let hit = transverse(first, [[0., -1., 0.], [0., 2., 0.]]);
    let evidence = hit.placement().unwrap();
    assert_eq!(evidence.position, [0.; 3]);
    assert_eq!(evidence.parameters[0], [0.5; 2]);
    assert_eq!(evidence.parameters[1], [1. / 3., (1_f64 / 3.).next_up()]);
    assert_eq!(evidence.error_upper, 0.);
}

#[test]
fn exact_world_error_includes_the_half_ulp_lost_at_midpoint() {
    let large = 9_007_199_254_740_992.;
    let adjacent = large + 2.;
    let hit = transverse(
        [[large, 0., 0.], [adjacent, 0., 0.]],
        [[large, -1., 0.], [adjacent, 1., 0.]],
    );
    let evidence = hit.placement().unwrap();
    assert_eq!(evidence.parameters, [[0.5; 2]; 2]);
    assert_eq!(evidence.position, [large, 0., 0.]);
    assert_eq!(evidence.coordinates[0], [large, adjacent]);
    assert_eq!(evidence.error_upper, 1.);
}

#[test]
fn skew_parallel_collinear_and_invalid_sources_have_distinct_results() {
    let tiny = f64::from_bits(1);
    assert_eq!(
        directed_line_contact(X, [[0.5, -1., tiny], [0.5, 1., tiny]]),
        Ok(DirectedLineContact::Skew)
    );
    assert_eq!(
        directed_line_contact(X, [[0., 1., 0.], [1., 1., 0.]]),
        Ok(DirectedLineContact::Parallel)
    );
    let result = directed_line_contact(X, [[2., 0., 0.], [-1., 0., 0.]]).unwrap();
    let DirectedLineContact::Collinear([hi, lo]) = result else {
        panic!("expected collinearity");
    };
    assert_eq!(lo.location(), LineParameterLocation::BeforeStart);
    assert_eq!(hi.location(), LineParameterLocation::AfterEnd);
    assert_eq!(lo.compare_authored(-1.), Ok(Ordering::Equal));
    assert_eq!(hi.compare_authored(2.), Ok(Ordering::Equal));
    assert_eq!(
        directed_line_contact(X, [[0.; 3]; 2]),
        Err(DirectedLineIncidenceFailure::DegenerateSource(1))
    );
}

#[test]
fn directed_parameters_and_placement_are_independent_of_projection_axis() {
    let first = [[-3., 2., -7.], [5., 2., 9.]];
    let second = [[-1., 2., 0.], [-1., 2., -4.]];
    let exact_position = [-1., 2., -3.];
    for axes in [[0, 1, 2], [1, 2, 0], [2, 0, 1]] {
        let permute = |point: [f64; 3]| axes.map(|axis| point[axis]);
        let evidence = transverse(first.map(permute), second.map(permute))
            .placement()
            .unwrap();
        assert_eq!(evidence.position, permute(exact_position));
        assert_eq!(evidence.parameters, [[0.25; 2], [0.75; 2]]);
        assert_eq!(evidence.error_upper, 0.);
    }
}

#[test]
fn exact_forward_parameter_can_exceed_its_finite_output_enclosure() {
    let tiny = f64::from_bits(1);
    let source = [[0.; 3], [tiny, 0., 0.]];
    let hit = transverse(source, [[1., -1., 0.], [1., 1., 0.]]);
    assert_eq!(
        hit.parameters()[0].location(),
        LineParameterLocation::AfterEnd
    );
    assert_eq!(
        hit.parameters()[0].compare_authored(f64::MAX),
        Ok(Ordering::Greater)
    );
    assert_eq!(
        hit.placement(),
        Err(DirectedLineIncidenceFailure::Unrepresentable(
            LineEvidenceQuantity::Parameter(0)
        ))
    );
}
