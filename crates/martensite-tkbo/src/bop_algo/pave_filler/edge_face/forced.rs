//! `BOPAlgoPaveFiller_5` forced intersections: ForceInterfVF, ReduceIntersectionRange and
//! both ForceInterfEF overloads (edge/face common blocks after vertex growth).
use super::super::vertex_vertex::pair_compound;
use super::super::{BOPAlgoPaveFiller, missing_structure};
use super::EdgeFaceTask;
use crate::bop_algo::AlertSelfInterferingShape;
use crate::bop_algo::tools::{
    fill_map_one, pave_block_index_links, perform_common_blocks_on_faces,
};
use crate::bop_ds::{
    FaceInfo, InterfEF, InterfVF, PaveBlockHandle, PaveBlockIndexMap, VertexIndexSet,
};
use crate::bop_tools::algo_tools2d::intermediate_point;
use crate::bop_tools::{BoxSelector, BoxSet};
use crate::int_tools::Range;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{Edge, Face, Vertex};
use martensite_tkernel::message::AlertHandle;
use martensite_tkernel::ncollection::Map;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::box3_to_bvh;
use martensite_tkmath::bvh::{BvhSetTraverse, BvhVec3d};
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};
use martensite_tkmath::gp::{self, Point2, Vector3};
use std::sync::Arc;

impl BOPAlgoPaveFiller {
    /// Classifies vertex `vertex` on face `face` (accepting ComputeVF status 0 and -2) and
    /// records the VF interference and In vertex; `let on = filler.force_interf_vf(v, f)?;`.
    pub(in crate::bop_algo::pave_filler) fn force_interf_vf(
        &mut self,
        vertex: i32,
        face: i32,
    ) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let vertex_shape = Vertex::unchecked(structure.shape(vertex)?.clone());
        let face_shape = Face::unchecked(structure.shape(face)?.clone());
        let (mut u, mut v, mut tolerance) = (0., 0., 0.);
        let fuzzy = self.options.fuzzy_value();
        let status = self.context_ref()?.compute_vf(
            &vertex_shape,
            &face_shape,
            &mut u,
            &mut v,
            &mut tolerance,
            fuzzy,
        )?;
        if status != 0 && status != -2 {
            return Ok(false);
        }
        let structure = self.structure_mut()?;
        let position = structure.interf_vf_records().len();
        let record = structure.interf_vf().append(InterfVF::default());
        record.set_indices(vertex, face);
        record.set_uv(u, v);
        structure.add_interference(vertex, face);
        let updated = self.update_vertex(vertex, tolerance)?;
        let structure = self.structure_mut()?;
        if structure.is_new_shape(updated) {
            structure.interf_vf()[position].set_index_new(updated);
        }
        structure
            .change_face_info(face)?
            .change_vertices_in()
            .add(updated);
        let rank = structure.rank(vertex);
        if rank >= 0 && rank == structure.rank(face) {
            let pair = pair_compound(&vertex_shape.into_shape(), &face_shape.into_shape())?;
            let alert: AlertHandle = Arc::new(AlertSelfInterferingShape::new(&pair));
            self.options.add_warning(&alert);
        }
        Ok(true)
    }

    /// Shrinks the EF intersection range by the EE ranges of the new end vertices met on the
    /// face's edges (`ReduceIntersectionRange`).
    pub(in crate::bop_algo::pave_filler) fn reduce_intersection_range(
        &self,
        (vertex1, vertex2): (i32, i32),
        edge: i32,
        face: i32,
        first: &mut f64,
        last: &mut f64,
    ) -> Result<(), Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        if !structure.is_new_shape(vertex1) && !structure.is_new_shape(vertex2) {
            return Ok(());
        }
        if !structure.has_interference_shape_subshapes(edge, face)?
            || structure.interf_ee_records().is_empty()
        {
            return Ok(());
        }
        let mut face_edges: Map<i32> = Map::new(1);
        for sub_shape in structure.shape_info(face)?.sub_shapes() {
            if structure.shape_info(*sub_shape)?.shape_type() == ShapeEnum::Edge {
                face_edges.add(*sub_shape);
            }
        }
        for record in structure.interf_ee_records().iter() {
            let Some(vertex) = record.get_index_new() else {
                continue;
            };
            if vertex != vertex1 && vertex != vertex2 {
                continue;
            }
            let (edge1, edge2) = record.indices();
            if (edge != edge1 && edge != edge2)
                || (!face_edges.contains(&edge1) && !face_edges.contains(&edge2))
            {
                continue;
            }
            let part = record.common_part();
            let range = if edge == edge1 {
                *part.range1()
            } else {
                part.ranges2()[0]
            };
            if vertex == vertex1 {
                if *first < range.last() {
                    *first = range.last();
                }
            } else if *last > range.first() {
                *last = range.first();
            }
        }
        Ok(())
    }

    /// Collects the real pave blocks of all source edges and checks them against the faces
    /// holding both their vertices (`ForceInterfEF`); primary operations only.
    /// `filler.force_interf_ef()?`.
    pub fn force_interf_ef(&mut self) -> Result<(), Failure> {
        self.force_interf_ef_with_progress(
            &martensite_tkernel::message::ProgressRange::default(),
            &crate::bop_algo::ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    /// Intersects the given pave blocks with the faces containing both their vertices, making
    /// In blocks and (with `add_interf`) EF interferences and face common blocks.
    /// `filler.force_interf_ef_blocks(&blocks, true)?`.
    pub fn force_interf_ef_blocks(
        &mut self,
        blocks: &PaveBlockIndexMap,
        add_interf: bool,
    ) -> Result<(), Failure> {
        self.force_interf_ef_blocks_with_progress(
            blocks,
            add_interf,
            &martensite_tkernel::message::ProgressRange::default(),
            &crate::bop_algo::ProgressCancellation::default(),
        )
        .map(|_| ())
    }

    fn forced_ef_task(
        &self,
        face: i32,
        face_shape: &Face,
        block: &PaveBlockHandle,
        check_mode: bool,
    ) -> Result<Option<EdgeFaceTask>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let (has_edge, split_edge, original, (first, last), shrunk, paves) = {
            let locked = block.lock().expect("pave-block lock");
            let (s0, s1, _, _) = locked.shrunk_data();
            (
                locked.has_edge(),
                locked.edge(),
                locked.original_edge(),
                locked.indices(),
                (s0, s1),
                (locked.pave1().parameter(), locked.pave2().parameter()),
            )
        };
        let edge = if has_edge {
            split_edge
        } else {
            if original < 0 || structure.rank(face) == structure.rank(original) {
                return Ok(None);
            }
            original
        };
        let edge_shape = Edge::unchecked(structure.shape(edge)?.clone());
        let Some(additional) =
            self.forced_ef_tolerance(face_shape, &edge_shape, (first, last), shrunk, check_mode)?
        else {
            return Ok(None);
        };
        let mut intersect = additional > 0.;
        if !intersect {
            intersect = !self
                .fpb_done
                .get(&face)
                .is_some_and(|done| done.iter().any(|known| Arc::ptr_eq(known, block)));
        }
        if !intersect {
            return Ok(None);
        }
        let mut task = EdgeFaceTask::new((edge, face), block, &edge_shape, face_shape);
        let boxes = [
            structure.shape_info(edge)?.box_bounds().clone(),
            structure.shape_info(face)?.box_bounds().clone(),
        ];
        task.configure(
            boxes,
            self.options.fuzzy_value() + additional,
            true,
            Range::new(paves.0, paves.1),
        );
        Ok(Some(task))
    }

    /// The extra fuzzy value of a forced EF pair, or `None` when the pair is skipped: the
    /// middle point must project on the face within twice the vertex tolerance and inside it.
    fn forced_ef_tolerance(
        &self,
        face: &Face,
        edge: &Edge,
        (first, last): (i32, i32),
        shrunk: (f64, f64),
        check_mode: bool,
    ) -> Result<Option<f64>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let context = self.context_ref()?;
        let curve = BRepAdaptorCurve::with_edge(edge)?;
        let (middle, tangent) = curve.d1(intermediate_point(shrunk.0, shrunk.1))?;
        if tangent.square_magnitude() < gp::resolution() {
            return Ok(None);
        }
        let projector = context.proj_ps(face)?;
        let mut projection = projector.write();
        projection.perform(middle)?;
        if projection.nb_points()? == 0 {
            return Ok(None);
        }
        let fuzzy = self.options.fuzzy_value();
        let vertex1 = Vertex::unchecked(structure.shape(first)?.clone());
        let vertex2 = Vertex::unchecked(structure.shape(last)?.clone());
        let check = if check_mode {
            fuzzy
        } else {
            2. * BRepTool::vertex_tolerance(&vertex1)?.max(BRepTool::vertex_tolerance(&vertex2)?)
        };
        if projection.lower_distance()? > check + fuzzy {
            return Ok(None);
        }
        let (u, v) = projection.lower_distance_parameters()?;
        if !context.is_point_in_face(face, Point2::new(u, v))? {
            return Ok(None);
        }
        let mut use_additional = true;
        let surface_type = context.surface_adaptor(face)?.read().get_type()?;
        if surface_type != SurfaceType::Plane || curve.get_type()? != CurveType::Line {
            let on_surface = projection.nearest_point()?;
            let normal = Vector3::between(on_surface, middle);
            if normal.square_magnitude() > gp::resolution() {
                // The tangent must be within 25 degrees of perpendicular to the face normal.
                let cosine = normal.normalized()?.dot(tangent.normalized()?);
                if cosine.abs() > 0.4226 {
                    use_additional = false;
                }
            }
        }
        if !use_additional {
            return Ok(Some(0.));
        }
        let mut additional = 0.0_f64;
        for parameter in [shrunk.0, shrunk.1] {
            projection.perform(curve.value(parameter)?)?;
            if projection.nb_points()? > 0 {
                let distance = projection.lower_distance()?;
                if distance < check && distance > additional {
                    additional = distance;
                }
            }
        }
        if additional > 0. {
            additional -= BRepTool::edge_tolerance(edge)? + BRepTool::face_tolerance(face)?;
            if additional < 0. {
                additional = 0.;
            }
        }
        Ok(Some(additional))
    }
}

/// On, In and Section vertices of the face plus the end vertices of its blocks.
fn face_vertex_union(info: &FaceInfo) -> Map<i32> {
    let mut vertices: Map<i32> = Map::new(1);
    for set in [
        info.vertices_on(),
        info.vertices_in(),
        info.vertices_section(),
    ] {
        add_vertex_set(&mut vertices, set);
    }
    for blocks in [
        info.pave_blocks_on(),
        info.pave_blocks_in(),
        info.pave_blocks_section(),
    ] {
        for block in blocks.iter() {
            let locked = block.lock().expect("pave-block lock");
            vertices.add(locked.pave1().index());
            vertices.add(locked.pave2().index());
        }
    }
    vertices
}

fn add_vertex_set(target: &mut Map<i32>, set: &VertexIndexSet) {
    for vertex in set.iter().copied() {
        target.add(vertex);
    }
}

fn in_face_maps(info: &FaceInfo, block: &PaveBlockHandle) -> bool {
    info.pave_blocks_on().contains(block)
        || info.pave_blocks_in().contains(block)
        || info.pave_blocks_section().contains(block)
}

#[path = "forced/progress.rs"]
mod progress;
