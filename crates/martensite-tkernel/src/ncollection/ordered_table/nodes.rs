//! Lifetime-bound views of one entry, the safe form of the public first node and the
//! node's key, value and next-in-order link; node addresses stay private.
use super::{OrderSlot, OrderedItems, OrderedTable};

/// Shared ordered node view; `map.first_node().unwrap().key()`.
pub(crate) struct OrderedNodeRef<'a, K, V> {
    inner: OrderedItems<'a, K, V>,
    pair: (&'a K, &'a V),
}

/// Exclusive ordered-data node view; `*map.first_node_mut().unwrap().change_value()=7`.
pub(crate) struct OrderedNodeMut<'a, K, V> {
    order: &'a [OrderSlot],
    keys: &'a [K],
    values: &'a mut [V],
    slot: usize,
}

impl<K, V, H> OrderedTable<K, V, H> {
    pub(crate) fn first_node(&self) -> Option<OrderedNodeRef<'_, K, V>> {
        OrderedNodeRef::at(self.items())
    }
    pub(crate) fn first_node_mut(&mut self) -> Option<OrderedNodeMut<'_, K, V>> {
        let slot = self.first?;
        let (keys, values) = self.storage.payload_mut().split_mut();
        Some(OrderedNodeMut {
            order: &self.order,
            keys,
            values,
            slot,
        })
    }
}

impl<'a, K, V> OrderedNodeRef<'a, K, V> {
    // A view exists only on an entry.
    fn at(inner: OrderedItems<'a, K, V>) -> Option<Self> {
        let pair = inner.current_pair()?;
        Some(Self { inner, pair })
    }
    /// Borrows the stored key; `node.key()`.
    pub fn key(&self) -> &'a K {
        self.pair.0
    }
    /// Borrows the item (unit for the set); `node.value()`.
    pub fn value(&self) -> &'a V {
        self.pair.1
    }
    /// Follows the insertion-order link, `None` after the last entry; `node.order_next()`.
    #[cfg(test)]
    pub(crate) fn order_next(&self) -> Option<Self> {
        let mut inner = self.inner.clone();
        inner.advance();
        Self::at(inner)
    }
}

impl<K, V> OrderedNodeMut<'_, K, V> {
    /// Borrows the immutable key; `node.key()`.
    pub fn key(&self) -> &K {
        &self.keys[self.slot]
    }
    /// Borrows the current value; `node.value()`.
    #[cfg(test)]
    pub(crate) fn value(&self) -> &V {
        &self.values[self.slot]
    }
    /// Edits only the current value; `*node.change_value()=7`.
    pub fn change_value(&mut self) -> &mut V {
        &mut self.values[self.slot]
    }
    /// Consumes this view and follows the insertion-order link; `node.order_next()`.
    pub fn order_next(mut self) -> Option<Self> {
        self.slot = self.order[self.slot].next?;
        Some(self)
    }
}
