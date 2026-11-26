// SPDX-License-Identifier: GPL-2.0-only
//! Source checked point queries against the same immutable original spherical band.
use super::{
    AttachmentSource, FaceAttachmentError, SphericalBandError, SphericalFaceDomain,
    SphericalFaceError as Error, numeric_failure,
};
use martensite_tkmath::predicates::roots::metric_bounds::spherical_band::require_spherical_lift;
pub use martensite_tkmath::predicates::roots::metric_bounds::spherical_band::{
    SphericalBandPlace, SphericalPointChart, SphericalPointQuery,
};

impl SphericalFaceDomain {
    /// Whole-band lower bound on world tangent area relative to the radius squared.
    /// Example: `domain.relative_area_lower()`.
    pub fn relative_area_lower(&self) -> f64 {
        self.band.relative_area_lower()
    }

    /// Seed UV near the caller's previous longitude. Example: `domain.point_seed(source, xyz, prior_u)?`.
    pub fn point_seed(
        &self,
        current: AttachmentSource,
        xyz: [f64; 3],
        near: f64,
    ) -> Result<[f64; 2], Error> {
        self.require_source(current)?;
        self.band.point_seed(xyz, near).map_err(numeric_failure)
    }

    /// Read the certified latitude of a retained world ray. Example: `domain.point_place(source, xyz)?`.
    pub fn point_place(
        &self,
        current: AttachmentSource,
        xyz: [f64; 3],
    ) -> Result<SphericalBandPlace, Error> {
        self.require_source(current)?;
        self.band.point_place(xyz).map_err(numeric_failure)
    }

    /// Verify the locally unique longitude step and optional exact source direction pair.
    /// Example: `domain.point_branch(source, previous_u, accepted_u, None)?`.
    pub fn point_branch(
        &self,
        current: AttachmentSource,
        prior: f64,
        next: f64,
        directions: Option<[[f64; 3]; 2]>,
    ) -> Result<(), Error> {
        self.require_source(current)?;
        require_spherical_lift(prior, next).map_err(numeric_failure)?;
        if let Some([a, b]) = directions
            && self.band.point_antipodes(a, b).map_err(numeric_failure)?
        {
            return Err(Error::Numeric(SphericalBandError::Lift { prior, next }));
        }
        Ok(())
    }

    /// Certify the accepted world point and actual original lift against the same band.
    /// Example: `domain.point_qualify(source, uv, xyz, 1e-8, || false)?`.
    pub fn point_qualify(
        &self,
        current: AttachmentSource,
        uv: [f64; 2],
        xyz: [f64; 3],
        limit: f64,
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<SphericalPointQuery, Error> {
        self.require_source(current)?;
        self.band
            .point_qualify(uv, xyz, limit, cancelled)
            .map_err(numeric_failure)
    }

    /// Read a nominal local solver chart from the actual captured coefficients.
    /// Example: `domain.point_chart(source, uv, || false)?`.
    pub fn point_chart(
        &self,
        current: AttachmentSource,
        uv: [f64; 2],
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<SphericalPointChart, Error> {
        self.require_source(current)?;
        self.band
            .point_chart(uv, cancelled)
            .map_err(numeric_failure)
    }

    fn require_source(&self, current: AttachmentSource) -> Result<(), Error> {
        if current != self.source {
            return Err(Error::Source(FaceAttachmentError::StaleSource {
                captured: self.source,
                current,
            }));
        }
        Ok(())
    }
}
