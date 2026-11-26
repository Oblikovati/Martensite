// SPDX-License-Identifier: GPL-2.0-only
use super::{FitSplineFailure, FitSplinePolicy, FitSplineQuantity, SketchFitSpline, poll, refusal};
use crate::{EditFailure, MinimumLength, PointId, Snapshot, control_spline::CubicEvidence};
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
    read_station_points(snapshot, recipe.stations)
}

pub(crate) fn read_station_points<const N: usize>(
    snapshot: &Snapshot,
    ids: [PointId; N],
) -> Result<[Point3; N], FitSplineFailure> {
    let mut stations = [Point3::default(); N];
    for (index, id) in ids.into_iter().enumerate() {
        if ids[..index].contains(&id) {
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
            return Err(FitSplineFailure::CoincidentStations([ids[previous], id]));
        }
        stations[index] = current;
    }
    Ok(stations)
}

pub(crate) fn check_policy(policy: FitSplinePolicy) -> Result<(), FitSplineFailure> {
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
    check_minimum_chords(recipe.stations, coordinates, minimum)?;
    let spread = cubic_chord_distortion(coordinates, recipe.parameters.values())
        .map_err(FitSplineFailure::Numeric)?;
    check_limit(
        FitSplineQuantity::Distortion,
        spread.ratio_upper,
        recipe.policy.max_distortion,
    )?;
    Ok(spread)
}

pub(crate) fn check_minimum_chords<const N: usize>(
    ids: [PointId; N],
    stations: [[f64; 3]; N],
    minimum: MinimumLength,
) -> Result<(), FitSplineFailure> {
    let end = N - 1;
    check_chord([ids[0], ids[end]], [stations[0], stations[end]], minimum)?;
    for i in 0..end {
        check_chord(
            [ids[i], ids[i + 1]],
            [stations[i], stations[i + 1]],
            minimum,
        )?;
    }
    Ok(())
}

fn check_chord(
    ids: [PointId; 2],
    [a, b]: [[f64; 3]; 2],
    minimum: MinimumLength,
) -> Result<(), FitSplineFailure> {
    let comparison = compare_squared_length_3d(a, b, minimum.value())
        .map_err(|cause| FitSplineFailure::Numeric(CubicFitNumericError::NonFinite(cause)))?;
    if comparison == Ordering::Less {
        return Err(FitSplineFailure::ShortChord {
            stations: ids,
            minimum,
        });
    }
    Ok(())
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

pub(crate) fn check_limit(
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
