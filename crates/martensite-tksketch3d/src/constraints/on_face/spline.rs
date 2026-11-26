// SPDX-License-Identifier: GPL-2.0-only
//! Atomic source-qualified creation and replay of one fitted Curve On Face cubic.
mod failure;
pub use failure::PlanarFaceSplineFailure;

use super::{PlanarFaceBinding, PlanarFaceRevision};
use crate::{
    Edit, FitSplineId, FitSplineParameters, FitSplinePolicy, PointId, PointRole, SketchFitSpline,
    SketchPoint, Snapshot,
};
use failure::{
    PlanarFaceSplineFailure as Failure, capture_failure, native_failure, poll, require_source,
};
use martensite_tkbrep::brep::planar_attachment::{
    DetachedPlanarCurve, FaceAttachmentCubic, PlanarAttachmentLocation, PlanarCubicCertificate,
    PlanarCubicOptions,
};
use martensite_tkmath::gp::Point3;

/// One original UV station, with explicit point ownership intent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanarFaceSplineStation {
    /// Create a defining point with the caller's reserved identity.
    Create {
        /// Fresh shared sketch identity.
        point: PointId,
        /// Exact authored coordinate on the original chart.
        uv: [f64; 2],
    },
    /// Keep an existing sketch point and use its identity in the new fit.
    Reuse {
        /// Original owned sketch point.
        point: PointId,
        /// Exact authored chart coordinate for the same point.
        uv: [f64; 2],
    },
}

impl PlanarFaceSplineStation {
    fn operands(self) -> (PointId, [f64; 2]) {
        match self {
            Self::Create { point, uv } | Self::Reuse { point, uv } => (point, uv),
        }
    }
}

/// Persisted authoring inputs for one source-bound fitted curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarFaceSplineRecipe {
    /// Actual original face revision and host reservation.
    pub source: PlanarFaceRevision,
    /// Fresh identity for the admitted ordinary fit spline.
    pub spline: FitSplineId,
    /// Ordered directed original chart samples.
    pub stations: [PlanarFaceSplineStation; 4],
    /// Fixed original fit parameter at each station.
    pub parameters: FitSplineParameters,
    /// Ordinary fit conditioning and source regularity limits.
    pub fit_policy: FitSplinePolicy,
    /// Separate native pair distance and trim work limits.
    pub native_options: PlanarCubicOptions,
}

impl PlanarFaceSplineRecipe {
    fn fit(self) -> SketchFitSpline {
        SketchFitSpline {
            id: self.spline,
            stations: self.stations.map(|station| station.operands().0),
            parameters: self.parameters,
            policy: self.fit_policy,
        }
    }
}

/// One complete ordinary sketch edit and its native Curve On Face result.
#[derive(Debug)]
pub struct PlanarFaceSplineEdit {
    /// Snapshot containing the new spline, reused points and any fresh defining points.
    pub snapshot: Snapshot,
    /// Immutable source, trim and native representation evidence for the new fit.
    pub curve: PlanarFaceSpline,
}

/// An immutable authored fit and its bounded native planar pair.
#[derive(Clone, Copy, Debug)]
pub struct PlanarFaceSpline {
    recipe: PlanarFaceSplineRecipe,
    native: FaceAttachmentCubic,
}

impl PlanarFaceSpline {
    /// Read the exact authored recipe. Example: `curve.recipe().stations`.
    pub fn recipe(&self) -> PlanarFaceSplineRecipe {
        self.recipe
    }

    /// Read the full native trim and synchronized spatial proof.
    /// Example: `curve.certificate().distance_bound`.
    pub fn certificate(&self) -> PlanarCubicCertificate {
        self.native.certificate()
    }

    /// Read the largest effective native edge/vertex tolerance.
    /// Example: `curve.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.native.tolerance()
    }

    /// Export fresh independently owned edge and original pcurve support.
    /// Example: `let edge = curve.to_native()?.edge;`.
    pub fn to_native(&self) -> Result<DetachedPlanarCurve, Failure> {
        self.native.to_native().map_err(native_failure)
    }
}

impl PlanarFaceBinding {
    /// Create one ordinary fitted spline and its native face curve as an atomic edit.
    /// Example: `binding.create_fitted_curve(&snapshot, recipe, || source, || false)?`.
    pub fn create_fitted_curve(
        &self,
        source: &Snapshot,
        recipe: PlanarFaceSplineRecipe,
        mut current: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceSplineEdit, Failure> {
        self.begin_spline(recipe, &mut current, &mut cancelled)?;
        let positions = self.spline_stations(recipe, &mut cancelled)?;
        let edits = self.spline_edits(source, recipe, positions)?;
        let snapshot = source
            .apply_targeted_edit_batch(&edits, &mut cancelled)
            .map_err(failure::edit_failure)?;
        let curve =
            self.bound_spline(&snapshot, recipe, positions, &mut current, &mut cancelled)?;
        Ok(PlanarFaceSplineEdit { snapshot, curve })
    }

    /// Replay from named points and an already adopted ordinary fit spline.
    /// Example: `binding.replay_fitted_curve(&snapshot, recipe, || source, || false)?`.
    pub fn replay_fitted_curve(
        &self,
        source: &Snapshot,
        recipe: PlanarFaceSplineRecipe,
        mut current: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceSpline, Failure> {
        self.begin_spline(recipe, &mut current, &mut cancelled)?;
        let positions = self.spline_stations(recipe, &mut cancelled)?;
        self.bound_spline(source, recipe, positions, &mut current, &mut cancelled)
    }

    fn begin_spline(
        &self,
        recipe: PlanarFaceSplineRecipe,
        current: &mut impl FnMut() -> PlanarFaceRevision,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        poll(cancelled)?;
        require_source(recipe.source, self.revision)?;
        require_source(recipe.source, current())
    }

    fn spline_edits(
        &self,
        snapshot: &Snapshot,
        recipe: PlanarFaceSplineRecipe,
        positions: [Point3; 4],
    ) -> Result<Vec<Edit>, Failure> {
        let mut edits = Vec::with_capacity(5);
        for (station, position) in recipe.stations.into_iter().zip(positions) {
            let (id, _) = station.operands();
            match station {
                PlanarFaceSplineStation::Create { .. } => {
                    edits.push(Edit::InsertPoint(SketchPoint {
                        id,
                        role: PointRole::Defining,
                        position,
                    }))
                }
                PlanarFaceSplineStation::Reuse { .. } => require_point(snapshot, id, position)?,
            }
        }
        edits.push(Edit::InsertFitSpline(recipe.fit()));
        Ok(edits)
    }

    fn spline_stations(
        &self,
        recipe: PlanarFaceSplineRecipe,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<[Point3; 4], Failure> {
        let mut positions = [Point3::default(); 4];
        for (station, position) in recipe.stations.into_iter().zip(&mut positions) {
            let (id, uv) = station.operands();
            *position = self.chart_station(recipe, id, uv, cancelled)?;
        }
        Ok(positions)
    }

    fn chart_station(
        &self,
        recipe: PlanarFaceSplineRecipe,
        point: PointId,
        uv: [f64; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Point3, Failure> {
        poll(cancelled)?;
        let face = recipe.source.face;
        let world = self
            .proof
            .solver_chart(face, uv)
            .map_err(capture_failure)?
            .world;
        let query = self
            .proof
            .classify_uv(
                face,
                uv,
                world,
                recipe.native_options.distance_limit,
                cancelled,
            )
            .map_err(capture_failure)?;
        match query.location {
            PlanarAttachmentLocation::OutsideOuter | PlanarAttachmentLocation::HoleInterior => {
                return Err(Failure::StationTrim {
                    point,
                    location: query.location,
                });
            }
            _ => {}
        }
        let [x, y, z] = world;
        Ok(Point3::new(x, y, z))
    }

    fn bound_spline(
        &self,
        snapshot: &Snapshot,
        recipe: PlanarFaceSplineRecipe,
        positions: [Point3; 4],
        current: &mut impl FnMut() -> PlanarFaceRevision,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarFaceSpline, Failure> {
        let stored = snapshot
            .fit_spline(recipe.spline)
            .ok_or(Failure::MissingSpline(recipe.spline))?;
        if stored.recipe() != recipe.fit() {
            return Err(Failure::SplineRecipe(recipe.spline));
        }
        for (id, expected) in recipe.fit().stations.into_iter().zip(positions) {
            require_point(snapshot, id, expected)?;
        }
        let uv = recipe.stations.map(|station| station.operands().1);
        let paired = stored.shared_uv_poles(uv).map_err(Failure::FitNumeric)?;
        let world = stored.poles().map(|point| *point.coordinates());
        let native = self
            .proof
            .build_cubic_curve(
                recipe.source.face,
                world,
                paired,
                recipe.native_options,
                &mut *cancelled,
            )
            .map_err(native_failure)?;
        poll(cancelled)?;
        require_source(recipe.source, current())?;
        poll(cancelled)?;
        Ok(PlanarFaceSpline { recipe, native })
    }
}

fn require_point(snapshot: &Snapshot, id: PointId, expected: Point3) -> Result<(), Failure> {
    let actual = snapshot
        .point(id)
        .ok_or(Failure::MissingPoint(id))?
        .position;
    if actual != expected {
        return Err(Failure::PointChanged {
            point: id,
            expected: *expected.coordinates(),
            actual: *actual.coordinates(),
        });
    }
    Ok(())
}
