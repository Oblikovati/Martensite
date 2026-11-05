//! Native collection polls precede each FF record, sorted face, and edge set.
use super::{
    BOPAlgoBuilder, ConnectionMap, DataStructure, EdgeSetFaces, Failure, IndexedDataMap, Map,
    SameDomainShapes, Shape, ShapeMapHasher, intersected_faces, link_planar_pairs,
};
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use martensite_tkernel::message::ProgressScope;

/// Ordered adjacency from actual candidate input order; `SameDomainLinks::with_hasher(...)`.
pub(super) type SameDomainLinks = ConnectionMap<Shape, Shape, ShapeMapHasher>;
type CandidateSets = (SameDomainShapes, EdgeSetFaces, Map<Shape, ShapeMapHasher>);
type CollectedSets = (EdgeSetFaces, Map<Shape, ShapeMapHasher>);

/// Retain native graph and input pair order; `prepared.pairs` is never coordinate-ranked.
pub(super) struct SameDomainPreparation {
    pub(super) links: SameDomainLinks,
    pub(super) pairs: Vec<(Shape, Shape)>,
}

/// Collect only actual candidate shapes; `prepare(builder, ds, scope, observed)?`.
pub(super) fn prepare(
    builder: &BOPAlgoBuilder,
    structure: &DataStructure,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Option<SameDomainPreparation>, Failure> {
    let parents = builder.face_parents(structure)?;
    let Some(indices) = intersected_faces(structure, || poll(builder, scope, observed))? else {
        return Ok(None);
    };
    let Some((sets, planar)) = collect_sets(builder, structure, indices, scope, observed)? else {
        return Ok(None);
    };
    collect_pairs(builder, (parents, sets, planar), scope, observed)
}

fn collect_sets(
    builder: &BOPAlgoBuilder,
    structure: &DataStructure,
    indices: Vec<i32>,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Option<CollectedSets>, Failure> {
    let mut sets = IndexedDataMap::with_hasher(
        super::occurrence_keys::EdgeOccurrencePolicy(builder.options().face_grouping_version()),
        1,
    );
    let mut planar = Map::with_hasher(ShapeMapHasher, 1);
    for index in indices {
        if poll(builder, scope, observed) {
            return Ok(None);
        }
        builder.add_source_face_sets(structure, index, &mut sets, &mut planar)?;
    }
    Ok(Some((sets, planar)))
}

fn collect_pairs(
    builder: &BOPAlgoBuilder,
    (parents, sets, planar): CandidateSets,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<Option<SameDomainPreparation>, Failure> {
    let mut prepared = SameDomainPreparation {
        links: IndexedDataMap::with_hasher(ShapeMapHasher, 1),
        pairs: Vec::new(),
    };
    for position in 1..=sets.extent() {
        if poll(builder, scope, observed) {
            return Ok(None);
        }
        prepared.append_set(sets.find_from_index(position)?, &planar, &parents);
    }
    Ok(Some(prepared))
}

fn poll(
    builder: &BOPAlgoBuilder,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> bool {
    // Only these native collection sites observe parent UserBreak; no geometry query gets a range.
    observe_user_break(builder.options(), scope, observed)
}

impl SameDomainPreparation {
    fn append_set(
        &mut self,
        faces: &[Shape],
        planar: &Map<Shape, ShapeMapHasher>,
        parents: &SameDomainShapes,
    ) {
        link_planar_pairs(faces, planar, parents, &mut self.links, &mut self.pairs);
    }
}
