use super::super::{BaseSequence, Sequence, owner::take_owner, topology::SequenceTopology};
use crate::standard::Failure;

impl<T> Sequence<T> {
    /// Clears/rebinds destination before checking the split index; `sequence.split(2, &mut tail)?`.
    pub fn split(&mut self, index: usize, destination: &mut Self) -> Result<(), Failure> {
        destination.clear(Some(self.allocator().clone()));
        self.offset("Split", index)?;
        destination.owners.reserve(self.len() - index + 1);
        self.topology
            // SAFETY: the index was checked, `destination` was cleared to an empty base, and
            // this sequence exclusively owns its nodes.
            .edit_pair(&mut destination.topology, |base, tail| unsafe {
                base.split(index, tail)
            })?;
        let mut node = destination.topology.endpoints().0;
        while !node.is_null() {
            destination
                .owners
                .insert(node as usize, take_owner(&mut self.owners, node));
            // SAFETY: `node` is a non-null node of the transferred chain, whose owner has just
            // moved to `destination`.
            node = unsafe { (*node).next() };
        }
        Ok(())
    }
    /// Transfers actual base/cache state and leaves the source empty; `Sequence::move_from(&mut source)`.
    pub fn move_from(source: &mut Self) -> Self {
        let empty = BaseSequence::new(Some(source.allocator().clone()));
        let base = source.topology.edit(|base| std::mem::replace(base, empty));
        Self {
            topology: SequenceTopology::from_base(base),
            owners: std::mem::take(&mut source.owners),
        }
    }
    /// Destroys target before transferring source cache and ownership; `sequence.move_assign(&mut source)`.
    pub fn move_assign(&mut self, source: &mut Self) -> &mut Self {
        self.clear(Some(source.allocator().clone()));
        let empty = BaseSequence::new(Some(source.allocator().clone()));
        self.topology
            .edit_pair(&mut source.topology, |base, donor| {
                *base = std::mem::replace(donor, empty);
            });
        self.owners = std::mem::take(&mut source.owners);
        self
    }
}
