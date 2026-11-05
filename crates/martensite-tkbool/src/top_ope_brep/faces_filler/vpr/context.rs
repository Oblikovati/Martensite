//! Captures the two incident edges without retaining data-structure lock guards.
use super::TopOpeBRepFacesFiller;
use crate::top_ope_brep::TopOpeBRepVPointInter;
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;

pub(super) struct Incidence {
    pub edge: Edge,
    pub parameter: f64,
    pub face: Face,
    pub face_index: i32,
    pub edge_index: i32,
    pub section: bool,
    pub closing: bool,
    pub degenerate: bool,
    pub vertex: bool,
}

impl Incidence {
    fn new(
        filler: &TopOpeBRepFacesFiller<'_>,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        on: bool,
    ) -> Result<Self, Failure> {
        let face = filler.face(side)?.clone();
        let edge = topods::edge(&if on {
            point.edge_on(side)?.clone()
        } else {
            point.edge(side)?
        })?;
        let parameter = if on {
            point.edge_on_parameter(side)?
        } else {
            point.edge_parameter(side)?
        };
        let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
        let face_index = storage.change_ds().add_shape_with_rank(&face, side);
        let edge_index = storage.ds().shape_index(&edge, true);
        let section = storage.ds().is_section_edge(&edge, true);
        let closing = TopOpeBRepToolShapeTool::closed(&edge, &face)?;
        let degenerate = BRepTool::degenerated(&edge)?;
        let vertex = point.is_vertex(side)?;
        Ok(Self {
            edge,
            parameter,
            face,
            face_index,
            edge_index,
            section,
            closing,
            degenerate,
            vertex,
        })
    }

    pub(super) fn ensure_index(
        &mut self,
        filler: &TopOpeBRepFacesFiller<'_>,
        side: i32,
    ) -> Result<(), Failure> {
        if self.edge_index == 0 {
            self.edge_index = filler
                .change_data_structure()?
                .add_shape_with_rank(&self.edge, side);
        }
        Ok(())
    }
}

pub(super) struct Context {
    pub side: i32,
    pub other_side: i32,
    pub current: Incidence,
    pub other: Option<Incidence>,
    pub on_two: bool,
    pub on_other: bool,
    pub other_face: Face,
    pub other_face_index: i32,
}

impl Context {
    pub(super) fn new(
        filler: &TopOpeBRepFacesFiller<'_>,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
    ) -> Result<Self, Failure> {
        let other_side = if side == 1 { 2 } else { 1 };
        let on_two = point.shape_index() == 3;
        let on_other = point.state(other_side)? == State::On;
        let current = Incidence::new(filler, point, side, false)?;
        let other = if on_two || on_other {
            Some(Incidence::new(filler, point, other_side, !on_two)?)
        } else {
            None
        };
        let other_face = filler.face(other_side)?.clone();
        let other_face_index = filler
            .change_data_structure()?
            .add_shape_with_rank(&other_face, other_side);
        Ok(Self {
            side,
            other_side,
            current,
            other,
            on_two,
            on_other,
            other_face,
            other_face_index,
        })
    }

    pub(super) fn other_closing(&self) -> bool {
        self.other.as_ref().is_some_and(|entry| entry.closing)
    }
    pub(super) fn other_degenerate(&self) -> bool {
        self.other.as_ref().is_some_and(|entry| entry.degenerate)
    }
    pub(super) fn other_section(&self) -> bool {
        self.other.as_ref().is_some_and(|entry| entry.section)
    }
}
