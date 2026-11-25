// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tksolve::preview::{PreferenceReport, PreferenceSettlement};

const TANGENT: f64 = 1e-9;

struct Rail;
impl ResidualSource for Rail {
    fn tolerance_kind(&self) -> ResidualTolerance {
        ResidualTolerance::Unitless
    }

    fn evaluate(&self, point: &[Dual]) -> Result<Vec<Dual>, ResidualError> {
        Ok(vec![point[1].clone(), point[2].clone()])
    }
}

fn make(
    source: Arc<dyn ResidualSource + Send + Sync>,
    equations: usize,
    limits: Limits,
) -> Prepared {
    let block = Block {
        source,
        variables: vec![0, 1, 2],
        equations,
    };
    Prepared::new(
        vec![block],
        vec![Domain::Length(1.); 3],
        &[],
        1.,
        SolveOptions::default(),
        limits,
    )
    .unwrap()
}

fn settings() -> Limits {
    Limits {
        targets: 3,
        gram_terms: 4000,
        factor_updates: 4000,
        work: 100_000,
        ..LIMITS
    }
}

#[test]
fn projected_rail_keeps_normal_error_and_delivers_exact_tangent_motion() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<PreferenceReport>();
    let prepared = make(Arc::new(Rail), 2, settings());
    let prior = [0., 0., -0.];
    let target = [(0, 3.), (1, 4.), (2, 0.)];
    assert!(prepared.update(&prior, &target, || false).is_err());
    let (moved, evidence) = prepared
        .update_preferred(&prior, &target, TANGENT, || false)
        .unwrap();
    assert_eq!(moved.values, [3., 0., -0.]);
    assert_eq!(moved.values[2].to_bits(), (-0.0_f64).to_bits());
    assert_eq!((moved.rank, moved.dof, moved.hard_error), (2, 1, 0.));
    assert_eq!(
        evidence.settlement,
        PreferenceSettlement::FirstOrderStationary
    );
    assert_eq!(evidence.scaled_cursor_error, 4.);
    assert_eq!(evidence.max_tangential_gradient, 0.);
    let used = moved.work.work;
    let (verified, repeated) = prepared
        .verify_preferred(moved, &target, TANGENT, || false)
        .unwrap();
    assert_eq!(repeated, evidence);
    assert!(verified.work.work > used);
    assert!(matches!(
        prepared.verify_preferred(verified, &[(0, 6.), (1, 4.)], TANGENT, || false),
        Err(PreviewError::Incomplete(Incomplete::Iterations))
    ));
}

#[test]
fn first_order_radial_center_and_antipodal_preferences_keep_the_prior_bytes() {
    let prepared = make(Arc::new(FixedRadius), 1, settings());
    let prior = [5., 0., -0.];
    for x in [0., -5., 2., 10.] {
        let target = [(0, x), (1, 0.), (2, 0.)];
        let (moved, evidence) = prepared
            .update_preferred(&prior, &target, TANGENT, || false)
            .unwrap();
        assert_eq!(
            moved.values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            prior.map(f64::to_bits)
        );
        assert_eq!(
            evidence.settlement,
            PreferenceSettlement::FirstOrderStationary
        );
        assert_eq!(evidence.scaled_cursor_error, (5. - x).abs());
        assert_eq!(evidence.max_tangential_gradient, 0.);
    }
}

#[test]
fn fixed_preference_has_no_tangent_and_retains_original_fixed_bits() {
    let mut moved = make_fixed()
        .update_preferred(&[-0.], &[(0, 3.)], TANGENT, || false)
        .unwrap();
    assert_eq!(moved.0.values[0].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(moved.1.scaled_cursor_error, 3.);
    assert_eq!(moved.1.max_tangential_gradient, 0.);
    moved.0.values[0] = 0.;
    assert!(matches!(
        make_fixed().verify_preferred(moved.0, &[(0, 3.)], TANGENT, || false),
        Err(PreviewError::Incomplete(Incomplete::InfeasibleWarmState))
    ));
}

fn make_fixed() -> Prepared {
    Prepared::new(
        vec![],
        vec![Domain::Length(1.)],
        &[(0, -0.)],
        1.,
        SolveOptions::default(),
        settings(),
    )
    .unwrap()
}

#[test]
fn projected_final_proof_charges_the_last_work_and_cancels_at_every_poll() {
    let prior = [0., 0., 0.];
    let target = [(0, 3.), (1, 4.)];
    let prepared = make(Arc::new(Rail), 2, settings());
    let mut count = 0;
    let (moved, _) = prepared
        .update_preferred(&prior, &target, TANGENT, || {
            count += 1;
            false
        })
        .unwrap();
    let limits = Limits {
        work: moved.work.work - 1,
        ..settings()
    };
    let limited = make(Arc::new(Rail), 2, limits);
    assert_limit(
        limited
            .update_preferred(&prior, &target, TANGENT, || false)
            .unwrap_err(),
        Limit::Work,
    );
    for cancel_at in 1..=count {
        let mut polls = 0;
        assert!(matches!(
            prepared.update_preferred(&prior, &target, TANGENT, || {
                polls += 1;
                polls >= cancel_at
            }),
            Err(PreviewError::Incomplete(Incomplete::Cancelled))
        ));
    }
    assert_eq!(prior, [0.; 3]);
}

#[test]
fn a_large_fixed_normal_miss_cannot_hide_a_small_strict_tangent_improvement() {
    let prepared = make(Arc::new(Rail), 2, settings());
    let target = [(0, 3.), (1, 1e120)];
    let (moved, evidence) = prepared
        .update_preferred(&[0.; 3], &target, TANGENT, || false)
        .unwrap();
    assert_eq!(moved.values, [3., 0., 0.]);
    assert_eq!(evidence.scaled_cursor_error, 1e120);
    assert_eq!(evidence.max_tangential_gradient, 0.);
}

#[test]
fn preferred_tolerance_must_be_positive_and_finite() {
    let prepared = make(Arc::new(Rail), 2, settings());
    for tolerance in [0., -0.1, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            prepared.update_preferred(&[0.; 3], &[(0, 2.)], tolerance, || false),
            Err(PreviewError::Solve(
                martensite_tksolve::nonlinear::SolveError::Options(_)
            ))
        ));
    }
}
