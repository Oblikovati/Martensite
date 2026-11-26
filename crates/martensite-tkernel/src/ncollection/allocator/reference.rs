//! Owning allocator handles are separate from noncounting physical allocation leases.
use super::BaseAllocator;
use crate::ncollection::AllocatorSource;
use crate::standard::{Handle, HandleObserver, HandleUpcast};
use std::sync::Arc;

/// The state every facade of one embedded base shares.
struct FacadeParts {
    inner: Arc<super::AllocatorKind>,
    registration: Arc<super::SourceRegistration>,
    virtual_allocations: Arc<super::virtual_typed::VirtualAllocationRecords>,
}
pub(super) enum AllocatorReference {
    Policy,
    Owned(Handle<dyn AllocatorSource>),
    Storage(super::AllocatorStorageLease),
}
impl BaseAllocator {
    /// Acquire the SAME derived allocator; `BaseAllocator::from_handle(&allocator)`.
    /// This factory registers only a weak compatibility alias in the genuine embedded base.
    pub fn from_handle<T: ?Sized + AllocatorSource + HandleUpcast<dyn AllocatorSource>>(
        handle: &Handle<T>,
    ) -> Self {
        let source = handle.upcast::<dyn AllocatorSource>();
        assert!(
            !source.is_null(),
            "allocator={:?}; expected registered AllocatorSource interface",
            handle
        );
        let parts = source.allocator_object_base().resource().facade_parts();
        parts.registration.register(&source);
        Self::assembled(parts, AllocatorReference::Owned(source))
    }
    /// Retain physical storage with zero handle ownership; `let lease=allocator.storage_lease()`.
    pub fn storage_lease(&self) -> Self {
        if let AllocatorReference::Storage(_) = &self.fields.reference {
            return self.clone();
        }
        if !self.ever_custom() {
            // A built-in source leases this facade's own policy, so one lease serves every
            // allocation (#584).
            return self
                .fields
                .lease
                .get_or_init(|| self.builtin_lease())
                .clone();
        }
        let backend = self
            .dispatch_source(|source| source.physical_storage())
            .unwrap_or_else(|| self.physical_storage());
        let reference = AllocatorReference::Storage(backend);
        self.facade_with_reference(reference)
    }
    fn builtin_lease(&self) -> Self {
        self.facade_with_reference(AllocatorReference::Storage(self.physical_storage()))
    }
    fn facade_with_reference(&self, reference: AllocatorReference) -> Self {
        Self::assembled(self.facade_parts(), reference)
    }
    fn facade_parts(&self) -> FacadeParts {
        FacadeParts {
            inner: self.fields.inner.clone(),
            registration: self.fields.registration.clone(),
            virtual_allocations: self.fields.virtual_allocations.clone(),
        }
    }
    fn assembled(parts: FacadeParts, reference: AllocatorReference) -> Self {
        Self {
            fields: Arc::new(super::AllocatorFacadeFields {
                inner: parts.inner,
                registration: parts.registration,
                reference,
                virtual_allocations: parts.virtual_allocations,
                lease: std::sync::OnceLock::new(),
            }),
        }
    }
    pub(super) fn source_observer(&self) -> Option<HandleObserver<'_, dyn AllocatorSource>> {
        let observed = match &self.fields.reference {
            AllocatorReference::Owned(owner) => owner.observe(),
            AllocatorReference::Storage(_) | AllocatorReference::Policy => {
                self.fields.registration.observe()
            }
        };
        (!observed.is_null()).then_some(observed)
    }
    pub(super) fn source_invocation(&self) -> Option<crate::ncollection::AllocatorInvocation> {
        let observed = self.source_observer()?;
        // This observer is gone before the actual virtual body can release its last owner.
        Some(crate::ncollection::AllocatorInvocation::from_observer(
            &observed,
        ))
    }
    pub(super) fn dispatch_source<R>(
        &self,
        call: impl FnOnce(&dyn AllocatorSource) -> R,
    ) -> Option<R> {
        let observed = self.source_observer()?;
        Some(call(&**observed))
    }
}
impl Clone for BaseAllocator {
    fn clone(&self) -> Self {
        let reference = match &self.fields.reference {
            // A lease never changes what it retains, so its copies share one facade (#584).
            AllocatorReference::Storage(_) => {
                return Self {
                    fields: Arc::clone(&self.fields),
                };
            }
            _ => self
                .source_observer()
                .map_or(AllocatorReference::Policy, |source| {
                    AllocatorReference::Owned(source.into_handle())
                }),
        };
        self.facade_with_reference(reference)
    }
}
// These façades never pin their independently allocated pointee. All shared policy mutation
// remains synchronized; unwind does not expose an intermediate typed allocation record.
impl Unpin for BaseAllocator {}
impl std::panic::UnwindSafe for BaseAllocator {}
impl std::panic::RefUnwindSafe for BaseAllocator {}
