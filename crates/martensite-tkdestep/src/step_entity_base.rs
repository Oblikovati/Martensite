//! Owned views of embedded STEP base classes.
use martensite_tkernel::standard::{
    HandleUpcast, RuntimeType, RuntimeTyped, Transient, TransientHandleTarget, own_interface,
    project_interface,
};
use std::{ops::Deref, sync::Arc};

/// Retains the concrete entity while exposing the fields of an embedded base class.
///
/// A Cartesian point can be used through its point base while preserving identity.
///
/// ```
/// use martensite_tkdestep::{StepEntityBase, step_geom::{StepGeomCartesianPoint, StepGeomPoint}};
/// use martensite_tkernel::standard::Handle;
///
/// let point = Handle::new(StepGeomCartesianPoint::new());
/// let base: Handle<dyn StepEntityBase<StepGeomPoint>> = point.upcast();
/// assert_eq!(base.object_address(), point.object_address());
/// ```
pub trait StepEntityBase<Base>: Transient {
    /// Borrow the embedded base, as in `point.step_base()`.
    fn step_base(&self) -> &Base;
}

impl<Base: RuntimeTyped> RuntimeTyped for dyn StepEntityBase<Base> {
    const TYPE_NAME: &'static str = Base::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        Base::static_type()
    }
}

impl<Base: 'static> Deref for dyn StepEntityBase<Base> {
    type Target = Base;

    fn deref(&self) -> &Base {
        self.step_base()
    }
}

impl<Base: 'static> TransientHandleTarget for dyn StepEntityBase<Base> {
    fn as_transient(&self) -> &dyn Transient {
        self
    }

    fn into_transient(owner: Arc<Self>) -> Arc<dyn Transient> {
        owner
    }

    fn borrow_from_transient(object: &dyn Transient) -> Option<&Self> {
        project_interface(object)
    }

    fn from_transient(owner: Arc<dyn Transient>) -> Option<Arc<Self>> {
        own_interface(owner)
    }
}

impl<Base: 'static> HandleUpcast<dyn Transient> for dyn StepEntityBase<Base> {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn Transient> {
        owner
    }
}
