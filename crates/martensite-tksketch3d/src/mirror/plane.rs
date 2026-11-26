// SPDX-License-Identifier: GPL-2.0-only
use super::MirrorFailure;
use crate::PointId;
use martensite_tkmath::{
    gp::{Axis2, Point3},
    predicates::round_reflected_point_3d,
};
use std::cmp::Ordering;

pub(super) struct MirrorPlane {
    coefficients: [[f64; 3]; 2],
    limit: f64,
}

impl MirrorPlane {
    pub(super) fn new(plane: Axis2, limit: f64) -> Result<Self, MirrorFailure> {
        if !limit.is_finite() || limit < 0. {
            return Err(MirrorFailure::InvalidErrorLimit(limit));
        }
        let admitted = Axis2::from_stored_coefficients(
            plane.location(),
            *plane.direction().coordinates(),
            *plane.x_direction().coordinates(),
            *plane.y_direction().coordinates(),
        )
        .map_err(MirrorFailure::InvalidPlane)?;
        Ok(Self {
            coefficients: [
                *admitted.location().coordinates(),
                *admitted.direction().coordinates(),
            ],
            limit,
        })
    }

    pub(super) fn reflect(&self, point: PointId, source: Point3) -> Result<Point3, MirrorFailure> {
        let ([x, y, z], error) =
            round_reflected_point_3d(*source.coordinates(), self.coefficients, self.limit)
                .map_err(|source| MirrorFailure::MetricInput { point, source })?;
        if error == Ordering::Greater {
            return Err(MirrorFailure::PositionError {
                point,
                limit: self.limit,
            });
        }
        Ok(Point3::new(x, y, z))
    }
}
