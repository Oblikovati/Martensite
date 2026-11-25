// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const IDENTITY: DecodedCoaxialLocation = DecodedCoaxialLocation {
    scale: 1.,
    translation: [0.; 3],
    matrix: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
};
const CONE: DecodedConeFrame = DecodedConeFrame {
    semi_angle: 0.125,
    radius: 10.,
    origin: [0.; 3],
    axes: IDENTITY.matrix,
    location: IDENTITY,
};
const OUTER: [[f64; 4]; 2] = [[11., 0., 0., 1.], [11., 0., 0., 2.]];
const INNER: [[f64; 4]; 2] = [[9., 0., 0., 1.], [9., 0., 0., 2.]];
fn member<'a>(id: u64, row: &'a [[f64; 4]]) -> DecodedRuledSide<'a> {
    DecodedRuledSide {
        id,
        location: IDENTITY,
        rows: [row, row],
        domain: [[0., 1.]; 2],
    }
}
fn cap(radius: f64) -> DecodedCapTrim {
    DecodedCapTrim {
        id: 20,
        frame: DecodedConeFrame { radius, ..CONE },
        axial_parameters: [0., 0.25],
    }
}
fn rims() -> [DecodedRimPlane; 2] {
    [
        DecodedRimPlane {
            elevation: -1.,
            normal: [0., 0., -1.],
        },
        DecodedRimPlane {
            elevation: 1.,
            normal: [0., 0., 1.],
        },
    ]
}
fn query<'a>(
    sides: &'a [DecodedRuledSide<'a>],
    caps: &'a [DecodedCapTrim],
    sense: SeparationSense,
) -> CarrierSeparationQuery<'a> {
    CarrierSeparationQuery {
        stock: CONE,
        rims: rims(),
        sides,
        caps,
        sense,
        options: SeparationOptions {
            max_scalar_steps: 100_000,
            max_dyadic_operations: 100_000,
        },
    }
}

#[test]
fn deterministic_cap_side_receipt_does_not_erase_source_fields() {
    let sides = [member(1, &OUTER), member(2, &OUTER)];
    let caps = [cap(11.)];
    let source = query(&sides, &caps, SeparationSense::Outside);
    let result = certify_separation(source, || false).unwrap();
    assert_eq!(result, certify_separation(source, || false).unwrap());
    assert!(result.matches_source(source));
    assert_eq!(
        result
            .sides()
            .iter()
            .map(SideSeparation::id)
            .collect::<Vec<_>>(),
        [1, 2]
    );
    assert_eq!(result.caps()[0].id(), 20);
    assert_eq!(result.work().scalar_steps(), 34);
    assert_eq!(result.work().exact_sides(), 0);
    assert!(result.work().dyadic_operations() > 0);
    let mut shifted = source;
    shifted.rims[0].elevation = source.rims[0].elevation.next_up();
    assert!(!result.matches_source(shifted));
    let mut changed = sides;
    changed[0].location.translation[0] = -0.;
    assert!(!result.matches_source(CarrierSeparationQuery {
        sides: &changed,
        ..source
    }));
    changed[0].id = 2;
    let error = certify_separation(
        CarrierSeparationQuery {
            sides: &changed,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert!(matches!(
        error.cause,
        SeparationCause::SourceOrder {
            previous: 2,
            next: 2
        }
    ));
}

#[test]
fn reports_condition_failures_at_the_first_source_patch() {
    let sides = [member(1, &OUTER)];
    let caps = [cap(11.)];
    let source = query(&sides, &caps, SeparationSense::Outside);
    let mut frame = sides;
    frame[0].location.matrix[1][2] = f64::EPSILON;
    let error = certify_separation(
        CarrierSeparationQuery {
            sides: &frame,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert_eq!(
        (error.patch, error.cause),
        (Patch::Side(1), SeparationCause::UnsupportedFrame)
    );
    let error = certify_separation(
        CarrierSeparationQuery {
            sense: SeparationSense::Inside,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert!(matches!(
        error.cause,
        SeparationCause::NormalSign {
            sense: SeparationSense::Inside,
            ..
        }
    ));
    let bad_cap = [cap(-1.)];
    let error = certify_separation(
        CarrierSeparationQuery {
            caps: &bad_cap,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert_eq!(
        (error.patch, error.cause),
        (
            Patch::Cap(20),
            SeparationCause::PositiveRadius { lower: -1. }
        )
    );
    let mut rims = source.rims;
    rims[0].elevation = 0.;
    let error =
        certify_separation(CarrierSeparationQuery { rims, ..source }, || false).unwrap_err();
    assert!(matches!(
        (error.patch, error.cause),
        (Patch::Side(1), SeparationCause::RimSeparation { .. })
    ));
}

#[test]
fn dyadic_replay_resolves_the_correct_one_ulp_sense_only() {
    for (radius, sense, cap_radius) in [
        (10_f64.next_up(), SeparationSense::Outside, 11.),
        (10_f64.next_down(), SeparationSense::Inside, 9.),
    ] {
        let row = [[radius, 0., 0., 1.]; 2];
        let sides = [member(1, &row)];
        let caps = [cap(cap_radius)];
        let result = certify_separation(query(&sides, &caps, sense), || false).unwrap();
        assert_eq!(result.work().exact_sides(), 1);
        assert!(
            result.sides()[0]
                .signed_normal_lower()
                .iter()
                .all(|v| *v > 0.)
        );
    }
    let row = [[10., 0., 0., 1.]; 2];
    let sides = [member(1, &row)];
    let caps = [cap(11.)];
    let fail =
        certify_separation(query(&sides, &caps, SeparationSense::Outside), || false).unwrap_err();
    assert!(matches!(fail.cause, SeparationCause::NormalSign { .. }));
    let crossing = [
        [10_f64.next_up(), 0., 0., 1.],
        [10_f64.next_down(), 0., 0., 1.],
    ];
    let sides = [member(1, &crossing)];
    let fail =
        certify_separation(query(&sides, &caps, SeparationSense::Outside), || false).unwrap_err();
    assert!(matches!(fail.cause, SeparationCause::NormalSign { .. }));
}

#[test]
fn cancellation_and_work_refuse_without_partial_receipts() {
    let sides = [member(1, &INNER), member(2, &INNER)];
    let caps = [cap(9.)];
    let source = query(&sides, &caps, SeparationSense::Inside);
    let allowed = certify_separation(source, || false).unwrap();
    let work_options = SeparationOptions {
        max_scalar_steps: 16,
        ..source.options
    };
    let error = certify_separation(
        CarrierSeparationQuery {
            options: work_options,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert_eq!(error.patch, Patch::Side(2));
    assert_eq!(
        error.cause,
        SeparationCause::ScalarWorkLimit {
            required: 32,
            limit: 16
        }
    );
    assert_eq!(error.work.scalar_steps(), 16);
    let work_options = SeparationOptions {
        max_dyadic_operations: 9,
        ..source.options
    };
    let error = certify_separation(
        CarrierSeparationQuery {
            options: work_options,
            ..source
        },
        || false,
    )
    .unwrap_err();
    assert!(matches!(
        error.cause,
        SeparationCause::Arithmetic(BoundedArithmeticFailure::WorkLimit {
            completed: 9,
            limit: 9
        })
    ));
    let mut calls = 0;
    let error = certify_separation(source, || {
        calls += 1;
        calls > allowed.work().dyadic_operations() + 2
    })
    .unwrap_err();
    assert_eq!(error.patch, Patch::Side(2));
    assert_eq!(error.work.scalar_steps(), 16);
    assert_eq!(
        error.cause,
        SeparationCause::Arithmetic(BoundedArithmeticFailure::Cancelled)
    );
}

#[test]
fn sealed_receipt_requires_the_exact_decoded_query_and_limits() {
    let original = [member(41, &OUTER)];
    let caps = [cap(11.)];
    let input = query(&original, &caps, SeparationSense::Outside);
    let receipt = certify_separation(input, || false).unwrap();
    let mut changed_row = OUTER;
    changed_row[0][3] = changed_row[0][3].next_up();
    let changed_sides = [member(41, &changed_row)];
    assert!(!receipt.matches_source(CarrierSeparationQuery {
        sides: &changed_sides,
        ..input
    }));
    let mut changed_caps = caps;
    changed_caps[0].axial_parameters[1] = changed_caps[0].axial_parameters[1].next_down();
    assert!(!receipt.matches_source(CarrierSeparationQuery {
        caps: &changed_caps,
        ..input
    }));
    let mut limits = input.options;
    limits.max_dyadic_operations -= 1;
    assert!(!receipt.matches_source(CarrierSeparationQuery {
        options: limits,
        ..input
    }));
}

#[test]
fn nontrivial_rotated_rows_retain_strict_cone_and_rim_gaps() {
    let first = [[11., 0., -0.25, 1.], [11.5, 0., 0., 2.]];
    let last = [[12., 0., 0., 2.], [12.5, 0., 0.25, 1.]];
    let location = DecodedCoaxialLocation {
        translation: [0., 0., 0.125],
        matrix: [[0., -1., 0.], [1., 0., 0.], [0., 0., 1.]],
        ..IDENTITY
    };
    let sides = [DecodedRuledSide {
        location,
        rows: [&first, &last],
        ..member(1, &first)
    }];
    let caps = [cap(11.)];
    let proof =
        certify_separation(query(&sides, &caps, SeparationSense::Outside), || false).unwrap();
    let side = &proof.sides()[0];
    assert!(side.signed_normal_lower().iter().all(|&gap| gap > 0.));
    assert!(side.zero_normal_upper() > 0.);
    assert!(side.rim_lower().iter().all(|&gap| gap >= 0.5));
    assert!(proof.caps()[0].rim_lower().iter().all(|&gap| gap > 0.));
}

#[test]
fn public_receipt_values_share_immutable_results() {
    fn require_shareable<T: Send + Sync>() {}
    require_shareable::<CarrierSeparationQuery<'static>>();
    require_shareable::<CarrierSeparationReceipt>();
    require_shareable::<SeparationError>();
}
