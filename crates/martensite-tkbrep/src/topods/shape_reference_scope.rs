//! Martensite's bounded read-only topology frame lends actual observed handles; it never exports an uncounted owning value.
use super::{Shape, ShapeView, TShape, shape_topology_storage::ShapeTopologyStorage};
use martensite_tkernel::standard::{Failure, Handle, HandleReference, catch_native_failure};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::toploc::Location;
impl Shape {
    /// Lend a read-only shape for one callback without retaining its TShape; `Shape::with_reference_parts(slot,&location,orientation,|shape|Ok(shape.nb_children()))?`.
    /// The callback cannot return the borrowed frame; `to_shape` acquires native ownership.
    ///
    /// ```compile_fail
    /// use martensite_tkbrep::topods::{Shape,TCompound,TShape};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkg3d::top_abs::Orientation;
    /// use martensite_tkmath::toploc::Location;
    /// let topology:Handle<dyn TShape>=Handle::new(TCompound::new()).into_upcast();
    /// let escaped=Shape::with_reference_parts(&topology,&Location::default(),Orientation::Forward,|shape|Ok(shape));
    /// ```
    pub fn with_reference_parts<R>(
        topology: &dyn HandleReference<dyn TShape>,
        location: &Location,
        orientation: Orientation,
        project: impl FnOnce(&ShapeView<'_>) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        catch_native_failure(|| {
            let observed = topology.observe();
            let shape = scoped_reference_shape(&observed, location, orientation);
            project(&shape)
        })
    }
}

pub(super) fn scoped_reference_shape<'a>(
    topology: &'a Handle<dyn TShape>,
    location: &Location,
    orientation: Orientation,
) -> ShapeView<'a> {
    ShapeView {
        tshape: ShapeTopologyStorage::Observed(topology),
        location: location.clone(),
        orient: orientation,
    }
}
