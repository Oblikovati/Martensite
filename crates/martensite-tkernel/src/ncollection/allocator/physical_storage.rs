//! Allocation backing storage lives independently of the allocator object's lifetime.
use super::BaseAllocator;
use crate::standard::Failure;
use std::sync::Arc;
/// Owned physical release interface; `AllocatorStorageLease::new(Arc::new(MyPoolBackend))`.
///
/// # Safety
/// The backend must retain every associated live allocation independently of the allocator
/// object, and must accept the original raw pointers once. It must not retain the derived
/// allocator itself: doing so changes destructor order or creates ownership cycles.
pub unsafe trait AllocatorPhysicalStorage: Send + Sync {
    /// Return original storage without invoking a destroyed allocator; `unsafe { backend.free(pointer) }?`.
    /// # Safety
    /// The pointer must be an original live allocation accepted by this actual physical backend.
    unsafe fn free(&self, pointer: *mut u8) -> Result<(), Failure>;
}
/// A noncounting memory-provider lease, never a handle to the allocator object.
#[derive(Clone)]
pub struct AllocatorStorageLease {
    backend: Arc<dyn AllocatorPhysicalStorage>,
}
impl AllocatorStorageLease {
    /// Inject an independent physical provider; `AllocatorStorageLease::new(Arc::new(MyPoolBackend))`.
    pub fn new<T: AllocatorPhysicalStorage + 'static>(backend: Arc<T>) -> Self {
        // This Arc owns allocation backing only; it has no intrusive allocator reference count.
        Self { backend }
    }
    pub(super) unsafe fn free(&self, pointer: *mut u8) -> Result<(), Failure> {
        // SAFETY: this original live pointer belongs to the leased provider.
        unsafe { self.backend.free(pointer) }
    }
}
struct BuiltinAllocatorBacking {
    policy: BaseAllocator,
}
// SAFETY: the detached policy retains heap/arena storage without the handled allocator object.
unsafe impl AllocatorPhysicalStorage for BuiltinAllocatorBacking {
    unsafe fn free(&self, pointer: *mut u8) -> Result<(), Failure> {
        // SAFETY: allocation provenance is restored before this independent raw release.
        unsafe { self.policy.free_backing_nullable(pointer) }
    }
}
impl BaseAllocator {
    pub(in crate::ncollection) fn physical_storage(&self) -> AllocatorStorageLease {
        let policy = self.detached_policy();
        AllocatorStorageLease::new(Arc::new(BuiltinAllocatorBacking { policy }))
    }
    pub(in crate::ncollection) fn detached_policy(&self) -> Self {
        Self {
            fields: Arc::new(super::AllocatorFacadeFields {
                inner: self.fields.inner.clone(),
                registration: Arc::default(),
                virtual_allocations: self.fields.virtual_allocations.clone(),
                reference: super::reference::AllocatorReference::Policy,
                lease: std::sync::OnceLock::new(),
            }),
        }
    }
    pub(super) fn storage_source_retired(&self) -> bool {
        let registered = self.fields.registration.is_registered();
        matches!(
            self.fields.reference,
            super::reference::AllocatorReference::Storage(_)
        ) && registered
            && self.source_observer().is_none()
    }
    /// Whether this is a lease of an accumulating arena: its fallback Free releases an owned
    /// block count, which differs from the policy Free, so it keeps the dispatch path (#584).
    pub(super) fn leases_accumulating_storage(&self) -> bool {
        matches!(
            self.fields.reference,
            super::reference::AllocatorReference::Storage(_)
        ) && matches!(&*self.fields.inner, super::AllocatorKind::Accumulating(_))
    }
    unsafe fn free_backing_nullable(&self, pointer: *mut u8) -> Result<(), Failure> {
        if let super::AllocatorKind::Accumulating(engine) = &*self.fields.inner {
            // Safe owned leases know real block extents even beyond the lookup window.
            return super::locked(engine).free_owned(pointer.addr());
        }
        // SAFETY: physical storage retains this original allocation independently of its object.
        unsafe { self.free_builtin_nullable(pointer) }
    }
    pub(super) unsafe fn free_storage_fallback(&self, pointer: *mut u8) -> Result<(), Failure> {
        if let super::reference::AllocatorReference::Storage(backing) = &self.fields.reference {
            // SAFETY: storage retains the original raw provider independently of the allocator object.
            return unsafe { backing.free(pointer) };
        }
        // SAFETY: unregistered standalone policies retain their own original raw backing.
        unsafe { self.free_builtin_nullable(pointer) }
    }
}
