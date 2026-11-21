//! Handle fields keep their pointer visible while the prior owner is destroyed.
use super::{
    Failure, Handle, HandleObserver, HandleReference, RuntimeTyped, SharedState, Transient,
    TransientHandleTarget, WeakHandle,
};
struct ReferenceSlotMembers<T: ?Sized + RuntimeTyped> {
    pointer: WeakHandle<T>,
    ownership: Option<Handle<T>>,
    retiring: bool,
}
/// Actual pointer slot plus its Rust ownership lease; `slot.assign(&replacement)?`.
/// The pointer, rather than an owning snapshot, is the authoritative borrowed field value.
pub struct IntrusiveReferenceSlot<T: ?Sized + RuntimeTyped> {
    members: SharedState<ReferenceSlotMembers<T>>,
}
impl<T: ?Sized + RuntimeTyped> Default for IntrusiveReferenceSlot<T> {
    fn default() -> Self {
        Self {
            members: SharedState::new(ReferenceSlotMembers {
                pointer: WeakHandle::default(),
                ownership: None,
                retiring: false,
            }),
        }
    }
}
impl<T: ?Sized + RuntimeTyped> IntrusiveReferenceSlot<T> {
    /// Move existing ownership into the actual slot; `IntrusiveReferenceSlot::from_owned(target)`.
    pub fn from_owned(target: Handle<T>) -> Self {
        // Constructor transfer must not manufacture a temporary owning reference.
        let pointer = target.downgrade();
        Self {
            members: SharedState::new(ReferenceSlotMembers {
                pointer,
                ownership: Some(target),
                retiring: false,
            }),
        }
    }
    /// Keep the end-scope/begin-scope sequencing; `slot.assign(&source)?`.
    pub fn assign(&self, source: &dyn HandleReference<T>) -> Result<(), Failure> {
        let pointer = source.object_address();
        if self.object_address() == pointer {
            return Ok(());
        }
        // Assignment captures its pointer argument before ending the old scope, not an owning copy.
        let incoming = source.observe().downgrade();
        let old = self.begin_retirement(pointer)?;
        drop(old);
        self.finish_retirement(incoming.upgrade());
        Ok(())
    }
    /// Release ownership without prematurely clearing the pointer; `slot.clear()?`.
    pub fn clear(&self) -> Result<(), Failure> {
        let null = Handle::<T>::default();
        self.assign(&null)
    }
    fn begin_retirement(&self, incoming: *const ()) -> Result<Option<Handle<T>>, Failure> {
        let mut members = self.members.write();
        if members.retiring {
            return Err(super::exceptions::program_error(&format!(
                "incoming={incoming:p}, retiring={:p}; expected idle slot or same target",
                members.pointer.object_address()
            )));
        }
        members.retiring = true;
        Ok(members.ownership.take())
    }
    fn finish_retirement(&self, ownership: Handle<T>) {
        let pointer = ownership.downgrade();
        let mut members = self.members.write();
        let displaced = members.ownership.replace(ownership);
        members.pointer = pointer;
        members.retiring = false;
        drop(members);
        drop(displaced);
    }
    /// Observe pointer nullness even during a destructor; `slot.is_null()`.
    pub fn is_null(&self) -> bool {
        let members = self.members.read();
        members.pointer.is_null()
    }
    /// Read the actual pointer field, including its retiring phase; `slot.object_address()`.
    pub fn object_address(&self) -> *const () {
        let members = self.members.read();
        members.pointer.object_address()
    }
    /// Acquire a snapshot of the current object; `slot.observe()`.
    /// A retiring pointee cannot be safely projected through an expired Arc; no live facade is invented.
    pub fn observe(&self) -> HandleObserver<'_, T> {
        let members = self.members.read();
        let observed = members.pointer.upgrade();
        let pointer = members.pointer.object_address();
        drop(members);
        if !pointer.is_null() && observed.is_null() {
            super::raise_native_failure::<()>(Err(super::exceptions::null_object(&format!(
                "pointer={pointer:p}, strong count=0; expected a projectable live pointee"
            ))));
        }
        HandleObserver::Snapshot(observed)
    }
    /// Reborrow the actual destructor pointee; `slot.with_retiring_pointee(self, |p| p.dynamic_type())?`.
    /// The closure cannot return a reference borrowed from its pointee argument.
    /// ```compile_fail
    /// use martensite_tkernel::standard::{IntrusiveReferenceSlot, StandardTransient};
    /// let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    /// let retiring = StandardTransient;
    /// let escaped = slot.with_retiring_pointee(&retiring, |pointee| pointee);
    /// ```
    pub fn with_retiring_pointee<R>(
        &self,
        retiring: &dyn Transient,
        inspect: impl FnOnce(&T) -> R,
    ) -> Result<R, Failure>
    where
        T: TransientHandleTarget,
    {
        self.require_retiring_pointer(retiring)?;
        let pointee = T::borrow_from_transient(retiring).ok_or_else(|| {
            super::exceptions::type_mismatch(&format!(
                "type={}; expected retiring {} projection",
                retiring.dynamic_type().name(),
                T::TYPE_NAME
            ))
        })?;
        Ok(inspect(pointee))
    }
    /// Lend mutation scope without escaping the destructor borrow; `slot.with_retiring_reference(self, |r| r.clear())?`.
    /// ```compile_fail
    /// use martensite_tkernel::standard::{IntrusiveReferenceSlot, StandardTransient};
    /// let slot = IntrusiveReferenceSlot::<StandardTransient>::default();
    /// let retiring = StandardTransient;
    /// let escaped = slot.with_retiring_reference(&retiring, |scope| scope);
    /// ```
    pub fn with_retiring_reference<R>(
        &self,
        retiring: &dyn Transient,
        inspect: impl FnOnce(IntrusiveRetirementBorrow<'_, T>) -> R,
    ) -> Result<R, Failure> {
        self.require_retiring_pointer(retiring)?;
        Ok(inspect(IntrusiveRetirementBorrow { slot: self }))
    }
    fn require_retiring_pointer(&self, retiring: &dyn Transient) -> Result<(), Failure> {
        let members = self.members.read();
        let incoming = std::ptr::from_ref(retiring).cast::<()>();
        let expected = members.pointer.object_address();
        if !members.retiring || incoming != expected {
            return Err(super::exceptions::program_error(&format!(
                "pointer={incoming:p}, retiring={}; expected current retiring pointer={expected:p}",
                members.retiring
            )));
        }
        Ok(())
    }
    /// Inspect the reference count without keeping a field guard; `slot.ref_count()`.
    pub fn ref_count(&self) -> usize {
        let members = self.members.read();
        members.pointer.ref_count()
    }
    /// Identify the actual pointer member for retained aliases; `slot.reference_address()`.
    pub fn reference_address(&self) -> *const () {
        let members = self.members.read();
        std::ptr::from_ref(&members.pointer).cast()
    }
}
#[cfg(test)]
#[path = "intrusive_reference_slot_tests.rs"]
mod tests;
impl<T: ?Sized + RuntimeTyped> HandleReference<T> for IntrusiveReferenceSlot<T> {
    fn observe(&self) -> HandleObserver<'_, T> {
        let slot = self;
        IntrusiveReferenceSlot::observe(slot)
    }
    fn reference_address(&self) -> *const () {
        let slot = self;
        IntrusiveReferenceSlot::reference_address(slot)
    }
    fn object_address(&self) -> *const () {
        let slot = self;
        IntrusiveReferenceSlot::object_address(slot)
    }
    fn is_null(&self) -> bool {
        let slot = self;
        IntrusiveReferenceSlot::is_null(slot)
    }
}

/// A real destructor reborrow plus its original field; `borrow.assign(&incoming)?`.
/// The scope carries no target ownership and must not escape its constructor closure.
pub struct IntrusiveRetirementBorrow<'a, T: ?Sized + RuntimeTyped> {
    slot: &'a IntrusiveReferenceSlot<T>,
}
impl<T: ?Sized + RuntimeTyped> IntrusiveRetirementBorrow<'_, T> {
    /// Assign within a retiring object's callback; `borrow.assign(&replacement)?`.
    pub fn assign(&self, source: &dyn HandleReference<T>) -> Result<(), Failure> {
        if self.slot.object_address() == source.object_address() {
            return Ok(());
        }
        let incoming = source.observe().downgrade();
        self.release_current_reference();
        let owner = incoming.upgrade();
        let mut members = self.slot.members.write();
        members.pointer = owner.downgrade();
        members.ownership = Some(owner);
        Ok(())
    }
    /// Nested nullification; `borrow.clear()?`.
    pub fn clear(&self) -> Result<(), Failure> {
        let null = Handle::<T>::default();
        self.assign(&null)
    }
    fn release_current_reference(&self) {
        let mut members = self.slot.members.write();
        let ownership = members.ownership.take();
        members.pointer = WeakHandle::default();
        drop(members);
        drop(ownership);
    }
}
