// SPDX-License-Identifier: GPL-2.0-only
//! Atomic creation and replay of the three authored stations of an actual planar quadratic.
use super::{
    Edit, Failure, FitSplineId, FitSplinePolicy, PlanarFaceBinding, PlanarFaceRevision,
    PlanarFaceSplineStation, Point3, Snapshot, failure, native_failure, poll, require_point,
    require_source,
};
use crate::SketchQuadraticFitSpline;
use martensite_tkbrep::brep::planar_attachment::{
    DetachedPlanarCurve, FaceAttachmentQuadratic, PlanarCubicOptions, PlanarQuadraticCertificate,
};
use martensite_tkmath::predicates::roots::materialize_fit_quadratic;

/// Source and the three actual authored station intents at fixed parameters 0, 1/2 and 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarFaceQuadraticSplineRecipe {
    /// Exact selected face revision and host reservation.
    pub source: PlanarFaceRevision,
    /// Fresh identity in the ordinary fit spline namespace.
    pub spline: FitSplineId,
    /// Three directed original chart stations, including the two actual endpoints.
    pub stations: [PlanarFaceSplineStation; 3],
    /// Ordinary fit conditioning, station distance and regularity limits.
    pub fit_policy: FitSplinePolicy,
    /// Separate native pair position and exact trim work limits.
    pub native_options: PlanarCubicOptions,
}

impl PlanarFaceQuadraticSplineRecipe {
    fn fit(self) -> SketchQuadraticFitSpline {
        SketchQuadraticFitSpline {
            id: self.spline,
            stations: self.stations.map(|station| station.operands().0),
            policy: self.fit_policy,
        }
    }
}

/// Immutable selected-face evidence for one genuine acquired quadratic pair.
#[derive(Clone, Copy, Debug)]
pub struct PlanarFaceQuadraticSpline {
    recipe: PlanarFaceQuadraticSplineRecipe,
    native: FaceAttachmentQuadratic,
    uv_miss: f64,
}

impl PlanarFaceQuadraticSpline {
    /// Read the actual three-point recipe and original selected occurrence.
    /// Example: `curve.recipe().stations`.
    pub fn recipe(&self) -> PlanarFaceQuadraticSplineRecipe {
        self.recipe
    }

    /// Read whole-span proof of the actual original pair.
    /// Example: `curve.certificate().distance_bound`.
    pub fn certificate(&self) -> PlanarQuadraticCertificate {
        self.native.certificate()
    }

    /// Read the largest native edge and vertex tolerance. Example: `curve.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.native.tolerance()
    }

    /// Read the three actual pcurve station misses in original UV L1 units.
    /// Example: `curve.uv_station_miss_upper()[1]`.
    pub fn uv_station_miss_upper(&self) -> [f64; 3] {
        [0., self.uv_miss, 0.]
    }

    /// Export fresh native degree-two curve, pcurve and isolated original support.
    /// Example: `let edge = curve.to_native()?.edge;`.
    pub fn to_native(&self) -> Result<DetachedPlanarCurve, Failure> {
        self.native.to_native().map_err(native_failure)
    }
}

/// Complete atomic creation of the actual three-station face curve.
#[derive(Debug)]
pub struct PlanarFaceQuadraticSplineEdit {
    /// Sketch snapshot with the ordinary fitted quadratic and its three owned stations.
    pub snapshot: Snapshot,
    /// Verified native carrier, pcurve and original face provenance.
    pub curve: PlanarFaceQuadraticSpline,
}

impl PlanarFaceBinding {
    /// Create a native on-face quadratic with exactly three original stations.
    /// Example: `binding.create_three_station_curve(&snapshot, recipe, || source, || false)?`.
    pub fn create_three_station_curve(
        &self,
        source: &Snapshot,
        recipe: PlanarFaceQuadraticSplineRecipe,
        mut current: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceQuadraticSplineEdit, Failure> {
        self.begin_spline(recipe.source, &mut current, &mut cancelled)?;
        let positions = self.quadratic_stations(recipe, &mut cancelled)?;
        let mut edits = self.station_edits(source, recipe.stations, positions)?;
        edits.push(Edit::InsertQuadraticFitSpline(recipe.fit()));
        let snapshot = source
            .apply_targeted_edit_batch(&edits, &mut cancelled)
            .map_err(failure::edit_failure)?;
        let curve =
            self.bound_quadratic(&snapshot, recipe, positions, &mut current, &mut cancelled)?;
        Ok(PlanarFaceQuadraticSplineEdit { snapshot, curve })
    }

    /// Recreate a native pair from the three original stored station identities.
    /// Example: `binding.replay_three_station_curve(&snapshot, recipe, || source, || false)?`.
    pub fn replay_three_station_curve(
        &self,
        source: &Snapshot,
        recipe: PlanarFaceQuadraticSplineRecipe,
        mut current: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceQuadraticSpline, Failure> {
        self.begin_spline(recipe.source, &mut current, &mut cancelled)?;
        let positions = self.quadratic_stations(recipe, &mut cancelled)?;
        self.bound_quadratic(source, recipe, positions, &mut current, &mut cancelled)
    }

    fn quadratic_stations(
        &self,
        recipe: PlanarFaceQuadraticSplineRecipe,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<[Point3; 3], Failure> {
        self.spline_stations(
            recipe.source,
            recipe.stations,
            recipe.native_options.distance_limit,
            cancelled,
        )
    }

    fn bound_quadratic(
        &self,
        snapshot: &Snapshot,
        recipe: PlanarFaceQuadraticSplineRecipe,
        positions: [Point3; 3],
        current: &mut impl FnMut() -> PlanarFaceRevision,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarFaceQuadraticSpline, Failure> {
        let stored = snapshot
            .quadratic_fit_spline(recipe.spline)
            .ok_or_else(|| {
                if snapshot.fit_spline(recipe.spline).is_some() {
                    Failure::SplineRecipe(recipe.spline)
                } else {
                    Failure::MissingSpline(recipe.spline)
                }
            })?;
        let fit = recipe.fit();
        if stored.recipe() != fit {
            return Err(Failure::SplineRecipe(recipe.spline));
        }
        for (id, expected) in fit.stations.into_iter().zip(positions) {
            require_point(snapshot, id, expected)?;
        }
        let native_uv = recipe.stations.map(|station| {
            let (_, [u, v]) = station.operands();
            [u, v, 0.]
        });
        let paired = materialize_fit_quadratic(native_uv).map_err(Failure::FitNumeric)?;
        let world = stored.poles().map(|p| *p.coordinates());
        let uv = paired.poles.map(|[u, v, _]| [u, v]);
        let native = self
            .proof
            .build_quadratic_curve(
                recipe.source.face,
                world,
                uv,
                recipe.native_options,
                &mut *cancelled,
            )
            .map_err(native_failure)?;
        poll(cancelled)?;
        require_source(recipe.source, current())?;
        poll(cancelled)?;
        Ok(PlanarFaceQuadraticSpline {
            recipe,
            native,
            uv_miss: paired.midpoint_miss_upper,
        })
    }
}
