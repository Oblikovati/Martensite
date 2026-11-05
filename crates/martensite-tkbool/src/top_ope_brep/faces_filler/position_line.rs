//! Restriction classification precedes ordinary-line point matching.
use super::*;
use crate::top_ope_brep_tool::{fc2d_has_c3d, fun_tool_proj_ponc, fun_tool_proj_ponc2d};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d},
};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, top_abs::State};
use martensite_tkmath::{geom_abs::CurveType, gp::Point3};

impl TopOpeBRepFacesFiller<'_> {
    /// Classifies all retained intersections in restriction-first order; `filler.vp_position_intersections()?`.
    pub fn vp_position_intersections(&mut self) -> Result<(), Failure> {
        for restriction in [true, false] {
            self.change_faces_intersector()?.init_line()?;
            while self.change_faces_intersector()?.more_line() {
                self.position_current_array_line(restriction)?;
                self.change_faces_intersector()?.next_line()?;
            }
        }
        Ok(())
    }

    fn position_current_array_line(&mut self, restriction: bool) -> Result<(), Failure> {
        let pair = self
            .intersector
            .as_deref()
            .ok_or_else(classification::missing_intersector)?;
        let index = pair.current_line_index();
        let lines = pair.lines();
        let mut array = lines
            .get()
            .ok_or_else(classification::missing_intersector)?
            .change_array1();
        let offset = (index - array.lower()) as usize;
        let (before, rest) = array.as_mut_slice().split_at_mut(offset);
        let (line, after) = rest
            .split_first_mut()
            .ok_or_else(classification::missing_intersector)?;
        if (line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction) != restriction {
            return Ok(());
        }
        let others: Vec<_> = before.iter().chain(after.iter()).collect();
        self.vp_position_line(line, &others)
    }

    /// Classifies a line with explicit disjoint other-line borrows, then updates its bounds; `filler.vp_position_line(&mut line,&others)?`.
    pub fn vp_position_line(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        others: &[&TopOpeBRepLineInter],
    ) -> Result<(), Failure> {
        self.current_kind = Some(line.type_line_curve());
        if line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction {
            self.vp_position_on_r(line)?;
        } else {
            self.vp_position_on_l(line, others)?;
        }
        line.set_vp_bounds();
        Ok(())
    }

    /// Reuses an already classified restriction point before face classification; `filler.vp_position_on_l(&mut line,&others)?`.
    pub fn vp_position_on_l(
        &mut self,
        line: &mut TopOpeBRepLineInter,
        others: &[&TopOpeBRepLineInter],
    ) -> Result<(), Failure> {
        let index = line.index();
        let mut classifier = TopOpeBRepVPointInterClassifier::new();
        for point_index in 1..=line.nb_vpoint() {
            let point = line.change_vpoint(point_index)?;
            let mut found = false;
            for other in others {
                if other.index() == index {
                    continue;
                }
                if self.pequal_vpon_r(point.value()?, point.shape_index(), point, other)? {
                    found = true;
                    break;
                }
            }
            if !found {
                self.vp_position(point, &mut classifier)?;
            }
        }
        Ok(())
    }

    fn pequal_vpon_r(
        &self,
        position: Point3,
        mask: i32,
        point: &mut TopOpeBRepVPointInter<'_>,
        restriction: &TopOpeBRepLineInter,
    ) -> Result<bool, Failure> {
        if restriction.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(false);
        }
        let edge = topods::edge(&restriction.arc()?)?;
        let side = if restriction.arc_is_edge(1)? { 1 } else { 2 };
        for index in 1..=restriction.nb_vpoint() {
            let other = restriction.vpoint(index)?;
            if other.is_vertex(1)? || other.shape_index() != mask || other.state(side)? != State::On
            {
                continue;
            }
            if !other
                .value()?
                .is_equal(&position, BRepTool::edge_tolerance(&edge)?)
            {
                continue;
            }
            point.set_state(State::On, side)?;
            point.set_edge_on(edge.as_ref(), other.edge_on_parameter(side)?, side);
            return Ok(true);
        }
        Ok(false)
    }

    /// Projects nonvertex restriction points and preserves the native straight-vertex skip; `filler.vp_position_on_r(&mut line)?`.
    pub fn vp_position_on_r(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        let side = if line.arc_is_edge(1)? { 1 } else { 2 };
        let other_side = if side == 1 { 2 } else { 1 };
        let edge = topods::edge(&line.arc()?)?;
        let straight = self.restriction_is_line(&edge, side)?;
        let domains = [line.arc_is_edge(side)?, line.arc_is_edge(other_side)?];
        let mut classifier = TopOpeBRepVPointInterClassifier::new();
        for index in 1..=line.nb_vpoint() {
            let point = line.change_vpoint(index)?;
            if point.is_vertex(side)? || point.is_vertex(other_side)? {
                if !straight {
                    self.vp_position(point, &mut classifier)?;
                }
                continue;
            }
            for (input, on_domain) in [(side, domains[0]), (other_side, domains[1])] {
                if on_domain {
                    self.position_point_on_restriction(point, &edge, input, side)?;
                }
            }
        }
        Ok(())
    }

    fn restriction_is_line(
        &self,
        edge: &martensite_tkbrep::topods::Edge,
        side: i32,
    ) -> Result<bool, Failure> {
        if fc2d_has_c3d(edge)? {
            return Ok(BRepAdaptorCurve::with_edge(edge)?.get_type()? == CurveType::Line);
        }
        Ok(
            BRepAdaptorCurve2d::with_edge_on_face(edge, self.face(side)?)?.get_type()?
                == CurveType::Line,
        )
    }

    fn position_point_on_restriction(
        &self,
        point: &mut TopOpeBRepVPointInter<'_>,
        edge: &martensite_tkbrep::topods::Edge,
        input: i32,
        side: i32,
    ) -> Result<(), Failure> {
        let mut parameter = 0.;
        let mut distance = 0.;
        let found = if fc2d_has_c3d(edge)? {
            fun_tool_proj_ponc(
                point.value()?,
                &BRepAdaptorCurve::with_edge(edge)?,
                &mut parameter,
                &mut distance,
            )?
        } else {
            fun_tool_proj_ponc2d(
                point.value()?,
                &BRepAdaptorCurve2d::with_edge_on_face(edge, self.face(side)?)?,
                &mut parameter,
                &mut distance,
            )?
        };
        if found {
            point.set_state(State::On, input)?;
            point.set_edge_on(edge.as_ref(), parameter, input);
        } else {
            point.change_keep(false);
        }
        Ok(())
    }
}
