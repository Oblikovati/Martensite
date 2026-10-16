//! Construction, assignment and casts between handle types.
use super::{Handle, HandleView, RuntimeTyped, Transient};
impl<T: ?Sized + RuntimeTyped> Handle<T> {
    /// Shares `other`'s object when it is a `T`, else null.
    /// Every down-cast form (from a base, the same type or an unrelated type)
    /// is one runtime check here; `Handle::<Derived>::down_cast(&base)`.
    pub fn down_cast<U: ?Sized + RuntimeTyped>(other: &Handle<U>) -> Self {
        other.cast::<T>()
    }
    /// A handle for an object another handle already owns:
    /// a view is Rust's non-owning pointer, and the new handle adds one reference, or is
    /// null when the view is null or not a `T`; `Handle::<Base>::from_view(&derived.view())`.
    pub fn from_view<U: ?Sized + RuntimeTyped>(view: &HandleView<'_, U>) -> Self {
        Self::adopt(view.owner())
    }
    /// Moves ownership into a handle of another type without touching the count, as the
    /// converting move does; a non-`U` object gives null
    /// and releases the reference; `derived.into_cast::<Base>()`.
    pub fn into_cast<U: ?Sized + RuntimeTyped>(mut self) -> Handle<U> {
        // Taking the owner leaves `self` null, so its Drop releases nothing.
        let Some(owner) = self.owner.take() else {
            return Handle::default();
        };
        if U::project_from(owner.get()).is_none() {
            owner.get().transient_base().decrement_ref_counter();
            return Handle::default();
        }
        Handle {
            owner: Some(owner),
            projected: std::marker::PhantomData,
        }
    }
    /// `reset(new U(...))`: releases the current object and owns a fresh `U`, which may be
    /// `T` or a class derived from it; a `U` that is not a `T` leaves
    /// the handle null; `base.reset(Derived::default())`.
    pub fn reset<U: Transient + RuntimeTyped>(&mut self, value: U) {
        *self = Handle::new(value).into_cast::<T>();
    }
    /// The owned object's address, 0 for null; `handle.hash_code()`.
    pub fn hash_code(&self) -> usize {
        self.address()
    }
}
