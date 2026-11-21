//! Read a handle through its original slot, including after replacement.
use super::{
    Failure, Handle, HandleFieldRef, HandleObserver, RetainedHandleReference, RuntimeTyped,
    TransientHandleTarget,
};
use std::marker::PhantomData;

/// Project a genuine live handle slot into a registered base/interface without owning it.
/// Example: `let base=ProjectedHandleReference::<Concrete,dyn Source>::new(&slot)`.
pub struct ProjectedHandleReference<
    'a,
    T: ?Sized + RuntimeTyped + TransientHandleTarget,
    U: ?Sized + RuntimeTyped + TransientHandleTarget,
> {
    source: &'a dyn HandleReference<T>,
    projected: PhantomData<fn() -> U>,
}
impl<
    'a,
    T: ?Sized + RuntimeTyped + TransientHandleTarget,
    U: ?Sized + RuntimeTyped + TransientHandleTarget,
> ProjectedHandleReference<'a, T, U>
{
    /// Borrow the actual original slot; `ProjectedHandleReference::<Concrete,dyn Source>::new(&slot)`.
    pub fn new(source: &'a dyn HandleReference<T>) -> Self {
        Self {
            source,
            projected: PhantomData,
        }
    }
}
impl<
    T: ?Sized + RuntimeTyped + TransientHandleTarget,
    U: ?Sized + RuntimeTyped + TransientHandleTarget,
> HandleReference<U> for ProjectedHandleReference<'_, T, U>
{
    fn retain_reference(&self) -> Result<RetainedHandleReference<U>, Failure> {
        self.source
            .retain_reference()
            .map(RetainedHandleReference::project)
    }
    fn observe(&self) -> HandleObserver<'_, U> {
        // Reread on every use: a callback may have replaced the original caller's slot.
        HandleObserver::Snapshot(self.source.observe().into_handle().into_cast::<U>())
    }
    fn reference_address(&self) -> *const () {
        self.source.reference_address()
    }
    fn object_address(&self) -> *const () {
        // Reads the pointer without type checks or pointee access.
        self.source.object_address()
    }
    fn is_null(&self) -> bool {
        // Tests the entity, even after a virtual delete failed.
        self.source.is_null()
    }
}
/// Const handle reference contract for local slots and retained fields.
/// Example: `let observed=reference.observe(); assert_eq!(reference.object_address(),observed.object_address())`.
pub trait HandleReference<T: ?Sized + RuntimeTyped> {
    /// Retain a supported original slot, never its target; `let alias=slot.retain_reference()?`.
    fn retain_reference(&self) -> Result<RetainedHandleReference<T>, Failure> {
        Err(Failure::with_message(
            "Standard_ProgramError",
            &format!(
                "handle slot {:p} has borrowed lifetime; expected an explicit retained-slot capability",
                self.reference_address()
            ),
        ))
    }
    /// Borrow a stable slot or acquire a snapshot of a mutable one; `reference.observe()`.
    fn observe(&self) -> HandleObserver<'_, T>;
    /// Identify the actual handle slot, independently of its target; `reference.reference_address()`.
    fn reference_address(&self) -> *const ();
    /// Read target identity after any field replacement; `reference.object_address()`.
    fn object_address(&self) -> *const () {
        self.observe().object_address()
    }
    /// Read current null assignment; `reference.is_null()`.
    fn is_null(&self) -> bool {
        self.observe().is_null()
    }
    /// Explicitly own a target snapshot; `let owned=reference.clone_handle()`.
    fn clone_handle(&self) -> Handle<T> {
        self.observe().into_handle()
    }
}
impl<T: ?Sized + RuntimeTyped> HandleReference<T> for Handle<T> {
    fn observe(&self) -> HandleObserver<'_, T> {
        Handle::observe(self)
    }
    fn reference_address(&self) -> *const () {
        std::ptr::from_ref(self).cast()
    }
    fn object_address(&self) -> *const () {
        Handle::object_address(self)
    }
    fn is_null(&self) -> bool {
        Handle::is_null(self)
    }
}
impl<S, T: ?Sized + RuntimeTyped> HandleReference<T> for HandleFieldRef<'_, S, T> {
    fn observe(&self) -> HandleObserver<'_, T> {
        HandleFieldRef::observe(self)
    }
    fn reference_address(&self) -> *const () {
        HandleFieldRef::reference_address(self)
    }
    fn object_address(&self) -> *const () {
        HandleFieldRef::object_address(self)
    }
    fn is_null(&self) -> bool {
        HandleFieldRef::is_null(self)
    }
}
#[cfg(test)]
#[path = "handle_reference_tests.rs"]
mod tests;
