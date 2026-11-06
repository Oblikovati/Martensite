//! Closing-edge interferences retain face and optional opposite-edge supports.
use super::*;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSTransition as Transition,
    export::fds_lo_iinfsup,
    process_interferences_tool::{make_epv_interference, make_epv_interference_sk},
};
use crate::top_ope_brep_tool::{TopOpeBRepToolMakeTransition, fun_tool_bounds, fun_tool_etg_f};
use martensite_tkbrep::topods::Edge;
use martensite_tkg3d::top_abs::State;

struct EdgeIncidence {
    edge: Edge,
    parameter: f64,
    other_side: i32,
    opposite: Option<(Edge, f64)>,
}

impl TopOpeBRepFacesFiller<'_> {
    /// Computes a closing-edge transition in the other face's matter; `filler.get_edge_trans(&point,kind,index,1,&face)?`.
    pub fn get_edge_trans(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        kind: TopOpeBRepDSKind,
        index: i32,
        side: i32,
        face: &Face,
    ) -> Result<Transition, Failure> {
        let mask = point.shape_index();
        if mask != 3 && mask != side {
            return Err(Failure::with_message(
                "Standard_Failure",
                "TopOpeBRepFacesFiller::GetEdgeTrans",
            ));
        }
        let incidence = edge_incidence(point, side)?;
        let range =
            self.interference_parameter_bounds(&incidence.edge, incidence.parameter, kind, index)?;
        let uv = point.surface_parameters(incidence.other_side)?;
        let mut maker = TopOpeBRepToolMakeTransition::new();
        if !maker.initialize(
            &incidence.edge,
            (range.0, range.1, incidence.parameter),
            face,
            uv,
            1.0e-4,
        )? {
            return Ok(Transition::default());
        }
        if self.needs_restriction_transition(&incidence, uv, face)?
            && maker.is_t2d()
            && let Some((opposite, parameter)) = &incidence.opposite
            && !maker.set_rest(opposite, *parameter)?
        {
            return Ok(Transition::default());
        }
        let (ok, before, after) = maker.mk_ton_e()?;
        let mut transition = Transition::default();
        if ok {
            transition.set_state_before(before);
            transition.set_state_after(after);
        }
        Ok(transition)
    }

    fn interference_parameter_bounds(
        &self,
        edge: &Edge,
        parameter: f64,
        kind: TopOpeBRepDSKind,
        index: i32,
    ) -> Result<(f64, f64), Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let ds = storage.ds();
        if !ds.has_shape(edge.as_ref(), true) {
            return fun_tool_bounds(edge);
        }
        let (before, after, _) = fds_lo_iinfsup(
            &ds,
            edge,
            parameter,
            kind,
            index,
            ds.shape_interferences(edge.as_ref(), true),
        )?;
        Ok((before, after))
    }

    fn needs_restriction_transition(
        &self,
        incidence: &EdgeIncidence,
        uv: martensite_tkmath::gp::Point2,
        face: &Face,
    ) -> Result<bool, Failure> {
        let tangent = fun_tool_etg_f(
            incidence.parameter,
            &incidence.edge,
            uv,
            face,
            precision::angular() * 1.0e2,
        )?;
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let restriction = self
            .restriction_edges
            .iter()
            .any(|edge| edge.is_same(incidence.edge.as_ref()))
            || storage.ds().is_section_edge(&incidence.edge, true);
        Ok(tangent && incidence.opposite.is_some() && restriction)
    }

    /// Stores a closing arc's supplied or computed transition on both supports; `filler.process_vponclosing_r(&point,&face,1,&transition,kind,index,false,&interference)?`.
    // The parameters mirror the bound public declaration, so grouping them would change that API.
    #[allow(clippy::too_many_arguments)]
    pub fn process_vponclosing_r(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        _face: &Shape,
        side: i32,
        transition: &Transition,
        kind: TopOpeBRepDSKind,
        index: i32,
        _found: bool,
        _previous: &Handle<TopOpeBRepDSInterference>,
    ) -> Result<(), Failure> {
        let incidence = edge_incidence(point, side)?;
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let other_face = self.face(incidence.other_side)?;
        let (face_index, opposite_index) =
            self.register_closing_supports(storage, &incidence, side, other_face);
        let mut transition = if transition.is_unknown() {
            self.get_edge_trans(point, kind, index, side, other_face)?
        } else {
            *transition
        };
        transition.set_index(face_index);
        let is_vertex = kind == TopOpeBRepDSKind::Vertex;
        let face_interference = make_epv_interference_sk(
            &transition,
            face_index,
            index,
            incidence.parameter,
            kind,
            TopOpeBRepDSKind::Face,
            is_vertex,
        )?;
        storage.store_interference(&face_interference, incidence.edge.as_ref())?;
        if let Some(opposite) = opposite_index {
            let edge_interference = make_epv_interference(
                &transition,
                opposite,
                index,
                incidence.parameter,
                kind,
                is_vertex,
            );
            storage.store_interference(&edge_interference, incidence.edge.as_ref())?;
        }
        Ok(())
    }

    fn register_closing_supports(
        &self,
        storage: &TopOpeBRepDSHDataStructure,
        incidence: &EdgeIncidence,
        side: i32,
        other_face: &Face,
    ) -> (i32, Option<i32>) {
        let mut ds = storage.change_ds();
        let face_index = if ds.has_shape(other_face.as_ref(), true) {
            ds.shape_index(other_face.as_ref(), true)
        } else {
            ds.add_shape_with_rank(other_face.as_ref(), incidence.other_side)
        };
        if !ds.has_shape(incidence.edge.as_ref(), true) {
            ds.add_shape_with_rank(incidence.edge.as_ref(), side);
        }
        let opposite_index = incidence.opposite.as_ref().map(|(edge, _)| {
            if ds.has_shape(edge.as_ref(), true) {
                ds.shape_index(edge.as_ref(), true)
            } else {
                ds.add_shape_with_rank(edge.as_ref(), incidence.other_side)
            }
        });
        (face_index, opposite_index)
    }
}

fn edge_incidence(point: &TopOpeBRepVPointInter<'_>, side: i32) -> Result<EdgeIncidence, Failure> {
    let other_side = if side == 1 { 2 } else { 1 };
    let on_two = point.shape_index() == 3;
    let has_opposite = on_two || point.state(other_side)? == State::On;
    let opposite = if !has_opposite {
        None
    } else if on_two {
        Some((
            topods::edge(&point.edge(other_side)?)?,
            point.edge_parameter(other_side)?,
        ))
    } else {
        Some((
            topods::edge(point.edge_on(other_side)?)?,
            point.edge_on_parameter(other_side)?,
        ))
    };
    Ok(EdgeIncidence {
        edge: topods::edge(&point.edge(side)?)?,
        parameter: point.edge_parameter(side)?,
        other_side,
        opposite,
    })
}
