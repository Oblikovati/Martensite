// SPDX-License-Identifier: GPL-2.0-only
//! Whole-interval evidence for paired exact affine binary64 controls.
use super::{
    CertifiedPlanarAttachment, PlanarAttachmentError, PlanarAttachmentLocation as Location,
    PlanarAttachmentQuery, chart::Projection, poll,
};
use std::{error::Error, fmt};

/// Which of the two independently checked carrier paths intersects excluded trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarCurvePath {
    /// Orthogonal chart projection of the retained 3D controls.
    WorldProjection,
    /// Original chart coordinates of the retained 2D controls.
    UvLift,
}

/// Exact whole-interval trim and Euclidean distance evidence for paired affine paths.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarCurveCertificate {
    /// Endpoint bounds from projecting the retained 3D curve into the chart.
    pub world_projection: [PlanarAttachmentQuery; 2],
    /// Endpoint bounds from lifting the retained pcurve and comparing against 3D.
    pub uv_lift: [PlanarAttachmentQuery; 2],
    /// Certified Euclidean separation of the paired 3D curve and lifted pcurve on [0, 1].
    pub distance_bound: f64,
}

/// Typed refusal to certify a supported affine planar curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanarCurveProofFailure {
    /// Selected controls are geometrically equal in either representation.
    CoincidentControls(PlanarCurvePath),
    /// Curve visits excluded trim, possibly between individually accepted endpoints.
    Trim {
        /// Representation whose path visits excluded trim.
        path: PlanarCurvePath,
        /// Excluded part of the exact complete trim.
        location: Location,
    },
    /// Distance, coordinate, representation, or cancellation failure from the chart.
    Attachment(PlanarAttachmentError),
}

impl fmt::Display for PlanarCurveProofFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoincidentControls(path) => write!(output, "{path:?} requires distinct controls"),
            Self::Trim { path, location } => write!(output, "{path:?} visits {location:?}"),
            Self::Attachment(cause) => cause.fmt(output),
        }
    }
}

impl Error for PlanarCurveProofFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Attachment(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<PlanarAttachmentError> for PlanarCurveProofFailure {
    fn from(cause: PlanarAttachmentError) -> Self {
        Self::Attachment(cause)
    }
}

impl CertifiedPlanarAttachment {
    /// Prove the two affine paths over a common [0, 1] parameter against the complete trim.
    /// Example: `proof.certify_straight_curve(world_poles, uv_poles, 1e-7, || false)`.
    pub fn certify_straight_curve(
        &self,
        [start, end]: [[f64; 3]; 2],
        [uv_start, uv_end]: [[f64; 2]; 2],
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarCurveCertificate, PlanarCurveProofFailure> {
        use PlanarCurvePath::{UvLift, WorldProjection};
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(PlanarAttachmentError::DistanceLimit(limit).into());
        }
        let projected = [self.world.project(start)?, self.world.project(end)?];
        let lifted = [
            self.world.proposed(uv_start, start)?,
            self.world.proposed(uv_end, end)?,
        ];
        if start == end {
            return Err(PlanarCurveProofFailure::CoincidentControls(WorldProjection));
        }
        let world_projection =
            self.certify_path(projected, WorldProjection, limit, &mut cancelled)?;
        let uv_lift = self.certify_path(lifted, UvLift, limit, &mut cancelled)?;
        poll(&mut cancelled)?;
        // The difference of paired affine paths is affine. Its Euclidean norm
        // is convex, so the maximum endpoint upper bound encloses the interval.
        let distance_bound = uv_lift[0].distance_bound.max(uv_lift[1].distance_bound);
        Ok(PlanarCurveCertificate {
            world_projection,
            uv_lift,
            distance_bound,
        })
    }

    fn certify_path(
        &self,
        [start, end]: [Projection; 2],
        path: PlanarCurvePath,
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<[PlanarAttachmentQuery; 2], PlanarCurveProofFailure> {
        if start.uv == end.uv {
            return Err(PlanarCurveProofFailure::CoincidentControls(path));
        }
        let enters_hole = match &self.hole {
            Some(hole) => hole.intersects_open_interior([&start.uv, &end.uv], cancelled)?,
            None => false,
        };
        let endpoints = [
            self.qualify(start, limit, cancelled)?,
            self.qualify(end, limit, cancelled)?,
        ];
        // Both endpoints in the convex closed outer ring prove the entire path is inside.
        if endpoints
            .iter()
            .any(|end| end.location == Location::OutsideOuter)
        {
            return Err(PlanarCurveProofFailure::Trim {
                path,
                location: Location::OutsideOuter,
            });
        }
        if enters_hole {
            return Err(PlanarCurveProofFailure::Trim {
                path,
                location: Location::HoleInterior,
            });
        }
        Ok(endpoints)
    }
}
