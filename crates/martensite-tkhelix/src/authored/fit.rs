// SPDX-License-Identifier: GPL-2.0-only
//! Bounded caller-owned evaluation through the existing AdvApprox fitter.
use super::{
    ConstantHelix, HelixFailure, HelixFitOptions, HelixStage,
    failure::{native, poll},
};
use crate::helix_geom::{
    HelixGeomTools,
    tools::{Approximation, approximation_tolerance},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{adv_approx::AdvApproxEvaluatorFunction, geom::GeomBSplineCurve};
use martensite_tkmath::geom_abs::Shape;

pub(super) struct Fit {
    pub curve: Handle<GeomBSplineCurve>,
    pub estimate: f64,
    pub evaluations: u32,
}

pub(super) fn build(
    law: ConstantHelix,
    options: HelixFitOptions,
    segments: i32,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Fit, HelixFailure> {
    build_range(
        [0., law.resolved().parameter_end],
        options,
        segments,
        |parameter, order| law.component(parameter, order),
        cancelled,
    )
}

pub(super) fn build_range(
    range: [f64; 2],
    options: HelixFitOptions,
    segments: i32,
    component: impl FnMut(f64, u8) -> Result<[f64; 3], HelixFailure>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Fit, HelixFailure> {
    poll(cancelled, HelixStage::Fit)?;
    let tolerance = native(
        approximation_tolerance(options.approximation_tolerance),
        HelixStage::Fit,
    )?;
    let mut evaluator = Evaluator {
        component,
        limit: options.max_evaluations,
        evaluations: 0,
        refused: None,
        cancelled,
    };
    let build = Approximation {
        range,
        continuity: Shape::C2,
        degree: 8,
        segments,
    }
    .fit(&tolerance, &mut evaluator);
    if let Some(refused) = evaluator.refused {
        return Err(refused);
    }
    let evaluations = evaluator.evaluations;
    poll(cancelled, HelixStage::Fit)?;
    let build = native(build, HelixStage::Fit)?;
    let (mut curve, mut estimate) = (Handle::default(), f64::NAN);
    let status = native(
        HelixGeomTools::extract_result(&build, &mut curve, &mut estimate),
        HelixStage::Fit,
    )?;
    if status != 0 || curve.is_null() {
        return Err(HelixFailure::NativeFitIncomplete(status));
    }
    let limit = options.approximation_tolerance;
    if !estimate.is_finite() || estimate < 0. || estimate > limit {
        return Err(HelixFailure::NativeEstimate { estimate, limit });
    }
    Ok(Fit {
        curve,
        estimate,
        evaluations,
    })
}

struct Evaluator<'a, E, C> {
    component: E,
    limit: u32,
    evaluations: u32,
    refused: Option<HelixFailure>,
    cancelled: &'a mut C,
}

impl<E, C> Evaluator<'_, E, C>
where
    E: FnMut(f64, u8) -> Result<[f64; 3], HelixFailure>,
    C: FnMut() -> bool,
{
    fn evaluate_checked(&mut self, parameter: f64, order: u8) -> Result<[f64; 3], HelixFailure> {
        poll(self.cancelled, HelixStage::Fit)?;
        if self.evaluations == self.limit {
            return Err(HelixFailure::EvaluationLimit {
                completed: self.evaluations,
                limit: self.limit,
            });
        }
        self.evaluations += 1;
        (self.component)(parameter, order)
    }
}

impl<E, C> AdvApproxEvaluatorFunction for Evaluator<'_, E, C>
where
    E: FnMut(f64, u8) -> Result<[f64; 3], HelixFailure>,
    C: FnMut() -> bool,
{
    fn evaluate(
        &mut self,
        dimension: i32,
        _range: [f64; 2],
        parameter: f64,
        order: i32,
        result: &mut [f64],
    ) -> i32 {
        if self.refused.is_some() || dimension != 3 || result.len() < 3 || !(0..=2).contains(&order)
        {
            return 1;
        }
        match self.evaluate_checked(parameter, order as u8) {
            Ok(values) => {
                result[..3].copy_from_slice(&values);
                0
            }
            Err(refused) => {
                self.refused = Some(refused);
                1
            }
        }
    }
}
