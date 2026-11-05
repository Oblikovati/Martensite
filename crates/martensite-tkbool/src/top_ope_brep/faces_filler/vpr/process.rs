//! Restriction-point orchestration keeps curve, edge, closing and ON correction paths.
use super::{
    GeometrySearch, TopOpeBRepFacesFiller,
    context::Context,
    edge_interferences as edges,
    line_transition::line_transition,
    new_transition::{EdgeTransitionQuery, contains_shape, new_transition_edge},
    selection,
};
use crate::top_ope_brep::{
    TopOpeBRepFFTransitionTool, TopOpeBRepLineInter, TopOpeBRepPointGeomTool, TopOpeBRepVPointInter,
};
use crate::top_ope_brep_ds::{
    TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition as Transition, export::fds_has_unk,
};
use crate::top_ope_brep_tool::fun_tool_etg_f;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

fn copy_states(target: &mut Transition, source: &Transition) {
    target.set_state_before(source.before());
    target.set_state_after(source.after());
}

fn edge_transition(
    filler: &TopOpeBRepFacesFiller<'_>,
    line: &mut TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    context: &Context,
    search: &GeometrySearch,
    face_transition: &Transition,
) -> Result<(Transition, bool), Failure> {
    let face_orientation = face_transition.orientation(State::In, ShapeEnum::Face);
    let mut transition =
        TopOpeBRepFFTransitionTool::process_edge_transition(point, context.side, face_orientation)?;
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let candidates = if context.current.edge_index != 0 {
        let list = storage
            .ds()
            .shape_interferences(&context.current.edge, true)
            .clone();
        let geometry = if search.index == 0 {
            selection::at_point(
                storage,
                &list,
                &TopOpeBRepPointGeomTool::from_vpoint(point)?,
            )?
        } else {
            selection::on_geometry(&list, search.index)
        };
        selection::on_transition_shape(&geometry, context.other_face_index)?
    } else {
        Default::default()
    };
    let found = !candidates.is_empty();
    // The native allINT/allEXT conjunction cancels its initial tangent-flag test.
    let tangent = matches!(
        face_orientation,
        Orientation::Internal | Orientation::External
    );
    let edge_orientation = transition.orientation(State::In, ShapeEnum::Face);
    let all_internal =
        face_orientation == Orientation::Internal || edge_orientation == Orientation::Internal;
    let all_external =
        face_orientation == Orientation::External || edge_orientation == Orientation::External;
    let recompute = ((tangent && !all_internal && !all_external)
        || fds_has_unk(&transition)
        || context.current.closing
        || context.other_closing())
        && !filler.line_inl;
    if !recompute {
        return Ok((transition, found));
    }
    if let Some(first) = candidates.iter().next() {
        copy_states(
            &mut transition,
            &first
                .get()
                .ok_or_else(super::super::missing_interference)?
                .transition(),
        );
    } else if let Some(computed) = new_transition_edge(
        filler,
        line,
        &EdgeTransitionQuery {
            line_on_restriction: filler.line_is_on_edge,
            point,
            kind: search.kind,
            index: search.index,
            other_side: context.other_side,
            edge: &context.current.edge,
            restrictions: &filler.restriction_edges,
        },
    )? {
        copy_states(&mut transition, &computed);
    }
    Ok((transition, found))
}

impl TopOpeBRepFacesFiller<'_> {
    /// Records one restriction crossing with the source transition and support rules; `filler.process_vpon_r(&point,&t,&face,1,&mut line)?`.
    pub fn process_vpon_r(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        transition: &Transition,
        _face: &Shape,
        side: i32,
        line: &mut TopOpeBRepLineInter,
    ) -> Result<(), Failure> {
        let mut context = Context::new(self, point, side)?;
        let mut search = GeometrySearch::default();
        self.process_vpondg_e(point, side, line, &mut search)?;
        let found_geometry = search.index != 0;
        let Some((line_transition, keep)) = line_transition(self, line, point, &context)? else {
            return Ok(());
        };
        let (mut edge_transition, found) =
            edge_transition(self, line, point, &context, &search, transition)?;
        if edge_transition.is_unknown() {
            if !line_transition.is_unknown() {
                edge_transition = line_transition.complement();
            } else if !keep {
                return Ok(());
            }
        }
        context.current.ensure_index(self, side)?;
        let multiple_kind = found_geometry
            && !search.edge_found
            && !search.curve_found
            && (context.current.vertex || context.other.as_ref().is_some_and(|entry| entry.vertex))
            && search.kind == Kind::Point;
        if !found_geometry || multiple_kind {
            self.find_vp_geometry(line, point, side, 1, &mut search)?;
        }
        let geometric_point = TopOpeBRepPointGeomTool::from_vpoint(point)?;
        let on_endpoint = search.kind == Kind::Vertex
            && edges::on_endpoint(&geometric_point, &context.current.edge)?;
        if self.line_inl
            && edges::process_inl(
                self,
                point,
                &mut context,
                &search,
                on_endpoint,
                &mut edge_transition,
            )?
        {
            return Ok(());
        }
        let no_curve = self.line_is_on_edge
            || (!context.on_two
                && context.other.is_some()
                && (context.other_section() || context.current.section));
        if !no_curve && !self.line_inl {
            self.process_curve_point(point, &context, line, &line_transition, &search)?;
        }
        self.store_other_edge_interference(&mut context, &search, &edge_transition)?;
        let correction = edges::needs_on_correction(self, &context, &geometric_point)?;
        let corrected = correction && edges::correct_on(self, point, &mut context, &search)?;
        if correction && !corrected && no_curve && !self.line_is_on_edge {
            self.process_curve_point(point, &context, line, &line_transition, &search)?;
        }
        if (context.current.closing || context.other_closing()) && !found {
            return self.process_vponclosing_r(
                point,
                &context.current.face,
                side,
                &edge_transition,
                search.kind,
                search.index,
                search.edge_found,
                &search.edge_interference,
            );
        }
        let append = edges::should_add_face(self, &context, &search, &edge_transition)?;
        if self.line_inl
            && fun_tool_etg_f(
                context.current.parameter,
                &context.current.edge,
                point.surface_parameters(context.other_side)?,
                &context.other_face,
                precision::angular() * 1e4,
            )?
            && contains_shape(&self.restriction_edges, &context.current.edge)
        {
            return Ok(());
        }
        if append && !context.current.degenerate {
            edges::store_edge(
                self,
                &edges::EdgeInterferenceRequest {
                    edge: &context.current.edge,
                    transition: &edge_transition,
                    face_index: context.other_face_index,
                    support: context.other_face_index,
                    parameter: context.current.parameter,
                    search: &search,
                    vertex: context.current.vertex,
                    face_support: true,
                },
            )?;
        }
        Ok(())
    }

    fn store_other_edge_interference(
        &self,
        context: &mut Context,
        search: &GeometrySearch,
        transition: &Transition,
    ) -> Result<(), Failure> {
        if context.current.closing || context.current.degenerate {
            return Ok(());
        }
        if let Some(other) = &mut context.other {
            other.ensure_index(self, context.other_side)?;
            edges::store_edge(
                self,
                &edges::EdgeInterferenceRequest {
                    edge: &context.current.edge,
                    transition,
                    face_index: context.other_face_index,
                    support: other.edge_index,
                    parameter: context.current.parameter,
                    search,
                    vertex: context.current.vertex,
                    face_support: false,
                },
            )?;
        }
        Ok(())
    }
}
