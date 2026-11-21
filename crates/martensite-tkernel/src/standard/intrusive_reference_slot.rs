//! Handle fields keep their pointer visible while the prior owner is destroyed.
use super::{
    Failure, Handle, HandleObserver, HandleReference, HandleSlot, RuntimeTyped, SharedState,
    Transient, TransientHandleTarget, WeakHandle,
};
struct ReferenceSlotMembers<T: ?Sized + RuntimeTyped> {
    pointer: WeakHandle<T>,
    ownership: ReferenceSlotOwnership<T>,
    retiring: bool,
}
enum ReferenceSlotOwnership<T: ?Sized> {
    Direct(Handle<T>),
    Shared(Option<HandleSlot<T>>),
}
impl<T: ?Sized> ReferenceSlotOwnership<T> {
    fn is_empty(&self) -> bool {
        match self {
            Self::Direct(handle) => handle.is_null(),
            Self::Shared(slot) => slot.is_none(),
        }
    }
    fn take(&mut self) -> Self {
        match self {
            Self::Direct(handle) => Self::Direct(std::mem::take(handle)),
            Self::Shared(slot) => Self::Shared(slot.take()),
        }
    }
    fn replace(&mut self, handle: Handle<T>) -> Self {
        match self {
            Self::Direct(current) => Self::Direct(std::mem::replace(current, handle)),
            Self::Shared(slot) => {
                let incoming = (!handle.is_null()).then(|| HandleSlot::new(handle));
                Self::Shared(std::mem::replace(slot, incoming))
            }
        }
    }
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
                ownership: ReferenceSlotOwnership::Direct(Handle::default()),
                retiring: false,
            }),
        }
    }
}
impl<T: ?Sized + RuntimeTyped> IntrusiveReferenceSlot<T> {
    /// Move existing ownership into the actual slot; `IntrusiveReferenceSlot::from_owned(target)`.
    pub fn from_owned(target: Handle<T>) -> Self {
        Self::with_ownership(target, false)
    }
    /// Share the slot's owner across unlocked callbacks; `IntrusiveReferenceSlot::from_owned_shared(target)`.
    pub fn from_owned_shared(target: Handle<T>) -> Self {
        Self::with_ownership(target, true)
    }
    fn with_ownership(target: Handle<T>, shared: bool) -> Self {
        let pointer = target.downgrade();
        let mut ownership = if shared {
            ReferenceSlotOwnership::Shared(None)
        } else {
            ReferenceSlotOwnership::Direct(Handle::default())
        };
        ownership.replace(target);
        Self {
            members: SharedState::new(ReferenceSlotMembers {
                pointer,
                ownership,
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
        let mut completion = RetirementCompletion(Some(self));
        drop(old);
        self.finish_retirement(incoming, false);
        completion.0 = None;
        Ok(())
    }
    /// Release ownership without prematurely clearing the pointer; `slot.clear()?`.
    pub fn clear(&self) -> Result<(), Failure> {
        let null = Handle::<T>::default();
        self.assign(&null)
    }
    fn begin_retirement(&self, incoming: *const ()) -> Result<ReferenceSlotOwnership<T>, Failure> {
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
    fn finish_retirement(&self, mut incoming: WeakHandle<T>, retiring: bool) {
        loop {
            let mut members = self.members.write();
            if members.settle_retirement(&mut incoming, retiring) {
                return;
            }
            // A callback can publish another owned target. Its destructor must still
            // see its own pointer and may in turn publish another target.
            let displaced = members.ownership.take();
            drop(members);
            drop(displaced);
        }
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
    /// Lend the current slot or acquire an owning snapshot; `slot.observe()`.
    /// A retiring pointee cannot be safely projected through an expired Arc; no live facade is invented.
    pub fn observe(&self) -> HandleObserver<'_, T> {
        let members = self.members.read();
        if let ReferenceSlotOwnership::Shared(Some(slot)) = &members.ownership {
            return HandleObserver::Leased(slot.lease());
        }
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
impl<T: ?Sized + RuntimeTyped> ReferenceSlotMembers<T> {
    fn settle_retirement(&mut self, incoming: &mut WeakHandle<T>, retiring: bool) -> bool {
        if self.ownership.is_empty() {
            let owner = incoming.upgrade();
            self.pointer = if owner.is_null() {
                WeakHandle::default()
            } else {
                std::mem::take(incoming)
            };
            self.ownership.replace(owner);
        } else if self.pointer.object_address() != incoming.object_address() {
            return false;
        }
        self.retiring = retiring;
        true
    }
}

struct RetirementCompletion<'a, T: ?Sized + RuntimeTyped>(Option<&'a IntrusiveReferenceSlot<T>>);
impl<T: ?Sized + RuntimeTyped> Drop for RetirementCompletion<'_, T> {
    fn drop(&mut self) {
        let Some(slot) = self.0 else {
            return;
        };
        // An unwinding destructor may have published a new target before it failed.
        let mut members = slot.members.write();
        if members.ownership.is_empty() {
            members.pointer = WeakHandle::default();
        }
        members.retiring = false;
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
    fn ref_count(&self) -> usize {
        IntrusiveReferenceSlot::ref_count(self)
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
        self.slot.finish_retirement(incoming, true);
        Ok(())
    }
    /// Nested nullification; `borrow.clear()?`.
    pub fn clear(&self) -> Result<(), Failure> {
        let null = Handle::<T>::default();
        self.assign(&null)
    }
}
