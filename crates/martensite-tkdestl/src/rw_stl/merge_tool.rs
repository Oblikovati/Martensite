//! The node merging of the reader.
use super::RwStlReader;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{gp::Xyz, poly::MergeNodesTool};
use std::collections::HashMap;

/// Merges facet nodes and forwards the result to a reader's callbacks.
pub(super) struct ReaderMergeTool<'r, R: RwStlReader + ?Sized> {
    tool: MergeNodesTool,
    reader: &'r R,
    dense_map: Vec<Option<i32>>,
    fallback_map: HashMap<i32, i32>,
}
impl<'r, R: RwStlReader + ?Sized> ReaderMergeTool<'r, R> {
    /// A tool on the reader's angle and tolerance; `nb_facets` sizes the node map, -1 is unknown.
    pub(super) fn new(reader: &'r R, nb_facets: i32) -> Self {
        let mut tool = MergeNodesTool::new(reader.merge_angle(), 0.0, nb_facets);
        // The final triangulation belongs to the reader, so the tool keeps only its merge map.
        *tool.change_output() = Default::default();
        tool.set_merge_angle(reader.merge_angle());
        tool.set_merge_tolerance(reader.merge_tolerance());
        let cap = if nb_facets > 0 {
            (nb_facets as usize).min(1_000_000)
        } else {
            0
        };
        Self {
            tool,
            reader,
            dense_map: Vec::with_capacity(cap),
            fallback_map: HashMap::new(),
        }
    }
    /// Adds a facet; one with two equal merged nodes creates no triangle.
    pub(super) fn add_triangle(&mut self, nodes: &[Xyz; 3]) -> Result<(), Failure> {
        self.tool.add_triangle(nodes)?;
        let first = self.remapped_node(0, nodes);
        let second = self.remapped_node(1, nodes);
        let third = self.remapped_node(2, nodes);
        if first != second && second != third && third != first {
            self.reader.add_triangle(first, second, third);
        }
        Ok(())
    }
    // Keeps this remap for the identifiers AddNode returns, even when they are sequential.
    #[inline]
    fn remapped_node(&mut self, slot: usize, nodes: &[Xyz; 3]) -> i32 {
        let merged = self.tool.element_node_index(slot);
        if merged >= 0 {
            let idx = merged as usize;
            if idx < self.dense_map.len() {
                if let Some(mapped) = self.dense_map[idx] {
                    return mapped;
                }
            } else {
                self.dense_map.resize(idx + 1, None);
            }
            let mapped = self.reader.add_node(&nodes[slot]);
            self.dense_map[idx] = Some(mapped);
            return mapped;
        }
        let reader = self.reader;
        *self
            .fallback_map
            .entry(merged)
            .or_insert_with(|| reader.add_node(&nodes[slot]))
    }
}
