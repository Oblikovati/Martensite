//! `BOPAlgoBuilder::FillInternalVertices` (`BOPAlgoBuilder_2`).
//! Collect and classify all alone-vertex candidates before mutating any split face.
use super::{BOPAlgoBuilder, Context, Failure, Shape, required_context};
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressOperation, ProgressStage,
    builder_area::progress::observe_user_break,
};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Face, Vertex},
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
type InternalVertexPairs = Vec<(Shape, Shape)>;

impl BOPAlgoBuilder {
    /// Puts the alone vertices of each face into the splits that contain them, as INTERNAL
    /// vertices (`FillInternalVertices`); `builder.fill_internal_vertices(&ds)?`.
    pub fn fill_internal_vertices(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        required_context(&self.context)?;
        self.fill_internal_vertices_progress_raw(
            structure,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    /// Runs the actual ranged stage; `builder.fill_internal_vertices_progress_raw(ds, range, observed)?`.
    /// Raw mutations are stage diagnostics and do not certify a published operation result.
    pub(crate) fn fill_internal_vertices_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let outer = ProgressScope::unnamed(range, 1.0, false);
        if let Some(pairs) = self.collect_internal_vertex_pairs(structure, &outer, observed)? {
            self.execute_internal_vertex_pairs(pairs, &outer, observed)?;
        }
        Ok(internal_vertex_completion(observed))
    }

    fn execute_internal_vertex_pairs(
        &self,
        pairs: InternalVertexPairs,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = outer.next(1.0);
        let assigning = ProgressScope::with_literal(
            &range,
            "Looking for internal shapes",
            pairs.len() as f64,
            false,
        );
        // All pairs are classified before any vertex is added, as in the native parallel run.
        let inside = workers::classify_pairs(
            pairs,
            &self.context,
            self.options().fuzzy_value(),
            self.options().run_parallel(),
            &assigning,
            observed,
        )?;
        if observe_user_break(self.options(), outer, observed) {
            return Ok(());
        }
        insert_internal_vertices(inside)
    }

    fn collect_internal_vertex_pairs(
        &self,
        structure: &DataStructure,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<InternalVertexPairs>, Failure> {
        let mut pairs = Vec::new();
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() != ShapeEnum::Face {
                continue;
            }
            if observe_user_break(self.options(), scope, observed) {
                return Ok(None);
            }
            let Some(splits) = self.images.seek(info.shape()) else {
                continue;
            };
            append_internal_vertex_pairs(structure, index, splits, &mut pairs)?;
        }
        Ok(Some(pairs))
    }
}

fn append_internal_vertex_pairs(
    structure: &DataStructure,
    face: i32,
    splits: &List<Shape>,
    pairs: &mut InternalVertexPairs,
) -> Result<(), Failure> {
    for vertex in structure.alone_vertices(face)? {
        let vertex = structure.shape(vertex)?.oriented(Orientation::Internal);
        for split in splits.iter() {
            pairs.push((vertex.clone(), split.clone()));
        }
    }
    Ok(())
}

fn internal_vertex_is_inside(
    vertex: &Shape,
    split: &Shape,
    context: &Context,
    fuzzy: f64,
) -> Result<bool, Failure> {
    let (mut u, mut v, mut tolerance) = (0., 0., 0.);
    let flag = context.compute_vf(
        &Vertex::unchecked(vertex.clone()),
        &Face::unchecked(split.clone()),
        &mut u,
        &mut v,
        &mut tolerance,
        fuzzy,
    )?;
    Ok(flag == 0)
}

fn insert_internal_vertices(inside: InternalVertexPairs) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    for (vertex, mut split) in inside {
        builder.add(&mut split, &vertex)?;
    }
    Ok(())
}

fn internal_vertex_completion(observed: &ProgressCancellation) -> ProgressCompletion {
    if !observed.is_cancelled() {
        return ProgressCompletion::Completed;
    }
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        context: None,
    }
}
#[path = "internal_vertices/workers.rs"]
mod workers;
