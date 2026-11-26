// SPDX-License-Identifier: GPL-2.0-only
//! Source-checked native point queries on the same captured full-period wall.
use super::{
    AttachmentSource, CylinderPointChart, CylinderPointError, CylinderPointQuery,
    CylinderWallPlace, CylindricalFaceError as Error, CylindricalSplitDomain,
};
use martensite_tkmath::predicates::roots::metric_bounds::segment_cylinder::require_nearby_lift;

impl CylindricalSplitDomain {
    /// Seed an angular lift near a caller's previous original parameter.
    /// Example: `wall.point_seed(current, xyz, previous_u)?`.
    pub fn point_seed(
        &self,
        current: AttachmentSource,
        xyz: [f64; 3],
        near: f64,
    ) -> Result<[f64; 2], Error> {
        self.require_current(current)?;
        self.proof.point_seed(xyz, near).map_err(point_failure)
    }

    /// Return the exact axial location of an actual XYZ on the original chart.
    /// Example: `wall.point_place(current, xyz)?`.
    pub fn point_place(
        &self,
        current: AttachmentSource,
        xyz: [f64; 3],
    ) -> Result<CylinderWallPlace, Error> {
        self.require_current(current)?;
        self.proof.point_place(xyz).map_err(point_failure)
    }

    /// Verify that the requested periodic branch is locally unique.
    /// If supplied, the exact original chart directions of the previous and desired XYZ must not be antipodal.
    /// Example: `wall.point_branch(current, previous_u, accepted_u, None)?`.
    pub fn point_branch(
        &self,
        current: AttachmentSource,
        prior: f64,
        next: f64,
        directions: Option<[[f64; 3]; 2]>,
    ) -> Result<(), Error> {
        self.require_current(current)?;
        require_nearby_lift(prior, next).map_err(point_failure)?;
        if let Some([before, after]) = directions
            && self
                .proof
                .point_antipodes(before, after)
                .map_err(point_failure)?
        {
            return Err(Error::Point(CylinderPointError::AmbiguousLift {
                prior,
                next,
            }));
        }
        Ok(())
    }

    /// Certify retained XYZ, original unwrapped UV and their model discrepancy.
    /// Example: `wall.point_qualify(current, uv, xyz, 1e-8, || false)?`.
    pub fn point_qualify(
        &self,
        current: AttachmentSource,
        uv: [f64; 2],
        xyz: [f64; 3],
        limit: f64,
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylinderPointQuery, Error> {
        self.require_current(current)?;
        self.proof
            .point_qualify(uv, xyz, limit, cancelled)
            .map_err(point_failure)
    }

    /// Read the nominal local solver chart, using the frozen exact placed support.
    /// Example: `wall.point_chart(current, uv, || false)?`.
    pub fn point_chart(
        &self,
        current: AttachmentSource,
        uv: [f64; 2],
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylinderPointChart, Error> {
        self.require_current(current)?;
        self.proof.point_chart(uv, cancelled).map_err(point_failure)
    }
}

fn point_failure(cause: CylinderPointError) -> Error {
    match cause {
        CylinderPointError::Cancelled => Error::Cancelled,
        cause => Error::Point(cause),
    }
}
