// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

const UV: [[f64; 2]; 3] = [[0., 3.], [0.06, 3.5], [0.125, 4.]];
const NEAR: [[f64; 3]; 3] = [[1., 0., 3.], [1., 0.06, 3.5], [0.99, 0.125, 4.]];
const FAR: [[f64; 3]; 3] = [[2., 0., 3.], [2.015, 0.12, 3.5], [1.98, 0.25, 4.1]];

fn cap(poles: &[[f64; 3]], radius: f64, limit: f64) -> CylindricalTrimQuery<'_> {
    CylindricalTrimQuery {
        curve: MetricBezier {
            poles,
            weights: None,
        },
        pcurve: &UV,
        radius,
        spatial_limit: limit,
        options: Default::default(),
    }
}

fn pair() -> [CylindricalTrimQuery<'static>; 2] {
    let near = cap(&NEAR, 1., 0.5);
    let mut far = cap(&FAR, 2., 0.75);
    far.curve.weights = Some(&[1., 0.875, 1.]);
    [near, far]
}

fn assert_receipt(actual: CertifiedCylindricalTrim, expected: CertifiedCylindricalTrim) {
    assert_eq!(
        actual.bounds.map(f64::to_bits),
        expected.bounds.map(f64::to_bits)
    );
    assert_eq!(actual.bits, expected.bits);
    assert_eq!(actual.limit.to_bits(), expected.limit.to_bits());
    assert_eq!(actual.options, expected.options);
    assert_eq!(actual.operations, expected.operations);
}

#[test]
fn reused_trace_preserves_literal_independent_rail_receipts() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CylindricalTrimWorkspace>();
    let [near, far] = pair();
    let mut slot = CylindricalTrimWorkspace::default();
    let first = slot.certify(near, || false).unwrap();
    let second = slot.certify(far, || false).unwrap();
    assert!(first.matches_source(near) && second.matches_source(far));
    assert!(!first.matches_source(far) && !second.matches_source(near));
    assert_ne!(
        first.spatial_residual_bound(),
        second.spatial_residual_bound()
    );
    assert_receipt(first, certify_cylindrical_trim(near, || false).unwrap());
    assert_receipt(second, certify_cylindrical_trim(far, || false).unwrap());
}

#[test]
fn independent_rail_captures_produce_identical_uv_and_trig_coefficients() {
    let independent_capture = |query| {
        let mut workspace = CylindricalTrimWorkspace::default();
        workspace.certify(query, || false).unwrap();
        workspace.last.unwrap().capture
    };
    let [near, far] = pair().map(independent_capture);
    assert_eq!(near.height, far.height);
    assert_eq!(near.trig, far.trig);
    assert_eq!(near.uv_steps, far.uv_steps);
    assert_eq!(near.trig_steps, far.trig_steps);
}

#[test]
fn changed_uv_bits_degree_or_precision_require_a_fresh_trace() {
    let [near, far] = pair();
    let mut changed = UV;
    changed[0][0] = -0.;
    let linear = [UV[0], UV[2]];
    let mut lower_precision = far.options;
    lower_precision.fractional_bits = 96;
    for input in [
        CylindricalTrimQuery {
            pcurve: &changed,
            ..far
        },
        CylindricalTrimQuery {
            pcurve: &linear,
            ..far
        },
        CylindricalTrimQuery {
            options: lower_precision,
            ..far
        },
    ] {
        let mut slot = CylindricalTrimWorkspace::default();
        slot.certify(near, || false).unwrap();
        assert!(slot.matching(far).is_some());
        assert!(slot.matching(input).is_none());
        let expected = certify_cylindrical_trim(input, || false).unwrap();
        assert_receipt(slot.certify(input, || false).unwrap(), expected);
        assert!(slot.matching(input).is_some());
    }
}

#[test]
fn shared_capture_preserves_work_limits_and_poll_positions() {
    let [near, far] = pair();
    let mut slot = CylindricalTrimWorkspace::default();
    slot.certify(near, || false).unwrap();
    let reusable = slot.matching(far).unwrap();
    let start_trig = reusable.uv_steps + 7 * far.curve.poles.len() as u32;
    let end_trig = start_trig + reusable.trig_steps;
    let total = certify_cylindrical_trim(far, || false)
        .unwrap()
        .operations();
    for cutoff in [
        0,
        reusable.uv_steps,
        start_trig - 1,
        start_trig,
        end_trig - 1,
        end_trig,
        total - 1,
    ] {
        let mut exhausted = far;
        exhausted.options.max_operations = cutoff;
        let direct = certify_cylindrical_trim(exhausted, || false).unwrap_err();
        assert_eq!(slot.certify(exhausted, || false).unwrap_err(), direct);
        assert_cancel_at(&mut slot, far, cutoff);
        assert_cancel_at(&mut slot, exhausted, cutoff);
    }
}

fn assert_cancel_at(
    slot: &mut CylindricalTrimWorkspace,
    query: CylindricalTrimQuery<'_>,
    cutoff: u32,
) {
    let mut before = 0;
    let mut after = 0;
    let direct = certify_cylindrical_trim(query, || poll_at(&mut before, cutoff)).unwrap_err();
    let reused = slot
        .certify(query, || poll_at(&mut after, cutoff))
        .unwrap_err();
    assert_eq!(reused, direct);
    assert_eq!(before, cutoff + 1);
    assert_eq!(after, before);
}

fn poll_at(calls: &mut u32, cutoff: u32) -> bool {
    let polled = *calls;
    *calls += 1;
    polled == cutoff
}
