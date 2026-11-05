//! Restriction lines create reciprocal face-edge and edge-point incidences.
use super::super::TopOpeBRepFFTransitionTool;
use super::*;
use crate::top_ope_brep_ds::TopOpeBRepDSInterferenceTool;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSConfig, TopOpeBRepDSTransition as Transition,
    export::{fds_has_unk, fun_ds_sdm},
    process_interferences_tool::{make_epv_interference, make_epv_interference_sk},
};
use crate::top_ope_brep_tool::{
    TopOpeBRepToolTOOL, fun_tool_etg_ooe, fun_tool_orient_e_in_f_forward,
};
use martensite_tkbrep::topods::Edge;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

struct RestrictionPair {
    edge: Edge,
    index: i32,
    rank: i32,
    other_rank: i32,
}
struct RestrictionIncidence {
    edge: Edge,
    other: Edge,
    other_index: i32,
    parameter: f64,
    on_boundary: i32,
    is_restriction: bool,
    tangent: bool,
}
impl TopOpeBRepFacesFiller<'_> {
    /// Registers a restriction line's reciprocal incidences; `filler.process_rline(&mut line)?`.
    pub fn process_rline(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(());
        }
        let edge = topods::edge(&line.arc()?)?;
        if !self
            .intersector
            .as_deref()
            .ok_or_else(classification::missing_intersector)?
            .is_restriction(edge.as_ref())
        {
            return Ok(());
        }
        let pair = self.register_restriction_line(line, edge)?;
        if line.nb_vpoint() == 0 {
            return Err(Failure::with_message(
                "Standard_ProgramError",
                "TopOpeBRepVPointInterIterator::CurrentVP",
            ));
        }
        let first_parameter = line.vpoint(1)?.parameter_on_line()?;
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?.clone();
            self.process_restriction_point(line, &pair, &point, first_parameter)?;
        }
        Ok(())
    }
    fn register_restriction_line(
        &self,
        line: &TopOpeBRepLineInter,
        edge: Edge,
    ) -> Result<RestrictionPair, Failure> {
        let rank = if line.arc_is_edge(1)? {
            1
        } else if line.arc_is_edge(2)? {
            2
        } else {
            0
        };
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let (index, rank, faces) = {
            let mut ds = storage.change_ds();
            let index = ds.add_shape_with_rank(edge.as_ref(), rank);
            (
                index,
                ds.ancestor_rank(edge.as_ref()),
                [
                    ds.add_shape_with_rank(self.faces[0].as_ref(), 1),
                    ds.add_shape_with_rank(self.faces[1].as_ref(), 2),
                ],
            )
        };
        for side in 1..=2 {
            let other = 3 - side;
            let mut transition = *line.face_face_transition(side)?;
            transition.set_index(faces[(other - 1) as usize]);
            let interference = TopOpeBRepDSInterferenceTool::make_face_edge_interference(
                &transition,
                faces[(other - 1) as usize],
                index,
                line.arc_is_edge(side)?,
                TopOpeBRepDSConfig::UnshGeometry,
            );
            storage.store_interference_index(&interference, faces[(side - 1) as usize])?;
        }
        Ok(RestrictionPair {
            edge,
            index,
            rank,
            other_rank: if rank == 1 { 2 } else { 1 },
        })
    }
    fn process_restriction_point(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        pair: &RestrictionPair,
        point: &TopOpeBRepVPointInter<'_>,
        first_parameter: f64,
    ) -> Result<(), Failure> {
        if point.shape_index() != 3 && point.shape_index() != pair.other_rank {
            return Ok(());
        }
        let other = topods::edge(&point.edge(pair.other_rank)?)?;
        let mut parameter = 0.;
        if !point.par_on_edge(&pair.edge, &mut parameter)? {
            parameter = point.parameter_on_line()?;
        }
        let mut other_parameter = 0.;
        point.par_on_edge(&other, &mut other_parameter)?;
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let other_index = storage
            .change_ds()
            .add_shape_with_rank(other.as_ref(), pair.other_rank);
        if fun_ds_sdm(&storage.ds(), pair.edge.as_ref(), other.as_ref()) {
            return Ok(());
        }
        let tangent = fun_tool_etg_ooe(
            parameter,
            &pair.edge,
            other_parameter,
            &other,
            precision::angular() * 1e5,
        )?;
        let mut search = vpr::GeometrySearch::default();
        for side in 1..=2 {
            let own = side == pair.rank;
            let incidence = RestrictionIncidence {
                edge: if own {
                    pair.edge.clone()
                } else {
                    other.clone()
                },
                other: if own {
                    other.clone()
                } else {
                    pair.edge.clone()
                },
                other_index: if own { other_index } else { pair.index },
                parameter: if own { parameter } else { other_parameter },
                on_boundary: TopOpeBRepToolTOOL::on_boundary(
                    if own { parameter } else { other_parameter },
                    if own { &pair.edge } else { &other },
                )?,
                is_restriction: own,
                tangent,
            };
            self.process_restriction_incidence(
                line,
                point,
                side,
                &incidence,
                first_parameter,
                &mut search,
            )?;
        }
        Ok(())
    }
    fn process_restriction_incidence(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        incidence: &RestrictionIncidence,
        first_parameter: f64,
        search: &mut vpr::GeometrySearch,
    ) -> Result<(), Failure> {
        search.edge_found = false;
        search.curve_found = false;
        self.process_vpondg_e(point, side, line, search)?;
        if search.index == 0 {
            vpr::vp_index(self, line, point, side, 2, search)?;
        }
        if !search.edge_found && !search.curve_found && search.index == 0 {
            let vertex_side = if point.is_vertex(side)? {
                side
            } else if point.is_vertex(3 - side)? {
                3 - side
            } else {
                side
            };
            (search.kind, search.index) = self.make_geometry(point, vertex_side)?;
        }
        let Some(transition) = self.restriction_incidence_transition(
            line,
            point,
            side,
            incidence,
            first_parameter,
            search,
        )?
        else {
            return Ok(());
        };
        self.store_restriction_incidence(point, side, incidence, search, &transition)
    }
    fn restriction_incidence_transition(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        incidence: &RestrictionIncidence,
        first_parameter: f64,
        search: &vpr::GeometrySearch,
    ) -> Result<Option<Transition>, Failure> {
        let mut transition = Transition::default();
        if !line.face_face_transition(side)?.is_unknown()
            && (point.shape_index() == side || point.shape_index() == 3)
            && incidence.is_restriction
        {
            transition = TopOpeBRepFFTransitionTool::process_line_transition(
                point,
                side,
                incidence.edge.orientation(),
            )?;
            if matches!(incidence.on_boundary, 1 | 2) && incidence.tangent {
                transition.set_state_before(State::Unknown);
                transition.set_state_after(State::Unknown);
            }
        }
        if fds_has_unk(&transition) {
            if !incidence.is_restriction {
                let computed = vpr::new_transition_edge(
                    self,
                    line,
                    &vpr::EdgeTransitionQuery {
                        line_on_restriction: self.line_is_on_edge,
                        point,
                        kind: search.kind,
                        index: search.index,
                        other_side: 3 - side,
                        edge: &incidence.edge,
                        restrictions: &self.restriction_edges.clone(),
                    },
                )?;
                return Ok(computed.map(|computed| {
                    let mut transition = Transition::default();
                    transition.set_state_before(computed.before());
                    transition.set_state_after(computed.after());
                    transition
                }));
            }
            let (found, orientation) =
                fun_tool_orient_e_in_f_forward(&incidence.other, &self.faces[(2 - side) as usize])?;
            if !found {
                return Ok(None);
            }
            let orientation = if orientation == Orientation::Internal {
                Orientation::Internal
            } else if (point.parameter_on_line()? - first_parameter).abs() < precision::confusion()
            {
                Orientation::Forward
            } else {
                Orientation::Reversed
            };
            if !incidence.tangent {
                transition = Transition::from_orientation(orientation);
            }
        }
        Ok((!fds_has_unk(&transition)).then_some(transition))
    }
    fn store_restriction_incidence(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        incidence: &RestrictionIncidence,
        search: &vpr::GeometrySearch,
        transition: &Transition,
    ) -> Result<(), Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let other = 3 - side;
        let face_index = storage
            .change_ds()
            .add_shape_with_rank(self.faces[(other - 1) as usize].as_ref(), other);
        if restriction_duplicate(storage, incidence, search.index, face_index, transition)? {
            return Ok(());
        }
        let mut transition = *transition;
        transition.set_index(face_index);
        let is_vertex = point.is_vertex(side)?;
        let interference = make_epv_interference(
            &transition,
            incidence.other_index,
            search.index,
            incidence.parameter,
            search.kind,
            is_vertex,
        );
        storage.store_interference(&interference, incidence.edge.as_ref())?;
        if !incidence.is_restriction {
            let face_interference = make_epv_interference_sk(
                &transition,
                face_index,
                search.index,
                incidence.parameter,
                search.kind,
                TopOpeBRepDSKind::Face,
                is_vertex,
            )?;
            storage.store_interference(&face_interference, incidence.edge.as_ref())?;
        }
        Ok(())
    }
}

fn restriction_duplicate(
    storage: &TopOpeBRepDSHDataStructure,
    incidence: &RestrictionIncidence,
    geometry: i32,
    face_index: i32,
    transition: &Transition,
) -> Result<bool, Failure> {
    use crate::top_ope_brep_ds::process_interferences_tool::{
        fun_select_gi_interference, fun_select_itrasha_interference, fun_select_si_interference,
        fun_select_traori_interference,
    };
    let mut source = storage
        .ds()
        .shape_interferences(incidence.edge.as_ref(), true)
        .clone();
    let mut geometry_matches = ListOfInterference::default();
    let mut support_matches = ListOfInterference::default();
    fun_select_gi_interference(&mut source, geometry, &mut geometry_matches)?;
    let count = fun_select_si_interference(
        &mut geometry_matches,
        incidence.other_index,
        &mut support_matches,
    )?;
    if count == 0 || face_index == 0 {
        return Ok(false);
    }
    let mut face_matches = ListOfInterference::default();
    let mut orientation_matches = ListOfInterference::default();
    if fun_select_itrasha_interference(&mut support_matches, face_index, &mut face_matches)? == 0 {
        return Ok(false);
    }
    Ok(fun_select_traori_interference(
        &mut face_matches,
        transition.orientation(State::In, ShapeEnum::Face),
        &mut orientation_matches,
    )? != 0)
}
