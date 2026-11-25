// SPDX-License-Identifier: GPL-2.0-only
//! Prepare the last three columns of the fixed station-to-pole map.
use super::{
    FitSplineBandStage, FitSplineFailure, FitSplineOperatorReport, FitSplineQuantity,
    SketchFitSpline, validation,
};
use martensite_tkernel::ncollection::Array1;
use martensite_tkmath::{
    bsplclib::{build_bsp_matrix, factor_banded_matrix, solve_banded_system_points},
    gp::Point3,
    math::Matrix,
    predicates::roots::CubicFitOperator,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PreparedFit {
    pub(super) operator: CubicFitOperator,
    pub(super) report: FitSplineOperatorReport,
}

pub(super) fn build(recipe: SketchFitSpline) -> Result<PreparedFit, FitSplineFailure> {
    let parameters = recipe.parameters.values();
    let operator =
        CubicFitOperator::new(solve_columns(parameters)?).map_err(FitSplineFailure::Numeric)?;
    let inverse_gain_upper = operator.gain_upper().map_err(FitSplineFailure::Numeric)?;
    let station_identity_upper = operator
        .station_identity_upper(parameters)
        .map_err(FitSplineFailure::Numeric)?;
    validation::check_limit(
        FitSplineQuantity::InverseGain,
        inverse_gain_upper,
        recipe.policy.max_inverse_gain,
    )?;
    validation::check_limit(
        FitSplineQuantity::StationIdentity,
        station_identity_upper,
        recipe.policy.max_station_identity_error,
    )?;
    Ok(PreparedFit {
        operator,
        report: FitSplineOperatorReport {
            inverse_gain_upper,
            station_identity_upper,
        },
    })
}

fn solve_columns(parameters: [f64; 4]) -> Result<[[f64; 3]; 2], FitSplineFailure> {
    let (mut band, upper, lower) = collocation(parameters)?;
    let (status, pivot) = factor_banded_matrix(&mut band, upper, lower)
        .map_err(|_| native(FitSplineBandStage::Factor, None, None))?;
    if status != 0 {
        return Err(native(
            FitSplineBandStage::Factor,
            Some(status),
            Some(pivot),
        ));
    }
    // The origin-relative map needs only columns Q1, Q2 and Q3. The three point
    // coordinates solve their identity right-hand sides in one banded call.
    let rhs = [
        Point3::default(),
        Point3::new(1., 0., 0.),
        Point3::new(0., 1., 0.),
        Point3::new(0., 0., 1.),
    ];
    let mut columns = Array1::from_shared(&rhs, 1);
    let status = solve_banded_system_points(&band, upper, lower, &mut columns)
        .map_err(|_| native(FitSplineBandStage::Solve, None, None))?;
    if status != 0 {
        return Err(native(FitSplineBandStage::Solve, Some(status), None));
    }
    Ok([*columns[2].coordinates(), *columns[3].coordinates()])
}

fn collocation(parameters: [f64; 4]) -> Result<(Matrix<'static>, i32, i32), FitSplineFailure> {
    let mut matrix = Matrix::filled(1, 4, 1, 7, 0.)
        .map_err(|_| native(FitSplineBandStage::Build, None, None))?;
    let parameters = Array1::from_shared(&parameters, 1);
    let orders = Array1::from_shared(&[0; 4], 1);
    let knots = Array1::from_shared(&[0., 0., 0., 0., 1., 1., 1., 1.], 1);
    let (status, upper, lower) = build_bsp_matrix(&parameters, &orders, &knots, 3, &mut matrix)
        .map_err(|_| native(FitSplineBandStage::Build, None, None))?;
    if status != 0 {
        return Err(native(FitSplineBandStage::Build, Some(status), None));
    }
    Ok((matrix, upper, lower))
}

fn native(stage: FitSplineBandStage, status: Option<i32>, pivot: Option<i32>) -> FitSplineFailure {
    FitSplineFailure::Banded {
        stage,
        status,
        pivot,
    }
}
