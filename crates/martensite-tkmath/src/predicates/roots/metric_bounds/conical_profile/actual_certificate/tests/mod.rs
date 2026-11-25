// SPDX-License-Identifier: GPL-2.0-only
//! Public authority, first-cause, remaining-work and independent numeric controls.
use super::super::boundary_chart::tests::{domain, options, poles, query};
use super::*;
mod frame;
mod gamma;
mod thread_traits;
fn fixture(body: impl FnOnce(ConicalDirectedGammaQuery<'_>)) {
    let controls = poles();
    let source = query(&controls);
    let domain = domain(0.125);
    let parent = certify_conical_boundary_in_chart(source, &domain, options(), || false).unwrap();
    let caps = carriers(&controls);
    let spans = [ConicalDirectedGammaSpan {
        id: ConicalActualSideId(9),
        member: ConicalActualMemberId(1),
        previous: source,
        parent: &parent,
        domain: &domain,
        caps: [caps[0].id, caps[1].id],
    }];
    body(ConicalDirectedGammaQuery {
        spans: &spans,
        caps: &caps,
        refinement: ConicalPrimitiveRefinement::UnsplitV1,
        options: options(),
    });
}
#[test]
fn certificate_steps_share_the_actual_limit_and_cancel_before_charge() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 1,
        },
        &mut never,
    );
    arithmetic.certificate_step().unwrap();
    assert_eq!(arithmetic.operations(), 1);
    assert!(matches!(
        arithmetic.certificate_step(),
        Err(BoundedArithmeticFailure::WorkLimit {
            completed: 1,
            limit: 1
        })
    ));
    cancelled_step();
}

#[test]
fn typed_public_receipts_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<ConicalDirectedGammaEvidence>();
    check::<ConicalActualPatchCertificate>();
    check::<ConicalActualPatchError>();
    check::<ConicalActualPatchFailure>();
    check::<ConicalActualPatchCoverage>();
    check::<ConicalActualCapSnapshot>();
}

fn carriers(controls: &[[[f64; 3]; 2]; 2]) -> [ConicalActualCapCarrier; 2] {
    std::array::from_fn(|row| {
        let point = controls[row][0];
        ConicalActualCapCarrier {
            id: ConicalActualCapId(10 + row as u64),
            member: ConicalActualMemberId(1),
            frame: [[0., 0., point[2]], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            radius: point[0],
            axial_origin: point[2],
            semi_angle: 0.125,
        }
    })
}

fn cancelled_step() {
    let mut cancelled = || true;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 1,
        },
        &mut cancelled,
    );
    assert!(matches!(
        arithmetic.certificate_step(),
        Err(BoundedArithmeticFailure::Cancelled)
    ));
    assert_eq!(arithmetic.operations(), 0);
}

#[test]
fn negative_orientation_retains_the_actual_signed_value() {
    assert!(matches!(
        numeric::orientation(-2, "side face orientation"),
        Err(ConicalActualPatchFailure::Value {
            actual: -2.,
            expected: "orientation -1 or +1",
            ..
        })
    ));
}
