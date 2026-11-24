//! Track buffer edits without requiring a copy or a scan of its bytes.
use super::{BaseAllocator, Buffer, Failure};
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Identity of one observed buffer revision.
/// Example: `buffer.content_revision().is_some_and(|revision| buffer.matches_content_revision(&revision))`.
#[derive(Clone, Debug)]
pub struct BufferRevision {
    source: Arc<AtomicU64>,
    number: NonZeroU64,
}

impl PartialEq for BufferRevision {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && Arc::ptr_eq(&self.source, &other.source)
    }
}

impl Eq for BufferRevision {}

impl Hash for BufferRevision {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.source).hash(state);
        self.number.hash(state);
    }
}

#[derive(Default)]
pub(super) struct BufferRevisionTracker(Option<Arc<AtomicU64>>);

impl BufferRevisionTracker {
    fn new() -> Self {
        Self(Some(Arc::new(AtomicU64::new(1))))
    }

    fn read(&self) -> Option<BufferRevision> {
        let source = self.0.as_ref()?;
        let number = NonZeroU64::new(source.load(Ordering::Relaxed))?;
        Some(BufferRevision {
            source: Arc::clone(source),
            number,
        })
    }

    fn matches(&self, revision: &BufferRevision) -> bool {
        self.0.as_ref().is_some_and(|source| {
            Arc::ptr_eq(source, &revision.source)
                && source.load(Ordering::Relaxed) == revision.number.get()
        })
    }

    pub(super) fn advance(&mut self) {
        let Some(source) = &self.0 else {
            return;
        };
        let current = source.load(Ordering::Relaxed);
        // Zero marks storage whose writes cannot be tracked. Never reuse a revision number.
        if current != 0 {
            source.store(current.wrapping_add(1), Ordering::Relaxed);
        }
    }

    pub(super) fn expose(&self) {
        if let Some(source) = &self.0 {
            source.store(0, Ordering::Relaxed);
        }
    }

    pub(super) fn retire_storage(&mut self) {
        if self
            .0
            .as_ref()
            .is_some_and(|source| source.load(Ordering::Relaxed) == 0)
        {
            // A pointer into the retired allocation cannot write into replacement storage.
            *self = Self::new();
        } else {
            self.advance();
        }
    }
}

impl Buffer {
    /// Allocates an owned buffer with tracking for its safe mutation methods.
    /// Example: `Buffer::new_versioned(Some(BaseAllocator::common()), 16)?`.
    pub fn new_versioned(allocator: Option<BaseAllocator>, size: usize) -> Result<Self, Failure> {
        let mut buffer = Self::new(allocator, size)?;
        buffer.revision = BufferRevisionTracker::new();
        Ok(buffer)
    }

    /// Captures the identity of the current bytes without retaining the allocation.
    /// Raw pointer access or exhaustion disables tracking until the storage is retired.
    /// Example: `let revision = buffer.content_revision();`.
    pub fn content_revision(&self) -> Option<BufferRevision> {
        self.revision.read()
    }

    /// Checks a saved revision without cloning it. Keep the buffer borrowed while
    /// reading its bytes and the revision to prevent a concurrent edit.
    /// Example: `buffer.matches_content_revision(&revision)`.
    pub fn matches_content_revision(&self, revision: &BufferRevision) -> bool {
        self.revision.matches(revision)
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseAllocator, Buffer, BufferRevision, Ordering};

    const _: fn() = || {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BufferRevision>();
    };

    fn buffer() -> Buffer {
        Buffer::new_versioned(Some(BaseAllocator::common()), 8).unwrap()
    }

    #[test]
    fn versioned_buffer_tracks_mutation_and_reallocation() {
        let mut buffer = buffer();
        let first = buffer.content_revision().unwrap();
        assert_eq!(buffer.content_revision(), Some(first.clone()));
        assert!(buffer.matches_content_revision(&first));
        buffer.as_mut_slice()[0] = 7;
        let edited = buffer.content_revision().unwrap();
        assert_ne!(first, edited);
        assert!(!buffer.matches_content_revision(&first));
        assert!(buffer.allocate(16).unwrap());
        assert!(!buffer.matches_content_revision(&edited));
        assert!(buffer.content_revision().is_some());
    }

    #[test]
    fn each_buffer_has_its_own_content_revision() {
        let first = buffer();
        let second = buffer();
        let saved = first.content_revision().unwrap();
        assert!(!second.matches_content_revision(&saved));
        assert_ne!(second.content_revision(), Some(saved));
    }

    #[test]
    fn raw_address_access_disables_tracking_until_storage_is_retired() {
        let mut buffer = buffer();
        let saved = buffer.content_revision().unwrap();
        assert!(!buffer.data_ptr().is_null());
        assert!(!buffer.matches_content_revision(&saved));
        buffer.as_mut_slice()[0] = 9;
        assert!(buffer.content_revision().is_none());
        assert!(buffer.allocate(8).unwrap());
        assert!(buffer.content_revision().is_some());
        assert!(!buffer.matches_content_revision(&saved));
        assert!(!buffer.data_mut_ptr().is_null());
        assert!(buffer.content_revision().is_none());
    }

    #[test]
    fn exhausted_revision_is_unavailable_for_the_same_storage() {
        let mut buffer = buffer();
        buffer
            .revision
            .0
            .as_ref()
            .unwrap()
            .store(u64::MAX, Ordering::Relaxed);
        let saved = buffer.content_revision().unwrap();
        buffer.as_mut_slice()[0] = 7;
        assert!(!buffer.matches_content_revision(&saved));
        assert!(buffer.content_revision().is_none());
        buffer.free().unwrap();
        assert!(!buffer.matches_content_revision(&saved));
        assert!(buffer.content_revision().is_some());
    }

    #[test]
    fn ordinary_buffers_remain_unversioned() {
        let mut buffer = Buffer::new(Some(BaseAllocator::common()), 8).unwrap();
        buffer.as_mut_slice()[0] = 7;
        assert!(buffer.allocate(8).unwrap());
        assert!(buffer.content_revision().is_none());
    }
}
