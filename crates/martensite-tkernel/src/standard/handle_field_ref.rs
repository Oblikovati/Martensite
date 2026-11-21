//! Borrowed handle fields observe replacement without retaining the target or a lock.
use super::{Handle, HandleObserver, RuntimeTyped, SharedState};
/// Live handle-field alias; `let alias=HandleFieldRef::new(&fields,select_target)`.
pub struct HandleFieldRef<'a, S, T: ?Sized + RuntimeTyped> {
    fields: &'a SharedState<S>,
    select: fn(&S) -> &Handle<T>,
}
impl<S, T: ?Sized + RuntimeTyped> Copy for HandleFieldRef<'_, S, T> {}
impl<S, T: ?Sized + RuntimeTyped> Clone for HandleFieldRef<'_, S, T> {
    fn clone(&self) -> Self {
        // A second alias retains neither owner nor target.
        *self
    }
}
impl<'a, S, T: ?Sized + RuntimeTyped> HandleFieldRef<'a, S, T> {
    /// Borrow an actual owned field; `HandleFieldRef::new(&fields,select_target)`.
    pub fn new(fields: &'a SharedState<S>, select: fn(&S) -> &Handle<T>) -> Self {
        // The selector borrows the actual slot; it never injects a copied handle facade.
        Self { fields, select }
    }
    /// Observe current null assignment; `alias.is_null()`.
    pub fn is_null(&self) -> bool {
        let fields = self.fields.read();
        (self.select)(&fields).is_null()
    }
    /// Read identity without increasing target ownership; `alias.object_address()`.
    pub fn object_address(&self) -> *const () {
        let fields = self.fields.read();
        (self.select)(&fields).object_address()
    }
    /// Count owners without retaining a new one; `let owners = alias.ref_count();`.
    pub fn ref_count(&self) -> usize {
        let fields = self.fields.read();
        (self.select)(&fields).ref_count()
    }
    /// Compare actual slot references; `left.is_same_reference(&right)`.
    pub fn is_same_reference(&self, other: &HandleFieldRef<'_, S, T>) -> bool {
        let left = self.fields.read();
        let right = other.fields.read();
        std::ptr::eq((self.select)(&left), (other.select)(&right))
    }
    /// Identify the actual field slot independently of its target; `alias.reference_address()`.
    pub fn reference_address(&self) -> *const () {
        let fields = self.fields.read();
        std::ptr::from_ref((self.select)(&fields)).cast()
    }
    /// Explicitly retain a target snapshot; `let owning=alias.clone_handle()`.
    pub fn clone_handle(&self) -> Handle<T> {
        let fields = self.fields.read();
        (self.select)(&fields).clone()
    }
    /// Acquire a snapshot of current storage; `let observed=alias.observe()`.
    pub fn observe(&self) -> HandleObserver<'_, T> {
        HandleObserver::Snapshot(self.clone_handle())
    }
    /// Inspect outside the field lock; `alias.with_object(|target|target.is_some())`.
    pub fn with_object<R>(&self, inspect: impl FnOnce(Option<&T>) -> R) -> R {
        let observed = self.observe();
        inspect(observed.get())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::standard::StandardTransient;
    struct HandleAliasFixture {
        target: Handle<StandardTransient>,
    }
    fn alias_target(fields: &HandleAliasFixture) -> &Handle<StandardTransient> {
        // Selection returns the retained storage, not a temporary getter copy.
        &fields.target
    }
    #[test]
    fn actual_slot_alias_observes_replacement_without_retaining_a_target() {
        let first = Handle::new(StandardTransient);
        let next = Handle::new(StandardTransient);
        let fields = SharedState::new(HandleAliasFixture {
            target: first.clone(),
        });
        let left = HandleFieldRef::new(&fields, alias_target);
        let right = HandleFieldRef::new(&fields, alias_target);
        assert_eq!(first.ref_count(), 2);
        assert!(left.is_same_reference(&right));
        assert_eq!(left.object_address(), first.object_address());
        *fields.write() = HandleAliasFixture {
            target: next.clone(),
        };
        assert_eq!(left.object_address(), next.object_address());
        assert_eq!(left.clone_handle(), next);
        fields.write().target = Handle::default();
        assert!(left.is_null());
    }
    #[test]
    fn inspecting_a_live_target_releases_the_field_lock_before_callback() {
        let target = Handle::new(StandardTransient);
        let fields = SharedState::new(HandleAliasFixture {
            target: target.clone(),
        });
        let alias = HandleFieldRef::new(&fields, alias_target);
        alias.with_object(|current| {
            assert_eq!(target.ref_count(), 3);
            assert!(current.is_some());
            fields.write().target = Handle::default();
        });
        assert!(alias.is_null());
    }
}
