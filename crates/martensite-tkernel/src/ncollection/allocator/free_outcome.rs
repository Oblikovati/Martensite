//! What freeing through one allocator class does with an address another produced.
//!
//! Callers that free through a different allocator handle than the one
//! that allocated (such as coherent triangulation storage) query the outcome first.
use super::{AllocatorKind, BaseAllocator, locked};
use crate::standard::Failure;

/// The result of `freer` freeing an address that `owner` produced.
#[derive(Debug)]
pub enum FreeOutcome {
    /// The storage goes back to its heap (`free`/`_mm_free`), or an accumulating
    /// allocator counts its own address down.
    Released,
    /// Nothing happens: the incremental allocator's free, or `free(NULL)`.
    Ignored,
    /// The accumulating allocator raises `Standard_ProgramError` for an address none
    /// of its live blocks holds (including null).
    Raises(Failure),
    /// The C library frees a pointer it never returned (an arena-interior address): the
    /// behaviour is undefined.
    Undefined,
}

/// The release backend of an allocator class's `Free`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReleaseFamily {
    /// `free` of calloc/malloc storage (Common and Heap).
    Heap,
    /// The aligned free of aligned storage.
    Aligned,
    /// Interior addresses of an arena block (Inc and Acc allocators).
    Arena,
}

impl BaseAllocator {
    /// What freeing through this allocator class does with `address`, which `owner`
    /// allocated (null is valid input); nothing is freed. Built-in classes only: an
    /// allocator source overriding `Free` is not described.
    ///
    /// On Linux the Common, Heap and Aligned classes all end in glibc `free` (`_mm_free`
    /// is `free`), so each releases the others' blocks.
    /// Example: `BaseAllocator::common().free_outcome(&BaseAllocator::global_heap(), p)`
    /// is `FreeOutcome::Released`.
    pub fn free_outcome(&self, owner: &Self, address: *const u8) -> FreeOutcome {
        match &*self.fields.inner {
            AllocatorKind::Incremental(_) => FreeOutcome::Ignored,
            AllocatorKind::Accumulating(engine) => {
                match locked(engine).freeable_block(address as usize) {
                    Ok(_) => FreeOutcome::Released,
                    Err(failure) => FreeOutcome::Raises(failure),
                }
            }
            _ if address.is_null() => FreeOutcome::Ignored,
            _ => heap_outcome(self.release_family(), owner.release_family()),
        }
    }

    fn release_family(&self) -> ReleaseFamily {
        match &*self.fields.inner {
            AllocatorKind::Common | AllocatorKind::Heap => ReleaseFamily::Heap,
            AllocatorKind::Aligned(_) => ReleaseFamily::Aligned,
            AllocatorKind::Incremental(_) | AllocatorKind::Accumulating(_) => ReleaseFamily::Arena,
        }
    }
}

/// A C-library free of `owner`'s storage through `freer`'s backend.
fn heap_outcome(freer: ReleaseFamily, owner: ReleaseFamily) -> FreeOutcome {
    if owner == ReleaseFamily::Arena {
        return FreeOutcome::Undefined;
    }
    if freer == owner || backends_interchange() {
        return FreeOutcome::Released;
    }
    FreeOutcome::Undefined
}

/// glibc (and the other non-MSVC C libraries) free aligned and plain storage
/// with the same `free`.
// TODO(platform): MSVC pairs `_aligned_malloc` with `_aligned_free` only, so mixing the
// Aligned class with Common/Heap is undefined there; runtime evidence is Linux-only.
const fn backends_interchange() -> bool {
    !cfg!(windows)
}
