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
    /// Layout-bearing shared heap storage (Common, Heap and Aligned).
    Heap,
    /// Interior addresses of an arena block (Inc and Acc allocators).
    Arena,
}

impl BaseAllocator {
    /// What freeing through this allocator class does with `address`, which `owner`
    /// allocated (null is valid input); nothing is freed. Built-in classes only: an
    /// allocator source overriding `Free` is not described.
    ///
    /// Common, Heap and Aligned share the layout-bearing Rust heap backend on every
    /// platform, so each releases the others' blocks. Arena-interior addresses remain
    /// incompatible with heap release (#78).
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
            _ => heap_outcome(owner.release_family()),
        }
    }

    fn release_family(&self) -> ReleaseFamily {
        match &*self.fields.inner {
            AllocatorKind::Common | AllocatorKind::Heap | AllocatorKind::Aligned(_) => {
                ReleaseFamily::Heap
            }
            AllocatorKind::Incremental(_) | AllocatorKind::Accumulating(_) => ReleaseFamily::Arena,
        }
    }
}

/// Classify the actual shared backend, rather than a platform C-runtime pairing (#78).
fn heap_outcome(owner: ReleaseFamily) -> FreeOutcome {
    if owner == ReleaseFamily::Arena {
        return FreeOutcome::Undefined;
    }
    FreeOutcome::Released
}
