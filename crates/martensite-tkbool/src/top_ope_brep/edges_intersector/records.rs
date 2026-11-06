use super::{TopOpeBRepEdgesIntersector, TopOpeBRepPoint2d};
use crate::top_ope_brep_ds::TopOpeBRepDSConfig;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Vertex},
};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkgeomalgo::int_res2d::{IntersectionPoint, IntersectionSegment, Position};
use martensite_tkmath::gp::Point3;
impl TopOpeBRepEdgesIntersector {
    pub(super) fn build_records(
        &mut self,
        points: &[IntersectionPoint],
        segments: &[IntersectionSegment],
    ) -> Result<(), Failure> {
        self.raw_cursor = 1;
        for &point in points {
            self.append_record(point, None)?;
        }
        for segment in segments {
            self.append_record(segment.first_point()?, Some(segment.is_opposite()))?;
            self.append_record(segment.last_point()?, Some(segment.is_opposite()))?;
        }
        self.cursor = 1;
        self.selected_count = self.points.len() as i32;
        Ok(())
    }
    fn append_record(
        &mut self,
        point: IntersectionPoint,
        opposite: Option<bool>,
    ) -> Result<(), Failure> {
        let mut record = TopOpeBRepPoint2d::new();
        record.set_pint(&point);
        self.assign_record_edges(&mut record, point, opposite)?;
        self.decorate_record(&mut record, point, opposite.is_some())?;
        self.points.push(record);
        self.raw_cursor += 1;
        Ok(())
    }
    fn decorate_record(
        &self,
        record: &mut TopOpeBRepPoint2d,
        point: IntersectionPoint,
        segment: bool,
    ) -> Result<(), Failure> {
        record.set_is_point_of_segment(segment);
        record.set_segment_ancestors(0, 0);
        record.set_status(self.status1());
        record.set_index(self.raw_cursor);
        record.set_value2d(&point.value());
        record.set_value(&self.record_position(point)?);
        record.set_tolerance(self.tolerance_max());
        record.set_edges_config(if segment {
            self.edge_configuration()?
        } else {
            TopOpeBRepDSConfig::UnshGeometry
        });
        Ok(())
    }
    fn assign_record_edges(
        &self,
        record: &mut TopOpeBRepPoint2d,
        point: IntersectionPoint,
        opposite: Option<bool>,
    ) -> Result<(), Failure> {
        let parameters = [point.param_on_first(), point.param_on_second()];
        let positions = [
            point.transition_of_first().position_on_curve(),
            point.transition_of_second().position_on_curve(),
        ];
        for index in [0, 1] {
            record.set_parameter(index as i32 + 1, parameters[index])?;
            record.set_transition(
                index as i32 + 1,
                &self.point_transition(point, index, opposite)?,
            )?;
            if let Some(vertex) = self.record_vertex(index, positions[index], parameters[index])? {
                record.set_vertex(index as i32 + 1, &vertex)?;
            }
        }
        Ok(())
    }
    fn record_position(&self, point: IntersectionPoint) -> Result<Point3, Failure> {
        let uv = point.value();
        if precision::is_infinite(uv.x()) || precision::is_infinite(uv.y()) {
            return Ok(Point3::new(
                precision::infinite(),
                precision::infinite(),
                precision::infinite(),
            ));
        }
        self.surfaces[0].value(uv.x(), uv.y())
    }
    fn record_vertex(
        &self,
        index: usize,
        position: Position,
        parameter: f64,
    ) -> Result<Option<Vertex>, Failure> {
        let edge = &self.edges[index];
        if position != Position::Middle {
            let (first, last) = topexp::vertices(edge, false);
            let vertex = if position == Position::Head {
                first
            } else {
                last
            };
            return Ok((!vertex.is_null()).then_some(vertex));
        }
        for vertex in Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)? {
            if vertex.orientation() != Orientation::Internal {
                continue;
            }
            let vertex = topods::vertex(&vertex)?;
            if (parameter - BRepTool::parameter_on_face(&vertex, edge, &self.faces[0])?).abs()
                <= precision::p_confusion()
            {
                return Ok(Some(vertex));
            }
        }
        Ok(None)
    }
    pub(super) fn reduce_records(&mut self) -> Result<(), Failure> {
        let initial = self.points.len();
        for index in 0..initial.saturating_sub(1) {
            let mut first = std::mem::take(&mut self.points[index]);
            let mut last = std::mem::take(&mut self.points[index + 1]);
            let mut midpoint = TopOpeBRepPoint2d::new();
            let reduced = self.reduce_segment(&mut first, &mut last, &mut midpoint)?;
            self.points[index] = first;
            self.points[index + 1] = last;
            if reduced {
                midpoint.set_index(self.points.len() as i32 + 1);
                self.points.push(midpoint);
            }
        }
        self.selected_count = self.points.len() as i32;
        self.segment_count = 0;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::top_ope_brep::TopOpeBRepP2DStatus;
    #[test]
    fn automatic_reduction_appends_midpoint_keeps_ancestors_and_clears_segment_count()
    -> Result<(), Failure> {
        let mut state = TopOpeBRepEdgesIntersector::new();
        for (index, status) in [
            (1, TopOpeBRepP2DStatus::SegmentFirst),
            (2, TopOpeBRepP2DStatus::SegmentLast),
        ] {
            let mut point = TopOpeBRepPoint2d::new();
            point.set_index(index);
            point.set_status(status);
            point.set_is_point_of_segment(true);
            state.points.push(point);
        }
        state.segment_count = 1;
        state.selected_count = 2;
        state.reduce_records()?;
        assert_eq!(state.points.len(), 3);
        assert_eq!(state.segment_count, 0);
        assert!(!state.points[0].keep() && !state.points[1].keep());
        assert_eq!(state.points[2].index(), 3);
        assert_eq!(state.points[2].segment_ancestors(), (true, 1, 2));
        state.init_point(true);
        assert_eq!(state.point()?.index(), 3);
        Ok(())
    }
}
