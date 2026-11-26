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
