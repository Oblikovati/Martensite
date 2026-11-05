//! Bidirectional shape ancestry used while offset intersections are assembled.
use martensite_tkbrep::{topods::Shape, toptools::ShapeMapHasher};
use martensite_tkernel::{
    impl_transient,
    ncollection::{DataMap, List},
    standard::{Failure, StandardTransient},
};
use std::sync::Mutex;
mod mutation;
use mutation::collect_common_descendants;
/// Stores descendant and ascendant shape associations; `BRepAlgoAsDes::new()`.
pub struct BRepAlgoAsDes {
    parent: StandardTransient,
    relations: Mutex<Relations>,
}
struct Relations {
    up: DataMap<Shape, List<Shape>, ShapeMapHasher>,
    down: DataMap<Shape, List<Shape>, ShapeMapHasher>,
    unbound_descendants: List<Shape>,
}
impl_transient!(BRepAlgoAsDes, "BRepAlgoAsDes", StandardTransient, parent);
impl Default for BRepAlgoAsDes {
    fn default() -> Self {
        Self::new()
    }
}
impl BRepAlgoAsDes {
    /// Creates empty bidirectional associations; `BRepAlgoAsDes::new()`.
    pub fn new() -> Self {
        Self {
            parent: StandardTransient::default(),
            relations: Mutex::new(Relations {
                up: DataMap::default(),
                down: DataMap::default(),
                unbound_descendants: List::default(),
            }),
        }
    }
    /// Removes every ancestor and descendant relation; `associations.clear()`.
    pub fn clear(&mut self) {
        let relations = self
            .relations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        relations.up.clear(true);
        relations.down.clear(true);
    }
    /// Adds one descendant without deduplication; `associations.add(&face, &edge)`.
    pub fn add(&mut self, source: &Shape, descendant: &Shape) {
        let relations = self
            .relations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Relations { down, up, .. } = relations;
        Self::add_to(down, up, source, descendant);
    }
    /// Adds one association through a shared native handle; `handle.add_shared(&face, &edge)`.
    pub fn add_shared(&self, source: &Shape, descendant: &Shape) {
        let mut relations = self
            .relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Relations { down, up, .. } = &mut *relations;
        Self::add_to(down, up, source, descendant);
    }
    fn add_to(
        down: &mut DataMap<Shape, List<Shape>, ShapeMapHasher>,
        up: &mut DataMap<Shape, List<Shape>, ShapeMapHasher>,
        source: &Shape,
        descendant: &Shape,
    ) {
        down.try_emplaced(source.clone(), List::default)
            .append(descendant.clone());
        up.try_emplaced(descendant.clone(), List::default)
            .append(source.clone());
    }
    /// Adds each descendant in input order; `associations.add_many(&face, &edges)`.
    pub fn add_many(&mut self, source: &Shape, descendants: &List<Shape>) {
        for descendant in descendants.iter() {
            self.add(source, descendant);
        }
    }
    /// Reports an ancestor association; `associations.has_ascendant(&edge)`.
    pub fn has_ascendant(&self, shape: &Shape) -> bool {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .up
            .is_bound(shape)
    }
    /// Reports a descendant association; `associations.has_descendant(&face)`.
    pub fn has_descendant(&self, shape: &Shape) -> bool {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .down
            .is_bound(shape)
    }
    /// Copies ancestors in insertion order; `associations.ascendant(&edge)`.
    pub fn ascendant(&self, shape: &Shape) -> impl Iterator<Item = Shape> {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .up
            .seek(shape)
            .into_iter()
            .flat_map(|list| list.iter().cloned())
            .collect::<Vec<_>>()
            .into_iter()
    }
    /// Copies descendants in insertion order; `associations.descendant(&face)`.
    pub fn descendant(&self, shape: &Shape) -> impl Iterator<Item = Shape> {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .down
            .seek(shape)
            .into_iter()
            .flat_map(|list| list.iter().cloned())
            .collect::<Vec<_>>()
            .into_iter()
    }
    /// Borrows the descendant list for `shape`.
    ///
    /// An unbound shape uses the association's inert empty-list scratch space, matching the
    /// native method's writable empty-list result without creating an association.
    /// Example: `associations.change_descendant(&face).clear()`.
    pub fn change_descendant(&mut self, shape: &Shape) -> &mut List<Shape> {
        let relations = self
            .relations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if relations.down.is_bound(shape) {
            return relations
                .down
                .change_seek(shape)
                .expect("bound descendant list");
        }
        &mut relations.unbound_descendants
    }
    /// Replaces an associated shape in both directions; `associations.replace(&old, &new)`.
    pub fn replace(&mut self, old: &Shape, new: &Shape) {
        self.relations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .replace(old, new);
    }
    /// Removes a non-ancestor leaf; errors when the shape is itself a source or unbound.
    /// Example: `associations.remove(&edge)?`.
    pub fn remove(&mut self, shape: &Shape) -> Result<(), Failure> {
        self.relations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(shape)
    }
    /// Removes every relation through a shared native handle; `handle.clear_shared()`.
    pub fn clear_shared(&self) {
        let mut relations = self
            .relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        relations.up.clear(true);
        relations.down.clear(true);
    }
    /// Adds each descendant in input order through a shared native handle;
    /// `handle.add_many_shared(&face, &edges)`.
    pub fn add_many_shared(&self, source: &Shape, descendants: &List<Shape>) {
        for descendant in descendants.iter() {
            self.add_shared(source, descendant);
        }
    }
    /// Edits a descendant list through a shared native handle.
    ///
    /// Unbound shapes use the association's inert writable empty-list scratch space;
    /// `handle.with_change_descendant(&face, List::len)`.
    pub fn with_change_descendant<R>(
        &self,
        shape: &Shape,
        edit: impl FnOnce(&mut List<Shape>) -> R,
    ) -> R {
        let mut relations = self
            .relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let descendants = if relations.down.is_bound(shape) {
            relations
                .down
                .change_seek(shape)
                .expect("bound descendant list")
        } else {
            &mut relations.unbound_descendants
        };
        edit(descendants)
    }
    /// Replaces an associated shape in both directions through a shared native handle;
    /// `handle.replace_shared(&old, &new)`.
    pub fn replace_shared(&self, old: &Shape, new: &Shape) {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .replace(old, new);
    }
    /// Removes a non-ancestor leaf through a shared native handle;
    /// `handle.remove_shared(&edge)?`.
    pub fn remove_shared(&self, shape: &Shape) -> Result<(), Failure> {
        self.relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(shape)
    }
    /// Lists descendants common to both shapes and reports whether any were found.
    /// Example: `let found = associations.has_common_descendant(&face1, &face2, &mut common);`.
    pub fn has_common_descendant(
        &self,
        first: &Shape,
        second: &Shape,
        common: &mut List<Shape>,
    ) -> bool {
        common.clear();
        let relations = self
            .relations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for shape in collect_common_descendants(&relations, first, second) {
            common.append(shape);
        }
        !common.is_empty()
    }
}
