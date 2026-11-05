//! Explicit bounds overloads group native scalar arguments without changing initialization order.
use super::{
    Adaptor3dCurve, BRepAdaptorCurve, BRepAdaptorSurface, BeanFaceIntersector, Failure, Handle,
    Range,
};

impl BeanFaceIntersector {
    /// Constructs explicitly bounded geometry without querying natural surface bounds; `Self::from_adaptors_with_bounds(curve,surface,range,bounds,tolerances)?`.
    pub fn from_adaptors_with_bounds(
        curve: &BRepAdaptorCurve,
        surface: &BRepAdaptorSurface,
        range: Range,
        bounds: [f64; 4],
        tolerances: [f64; 2],
    ) -> Result<Self, Failure> {
        let mut solver = Self {
            range,
            bounds,
            tolerances,
            ..Self::default()
        };
        solver.copy_bounded_adaptors(curve, surface)?;
        Ok(solver)
    }

    fn copy_bounded_adaptors(
        &mut self,
        curve: &BRepAdaptorCurve,
        surface: &BRepAdaptorSurface,
    ) -> Result<(), Failure> {
        // This overload also value-assigns adaptors before resolving geometry.
        self.curve = Handle::new(curve.clone());
        self.criteria = self.tolerances[0] + self.tolerances[1];
        self.curve_resolution = self
            .curve
            .get()
            .expect("bean curve")
            .resolution(self.criteria)?;
        self.surface = Handle::new(surface.clone());
        self.materialize_transformed_surface()
    }

    /// Reinitializes while preserving completion state; `solver.init_adaptors_with_bounds(curve,surface,range,bounds,tolerances)?`.
    pub fn init_adaptors_with_bounds(
        &mut self,
        curve: &BRepAdaptorCurve,
        surface: &BRepAdaptorSurface,
        range: Range,
        bounds: [f64; 4],
        tolerances: [f64; 2],
    ) -> Result<(), Failure> {
        self.init_adaptors(curve, surface, tolerances[0], tolerances[1])?;
        self.set_bean_parameters(range.first(), range.last());
        self.set_surface_parameters(bounds[0], bounds[1], bounds[2], bounds[3]);
        Ok(())
    }
}
