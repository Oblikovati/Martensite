// SPDX-License-Identifier: GPL-2.0-only
use super::{FitSplineFailure, FitSplinePolicy, FitSplineQuantity, SketchFitSpline, poll, refusal};
use crate::{EditFailure, MinimumLength, Snapshot, control_spline::CubicEvidence};
use martensite_tkmath::{
    gp::Point3,
    predicates::{
        compare_squared_length_3d,
        roots::{CubicChordDistortion, CubicFitNumericError, cubic_chord_distortion},
    },
};
use std::cmp::Ordering;

pub(super) fn read_stations(
    snapshot: &Snapshot,
    recipe: SketchFitSpline,
) -> Result<[Point3; 4], FitSplineFailure> {
    let mut stations = [Point3::default(); 4];
    for (index, id) in recipe.stations.into_iter().enumerate() {
        if recipe.stations[..index].contains(&id) {
            return Err(FitSplineFailure::RepeatedStation(id));
        }
        let current = snapshot
            .point(id)
            .ok_or(FitSplineFailure::MissingStation(id))?
            .position;
        if let Some(previous) = stations[..index]
            .iter()
            .position(|&position| position == current)
        {
            return Err(FitSplineFailure::CoincidentStations([
                recipe.stations[previous],
                id,
            ]));
        }
        stations[index] = current;
    }
    Ok(stations)
}

pub(super) fn check_policy(policy: FitSplinePolicy) -> Result<(), FitSplineFailure> {
    for (quantity, value) in [
        (FitSplineQuantity::Distortion, policy.max_distortion),
        (FitSplineQuantity::InverseGain, policy.max_inverse_gain),
    ] {
        if !value.is_finite() || value < 1. {
            return Err(FitSplineFailure::InvalidLimit(quantity, value));
        }
    }
    if !(0. ..1.).contains(&policy.max_station_identity_error) {
        return Err(FitSplineFailure::InvalidLimit(
            FitSplineQuantity::StationIdentity,
            policy.max_station_identity_error,
        ));
    }
    Ok(())
}

pub(super) fn check_chords(
    recipe: SketchFitSpline,
    stations: [Point3; 4],
    minimum: MinimumLength,
) -> Result<CubicChordDistortion, FitSplineFailure> {
    let coordinates = stations.map(|point| *point.coordinates());
    for [first, last] in [[0, 3], [0, 1], [1, 2], [2, 3]] {
        let comparison =
            compare_squared_length_3d(coordinates[first], coordinates[last], minimum.value())
                .map_err(|source| {
                    FitSplineFailure::Numeric(CubicFitNumericError::NonFinite(source))
                })?;
        if comparison == Ordering::Less {
            return Err(FitSplineFailure::ShortChord {
                stations: [recipe.stations[first], recipe.stations[last]],
                minimum,
            });
        }
    }
    let spread = cubic_chord_distortion(coordinates, recipe.parameters.values())
        .map_err(FitSplineFailure::Numeric)?;
    check_limit(
        FitSplineQuantity::Distortion,
        spread.ratio_upper,
        recipe.policy.max_distortion,
    )?;
    Ok(spread)
}

pub(super) fn measure_stations(
    recipe: SketchFitSpline,
    stations: [Point3; 4],
    cubic: &CubicEvidence,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[f64; 4], EditFailure> {
    let mut results = [0.; 4];
    for (index, (station, parameter)) in stations
        .into_iter()
        .zip(recipe.parameters.values())
        .enumerate()
    {
        poll(cancelled)?;
        results[index] = cubic
            .polynomial
            .position_miss_upper(parameter, *station.coordinates())
            .map_err(|cause| refusal(recipe.id, FitSplineFailure::Numeric(cause)))?;
        check_limit(
            FitSplineQuantity::StationMiss(index),
            results[index],
            recipe.policy.max_station_miss.value(),
        )
        .map_err(|cause| refusal(recipe.id, cause))?;
    }
    Ok(results)
}

pub(super) fn check_limit(
    quantity: FitSplineQuantity,
    upper: f64,
    limit: f64,
) -> Result<(), FitSplineFailure> {
    if upper > limit {
        return Err(FitSplineFailure::Limit {
            quantity,
            upper,
            limit,
        });
    }
    Ok(())
}
