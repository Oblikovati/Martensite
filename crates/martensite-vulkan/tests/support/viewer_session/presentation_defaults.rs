//! Context-wide presentation defaults applied to objects displayed afterwards.
use super::*;

impl ViewerSession {
    /// Whether shaded presentations triangulate shapes that have no triangulation, or show
    /// only the triangulation already stored; `session.set_auto_triangulation(false)`.
    pub fn set_auto_triangulation(&self, enabled: bool) {
        if let Some(drawer) = self.context.default_drawer().get() {
            drawer.set_auto_triangulation(enabled);
        }
    }

    /// Angular deflection, in degrees, of the shaded triangulation of objects that do not
    /// own one; `session.set_default_deviation_angle(1.)`.
    pub fn set_default_deviation_angle(&self, degrees: f64) {
        if let Some(drawer) = self.context.default_drawer().get() {
            drawer.set_deviation_angle(std::f64::consts::PI * degrees / 180.0);
        }
    }

    /// Relative deflection with `coefficient` for objects that do not own one
    /// Example: `session.set_default_deviation_coefficient(0.0001)`.
    pub fn set_default_deviation_coefficient(&self, coefficient: f64) {
        if let Some(drawer) = self.context.default_drawer().get() {
            drawer.set_type_of_deflection(
                martensite_tkservice::aspect::AspectTypeOfDeflection::Relative,
            );
            drawer.set_deviation_coefficient(coefficient);
        }
    }

    /// Give the shape object `name` its own deviation coefficient, then recompute it, as the
    /// shading-quality command does; `session.set_shading_quality("s", 0.01)?`.
    pub fn set_shading_quality(&self, name: &str, coefficient: f64) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            context.set_object_deviation_coefficient(&object, coefficient, false)?;
            context.redisplay(&object, false, false)
        })
    }

    /// Recompute the displayed object `name` from its current shape and triangulation
    /// Example: `session.redisplay("c")?`.
    pub fn redisplay(&self, name: &str) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| context.redisplay(&object, false, false))
    }
}
