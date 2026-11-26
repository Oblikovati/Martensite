// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    ArcDefinition, ArcFitTolerance, ArcId, CurveId, CurveRole, Edit, EntityId, MinimumLength,
    PointRole, Snapshot,
    bend::{BendFailure, BendIds, BendOptions, BendQuantity, BendRequest, BendResult},
    line_edit::{EndpointProtection, SupersededLineEndpoint},
};
use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2},
    num::NonZeroU64,
};

mod guards;

const CAPTURED: [[f64; 3]; 3] = [[30., 0., 0.], [60., 0., 0.], [60., 30., 0.]];

fn arc_id(value: u64) -> ArcId {
    ArcId::new(NonZeroU64::new(value).unwrap())
}

fn fixture(points: [[f64; 3]; 3]) -> Snapshot {
    fixture_with_corner_role(points, PointRole::Defining)
}

fn fixture_with_corner_role([first, corner, second]: [[f64; 3]; 3], role: PointRole) -> Snapshot {
    Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                seed(1, first, PointRole::Defining),
                seed(2, corner, role),
                seed(3, second, PointRole::Defining),
                line(4, 1, 2),
                line(5, 2, 3),
                seed(8, [17., -14., 2.], PointRole::Defining),
            ],
            || false,
        )
        .unwrap()
}

fn ids(base: u64) -> BendIds {
    BendIds {
        tangencies: [pid(base), pid(base + 1)],
        lines: [lid(base + 2), lid(base + 3)],
        arc: arc_id(base + 4),
        center: pid(base + 5),
    }
}

fn bend(radius: f64) -> BendRequest {
    BendRequest {
        corner: pid(2),
        lines: [lid(4), lid(5)],
        radius,
        ids: ids(51),
        options: BendOptions {
            max_position_error: 0.,
            max_radius_error: 0.,
            max_tangent_angle: f64::EPSILON,
            arc_fit: ArcFitTolerance::new(1e-12).unwrap(),
        },
    }
}

#[test]
fn captured_connected_corner_replaces_both_lines_and_matches_the_actual_quarter_arc() {
    // Controlled scenario bend.feature:27 models the connected 5 mm right-angle corner.
    let source = fixture(CAPTURED);
    let original = source.clone();
    let request = bend(5.);
    let result = source.bend_lines(request, || false).unwrap();
    assert_eq!(result.request, request);
    assert_eq!(
        result.history.corner,
        SupersededLineEndpoint::Retired(pid(2))
    );
    assert_eq!(result.history.legs.map(|leg| leg.source), request.lines);
    assert_eq!(
        result.history.legs.map(|leg| leg.line.endpoints),
        [[pid(1), pid(51)], [pid(52), pid(3)]]
    );
    let t = 1. / 6_f64;
    let complement = 5. / 6_f64;
    assert_eq!(
        result
            .history
            .legs
            .map(|leg| leg.retained_source_parameters),
        [
            [[0.; 2], [complement.next_down(), complement]],
            [[t, t.next_up()], [1.; 2]],
        ]
    );
    assert_native_quarter(&result, [[55., 0., 0.], [60., 5., 0.], [55., 5., 0.]]);
    assert!(result.sketch.point(pid(2)).is_none());
    assert!(result.sketch.line(lid(4)).is_none());
    assert!(result.sketch.line(lid(5)).is_none());
    assert_eq!(result.sketch.point(pid(8)), source.point(pid(8)));
    assert_eq!(source, original);
}

fn assert_native_quarter(result: &BendResult, coordinates: [[f64; 3]; 3]) {
    let ids = result.request.ids;
    let arc = result.sketch.arc(ids.arc).unwrap();
    assert!(matches!(arc.recipe().definition,
        ArcDefinition::Center { center, normal }
        if center == ids.center && *normal.coordinates() == [0., 0., 1.]));
    assert_eq!(arc.points(), coordinates.map(point));
    assert_eq!(arc.range(), [0., FRAC_PI_2]);
    assert_eq!(arc.radius(), 5.);
    assert_eq!(*arc.frame().direction().coordinates(), [0., 0., 1.]);
    let [first, second] = ids.lines.map(|id| result.sketch.line(id).unwrap());
    assert_eq!(first.carrier.direction(), arc.frame().y_direction());
    let outgoing = arc.frame().x_direction().reversed();
    assert_eq!(second.carrier.direction(), outgoing);
    let middle = point([55. + 5. / SQRT_2, 5. - 5. / SQRT_2, 0.]);
    assert!(arc.value(FRAC_PI_4).unwrap().distance(&middle) < 1e-12);
    assert_eq!(
        result.sketch.point(ids.center).unwrap().role,
        PointRole::CenterMarker
    );
    assert_eq!(result.evidence.stored_radius, 5.);
    assert_eq!(result.evidence.position_error_upper, 0.);
    assert_eq!(result.evidence.radius_error_upper, 0.);
    assert_eq!(result.evidence.tangent_angle_upper, [0., f64::EPSILON]);
    assert_eq!(
        result.evidence.nominal_terminal_error_upper,
        5. * f64::EPSILON
    );
}

#[test]
fn translated_yz_and_xz_turns_preserve_authored_direction_and_flip_branch_on_reversal() {
    for axes in [[1, 2, 0], [2, 0, 1]] {
        let place = |xyz: [f64; 3]| {
            let per_axis = [40., -70., 120.];
            std::array::from_fn(|i| xyz[axes[i]] + per_axis[i])
        };
        let source = fixture(CAPTURED.map(place));
        let head = source.bend_lines(bend(5.), || false).unwrap();
        let reversed = BendRequest {
            lines: [lid(5), lid(4)],
            ..bend(5.)
        };
        let tail = source.bend_lines(reversed, || false).unwrap();
        let [first, second] =
            [head, tail].map(|result| result.sketch.arc(result.request.ids.arc).unwrap());
        assert_eq!(first.range(), second.range());
        assert_eq!(
            first.frame().direction().reversed(),
            second.frame().direction()
        );
        assert_eq!(first.points()[0], second.points()[1]);
        assert_eq!(first.points()[1], second.points()[0]);
        assert_eq!(first.points()[2], second.points()[2]);
        assert_eq!(first.points()[2], point(place([55., 5., 0.])));
    }
}

#[test]
fn independent_limits_refuse_before_adoption_and_allow_a_common_rounded_radius() {
    let big = 9_007_199_254_740_992_f64;
    let source = fixture([[big + 8., big, 0.], [big, big, 0.], [big, big + 8., 0.]]);
    let mut request = bend(3.);
    assert!(
        matches!(source.bend_lines(request, || false), Err(BendFailure::Bound { quantity: BendQuantity::Placement, achieved, .. }) if achieved == SQRT_2)
    );
    request.options.max_position_error = SQRT_2;
    assert!(matches!(
        source.bend_lines(request, || false),
        Err(BendFailure::Bound {
            quantity: BendQuantity::Radius,
            achieved: 1.,
            ..
        })
    ));
    request.options.max_radius_error = 1.;
    request.options.max_tangent_angle = 0.;
    assert!(matches!(
        source.bend_lines(request, || false),
        Err(BendFailure::Bound {
            quantity: BendQuantity::TangentAngle,
            ..
        })
    ));
    request.options.max_tangent_angle = f64::EPSILON;
    request.options.arc_fit = ArcFitTolerance::new(f64::EPSILON).unwrap();
    assert!(matches!(
        source.bend_lines(request, || false),
        Err(BendFailure::Bound {
            quantity: BendQuantity::NominalTerminal,
            ..
        })
    ));
    request.options.arc_fit = bend(3.).options.arc_fit;
    let edited = source.bend_lines(request, || false).unwrap();
    assert_eq!(edited.evidence.stored_radius, 4.);
    assert_eq!(edited.evidence.position_error_upper, SQRT_2);
    assert_eq!(edited.evidence.radius_error_upper, 1.);
    assert_eq!(edited.sketch.arc(request.ids.arc).unwrap().radius(), 4.);
    assert!(source.arc(request.ids.arc).is_none());
}
