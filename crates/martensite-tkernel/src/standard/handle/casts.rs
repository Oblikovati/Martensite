//! Ownership conversions and borrow-only queries.
use super::{Handle, HandleView, Transient, TransientHandleTarget};
use std::sync::Arc;

/// A compile-time Arc coercion between two compatible object types.
pub trait HandleUpcast<U: ?Sized> {
    /// Coerce the Arc without copying its object or touching the owner count;
    /// `HandleUpcast::<dyn Transient>::upcast_arc(Arc::new(object))`.
    fn upcast_arc(owner: Arc<Self>) -> Arc<U>;
}

impl<T: ?Sized> HandleUpcast<T> for T {
    fn upcast_arc(owner: Arc<Self>) -> Arc<T> {
        owner
    }
}

impl<T: Transient> HandleUpcast<dyn Transient> for T {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn Transient> {
        owner
    }
}

impl<T: ?Sized> Handle<T> {
    /// Coerce to a statically known interface and share ownership; `line.upcast::<dyn Curve>()`.
    pub fn upcast<U: ?Sized>(&self) -> Handle<U>
    where
        T: HandleUpcast<U>,
    {
        self.clone().into_upcast()
    }

    /// Coerce to a statically known interface, moving ownership; `line.into_upcast::<dyn Curve>()`.
    pub fn into_upcast<U: ?Sized>(self) -> Handle<U>
    where
        T: HandleUpcast<U>,
    {
        Handle(self.0.map(T::upcast_arc))
    }
}

impl<T: ?Sized + TransientHandleTarget> Handle<T> {
    /// Recover a concrete object by Rust type, sharing ownership; `curve.downcast::<Line>()`.
    pub fn downcast<U: Transient>(&self) -> Handle<U> {
        self.clone().into_downcast()
    }

    /// Recover a concrete object by moving ownership; `curve.into_downcast::<Line>()`.
    pub fn into_downcast<U: Transient>(self) -> Handle<U> {
        let Some(owner) = self.0.map(T::into_transient) else {
            return Handle::default();
        };
        let erased: Arc<dyn std::any::Any + Send + Sync> = owner;
        Handle(Arc::downcast(erased).ok())
    }

    /// Borrow the concrete type without touching the owner count; `curve.downcast_ref::<Line>()`.
    pub fn downcast_ref<U: Transient>(&self) -> Option<&U> {
        super::super::project_concrete(self.get()?.as_transient())
    }

    /// Cross-cast a handle when only the dynamic interface is known; `curve.cast::<dyn BoundedCurve>()`.
    pub fn cast<U: ?Sized + TransientHandleTarget>(&self) -> Handle<U> {
        self.clone().into_cast()
    }

    /// Move a handle to a dynamic interface implemented by its object; `curve.into_cast::<dyn BoundedCurve>()`.
    pub fn into_cast<U: ?Sized + TransientHandleTarget>(self) -> Handle<U> {
        Handle(self.0.map(T::into_transient).and_then(U::from_transient))
    }

    /// Borrow a concrete object or interface without additional ownership; `curve.view::<Line>()`.
    pub fn view<U: ?Sized + TransientHandleTarget>(&self) -> HandleView<'_, U> {
        HandleView::new(self)
    }

    /// Recover a concrete or interface handle at a checked boundary; `Handle::<Line>::down_cast(&curve)`.
    pub fn down_cast<U: ?Sized + TransientHandleTarget>(other: &Handle<U>) -> Self {
        other.cast()
    }

    /// Own the object from a borrowed view; `Handle::<Line>::from_view(&curve.view::<Line>())`.
    pub fn from_view<U: ?Sized + TransientHandleTarget>(view: &HandleView<'_, U>) -> Self {
        Handle(view.clone_owner().and_then(T::from_transient))
    }

    /// Replace the target with a new value of the same type; `handle.reset(value)`.
    pub fn reset(&mut self, value: T)
    where
        T: Sized,
    {
        *self = Handle::new(value);
    }
}
