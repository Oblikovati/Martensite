//! Curve-point insertion preserves exact parameter rejection and the retained-point cap.
use super::{GeometrySearch, TopOpeBRepFacesFiller, context::Context};
use crate::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepVPointInter, funbrep_topokpart};
use crate::top_ope_brep_ds::{
    TopOpeBRepDSKind, TopOpeBRepDSTransition, export::fds_parameter,
    process_interferences_tool::make_cpv_interference,
};
use martensite_tkernel::standard::Failure;

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn process_curve_point(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        context: &Context,
        line: &mut TopOpeBRepLineInter,
        transition: &TopOpeBRepDSTransition,
        search: &GeometrySearch,
    ) -> Result<(), Failure> {
        let mut parameter = point.parameter_on_line()?;
        if let Some(last) = self.curve_interferences.iter().last()
            && fds_parameter(last)? == parameter
        {
            return Ok(());
        }
        let mut transition = *transition;
        let storage = self.hds.get().ok_or_else(super::super::missing_storage)?;
        let keep = funbrep_topokpart(
            &mut self.kpart_context,
            &search.curve_interference,
            &self.curve_interferences,
            line,
            point,
            &storage.ds(),
            &context.current.edge,
            &context.current.face,
            1e-8,
            &mut parameter,
            &mut transition,
        )?;
        if !keep {
            return Ok(());
        }
        self.kept_point_count += 1;
        if self.kept_point_count > 2 {
            return Ok(());
        }
        let kind = if search.edge_found || search.curve_found {
            search.kind
        } else if context.current.vertex || point.is_vertex(context.other_side)? {
            TopOpeBRepDSKind::Vertex
        } else {
            TopOpeBRepDSKind::Point
        };
        let interference = make_cpv_interference(&transition, 0, search.index, parameter, kind);
        self.store_curve_interference(&interference)
    }
}
