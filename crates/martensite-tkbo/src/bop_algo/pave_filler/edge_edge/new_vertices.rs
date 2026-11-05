//! `BOPAlgoPaveFiller::PerformNewVertices` and `TreatNewVertices`.
use super::super::{BOPAlgoPaveFiller, missing_structure};
use super::{VertexCouples, progress::EdgeEdgeBoundary};
use crate::bop_algo::tools::{
    PaveBlockIndexLinks, fill_map_one, intersect_vertices, pave_block_index_links,
};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use crate::bop_ds::{CoupleOfPaveBlocks, ShapeInfo};
use crate::bop_tools::make_vertex;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::Arc;

struct GeneratedVertexImages {
    vertices: Vec<(Vertex, Vec<Shape>)>,
    additional: f64,
}

impl BOPAlgoPaveFiller {
    /// Fuses the new intersection vertices, appends them to the DS (setting the new index of
    /// their EE or EF interferences) and puts them on their pave blocks through IntersectVE.
    /// Runs the actual new-vertex loops and forwards their remaining half to IntersectVE.
    /// Example: `filler.perform_new_vertices_with_progress(couples, true, range, observed)?`.
    pub(in crate::bop_algo::pave_filler) fn perform_new_vertices_with_progress(
        &mut self,
        couples: &mut VertexCouples,
        edge_edge: bool,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        if couples.extent() == 0 {
            return Ok(Vec::new());
        }
        let images = self.generated_vertex_images(couples)?;
        let scope = ProgressScope::unnamed(range, 2.0, false);
        let boundary = EdgeEdgeBoundary {
            scope: &scope,
            observed,
        };
        self.store_generated_vertices(images, couples, edge_edge, &boundary)
    }

    fn generated_vertex_images(
        &self,
        couples: &VertexCouples,
    ) -> Result<GeneratedVertexImages, Failure> {
        // Capture the native addition before TreatNewVertices, not after progress callbacks.
        let additional = self.options.fuzzy_value() / 2.;
        let vertices = self.treat_new_vertices(couples)?;
        Ok(GeneratedVertexImages {
            vertices,
            additional,
        })
    }

    fn store_generated_vertices(
        &mut self,
        images: GeneratedVertexImages,
        couples: &mut VertexCouples,
        edge_edge: bool,
        boundary: &EdgeEdgeBoundary<'_, '_>,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let range = boundary.scope.next(1.0);
        let count = images.vertices.len() + couples.extent();
        let scope = ProgressScope::unnamed(&range, count as f64, false);
        if !self.insert_generated_images(images, couples, edge_edge, boundary, &scope)? {
            return Ok(Vec::new());
        }
        let Some(blocks) = self.generated_blocks_with_progress(couples, boundary, &scope)? else {
            return Ok(Vec::new());
        };
        let range = boundary.scope.next(1.0);
        self.intersect_ve_with_progress(&blocks, false, &range, boundary.observed)
    }

    fn insert_generated_images(
        &mut self,
        images: GeneratedVertexImages,
        couples: &mut VertexCouples,
        edge_edge: bool,
        boundary: &EdgeEdgeBoundary<'_, '_>,
        scope: &ProgressScope<'_>,
    ) -> Result<bool, Failure> {
        for (vertex, sources) in images.vertices {
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
            self.append_generated_vertex(&vertex, &sources, images.additional, couples, edge_edge)?;
            drop(scope.next(1.0));
        }
        Ok(true)
    }

    fn generated_blocks_with_progress(
        &self,
        couples: &VertexCouples,
        boundary: &EdgeEdgeBoundary<'_, '_>,
        scope: &ProgressScope<'_>,
    ) -> Result<Option<PaveBlockIndexLinks>, Failure> {
        let mut blocks = pave_block_index_links();
        for position in 1..=couples.extent() {
            if boundary.cancelled(&self.options) {
                return Ok(None);
            }
            append_generated_blocks(couples.find_from_index(position)?, &mut blocks);
            drop(scope.next(1.0));
        }
        Ok(Some(blocks))
    }

    fn append_generated_vertex(
        &mut self,
        vertex: &Vertex,
        sources: &[Shape],
        additional: f64,
        couples: &mut VertexCouples,
        edge_edge: bool,
    ) -> Result<(), Failure> {
        let index = self.append_new_vertex(vertex, additional)?;
        for source in sources {
            let couple = couples.change_from_key(source)?;
            couple.set_index(index);
            let interference = couple.interference_index() as usize;
            self.set_generated_vertex_index(edge_edge, interference, index)?;
        }
        Ok(())
    }

    fn set_generated_vertex_index(
        &mut self,
        edge_edge: bool,
        interference: usize,
        index: i32,
    ) -> Result<(), Failure> {
        let structure = self.structure_mut()?;
        if edge_edge {
            structure.interf_ee()[interference].set_index_new(index);
        } else {
            structure.interf_ef()[interference].set_index_new(index);
        }
        Ok(())
    }

    /// Groups the new vertices whose tolerances interfere and makes one vertex per group;
    /// returns each made vertex with the source vertices it replaces (`TreatNewVertices`).
    fn treat_new_vertices(
        &self,
        couples: &VertexCouples,
    ) -> Result<Vec<(Vertex, Vec<Shape>)>, Failure> {
        let mut vertices: Vec<(Shape, f64)> = Vec::with_capacity(couples.extent());
        for position in 1..=couples.extent() {
            let shape = couples.find_key(position)?.clone();
            vertices.push((shape, couples.find_from_index(position)?.tolerance()));
        }
        let mut images = Vec::new();
        for chain in intersect_vertices(&vertices, self.options.fuzzy_value())? {
            let list: List<Shape> = chain.iter().cloned().collect();
            let mut vertex = Vertex::default();
            make_vertex(&list, &mut vertex)?;
            images.push((vertex, chain));
        }
        Ok(images)
    }

    fn append_new_vertex(&mut self, vertex: &Vertex, additional: f64) -> Result<i32, Failure> {
        let structure = self.data_structure.as_mut().ok_or_else(missing_structure)?;
        let mut info = ShapeInfo::default();
        info.set_shape_type(ShapeEnum::Vertex);
        info.set_shape(&vertex.clone().into_shape());
        let index = structure.append_shape_info(&info);
        let point = BRepTool::pnt(vertex)?;
        let gap = BRepTool::vertex_tolerance(vertex)? + additional;
        let bounds = structure.change_shape_info(index)?.change_box();
        bounds.add_point(point);
        bounds.set_gap(gap);
        Ok(index)
    }
}

fn append_generated_blocks(couple: &CoupleOfPaveBlocks, blocks: &mut PaveBlockIndexLinks) {
    let (first, second) = (couple.pave_block1(), couple.pave_block2());
    let (Some(first), Some(second)) = (first, second) else {
        return;
    };
    fill_map_one(blocks, Arc::clone(first), couple.index());
    if !Arc::ptr_eq(first, second) {
        fill_map_one(blocks, Arc::clone(second), couple.index());
    }
}
