//! The source registered on an allocator's embedded base, and whether it was ever custom.
use crate::ncollection::{
    AccumulatingAllocatorObject, AlignedAllocatorObject, AllocatorObject, AllocatorSource,
    HeapAllocatorObject, IncrementalAllocatorObject,
};
use crate::standard::{Handle, HandleObserver, RuntimeType, RuntimeTyped, SharedState, WeakHandle};
use std::sync::atomic::{AtomicBool, Ordering};

/// A weak alias of the registered source and a flag that stays set once a source whose class
/// is not one of the five built-in allocator classes registers (#584).
///
/// The built-in classes keep the default dispatch bodies, which reduce to the embedded policy;
/// only an allocator that ever had a custom source pays for runtime dispatch on every call.
#[derive(Default)]
pub(super) struct SourceRegistration {
    source: SharedState<WeakHandle<dyn AllocatorSource>>,
    custom: AtomicBool,
}

impl SourceRegistration {
    /// Records `source` as the registered alias; `registration.register(&source)`.
    pub(super) fn register(&self, source: &Handle<dyn AllocatorSource>) {
        if source
            .dynamic_type()
            .is_some_and(|class| !builtin_source_class(class))
        {
            self.custom.store(true, Ordering::Release);
        }
        *self.source.write() = source.downgrade();
    }

    /// Whether a custom source class ever registered here; `registration.ever_custom()`.
    pub(super) fn ever_custom(&self) -> bool {
        self.custom.load(Ordering::Acquire)
    }

    /// The registered source, null when none registered or it retired.
    pub(super) fn observe(&self) -> HandleObserver<'_, dyn AllocatorSource> {
        HandleObserver::Snapshot(self.source.read().upgrade())
    }

    /// Reference count of the registered source, or 0 when retired/unregistered.
    pub(super) fn ref_count(&self) -> usize {
        self.source.read().ref_count()
    }

    /// Whether any source registered, even one that has retired since.
    pub(super) fn is_registered(&self) -> bool {
        !self.source.read().is_null()
    }
}

/// Whether `class` is one of the allocator classes this crate defines.
pub(super) fn builtin_source_class(class: &RuntimeType) -> bool {
    [
        AllocatorObject::static_type(),
        HeapAllocatorObject::static_type(),
        IncrementalAllocatorObject::static_type(),
        AccumulatingAllocatorObject::static_type(),
        AlignedAllocatorObject::static_type(),
    ]
    .into_iter()
    .any(|known| std::ptr::eq(class, known))
}

#[cfg(test)]
#[path = "registration_tests.rs"]
mod tests;
