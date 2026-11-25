// SPDX-License-Identifier: GPL-2.0-only

//! Integration coverage for preview.

#![cfg(feature = "sketch3d-preview")]

#[path = "preview/rebind.rs"]
mod rebind;

use martensite_tksolve::{
    ad::DualNumber as Dual,
    nonlinear::SolveOptions,
    preview::{Block, Domain, Incomplete, Limit, Limits, Prepared, PreviewError, Update},
    residual::{ResidualError, ResidualSource, ResidualTolerance},
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering::Relaxed},
};

const LIMITS: Limits = Limits {
    coordinates: 3,
    blocks: 1,
    equations: 2,
    block_inputs: 3,
    targets: 1,
    gram_terms: 256,
    factor_updates: 32,
    work: 4096,
};

struct FixedRadius;

impl ResidualSource for FixedRadius {
    fn tolerance_kind(&self) -> ResidualTolerance {
        ResidualTolerance::Unitless
    }

    fn evaluate(&self, point: &[Dual]) -> Result<Vec<Dual>, ResidualError> {
        let radius = point[0].hypot(&point[1])?.hypot(&point[2])?;
        Ok(vec![radius.shifted(-5.)?])
    }
}

#[test]
fn three_target_coordinates_follow_two_independent_tangent_directions() {
    let inputs = [3., 4., 0.];
    let target = [4., 3., 0.];
    let block = Block {
        source: Arc::new(FixedRadius),
        variables: vec![0, 1, 2],
        equations: 1,
    };
    let program = Prepared::new(
        vec![block],
        vec![Domain::Length(1.); 3],
        &[],
        1.,
        SolveOptions::default(),
        Limits {
            targets: 3,
            ..LIMITS
        },
    )
    .unwrap();
    let result = program
        .update(
            &inputs,
            &[(0, target[0]), (1, target[1]), (2, target[2])],
            || false,
        )
        .unwrap();
    assert_eq!((result.rank, result.dof), (1, 2));
    for (found, target) in result.values.into_iter().zip(target) {
        assert!((found - target).abs() < 1e-10);
    }
    assert!(result.hard_error <= 1e-10);
}

struct Coupled {
    evaluations: Arc<AtomicUsize>,
    redundant: bool,
    drift: bool,
}

impl ResidualSource for Coupled {
    fn tolerance_kind(&self) -> ResidualTolerance {
        ResidualTolerance::Unitless
    }

    fn evaluate(&self, v: &[Dual]) -> Result<Vec<Dual>, ResidualError> {
        let count = self.evaluations.fetch_add(1, Relaxed);
        let base = v[0].plus(&v[1])?;
        let first = if count > 0 && self.drift {
            base.shifted(0.25)?
        } else {
            base
        };
        let second = if self.redundant {
            first.clone()
        } else {
            v[1].plus(&v[2])?
        };
        Ok(vec![first, second])
    }
}

fn prepare(
    limits: Limits,
    evaluations: &Arc<AtomicUsize>,
    redundant: bool,
    drift: bool,
) -> Result<Prepared, PreviewError> {
    let block = Block {
        source: Arc::new(Coupled {
            evaluations: evaluations.clone(),
            redundant,
            drift,
        }),
        variables: vec![0, 1, 2],
        equations: 2,
    };
    Prepared::new(
        vec![block],
        vec![Domain::Length(1.); 3],
        &[],
        1.,
        SolveOptions::default(),
        limits,
    )
}

#[test]
fn prepared_preview_refuses_resource_overruns_without_partial_success() {
    let polls = Arc::new(AtomicUsize::new(0));
    for (limits, quantity) in [
        (
            Limits {
                coordinates: 2,
                ..LIMITS
            },
            Limit::Coordinates,
        ),
        (
            Limits {
                blocks: 0,
                ..LIMITS
            },
            Limit::Blocks,
        ),
        (
            Limits {
                equations: 1,
                ..LIMITS
            },
            Limit::Equations,
        ),
        (
            Limits {
                block_inputs: 2,
                ..LIMITS
            },
            Limit::BlockInputs,
        ),
    ] {
        let Err(error) = prepare(limits, &polls, false, false) else {
            panic!("admitted {quantity:?}");
        };
        assert_limit(error, quantity);
        assert_eq!(polls.load(Relaxed), 0);
    }
    for (limits, quantity) in [
        (
            Limits {
                targets: 0,
                ..LIMITS
            },
            Limit::Targets,
        ),
        (
            Limits {
                gram_terms: 0,
                ..LIMITS
            },
            Limit::GramTerms,
        ),
        (
            Limits {
                factor_updates: 0,
                ..LIMITS
            },
            Limit::FactorUpdates,
        ),
        (Limits { work: 0, ..LIMITS }, Limit::Work),
    ] {
        let program = prepare(limits, &polls, false, false).unwrap();
        let prior = [0.; 3];
        assert_limit(
            program.update(&prior, &[(0, 1.)], || false).unwrap_err(),
            quantity,
        );
        assert_eq!(prior, [0.; 3]);
    }
}

fn assert_limit(error: PreviewError, requested_quantity: Limit) {
    let PreviewError::Incomplete(Incomplete::Limit {
        quantity,
        limit,
        requested,
    }) = error
    else {
        panic!("expected {requested_quantity:?}, found {error:?}");
    };
    assert_eq!(quantity, requested_quantity);
    assert!(requested > limit);
}

#[test]
fn bounded_update_keeps_rank_recovery_in_the_explicit_diagnostic_path() {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let program = prepare(LIMITS, &evaluations, true, false).unwrap();
    assert!(matches!(
        program.update(&[0.; 3], &[], || false),
        Err(PreviewError::Incomplete(Incomplete::RankRecovery {
            rows: 2,
            columns: 3
        }))
    ));
    assert_eq!(program.inspect_rank(&[0.; 3]).unwrap(), (1, 2));
}

#[test]
fn sparse_update_preserves_hard_rows_and_exposes_its_actual_rank_and_charge() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Prepared>();
    assert_send_sync::<Update>();

    let evaluations = Arc::new(AtomicUsize::new(0));
    let program = prepare(LIMITS, &evaluations, false, false).unwrap();
    let initial = [0.; 3];
    let result = program.update(&initial, &[(0, 1.)], || false).unwrap();
    assert_eq!((result.rank, result.dof), (2, 1));
    assert_eq!(program.inspect_rank(&result.values).unwrap(), (2, 1));
    assert!(result.hard_error <= 1e-10);
    assert!(result.work.factor_updates > 0);
    assert!(result.work.work > result.work.gram_terms);
    for (found, expected) in result.values.into_iter().zip([1., -1., 1.]) {
        assert!((found - expected).abs() < 1e-10);
    }
}

#[test]
fn a_nonrepeatable_final_evaluator_cannot_publish_an_infeasible_snapshot() {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let program = prepare(LIMITS, &evaluations, false, true).unwrap();
    let error = program.update(&[0.; 3], &[], || false).unwrap_err();
    assert_eq!(evaluations.load(Relaxed), 2);
    let PreviewError::Incomplete(Incomplete::FinalHardResidual {
        observed,
        tolerance,
    }) = error
    else {
        panic!("expected final infeasibility, found {error:?}");
    };
    assert_eq!(observed, 0.25);
    assert_eq!(tolerance, SolveOptions::default().relative_tolerance);
}

#[test]
fn a_late_cancellation_refuses_publication_after_the_last_numerical_stage() {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let program = prepare(LIMITS, &evaluations, false, false).unwrap();
    let mut complete_polls = 0;
    program
        .update(&[0.; 3], &[], || {
            complete_polls += 1;
            false
        })
        .unwrap();
    let mut cancelled_polls = 0;
    evaluations.store(0, Relaxed);
    let result = program.update(&[0.; 3], &[], || {
        cancelled_polls += 1;
        cancelled_polls == complete_polls
    });
    assert!(matches!(
        result,
        Err(PreviewError::Incomplete(Incomplete::Cancelled))
    ));
    assert_eq!(cancelled_polls, complete_polls);
    assert_eq!(evaluations.load(Relaxed), 2);
}
