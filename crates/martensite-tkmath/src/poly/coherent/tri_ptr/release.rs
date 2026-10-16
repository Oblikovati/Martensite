//! Release through a caller-chosen allocator.
//!
//! `Remove` unlinks the member and then calls `Free` on the supplied allocator;
//! `RemoveList` calls `Free` on each member from the head without unlinking. What that
//! `Free` does with another allocator's block is `BaseAllocator::free_outcome`:
//! released, ignored (IncAllocator), raised (a foreign AccAllocator), or undefined (the C
//! library freeing an arena address); see the release outcome tests.
use super::{
    CoherentTriMember, CoherentTriPtr, MemberAllocation, missing_member, undefined_release,
};
use martensite_tkernel::ncollection::{BaseAllocator, FreeOutcome};
use martensite_tkernel::standard::Failure;

impl CoherentTriPtr {
    /// Removes a member as `Remove`: unlinks it, then frees it through `allocator`.
    /// A raising `Free` reports its failure after the unlink; an undefined
    /// release is refused before any change. `ring.remove_member(m, &arena)?`.
    pub fn remove_member(
        &mut self,
        member: CoherentTriMember,
        allocator: &BaseAllocator,
    ) -> Result<(), Failure> {
        let outcome = self.release_outcome(member, allocator)?;
        if matches!(outcome, FreeOutcome::Undefined) {
            return Err(undefined_release(member));
        }
        let allocation = self
            .detach_member(member)
            .ok_or_else(|| missing_member(member))?;
        settle_release(allocation, outcome)
    }

    /// Frees every member from the head through `allocator`, as `RemoveList`.
    /// When a `Free` raises, the members before it are gone and the raising member and
    /// the rest stay linked (unchecked, they dangle); an undefined release on the way is
    /// refused before any change. `ring.remove_list_in(&arena)?`.
    pub fn remove_list_in(&mut self, allocator: &BaseAllocator) -> Result<(), Failure> {
        let (released, stop) = self.list_release_plan(allocator)?;
        if let Some((member, FreeOutcome::Undefined)) = &stop {
            return Err(undefined_release(*member));
        }
        for (member, outcome) in released {
            let allocation = self
                .detach_member(member)
                .ok_or_else(|| missing_member(member))?;
            settle_release(allocation, outcome)?;
        }
        match stop {
            Some((_, FreeOutcome::Raises(failure))) => Err(failure),
            _ => Ok(()),
        }
    }

    fn release_outcome(
        &self,
        member: CoherentTriMember,
        allocator: &BaseAllocator,
    ) -> Result<FreeOutcome, Failure> {
        let allocation = self
            .members
            .get(&member)
            .ok_or_else(|| missing_member(member))?;
        Ok(allocator.free_outcome(&allocation.allocator, member_address(member)))
    }

    /// Members the loop frees, then the member whose `Free` stops it, if any.
    fn list_release_plan(&self, allocator: &BaseAllocator) -> Result<ReleasePlan, Failure> {
        let mut released = Vec::with_capacity(self.length);
        let mut current = self.head;
        for _ in 0..self.length {
            let Some(member) = current else { break };
            let outcome = self.release_outcome(member, allocator)?;
            if matches!(outcome, FreeOutcome::Raises(_) | FreeOutcome::Undefined) {
                return Ok((released, Some((member, outcome))));
            }
            released.push((member, outcome));
            current = Some(self.member(member)?.next);
        }
        Ok((released, None))
    }
}

type ReleasePlan = (
    Vec<(CoherentTriMember, FreeOutcome)>,
    Option<(CoherentTriMember, FreeOutcome)>,
);

/// The member token is the address of its allocator-owned node.
fn member_address(member: CoherentTriMember) -> *const u8 {
    member.0 as *const u8
}

/// Applies one `Free` outcome to a detached member's storage.
fn settle_release(allocation: MemberAllocation, outcome: FreeOutcome) -> Result<(), Failure> {
    match outcome {
        FreeOutcome::Released => drop(allocation),
        FreeOutcome::Raises(failure) => {
            keep_unreleased(allocation);
            return Err(failure);
        }
        // Callers refuse undefined releases before detaching; keeping the block is the
        // conservative choice should one arrive.
        FreeOutcome::Ignored | FreeOutcome::Undefined => keep_unreleased(allocation),
    }
    Ok(())
}

/// The owner never sees a `Free` for this block. An incremental owner reclaims it
/// only on `Reset`, which Rust models by dropping the lease; any other owner keeps the
/// block allocated for good (an accumulating block keeps its count), so it is leaked.
fn keep_unreleased(allocation: MemberAllocation) {
    let owner = allocation.allocator.clone();
    let address = (&*allocation.member as *const super::RingMember).cast::<u8>();
    if matches!(owner.free_outcome(&owner, address), FreeOutcome::Ignored) {
        drop(allocation);
        return;
    }
    std::mem::forget(allocation.member);
}
