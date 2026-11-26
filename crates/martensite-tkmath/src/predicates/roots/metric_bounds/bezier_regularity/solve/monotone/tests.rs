// SPDX-License-Identifier: GPL-2.0-only
use super::super::{
    BezierRegularityOptions, BezierRegularitySource, RootIsolationError, RootIsolationOptions,
    admit, classify_generic,
};
use super::*;

fn query(points: &[[f64; 3]; 4]) -> BezierRegularityQuery<'_> {
    BezierRegularityQuery::new(
        7,
        points,
        BezierRegularityOptions::v1(RootIsolationOptions::new(1e-8)),
    )
}

#[test]
fn regularity_reports_match_complete_root_isolation_including_exact_polynomial_degree() {
    for (y, degree) in [
        ([0., 1., 2., 3.], 0),
        ([0., 0., 1., 3.], 2),
        ([0., 4., 0., 1.], 4),
    ] {
        let points = [0, 1, 2, 3].map(|index| [-(index as f64), y[index], 2.]);
        compare_generic(query(&points), degree);
    }
}

#[test]
fn complete_report_is_unchanged_with_boundary_cancellation_and_nonsampled_coordinates() {
    for case in 0..32 {
        let offsets =
            [0, 1, 2, 3].map(|index| f64::from((case * 7 + index * (case + 3)) % 37) / 19.);
        let points = [0, 1, 2, 3].map(|index| [f64::from(index), offsets[index as usize], 1.]);
        let q = query(&points);
        let (bounds, width) = admit(q).unwrap();
        let mut work = 0;
        let expected = classify_generic(q, bounds, width, &mut || false, &mut work).unwrap();
        let candidate = classify(q, &mut || false).unwrap().unwrap();
        assert_eq!(
            candidate.report(BezierRegularitySource::retain(q), 0),
            expected.report(BezierRegularitySource::retain(q), work)
        );
    }
    let points = [[0., 0., 0.], [1., 0., 0.], [2., 1., 0.], [3., 1., 0.]];
    let mut polls = 0;
    let refused = classify(query(&points), &mut || {
        polls += 1;
        polls > 1
    });
    assert!(matches!(
        refused,
        Err(BezierRegularityCause::Coefficients(ResidualError::Roots(
            RootIsolationError::Cancelled
        )))
    ));
}

#[test]
fn weighting_placement_degree_limit_and_loss_of_sign_use_the_general_proof() {
    let points = [[0., 0., 0.], [1., 0., 0.], [2., 1., 0.], [3., 1., 0.]];
    let local = query(&points);
    let mut limited = local;
    limited.options.roots.max_degree = 3;
    let mut translated = local;
    translated.placement.origin[0] = 2.;
    let weighted = BezierRegularityQuery {
        weights: Some(&[1.; 4]),
        ..local
    };
    let nonmonotone = [[0., 0., 0.], [1., 0., 0.], [0., 0., 0.], [3., 1., 0.]];
    let trimmed = BezierRegularityQuery {
        interval: [0., 0.5],
        ..local
    };
    for refused in [limited, translated, weighted, query(&nonmonotone), trimmed] {
        assert!(classify(refused, &mut || false).unwrap().is_none());
    }
}

fn compare_generic(query: BezierRegularityQuery<'_>, degree: usize) {
    let (bounds, width) = admit(query).unwrap();
    let mut work = 0;
    let expected = classify_generic(query, bounds, width, &mut || false, &mut work).unwrap();
    let candidate = classify(query, &mut || false).unwrap().unwrap();
    assert_eq!(candidate.speed_degree, Some(degree));
    assert_eq!(
        candidate.report(BezierRegularitySource::retain(query), 0),
        expected.report(BezierRegularitySource::retain(query), work)
    );
}
