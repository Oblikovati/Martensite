use super::super::{BaseSequence, Sequence, topology::SequenceTopology};
use crate::standard::Failure;

impl<T> Sequence<T> {
    /// Clears/rebinds destination before checking the split index; `sequence.split(2, &mut tail)?`.
    pub fn split(&mut self, index: usize, destination: &mut Self) -> Result<(), Failure> {
        destination.clear(Some(self.allocator().clone()));
        destination.common_nodes = self.common_nodes;
        self.offset("Split", index)?;
        // A raw split publishes the new owner before it can discover a retired predecessor.
        // Verify that the prefix links are still live before transferring any ownership.
        self.check_clear_transfer("Split")?;
        self.topology
            // SAFETY: the index was checked, `destination` was cleared to an empty base, and
            // this sequence exclusively owns its nodes.
            .edit_pair(&mut destination.topology, |base, tail| unsafe {
                base.split(index, tail)
            })?;
        if let Some(registry) = &mut self.clear_nodes {
            let mut node = destination.topology.endpoints().0;
            while !node.is_null() {
                registry.release_link(node);
                // SAFETY: the split transferred the live suffix to the destination.
                node = unsafe { (*node).next() };
            }
        }
        Ok(())
    }
    /// Transfers actual base/cache state and leaves the source empty; `Sequence::move_from(&mut source)`.
    pub fn move_from(source: &mut Self) -> Self {
        let empty = BaseSequence::new(Some(source.allocator().clone()));
        let base = source.topology.edit(|base| std::mem::replace(base, empty));
        Self {
            topology: SequenceTopology::from_base(base),
            clear_nodes: source.clear_nodes.take(),
            items: std::marker::PhantomData,
            common_nodes: source.common_nodes,
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
        self.clear_nodes = source.clear_nodes.take();
        self.common_nodes = source.common_nodes;
        self
    }
}
