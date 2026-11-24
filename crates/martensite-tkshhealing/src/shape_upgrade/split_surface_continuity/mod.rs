//! ShapeUpgradeSplitSurfaceContinuity.
use super::{SplitSurface, SplitSurfaceData, SurfaceSplitValues};
use crate::shape_extend::ShapeExtendStatus;
use martensite_tkernel::{
    impl_transient, precision,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg3d::geom::{GeomSurface, GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution};
use martensite_tkmath::geom_abs::Shape;

mod compute;
mod spline;
mod wrappers;

#[derive(Clone, Copy)]
struct ContinuitySettings {
    criterion: Shape,
    tolerance: f64,
    order: i32,
}

impl Default for ContinuitySettings {
    fn default() -> Self {
        Self {
            criterion: Shape::C1,
            tolerance: precision::confusion(),
            // The constructor initializes myCont to zero and does not call SetCriterion.
            order: 0,
        }
    }
}

/// Correct a surface's knot continuity or add split values where correction exceeds tolerance.
/// `let splitter = ShapeUpgradeSplitSurfaceContinuity::new();`
pub struct ShapeUpgradeSplitSurfaceContinuity {
    state: SplitSurfaceData,
    settings: SharedState<ContinuitySettings>,
}
impl Default for ShapeUpgradeSplitSurfaceContinuity {
    fn default() -> Self {
        Self {
            state: SplitSurfaceData::default(),
            settings: SharedState::new(ContinuitySettings::default()),
        }
    }
}
impl_transient!(ShapeUpgradeSplitSurfaceContinuity,"ShapeUpgrade_SplitSurfaceContinuity",dyn SplitSurface,interfaces:[dyn SplitSurface]);

impl ShapeUpgradeSplitSurfaceContinuity {
    /// Create the C1 splitter with `precision::confusion()` tolerance;
    /// `ShapeUpgradeSplitSurfaceContinuity::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the continuity order used at interior knots; `splitter.set_criterion(Shape::C2)`.
    pub fn set_criterion(&self, criterion: Shape) {
        let mut settings = self.settings.write();
        settings.criterion = criterion;
        settings.order = surface_continuity_order(criterion);
    }
    /// Set the knot-removal tolerance without clamping; `splitter.set_tolerance(1e-7)`.
    pub fn set_tolerance(&self, tolerance: f64) {
        self.settings.write().tolerance = tolerance;
    }
    /// Shared surface split state; `splitter.split_surface_data()`.
    pub fn split_surface_data(&self) -> &SplitSurfaceData {
        &self.state
    }
    /// Shared U split values; `splitter.u_split_values()`.
    pub fn u_split_values(&self) -> SurfaceSplitValues {
        SplitSurface::u_split_values(self)
    }
    /// Shared V split values; `splitter.v_split_values()`.
    pub fn v_split_values(&self) -> SurfaceSplitValues {
        SplitSurface::v_split_values(self)
    }
    /// Decode a ShapeExtend flag; `splitter.status_flag(ShapeExtendStatus::Done3)`.
    pub fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        SplitSurface::status_flag(self, status)
    }
}

fn surface_continuity_order(criterion: Shape) -> i32 {
    match criterion {
        Shape::C2 => 2,
        Shape::C3 => 3,
        Shape::CN => 4,
        _ => 1,
    }
}
impl SplitSurface for ShapeUpgradeSplitSurfaceContinuity {
    fn split_surface_data(&self) -> &SplitSurfaceData {
        &self.state
    }
    fn compute(&self, segment: bool) -> Result<(), Failure> {
        self.compute_continuity(segment)
    }
}
