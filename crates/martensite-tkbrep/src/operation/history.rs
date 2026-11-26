// SPDX-License-Identifier: GPL-2.0-only
//! Unified many-to-many history relation vocabulary (#18).
use crate::brep_tools::History;
use crate::topexp;
use crate::topods::Shape;
use crate::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;

/// Semantic provenance relation between parent and product entities (#18).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HistoryRelationKind {
    /// New entity created without a direct topological parent.
    Created,
    /// One parent split into two or more product entities.
    Split,
    /// Two or more parent entities merged into one product entity.
    Merged,
    /// Entity derived from one or more parents (1-to-1 modification or generation).
    Derived,
    /// Parent entity removed with no surviving product.
    Deleted,
}

/// One many-to-many provenance relation entry (#18).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct HistoryRelation {
    kind: HistoryRelationKind,
    parents: Vec<Shape>,
    products: Vec<Shape>,
    non_unique: bool,
}

impl HistoryRelation {
    /// Relation classification.
    pub fn kind(&self) -> HistoryRelationKind {
        self.kind
    }

    /// Contributing parent shapes in deterministic order.
    pub fn parents(&self) -> &[Shape] {
        &self.parents
    }

    /// Resulting product shapes in deterministic order.
    pub fn products(&self) -> &[Shape] {
        &self.products
    }

    /// True when the mapping is non-unique (e.g., 1-to-many split/generation or many-to-1 merge).
    pub fn is_non_unique(&self) -> bool {
        self.non_unique
    }
}

/// Immutable, deterministic many-to-many operation history graph (#18).
///
/// ```
/// use martensite_tkbrep::operation::HistoryGraph;
/// let graph = HistoryGraph::empty();
/// assert!(graph.is_empty());
/// ```
#[derive(Clone, Debug, Default)]
pub struct HistoryGraph {
    relations: Vec<HistoryRelation>,
}

impl HistoryGraph {
    /// Returns an empty history graph.
    pub fn empty() -> Self {
        Self {
            relations: Vec::new(),
        }
    }

    /// Starts an explicit builder for constructing a [`HistoryGraph`].
    pub fn builder() -> HistoryGraphBuilder {
        HistoryGraphBuilder::default()
    }

    /// Translates legacy `BRepTools_History` lists for `arguments` into the unified
    /// many-to-many relation vocabulary in deterministic `TopExp::MapShapes` order (#18).
    pub fn from_brep_history(arguments: &[Shape], history: &History) -> Result<Self, Failure> {
        let mut parents_map = IndexedMapOfShape::default();
        for argument in arguments.iter().filter(|s| !s.is_null()) {
            topexp::map_shapes(argument, &mut parents_map, true, true);
        }
        let mut builder = HistoryGraphBuilder::default();
        let mut mod_entries: Vec<(Shape, Vec<Shape>)> = Vec::new();
        let mut gen_entries: Vec<(Shape, Vec<Shape>)> = Vec::new();
        for index in 1..=parents_map.extent() {
            let parent = parents_map.find_key(index)?.clone();
            if !History::is_supported_type(&parent)? {
                continue;
            }
            if history.is_removed(&parent)? {
                builder.record_deleted(parent.clone());
            }
            let modified = dedup_shapes(history.modified(&parent)?);
            if !modified.is_empty() {
                mod_entries.push((parent.clone(), modified));
            }
            let generated = dedup_shapes(history.generated(&parent)?);
            if !generated.is_empty() {
                gen_entries.push((parent, generated));
            }
        }
        classify_modifications(&mut builder, &mod_entries);
        classify_generations(&mut builder, &gen_entries);
        let null_generated = dedup_shapes(history.generated(&Shape::default())?);
        for product in null_generated {
            builder.record_created(product);
        }
        Ok(builder.build())
    }

    /// All recorded relations in deterministic order.
    pub fn relations(&self) -> &[HistoryRelation] {
        &self.relations
    }

    /// Number of recorded relations.
    pub fn len(&self) -> usize {
        self.relations.len()
    }

    /// Whether no relations were recorded.
    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    /// Filters relations by [`HistoryRelationKind`].
    pub fn by_kind(&self, kind: HistoryRelationKind) -> Vec<&HistoryRelation> {
        self.relations.iter().filter(|r| r.kind == kind).collect()
    }

    /// Returns all relations referencing `parent` (via `Shape::is_same`).
    pub fn relations_for_parent(&self, parent: &Shape) -> Vec<&HistoryRelation> {
        self.relations
            .iter()
            .filter(|r| r.parents.iter().any(|p| p.is_same(parent)))
            .collect()
    }

    /// Returns all relations producing `product` (via `Shape::is_same`).
    pub fn relations_for_product(&self, product: &Shape) -> Vec<&HistoryRelation> {
        self.relations
            .iter()
            .filter(|r| r.products.iter().any(|q| q.is_same(product)))
            .collect()
    }

    /// Composes two sequential operation history graphs (`first` followed by `second`)
    /// into a single deterministic many-to-many history graph (#19).
    pub fn compose(first: &Self, second: &Self) -> Self {
        let mut builder = HistoryGraphBuilder::default();
        for relation in &first.relations {
            match relation.kind {
                HistoryRelationKind::Deleted => {
                    for parent in &relation.parents {
                        builder.record_deleted(parent.clone());
                    }
                }
                _ => {
                    let mut final_products = Vec::new();
                    let mut any_deleted = false;
                    for mid in &relation.products {
                        let next_rels = second.relations_for_parent(mid);
                        if next_rels.is_empty() {
                            push_unique(&mut final_products, mid.clone());
                        } else {
                            for next in next_rels {
                                if next.kind == HistoryRelationKind::Deleted {
                                    any_deleted = true;
                                } else {
                                    for prod in &next.products {
                                        push_unique(&mut final_products, prod.clone());
                                    }
                                }
                            }
                        }
                    }
                    if final_products.is_empty() && any_deleted {
                        for parent in &relation.parents {
                            builder.record_deleted(parent.clone());
                        }
                    } else if !final_products.is_empty() {
                        if relation.parents.is_empty() {
                            for prod in final_products {
                                builder.record_created(prod);
                            }
                        } else if relation.parents.len() == 1 && final_products.len() > 1 {
                            builder.record_split(relation.parents[0].clone(), final_products);
                        } else if relation.parents.len() > 1 && final_products.len() == 1 {
                            builder.record_merged(
                                relation.parents.clone(),
                                final_products.remove(0),
                            );
                        } else {
                            let non_unique =
                                relation.parents.len() > 1 || final_products.len() > 1;
                            builder.record_derived(
                                relation.parents.clone(),
                                final_products,
                                non_unique,
                            );
                        }
                    }
                }
            }
        }
        // Carry over newly created entities from `second` that have no parent in `first`.
        for relation in &second.relations {
            if relation.kind == HistoryRelationKind::Created {
                for prod in &relation.products {
                    builder.record_created(prod.clone());
                }
            }
        }
        builder.build()
    }

    /// Evaluates whether every supported topological entity in `result_shape` is covered
    /// by at least one product entry in this history graph (#19).
    pub fn coverage_report(&self, result_shape: &Shape) -> Result<HistoryCoverageReport, Failure> {
        let mut entities = IndexedMapOfShape::default();
        if !result_shape.is_null() {
            topexp::map_shapes(result_shape, &mut entities, true, true);
        }
        let mut total_result_entities = 0usize;
        let mut covered_entities = 0usize;
        let mut uncovered_entities = Vec::new();
        for index in 1..=entities.extent() {
            let entity = entities.find_key(index)?.clone();
            if !History::is_supported_type(&entity)? {
                continue;
            }
            total_result_entities += 1;
            if self.relations_for_product(&entity).is_empty() {
                uncovered_entities.push(entity);
            } else {
                covered_entities += 1;
            }
        }
        Ok(HistoryCoverageReport {
            total_result_entities,
            covered_entities,
            uncovered_entities,
        })
    }
}

/// Strict coverage audit of a [`HistoryGraph`] against a result shape (#19).
#[derive(Clone, Debug)]
pub struct HistoryCoverageReport {
    total_result_entities: usize,
    covered_entities: usize,
    uncovered_entities: Vec<Shape>,
}

impl HistoryCoverageReport {
    /// Total supported entities inspected on the result shape.
    pub fn total_result_entities(&self) -> usize {
        self.total_result_entities
    }

    /// Count of result entities with at least one provenance relation.
    pub fn covered_entities(&self) -> usize {
        self.covered_entities
    }

    /// Result entities missing provenance coverage.
    pub fn uncovered_entities(&self) -> &[Shape] {
        &self.uncovered_entities
    }

    /// True when every supported result entity is covered.
    pub fn is_complete(&self) -> bool {
        self.uncovered_entities.is_empty()
    }

    /// Refuses incomplete history coverage with a structured [`super::OperationError`] (#19).
    pub fn require_complete(
        self,
        operation: super::OperationKind,
    ) -> Result<(), super::OperationError> {
        if self.is_complete() {
            return Ok(());
        }
        Err(super::OperationError::kernel_failure(
            operation,
            format!(
                "incomplete history coverage: {}/{} entities covered ({} uncovered)",
                self.covered_entities,
                self.total_result_entities,
                self.uncovered_entities.len()
            ),
        ))
    }
}

fn push_unique(list: &mut Vec<Shape>, shape: Shape) {
    if !list.iter().any(|existing| existing.is_same(&shape)) {
        list.push(shape);
    }
}

/// Operation-local builder for [`HistoryGraph`] (#18).
#[derive(Clone, Debug, Default)]
pub struct HistoryGraphBuilder {
    relations: Vec<HistoryRelation>,
}

impl HistoryGraphBuilder {
    /// Records a newly created entity with no direct parent.
    pub fn record_created(&mut self, product: Shape) -> &mut Self {
        self.relations.push(HistoryRelation {
            kind: HistoryRelationKind::Created,
            parents: Vec::new(),
            products: vec![product],
            non_unique: false,
        });
        self
    }

    /// Records a split from one `parent` into multiple `products` (marked `non_unique`).
    pub fn record_split(&mut self, parent: Shape, products: Vec<Shape>) -> &mut Self {
        self.relations.push(HistoryRelation {
            kind: HistoryRelationKind::Split,
            parents: vec![parent],
            products,
            non_unique: true,
        });
        self
    }

    /// Records a merge of multiple `parents` into one `product` (marked `non_unique`).
    pub fn record_merged(&mut self, parents: Vec<Shape>, product: Shape) -> &mut Self {
        self.relations.push(HistoryRelation {
            kind: HistoryRelationKind::Merged,
            parents,
            products: vec![product],
            non_unique: true,
        });
        self
    }

    /// Records a derived entity relation; marks `non_unique` when `parents.len() > 1 || products.len() > 1 || non_unique`.
    pub fn record_derived(
        &mut self,
        parents: Vec<Shape>,
        products: Vec<Shape>,
        non_unique: bool,
    ) -> &mut Self {
        let non_unique = non_unique || parents.len() > 1 || products.len() > 1;
        self.relations.push(HistoryRelation {
            kind: HistoryRelationKind::Derived,
            parents,
            products,
            non_unique,
        });
        self
    }

    /// Records a deleted parent entity.
    pub fn record_deleted(&mut self, parent: Shape) -> &mut Self {
        self.relations.push(HistoryRelation {
            kind: HistoryRelationKind::Deleted,
            parents: vec![parent],
            products: Vec::new(),
            non_unique: false,
        });
        self
    }

    /// Freezes the builder into an immutable [`HistoryGraph`].
    pub fn build(self) -> HistoryGraph {
        HistoryGraph {
            relations: self.relations,
        }
    }
}

fn dedup_shapes(list: Vec<Shape>) -> Vec<Shape> {
    let mut out = Vec::new();
    for shape in list {
        if !shape.is_null() && !out.iter().any(|existing: &Shape| existing.is_same(&shape)) {
            out.push(shape);
        }
    }
    out
}

fn classify_modifications(builder: &mut HistoryGraphBuilder, entries: &[(Shape, Vec<Shape>)]) {
    // Map each product to all modifying parents in deterministic insertion order.
    let mut product_parents: Vec<(Shape, Vec<Shape>)> = Vec::new();
    for (parent, products) in entries {
        for product in products {
            if let Some((_, parents)) = product_parents
                .iter_mut()
                .find(|(existing, _)| existing.is_same(product))
            {
                if !parents.iter().any(|p| p.is_same(parent)) {
                    parents.push(parent.clone());
                }
            } else {
                product_parents.push((product.clone(), vec![parent.clone()]));
            }
        }
    }
    let mut merged_products: Vec<Shape> = Vec::new();
    for (product, parents) in &product_parents {
        if parents.len() > 1 {
            builder.record_merged(parents.clone(), product.clone());
            merged_products.push(product.clone());
        }
    }
    for (parent, products) in entries {
        let remaining: Vec<Shape> = products
            .iter()
            .filter(|q| !merged_products.iter().any(|m| m.is_same(q)))
            .cloned()
            .collect();
        if remaining.len() > 1 {
            builder.record_split(parent.clone(), remaining);
        } else if remaining.len() == 1 {
            builder.record_derived(vec![parent.clone()], remaining, false);
        }
    }
}

fn classify_generations(builder: &mut HistoryGraphBuilder, entries: &[(Shape, Vec<Shape>)]) {
    for (parent, products) in entries {
        builder.record_derived(vec![parent.clone()], products.clone(), products.len() > 1);
    }
}

#[cfg(test)]
mod compose_and_coverage_tests {
    use super::*;
    use crate::brep::BRepBuilder;
    use crate::operation::OperationKind;
    use crate::topods::Vertex;

    fn make_v() -> Shape {
        let mut v = Vertex::default();
        BRepBuilder::default().make_vertex(&mut v);
        v.into_shape()
    }

    #[test]
    fn compose_chains_split_and_delete_and_strict_coverage_refuses_gaps() {
        let a = make_v();
        let b1 = make_v();
        let b2 = make_v();
        let c1 = make_v();
        let uncovered = make_v();

        let mut g1 = HistoryGraph::builder();
        g1.record_split(a.clone(), vec![b1.clone(), b2.clone()]);
        let mut g2 = HistoryGraph::builder();
        g2.record_derived(vec![b1], vec![c1.clone()], false);
        g2.record_deleted(b2);

        let composed = HistoryGraph::compose(&g1.build(), &g2.build());
        assert_eq!(composed.relations_for_product(&c1).len(), 1);
        assert!(composed.relations_for_product(&c1)[0].parents()[0].is_same(&a));

        let covered_report = composed.coverage_report(&c1).expect("coverage report");
        assert!(covered_report.is_complete());
        covered_report
            .require_complete(OperationKind::Boolean)
            .expect("complete coverage accepted");

        let gap_report = composed.coverage_report(&uncovered).expect("gap report");
        assert!(!gap_report.is_complete());
        assert!(gap_report.require_complete(OperationKind::Boolean).is_err());
    }
}
