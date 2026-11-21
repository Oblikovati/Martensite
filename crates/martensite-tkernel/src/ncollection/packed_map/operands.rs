//! Destination aliases are explicit because Rust cannot borrow a mutable map as an operand.
use super::{PackedInteger, PackedMap};
#[cfg(test)]
use crate::ncollection::packed_map_algo as algo;

/// Identifies a set-operation operand without overlapping Rust references.
/// Example: `PackedMapOperand::<i32>::Destination` selects the receiving map.
#[derive(Clone, Copy)]
pub enum PackedMapOperand<'a, T: PackedInteger> {
    /// Use the receiving map's current contents and object identity.
    Destination,
    /// Use a distinct map; two operands may borrow the same source.
    Borrowed(&'a PackedMap<T>),
}

impl<T: PackedInteger> PackedMap<T> {
    /// Applies union, including destination aliases; `map.union_operands(Destination, Borrowed(&other))`.
    #[cfg(test)]
    pub(crate) fn union_operands(
        &mut self,
        left: PackedMapOperand<'_, T>,
        right: PackedMapOperand<'_, T>,
    ) {
        use PackedMapOperand::{Borrowed, Destination};
        match (left, right) {
            (Borrowed(left), Borrowed(right)) => algo::union(self, left, right),
            (Destination, Borrowed(other)) | (Borrowed(other), Destination) => {
                self.union_destination(other);
            }
            (Destination, Destination) => {}
        }
    }

    /// Applies intersection with explicit aliases; `map.intersection_operands(Destination, Borrowed(&other))`.
    #[cfg(test)]
    pub(crate) fn intersection_operands(
        &mut self,
        left: PackedMapOperand<'_, T>,
        right: PackedMapOperand<'_, T>,
    ) {
        use PackedMapOperand::{Borrowed, Destination};
        match (left, right) {
            (Borrowed(left), Borrowed(right)) => algo::intersection(self, left, right),
            (Destination, Borrowed(other)) | (Borrowed(other), Destination) => {
                self.intersect_destination(other);
            }
            (Destination, Destination) if self.is_empty() => self.clear(),
            (Destination, Destination) => {}
        }
    }

    /// Applies subtraction with ordered aliases; `map.subtraction_operands(Borrowed(&left), Destination)`.
    #[cfg(test)]
    pub(crate) fn subtraction_operands(
        &mut self,
        left: PackedMapOperand<'_, T>,
        right: PackedMapOperand<'_, T>,
    ) {
        use PackedMapOperand::{Borrowed, Destination};
        match (left, right) {
            (Borrowed(left), Borrowed(right)) => algo::subtraction(self, left, right),
            (Destination, Borrowed(right)) => {
                algo::subtract(self, right);
            }
            (Borrowed(left), Destination) => self.subtract_from_left(left),
            (Destination, Destination) => {
                self.subtract_self();
            }
        }
    }

    /// Applies difference with explicit aliases; `map.difference_operands(Destination, Destination)` clears it.
    #[cfg(test)]
    pub(crate) fn difference_operands(
        &mut self,
        left: PackedMapOperand<'_, T>,
        right: PackedMapOperand<'_, T>,
    ) {
        use PackedMapOperand::{Borrowed, Destination};
        match (left, right) {
            (Borrowed(left), Borrowed(right)) => algo::difference(self, left, right),
            (Destination, Borrowed(other)) | (Borrowed(other), Destination) => {
                algo::differ(self, other);
            }
            (Destination, Destination) => self.clear(),
        }
    }

    #[cfg(test)]
    fn union_destination(&mut self, other: &Self) {
        if self.is_empty() {
            self.assign(other);
            return;
        }
        algo::unite(self, other);
    }

    #[cfg(test)]
    fn intersect_destination(&mut self, other: &Self) {
        if self.is_empty() || other.is_empty() {
            self.clear();
            return;
        }
        algo::intersect(self, other);
    }

    #[cfg(test)]
    fn subtract_from_left(&mut self, left: &Self) {
        // Right-alias subtraction builds a default map, resetting the destination modulus.
        let mut copy = Self::default();
        algo::subtraction(&mut copy, left, self);
        self.move_assign(&mut copy);
    }
}
