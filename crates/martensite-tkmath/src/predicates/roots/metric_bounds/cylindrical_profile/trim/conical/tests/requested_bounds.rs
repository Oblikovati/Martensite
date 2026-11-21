// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::BoundedArithmeticFailure;
use std::sync::atomic::{AtomicU32, Ordering};
#[test]
fn final_aggregate_refuses_three_individually_sub_limit_components() {
    let mut input = query(&[[2.25, 0.25, 3.25]; 2]);
    input.spatial_limit = 0.5;
    let cause = certify_conical_trim(input, || false).unwrap_err().cause;
    assert!(matches!(
        cause,
        CylindricalProfileFailure::Uncertified {
            bound: 0.75,
            requested: 0.5
        }
    ));
}
#[test]
fn conditioned_receipt_retains_allowance_separately_from_geometry_identity() {
    let mut input = query(&[[2.125, 0., 3.]; 2]);
    input.spatial_limit = 1.;
    let proof = certify_conical_trim(input, || false).unwrap();
    assert_eq!(proof.residual_bounds(), [0.125, 0., 0.125]);
    assert!(proof.matches_source(input));
    assert_eq!(proof.requested_spatial_limit(), 1.);
    assert_eq!(proof.requested_options(), input.options);
    // Singleton identity names geometry; the pair receipt separately binds generation policy.
    input.spatial_limit = 0.5;
    assert!(proof.matches_source(input));
    assert_eq!(proof.requested_spatial_limit(), 1.);
}
#[test]
fn conditioned_trace_retains_the_original_work_refusal() {
    let mut input = query(&[[2.125, 0., 3.]; 2]);
    input.spatial_limit = 1.;
    let operations = certify_conical_trim(input, || false).unwrap().operations();
    input.options.max_operations = operations - 1;
    let cause = certify_conical_trim(input, || false).unwrap_err().cause;
    let CylindricalProfileFailure::Arithmetic(cause) = cause else {
        panic!("expected bounded-work refusal");
    };
    assert!(
        matches!(*cause, BoundedArithmeticFailure::WorkLimit { completed, limit } if completed == operations - 1 && limit == completed)
    );
}
#[test]
fn conditioned_trace_cannot_publish_when_cancelled_at_its_last_required_poll() {
    let mut input = query(&[[2.125, 0., 3.]; 2]);
    input.spatial_limit = 1.;
    let polls = AtomicU32::new(0);
    certify_conical_trim(input, || {
        polls.fetch_add(1, Ordering::Relaxed);
        false
    })
    .unwrap();
    let total = polls.load(Ordering::Relaxed);
    polls.store(0, Ordering::Relaxed);
    let cause = certify_conical_trim(input, || polls.fetch_add(1, Ordering::Relaxed) + 1 == total)
        .unwrap_err()
        .cause;
    let CylindricalProfileFailure::Arithmetic(cause) = cause else {
        panic!("expected original cancellation");
    };
    assert!(matches!(*cause, BoundedArithmeticFailure::Cancelled));
}
