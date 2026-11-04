//! Point markers of an object: the change goes to
//! an own point aspect of the object (copied from the context defaults first), without a
//! marker image, and the object is recomputed.
use super::*;
use martensite_tkservice::aspect::AspectTypeOfMarker;

impl ViewerSession {
    /// Give the object `name` the marker `kind` drawn at `scale`.
    /// The object is recomputed.
    /// Example: `session.set_marker("p1", AspectTypeOfMarker::Point, 20.)?`.
    pub fn set_marker(
        &self,
        name: &str,
        kind: AspectTypeOfMarker,
        scale: f64,
    ) -> Result<(), Failure> {
        let object = self.named(name)?;
        let defaults = self.context.default_drawer();
        self.with_update(|context| {
            let drawer = object.get().map(|o| o.attributes()).unwrap_or_default();
            if let Some(drawer) = drawer.get() {
                drawer.setup_own_point_aspect(&defaults)?;
                if let Some(point) = drawer.point_aspect().get() {
                    point.set_type_of_marker(kind);
                    if let Some(aspect) = point.aspect().get() {
                        aspect.set_marker_image(Handle::default());
                    }
                    point.set_scale(scale)?;
                }
            }
            context.redisplay(&object, false, false)
        })
    }
}
