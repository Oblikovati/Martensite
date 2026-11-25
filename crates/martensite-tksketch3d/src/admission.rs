// SPDX-License-Identifier: GPL-2.0-only
//! Keep exact length admission separate from binary64 carrier construction.
use crate::{
    EditFailure, LineGeometry, LineId, MinimumComparison, MinimumLength, PointId, SketchLine,
    Snapshot,
};
use martensite_tkmath::{
    gp::{Direction3, Line3, Point3, Vector3, Xyz},
    math::Vector,
    math_opt::utils::norm,
    predicates::compare_squared_length_3d,
};
use std::cmp::Ordering;

pub(super) fn finite(point: PointId, position: Point3) -> Result<(), EditFailure> {
    for (axis, &value) in position.coordinates().iter().enumerate() {
        if !value.is_finite() {
            return Err(EditFailure::NonFinitePoint { point, axis, value });
        }
    }
    Ok(())
}

pub(super) fn line(
    snapshot: &Snapshot,
    source: SketchLine,
    previous: Option<&LineGeometry>,
) -> Result<LineGeometry, EditFailure> {
    let points = endpoints(snapshot, source)?;
    if let Some(previous) = previous.filter(|previous| matches_inputs(previous, source, points)) {
        return Ok(*previous);
    }
    line_from_points(source, points, snapshot.minimum)
}

pub(super) fn line_from_points(
    source: SketchLine,
    points: [Point3; 2],
    minimum: MinimumLength,
) -> Result<LineGeometry, EditFailure> {
    if points[0] == points[1] {
        return Err(EditFailure::CoincidentEndpoints(source));
    }
    let delta = displacement(source.id, points)?;
    let length_estimate = length_estimate(source.id, delta)?;
    let minimum_comparison = minimum_comparison(source.id, points, length_estimate, minimum)?;
    let carrier = Line3::new(points[0], direction(source.id, delta)?);
    Ok(LineGeometry {
        line: source,
        points,
        carrier,
        length_estimate,
        minimum_comparison,
    })
}

fn matches_inputs(previous: &LineGeometry, source: SketchLine, points: [Point3; 2]) -> bool {
    previous.line == source
        && previous.points.iter().zip(points).all(|(old, new)| {
            // Signed zero is part of the returned coordinates even when equality ignores it.
            old.coordinates().map(f64::to_bits) == new.coordinates().map(f64::to_bits)
        })
}

fn endpoints(snapshot: &Snapshot, source: SketchLine) -> Result<[Point3; 2], EditFailure> {
    let [a, b] = source.endpoints;
    if a == b {
        return Err(EditFailure::SameEndpoint {
            line: source.id,
            point: a,
        });
    }
    Ok([
        endpoint(snapshot, source.id, a)?,
        endpoint(snapshot, source.id, b)?,
    ])
}

fn endpoint(snapshot: &Snapshot, line: LineId, point: PointId) -> Result<Point3, EditFailure> {
    snapshot
        .point(point)
        .map(|point| point.position)
        .ok_or(EditFailure::MissingEndpoint { line, point })
}

fn displacement(line: LineId, [first, second]: [Point3; 2]) -> Result<[f64; 3], EditFailure> {
    let displacement = *Vector3::between(first, second).xyz().coordinates();
    for (axis, value) in displacement.into_iter().enumerate() {
        if !value.is_finite() {
            let endpoints = [first.coordinates()[axis], second.coordinates()[axis]];
            return Err(EditFailure::UnrepresentableDisplacement {
                line,
                axis,
                endpoints,
            });
        }
    }
    Ok(displacement)
}

fn length_estimate(line: LineId, displacement: [f64; 3]) -> Result<f64, EditFailure> {
    let [x, y, z] = displacement;
    let estimate = estimated_norm(Xyz::new(x, y, z));
    if !estimate.is_finite() {
        return Err(EditFailure::UnrepresentableLength { line, displacement });
    }
    Ok(estimate)
}

pub(super) fn estimated_norm(vector: Xyz) -> f64 {
    norm(&Vector::from_xyz(vector))
}

fn minimum_comparison(
    line: LineId,
    [first, second]: [Point3; 2],
    estimate: f64,
    minimum: MinimumLength,
) -> Result<MinimumComparison, EditFailure> {
    let comparison =
        compare_squared_length_3d(*first.coordinates(), *second.coordinates(), minimum.value())
            .map_err(|source| EditFailure::MetricInput { line, source })?;
    match comparison {
        Ordering::Less => Err(EditFailure::BelowMinimumLength {
            line,
            estimate,
            minimum,
        }),
        Ordering::Equal => Ok(MinimumComparison::Equal),
        Ordering::Greater => Ok(MinimumComparison::Greater),
    }
}

fn direction(line: LineId, displacement: [f64; 3]) -> Result<Direction3, EditFailure> {
    // The largest scaled component has magnitude 1, keeping Dir's squared norm usable.
    let scale = displacement.into_iter().map(f64::abs).fold(0., f64::max);
    let [x, y, z] = displacement.map(|component| component / scale);
    let direction = Direction3::new(x, y, z)
        .map_err(|_| EditFailure::DirectionConstruction { line, displacement })?;
    for (axis, (&source, &unit)) in displacement.iter().zip(direction.coordinates()).enumerate() {
        if source != 0. && unit == 0. {
            return Err(EditFailure::UnrepresentableDirection {
                line,
                axis,
                displacement,
            });
        }
    }
    Ok(direction)
}
