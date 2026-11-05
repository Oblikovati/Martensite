//! Degenerate restriction vertices carry line-local treatment state in the operation.
use super::vpr::GeometrySearch;
use super::*;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSTransition, process_interferences_tool::make_epv_interference_sk,
};
use crate::top_ope_brep_tool::{TopOpeBRepToolMkTondgE, TopOpeBRepToolShapeTool};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{Edge, Vertex},
    toptools::{IndexedDataMapOfShapeListOfShape, ShapeMapHasher},
};
use martensite_tkernel::ncollection::{DataMap, List};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

type ShapeEdges = DataMap<Shape, List<Shape>, ShapeMapHasher>;
type ShapeRanks = DataMap<Shape, i32, ShapeMapHasher>;
type VertexPeers = DataMap<Shape, Shape, ShapeMapHasher>;

#[derive(Default)]
pub(super) struct DegenerateContext {
    entries: ShapeEdges,
    treated: ShapeEdges,
    line_identity: Option<crate::top_ope_brep::line_inter::LineInterRecordIdentity>,
}

struct DegenerateCrossing {
    rank: i32,
    edge: Edge,
    transitions: (i32, f64, f64),
    planar: bool,
}

mod collect;
mod crossing;
mod treatment;

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn reset_degenerate_context(&mut self) {
        self.degenerate_context = DegenerateContext::default();
    }

    /// Enriches pole/seam incidences and links vertices shared by both operands.
    pub(super) fn initialize_degenerate_edges(
        &mut self,
        line: &TopOpeBRepLineInter,
    ) -> Result<(), Failure> {
        let mut entries = std::mem::take(&mut self.degenerate_context.entries);
        let result = self.collect_degenerate_data(line, &mut entries);
        self.degenerate_context.entries = entries;
        result?;
        self.fill_vertices_on_both_operands(line)
    }

    pub(super) fn fill_vertices_on_both_operands(
        &self,
        line: &TopOpeBRepLineInter,
    ) -> Result<(), Failure> {
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?;
            if point.shape_index() == 3 && point.is_vertex_on_s1()? && point.is_vertex_on_s2()? {
                self.change_data_structure()?.fill_shapes_same_domain(
                    &point.vertex_on_s1()?,
                    &point.vertex_on_s2()?,
                    true,
                )?;
            }
        }
        Ok(())
    }

    /// Supplies degenerate-edge transitions and shared point geometry; `filler.process_vpondg_e(&point, 1, &mut line, &mut search)?`.
    /// Returns false when no pole transition is found, including a completed empty search.
    /// Treatment is per record: moves and repeated loads retain it; a newly created replacement resets it.
    pub fn process_vpondg_e(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        line: &mut TopOpeBRepLineInter,
        search: &mut GeometrySearch,
    ) -> Result<bool, Failure> {
        if search.index == 0 {
            self.find_vp_geometry(line, point, side, 0, search)?;
        }
        if self.degenerate_context.entries.is_empty() {
            return Ok(false);
        }
        self.degenerate_context.reduce_treated(line);
        let Some(crossing) = self.degenerate_crossing(point)? else {
            return Ok(false);
        };
        if search.index == 0 {
            self.find_vp_geometry(line, point, side, 1, search)?;
        }
        self.store_degenerate_crossing(&crossing, search.index)?;
        Ok(true)
    }

    fn store_degenerate_crossing(
        &self,
        crossing: &DegenerateCrossing,
        geometry: i32,
    ) -> Result<(), Failure> {
        let opposite = if crossing.rank == 1 { 2 } else { 1 };
        let mut ds = self.change_data_structure()?;
        let face_index =
            ds.add_shape_with_rank(self.faces[(opposite - 1) as usize].as_ref(), opposite);
        ds.add_shape_with_rank(crossing.edge.as_ref(), crossing.rank);
        let rank = ds.ancestor_rank(ds.shape(geometry, true));
        drop(ds);
        if crossing.planar {
            return Ok(());
        }
        for (mask, orientation, parameter, vertex_rank) in [
            (1, Orientation::Forward, crossing.transitions.1, 1),
            (2, Orientation::Reversed, crossing.transitions.2, 2),
        ] {
            if crossing.transitions.0 & mask != 0 {
                self.store_degenerate_transition(
                    crossing,
                    geometry,
                    face_index,
                    orientation,
                    parameter,
                    rank == vertex_rank,
                )?;
            }
        }
        Ok(())
    }

    fn store_degenerate_transition(
        &self,
        crossing: &DegenerateCrossing,
        geometry: i32,
        face_index: i32,
        orientation: Orientation,
        parameter: f64,
        is_vertex: bool,
    ) -> Result<(), Failure> {
        let mut transition = TopOpeBRepDSTransition::from_orientation(orientation);
        transition.set_index(face_index);
        let interference = make_epv_interference_sk(
            &transition,
            face_index,
            geometry,
            parameter,
            TopOpeBRepDSKind::Vertex,
            TopOpeBRepDSKind::Face,
            is_vertex,
        )?;
        self.hds
            .get()
            .ok_or_else(missing_storage)?
            .store_interference(&interference, crossing.edge.as_ref())
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod binding_tests;
