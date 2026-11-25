// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const LEFT: BezierPairCurve<'static> = BezierPairCurve {
    controls: &[[0., 0.], [1., 2.]],
    weights: Some(&[0., 2.]),
    interval: [0., 1.],
};
const RIGHT: BezierPairCurve<'static> = BezierPairCurve {
    controls: &[[3., 0.], [4., 5.]],
    weights: None,
    interval: [0., 1.],
};
const SIGNED_ZERO: BezierPairCurve<'static> = BezierPairCurve {
    controls: &[[-0., 0.], [1., 2.]],
    weights: Some(&[-0., 2.]),
    interval: [-0., 1.],
};

#[test]
fn borrowed_pair_binding_matches_literal_order_and_signed_zero_equality() {
    let captured = CapturedBezierPair::capture_pair(LEFT, RIGHT).unwrap();
    assert_pair(&captured, LEFT, RIGHT, true);
    assert_pair(&captured, SIGNED_ZERO, RIGHT, true);
    assert_pair(&captured, RIGHT, LEFT, false);
    assert_self(&captured, LEFT, false);
}

#[test]
fn borrowed_pair_binding_keeps_missing_changed_and_malformed_weights_distinct() {
    let captured = CapturedBezierPair::capture_pair(LEFT, RIGHT).unwrap();
    for weights in [None, Some(&[1., 2.][..]), Some(&[0.][..])] {
        assert_pair(&captured, with_weights(LEFT, weights), RIGHT, false);
    }
    assert_pair(&captured, LEFT, with_weights(RIGHT, Some(&[])), false);
}

#[test]
fn borrowed_pair_binding_keeps_empty_shortened_and_retrimmed_curves_distinct() {
    let captured = CapturedBezierPair::capture_pair(LEFT, RIGHT).unwrap();
    let shorter = with_weights(with_controls(LEFT, &[[0., 0.]]), Some(&[0.]));
    let invalid = with_controls(LEFT, &[]);
    let retrimmed = BezierPairCurve {
        interval: [0., 0.5],
        ..LEFT
    };
    for curve in [shorter, invalid, retrimmed] {
        assert_pair(&captured, curve, RIGHT, false);
    }
}

#[test]
fn borrowed_self_binding_matches_literal_capture_and_retains_query_mode() {
    let captured = literal_self(LEFT).unwrap();
    for (curve, expected) in [(LEFT, true), (SIGNED_ZERO, true), (RIGHT, false)] {
        assert_self(&captured, curve, expected);
    }
    assert_self(&captured, with_weights(LEFT, None), false);
    assert_self(&captured, with_weights(LEFT, Some(&[])), false);
    assert_pair(&captured, LEFT, LEFT, false);
}

#[test]
fn borrowed_binding_keeps_nan_unequal_even_when_the_candidate_reuses_captured_storage() {
    let nonfinite = [
        with_controls(LEFT, &[[f64::NAN, 0.], [1., 2.]]),
        with_weights(LEFT, Some(&[f64::NAN, 2.])),
        BezierPairCurve {
            interval: [0., f64::NAN],
            ..LEFT
        },
    ];
    for curve in nonfinite {
        let pair = CapturedBezierPair::capture_pair(curve, RIGHT).unwrap();
        let [left, right] = pair.captured_curves();
        assert_pair(&pair, left, right, false);

        let self_query = literal_self(curve).unwrap();
        assert_self(&self_query, self_query.captured_curves()[0], false);
    }
}

fn with_weights<'a>(curve: BezierPairCurve<'a>, weights: Option<&'a [f64]>) -> BezierPairCurve<'a> {
    BezierPairCurve { weights, ..curve }
}

fn with_controls<'a>(curve: BezierPairCurve<'a>, controls: &'a [[f64; 2]]) -> BezierPairCurve<'a> {
    BezierPairCurve { controls, ..curve }
}

fn assert_pair(
    captured: &CapturedBezierPair,
    left: BezierPairCurve<'_>,
    right: BezierPairCurve<'_>,
    expected: bool,
) {
    let literal =
        CapturedBezierPair::capture_pair(left, right).is_ok_and(|fresh| fresh == *captured);
    assert_eq!(literal, expected);
    assert_eq!(captured.matches_pair(left, right), literal);
}

fn assert_self(captured: &CapturedBezierPair, curve: BezierPairCurve<'_>, expected: bool) {
    let literal = literal_self(curve).is_ok_and(|fresh| fresh == *captured);
    assert_eq!(literal, expected);
    assert_eq!(captured.matches_self_off_diagonal(curve), literal);
}

fn literal_self(
    curve: BezierPairCurve<'_>,
) -> Result<CapturedBezierPair, BezierPairCaptureFailure> {
    CapturedBezierPair::self_off_diagonal(curve.controls, curve.weights, curve.interval)
}
