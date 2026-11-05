//! Restriction point dispatch preserves operand order and kept-point filtering.
use super::*;
impl TopOpeBRepFacesFiller<'_> {
    /// Fills domain-bound points and restriction lines; `filler.fill_line_vpon_r(&mut line)?`.
    pub fn fill_line_vpon_r(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        self.initialize_degenerate_edges(line)?;
        self.kept_point_count = 0;
        if line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction {
            return self.process_rline(line);
        }
        let (_, last, count) = line.vp_bounds();
        if count == 0 {
            return Ok(());
        }
        self.line_is_on_edge =
            Self::l_same_domain_erl(line, &self.restriction_edges, self.face_tolerance)?;
        self.last_point_on_zero = line.type_line_curve() == TopOpeBRepTypeLineCurve::Walking
            && (1..=line.nb_vpoint()).any(|index| {
                line.vpoint(index)
                    .is_ok_and(|point| point.index() == last && point.shape_index() == 0)
            });
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?.clone();
            self.process_vpr(&point, line)?;
        }
        if self.line_is_on_edge && !self.curve_interferences.is_empty() {
            self.curve_interferences.clear();
        }
        Ok(())
    }
    /// Dispatches a kept point, handling the vertex-owning operand first; `filler.process_vpr(&point,&mut line)?`.
    pub fn process_vpr(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        line: &mut TopOpeBRepLineInter,
    ) -> Result<(), Failure> {
        if !point.keep() {
            return Ok(());
        }
        let order = match point.shape_index() {
            0 => return self.process_vpnoton_r(point, line),
            1 => vec![1],
            2 => vec![2],
            3 if point.is_vertex_on_s2()? && !point.is_vertex_on_s1()? => vec![2, 1],
            3 => vec![1, 2],
            _ => return Ok(()),
        };
        for side in order {
            let transition = *line.face_face_transition(side)?;
            let face = self.faces[(side - 1) as usize].clone();
            self.process_vpon_r(point, &transition, face.as_ref(), side, line)?;
        }
        Ok(())
    }
}
