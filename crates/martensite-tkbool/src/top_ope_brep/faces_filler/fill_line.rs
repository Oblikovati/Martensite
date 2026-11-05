//! Ordinary line vertices share point geometry and alternate coincident transitions.
use super::*;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSInterferenceTool as InterferenceTool, TopOpeBRepDSTransition as Transition,
};
use martensite_tkg3d::top_abs::Orientation;

impl TopOpeBRepFacesFiller<'_> {
    /// Stores kept line-point interferences using current-pair geometry; `filler.fill_line(&mut line)?`.
    pub fn fill_line(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        let (first, last, count) = line.vp_bounds();
        if count == 0 {
            return Ok(());
        }
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?;
            if point.keep() {
                self.fill_line_point(point, index, first, last)?;
            }
        }
        Ok(())
    }

    pub(super) fn fill_line_point(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        index: i32,
        first: i32,
        last: i32,
    ) -> Result<(), Failure> {
        let mut position = 0;
        let found = self.get_geometry(&self.curve_interferences, &mut position, point)?;
        let geometry = match found {
            Some((index, kind)) => (kind, index),
            None => self
                .get_ff_geometry_vpoint(point)?
                .map(Ok)
                .unwrap_or_else(|| self.make_geometry(point, 0))?,
        };
        let transition = self.point_transition(found.is_some(), position, index, first, last)?;
        let interference = InterferenceTool::make_curve_interference(
            &transition,
            TopOpeBRepDSKind::Curve,
            0,
            geometry.0,
            geometry.1,
            point.parameter_on_line()?,
        );
        self.store_curve_interference(&interference)
    }

    fn point_transition(
        &self,
        found: bool,
        position: usize,
        index: i32,
        first: i32,
        last: i32,
    ) -> Result<Transition, Failure> {
        if found {
            return self
                .curve_interferences
                .iter()
                .nth(position)
                .and_then(|entry| entry.get())
                .map(|entry| entry.transition().complement())
                .ok_or_else(missing_interference);
        }
        Ok(if index == first {
            Transition::from_orientation(Orientation::Forward)
        } else if index == last {
            Transition::from_orientation(Orientation::Reversed)
        } else {
            Transition::default()
        })
    }
}
