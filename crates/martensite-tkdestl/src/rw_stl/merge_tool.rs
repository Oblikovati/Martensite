//! The node merging of the reader.
use super::RwStlReader;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{gp::Xyz, poly::MergeNodesTool};
use std::collections::HashMap;

/// Merges facet nodes and forwards the result to a reader's callbacks.
pub(super) struct ReaderMergeTool<'r, R: RwStlReader + ?Sized> {
    tool: MergeNodesTool,
    reader: &'r R,
    node_index_map: HashMap<i32, i32>,
}
impl<'r, R: RwStlReader + ?Sized> ReaderMergeTool<'r, R> {
    /// A tool on the reader's angle and tolerance; `nb_facets` sizes the node map, -1 is unknown.
    pub(super) fn new(reader: &'r R, nb_facets: i32) -> Self {
        let mut tool = MergeNodesTool::new(reader.merge_angle(), 0.0, nb_facets);
        // The final triangulation belongs to the reader, so the tool keeps only its merge map.
        *tool.change_output() = Default::default();
        tool.set_merge_angle(reader.merge_angle());
        tool.set_merge_tolerance(reader.merge_tolerance());
        Self {
            tool,
            reader,
            node_index_map: HashMap::new(),
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
    fn remapped_node(&mut self, slot: usize, nodes: &[Xyz; 3]) -> i32 {
        let merged = self.tool.element_node_index(slot);
        let reader = self.reader;
        *self
            .node_index_map
            .entry(merged)
            .or_insert_with(|| reader.add_node(&nodes[slot]))
    }
}
