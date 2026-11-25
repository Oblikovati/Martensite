// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tksolve::{
    nonlinear::SolveError,
    preview::{SourceReplacement, Work},
    residual::EvaluationError,
};

#[derive(Clone, Copy)]
struct AxisRow {
    axis: usize,
    value: f64,
    rows: usize,
    tolerance: ResidualTolerance,
}

impl ResidualSource for AxisRow {
    fn tolerance_kind(&self) -> ResidualTolerance {
        self.tolerance
    }

    fn evaluate(&self, point: &[Dual]) -> Result<Vec<Dual>, ResidualError> {
        let row = point[self.axis].shifted(-self.value)?;
        Ok(vec![row; self.rows])
    }
}

fn row(axis: usize, value: f64) -> AxisRow {
    AxisRow {
        axis,
        value,
        rows: 1,
        tolerance: ResidualTolerance::Unitless,
    }
}

fn source(block: usize, row: AxisRow) -> SourceReplacement {
    SourceReplacement {
        block,
        source: Arc::new(row),
    }
}

fn program(limits: Limits) -> Prepared {
    let blocks = vec![Block {
        source: Arc::new(row(0, 0.)),
        variables: vec![0, 1, 2],
        equations: 1,
    }];
    Prepared::new(
        blocks,
        vec![Domain::Length(1.); 3],
        &[(2, -0.)],
        1.,
        SolveOptions::default(),
        limits,
    )
    .unwrap()
}

#[test]
fn source_rebind_accepts_existing_warm_values_and_preserves_layout_and_original_source() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SourceReplacement>();
    let original = program(LIMITS);
    let warm = original.update(&[0.; 3], &[], || false).unwrap();
    let (next, total, hard) = original
        .with_rebound_sources(
            &[source(0, row(1, 0.))],
            &warm.values,
            Some(warm.work),
            || false,
        )
        .unwrap();
    assert_eq!(hard, 0.);
    assert!(total.work > warm.work.work);
    assert!(total.gram_terms > warm.work.gram_terms);
    assert_eq!(
        next.inspect_rank(&warm.values).unwrap(),
        original.inspect_rank(&warm.values).unwrap()
    );
    let next_motion = next.update(&warm.values, &[(0, 1.)], || false).unwrap();
    let original_motion = original.update(&warm.values, &[(1, 1.)], || false).unwrap();
    assert_eq!(next_motion.values, [1., 0., -0.]);
    assert_eq!(original_motion.values, [0., 1., -0.]);
    assert_eq!(next_motion.values[2].to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn source_rebind_checks_row_count_units_and_order_before_publication() {
    let original = program(LIMITS);
    let repeat = [source(0, row(0, 0.)), source(0, row(1, 0.))];
    let outside = [source(1, row(0, 0.))];
    let units = [source(
        0,
        AxisRow {
            tolerance: ResidualTolerance::ModelLength,
            ..row(0, 0.)
        },
    )];
    for changes in [&repeat[..], &outside, &units] {
        assert!(matches!(
            original.with_rebound_sources(changes, &[0.; 3], None, || false),
            Err(PreviewError::Solve(SolveError::Options(_)))
        ));
    }
    let wrong_rows = [source(
        0,
        AxisRow {
            rows: 2,
            ..row(0, 0.)
        },
    )];
    assert!(matches!(
        original.with_rebound_sources(&wrong_rows, &[0.; 3], None, || false),
        Err(PreviewError::Solve(SolveError::Evaluation(
            EvaluationError::Rows { .. }
        )))
    ));
    assert!(original.update(&[0.; 3], &[], || false).is_ok());
}

#[test]
fn rebound_warm_error_and_rank_deficiency_remain_typed() {
    let original = program(LIMITS);
    let warm = original.update(&[0.; 3], &[], || false).unwrap();
    assert!(matches!(
        original.with_rebound_sources(
            &[source(0, row(1, 0.5))],
            &warm.values,
            Some(warm.work),
            || false
        ),
        Err(PreviewError::Incomplete(Incomplete::InfeasibleWarmState))
    ));
    let fixed_dependency = [source(0, row(2, 0.))];
    assert!(matches!(
        original.with_rebound_sources(&fixed_dependency, &warm.values, Some(warm.work), || false),
        Err(PreviewError::Incomplete(Incomplete::RankRecovery { .. }))
    ));
    let cold = original
        .with_rebound_sources(&fixed_dependency, &warm.values, None, || false)
        .unwrap()
        .0;
    assert_eq!(cold.inspect_rank(&warm.values).unwrap(), (1, 2));
    assert_eq!(original.inspect_rank(&warm.values).unwrap(), (2, 1));
}

#[test]
fn source_rebind_resumes_the_tick_limit_and_cancels_at_every_poll() {
    let original = program(LIMITS);
    let changes = [source(0, row(1, 0.))];
    let warm = original.update(&[0.; 3], &[], || false).unwrap();
    let remaining = Work {
        work: LIMITS.work - 1,
        ..warm.work
    };
    let outcome = original.with_rebound_sources(&changes, &warm.values, Some(remaining), || false);
    let Err(error) = outcome else {
        panic!("admitted an exhausted refresh")
    };
    assert_limit(error, Limit::Work);
    let mut total = 0;
    original
        .with_rebound_sources(&changes, &warm.values, Some(warm.work), || {
            total += 1;
            false
        })
        .unwrap();
    for cancel_at in 1..=total {
        let mut polls = 0;
        assert!(matches!(
            original.with_rebound_sources(&changes, &warm.values, Some(warm.work), || {
                polls += 1;
                polls >= cancel_at
            }),
            Err(PreviewError::Incomplete(Incomplete::Cancelled))
        ));
    }
    assert!(original.update(&warm.values, &[], || false).is_ok());
}
