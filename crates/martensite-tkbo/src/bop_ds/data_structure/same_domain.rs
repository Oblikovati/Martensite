//! Same-domain chains follow the pinned native DS lookup without path compression.
use super::DataStructure;
use martensite_tkernel::ncollection::DataMap;

impl DataStructure {
    /// Binds distinct indices; self-bindings are ignored. `ds.add_shape_sd(3, 8)`.
    pub fn add_shape_sd(&mut self, source: i32, representative: i32) {
        if source != representative {
            self.same_domain_vertices.bind(source, representative);
        }
    }

    /// Resolves the final representative; `ds.shape_sd(3) == Some(8)`.
    pub fn shape_sd(&self, source: i32) -> Option<i32> {
        let first = self.same_domain_vertices.find(&source).ok()?;
        Some(self.same_domain_index(*first))
    }

    /// Resolves a chain, retaining unmapped indices; `ds.same_domain_index(3)`.
    pub fn same_domain_index(&self, source: i32) -> i32 {
        let mut representative = source;
        while let Ok(next) = self.same_domain_vertices.find(&representative) {
            representative = *next;
        }
        representative
    }

    /// Writes the representative only when mapped; `ds.has_shape_sd_into(3, &mut index)`.
    pub fn has_shape_sd_into(&self, source: i32, output: &mut i32) -> bool {
        let Some(representative) = self.shape_sd(source) else {
            return false;
        };
        *output = representative;
        true
    }

    /// Traverses direct bindings in native bucket order; `ds.shape_sd_items().count()`.
    pub fn shape_sd_items(&self) -> impl Iterator<Item = (&i32, &i32)> {
        self.same_domain_vertices.items()
    }

    /// Reads the exact direct-binding count without traversal or mutable access.
    /// Example: `let count = ds.same_domain_binding_count();` for operation work estimates.
    pub(crate) fn same_domain_binding_count(&self) -> usize {
        // Representative resolution would change the meaning of the native direct-map count.
        self.same_domain_vertices.len()
    }

    /// The direct same-domain bindings themselves (`ShapesSD`), which the native callers may
    /// change; `ds.shapes_sd().is_bound(&3)`.
    pub fn shapes_sd(&mut self) -> &mut DataMap<i32, i32> {
        &mut self.same_domain_vertices
    }
}

#[cfg(test)]
mod count_contract {
    use super::DataStructure;

    #[test]
    fn direct_count_preserves_chains_and_bucket_iteration() {
        let mut structure = DataStructure::default();
        structure.add_shape_sd(3, 8);
        structure.add_shape_sd(8, 11);
        let before: Vec<(i32, i32)> = structure.shape_sd_items().map(|(a, b)| (*a, *b)).collect();
        assert_eq!(structure.same_domain_binding_count(), 2);
        assert_eq!(structure.shape_sd(3), Some(11));
        let after: Vec<(i32, i32)> = structure.shape_sd_items().map(|(a, b)| (*a, *b)).collect();
        assert_eq!(before, after);
        assert_eq!(structure.same_domain_binding_count(), 2);
    }
}
