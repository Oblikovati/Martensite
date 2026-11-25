// SPDX-License-Identifier: GPL-2.0-only
//! Apply a specified rigid motion without allowing a solver to move other points.
use super::{ConstrainedSketch, ConstraintFailure, adopt::apply_checked, poll, program::Program};
use crate::{Edit, PointId, Snapshot};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, Vector3},
    predicates::{NonFiniteCoordinate, point_on_anchored_direction_3d},
};
use std::{collections::BTreeSet, error::Error, fmt, sync::Arc};

/// Motion acting on the selected owned points in model coordinates.
#[derive(Clone, Copy, Debug)]
pub enum RigidMotion {
    /// A displacement in model length units.
    Translate(Vector3),
    /// A rotation about a located axis, in radians.
    Rotate {
        /// Axis direction and pivot location in model coordinates.
        axis: Axis1,
        /// Signed rotation about the axis.
        radians: f64,
    },
}

/// A declined rigid edit. The source sketch remains available for another edit.
#[derive(Debug)]
#[non_exhaustive]
pub enum RigidMotionFailure {
    /// Select at least one owned point.
    EmptySelection,
    /// A point identity appeared twice in the selection.
    RepeatedPoint(PointId),
    /// A translation, rotation angle or axis cannot define a finite motion.
    InvalidMotion(RigidMotion),
    /// The native rotation constructor refused the supplied axis or angle.
    NativeRotation(Failure),
    /// A coordinate is outside the exact on-axis test domain.
    AxisInput(NonFiniteCoordinate),
    /// A point, geometry check, relation check or cancellation declined adoption.
    Constraint(ConstraintFailure),
}

impl fmt::Display for RigidMotionFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySelection => {
                write!(output, "rigid motion requires at least one owned point")
            }
            Self::RepeatedPoint(point) => {
                write!(output, "rigid motion point {point:?} must occur once")
            }
            Self::InvalidMotion(motion) => write!(
                output,
                "rigid motion {motion:?} requires finite coordinates, a finite angle and a unit rotation axis"
            ),
            Self::NativeRotation(error) => {
                write!(output, "rigid rotation construction failed: {error}")
            }
            Self::AxisInput(error) => write!(output, "rigid rotation axis test failed: {error}"),
            Self::Constraint(error) => error.fmt(output),
        }
    }
}

impl Error for RigidMotionFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NativeRotation(error) => Some(error),
            Self::AxisInput(error) => Some(error),
            Self::Constraint(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ConstraintFailure> for RigidMotionFailure {
    fn from(error: ConstraintFailure) -> Self {
        Self::Constraint(error)
    }
}

impl ConstrainedSketch {
    /// Translate or rotate a selection in one checked edit.
    /// The matching point IDs and all incident line recipes stay in the adopted snapshot.
    /// A Fixed or captured point must retain its coordinates, and every authored relation
    /// must still pass its physical check. Example: `sketch.transform_points(&ids, motion, || false)?`.
    pub fn transform_points(
        &self,
        points: &[PointId],
        motion: RigidMotion,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, RigidMotionFailure> {
        poll(&mut cancelled)?;
        let transform = motion.native_transform()?;
        let edits = staged_moves(&self.geometry, points, transform, &mut cancelled)?;
        let geometry = apply_checked(
            &self.geometry,
            &self.relations,
            &edits,
            self.policy,
            &mut cancelled,
        )?;
        // A rotated reference can require a different stable frame for the next drag.
        let program = Program::new(&geometry, &self.relations, self.policy)?;
        poll(&mut cancelled)?;
        Ok(Self {
            geometry: Arc::new(geometry),
            program: Arc::new(program),
            ..self.clone()
        })
    }
}

#[derive(Clone, Copy)]
enum PreparedMotion {
    Translation(Transform3),
    Rotation { axis: Axis1, local: Transform3 },
}

impl RigidMotion {
    fn native_transform(self) -> Result<PreparedMotion, RigidMotionFailure> {
        match self {
            Self::Translate(offset) if finite(offset.xyz().coordinates()) => Ok(
                PreparedMotion::Translation(Transform3::translation_vector(offset)),
            ),
            Self::Rotate { axis, radians } if radians.is_finite() && valid_axis(axis) => {
                let local_axis = Axis1::new(Point3::default(), axis.direction());
                let local = Transform3::rotation(local_axis, radians)
                    .map_err(RigidMotionFailure::NativeRotation)?;
                Ok(PreparedMotion::Rotation { axis, local })
            }
            _ => Err(RigidMotionFailure::InvalidMotion(self)),
        }
    }
}

impl PreparedMotion {
    fn apply(self, point: Point3) -> Result<Point3, RigidMotionFailure> {
        match self {
            Self::Translation(translation) => Ok(point.transformed(translation)),
            Self::Rotation { axis, local } => rotate_about_axis(point, axis, local),
        }
    }
}

fn rotate_about_axis(
    point: Point3,
    axis: Axis1,
    local: Transform3,
) -> Result<Point3, RigidMotionFailure> {
    let pivot = axis.location();
    let on_axis = point_on_anchored_direction_3d(
        *pivot.coordinates(),
        *point.coordinates(),
        *axis.direction().coordinates(),
    )
    .map_err(RigidMotionFailure::AxisInput)?;
    if on_axis {
        return Ok(point);
    }
    // Work relative to the pivot so a large world-space offset does not erase local motion.
    let offset = Point3::from(point.xyz() - pivot.xyz());
    Ok(Point3::from(offset.transformed(local).xyz() + pivot.xyz()))
}

fn staged_moves(
    geometry: &Snapshot,
    points: &[PointId],
    transform: PreparedMotion,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Edit>, RigidMotionFailure> {
    if points.is_empty() {
        return Err(RigidMotionFailure::EmptySelection);
    }
    let mut seen = BTreeSet::new();
    let mut edits = Vec::new();
    for &point in points {
        poll(cancelled)?;
        if !seen.insert(point) {
            return Err(RigidMotionFailure::RepeatedPoint(point));
        }
        let current = geometry
            .point(point)
            .ok_or(ConstraintFailure::MissingPoint(point))?;
        let position = transform.apply(current.position)?;
        if position != current.position {
            edits.push(Edit::MovePoint { point, position });
        }
    }
    Ok(edits)
}

fn valid_axis(axis: Axis1) -> bool {
    finite(axis.location().coordinates())
        && Direction3::from_stored_coefficients(*axis.direction().coordinates()).is_ok()
}

fn finite(coordinates: &[f64; 3]) -> bool {
    coordinates.iter().all(|value| value.is_finite())
}
