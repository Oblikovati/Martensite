use super::TopOpeBRepEdgesIntersector;
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{brep::BRepTool, topods::Edge};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeomalgo::{
    geom2d_int::GInter,
    int_res2d::{Domain, IntersectionPoint, IntersectionSegment},
};
use martensite_tkmath::geom_abs::CurveType;
impl TopOpeBRepEdgesIntersector {
    /// Intersects two edges in the first face's parameter space; `state.perform(&a, &b, true)?`.
    pub fn perform(
        &mut self,
        first: &Edge,
        second: &Edge,
        reduce_segments: bool,
    ) -> Result<(), Failure> {
        self.points.clear();
        self.cursor = 1;
        self.selected_count = 0;
        self.edges = [first.clone(), second.clone()];
        let Some(domains) = self.load_curves()? else {
            return Ok(());
        };
        let (mut points, segments) = self.intersections(&domains)?;
        super::filters::filter_points(&mut points, &self.curves, self.tolerance_max())?;
        self.assign_intersections(&points, &segments, reduce_segments)?;
        self.correct_same_domain_vertices()?;
        self.init_point(true);
        while self.more_point() {
            self.next_point();
        }
        Ok(())
    }
    fn intersections(
        &self,
        domains: &[Domain; 2],
    ) -> Result<(Vec<IntersectionPoint>, Vec<IntersectionSegment>), Failure> {
        let intersection = GInter::with_domains(
            &self.curves[0],
            &domains[0],
            &self.curves[1],
            &domains[1],
            self.tolerances.0,
            self.tolerances.1,
        )?;
        let points = (1..=intersection.nb_points()?)
            .map(|i| intersection.point(i))
            .collect::<Result<Vec<_>, _>>()?;
        let segments = (1..=intersection.nb_segments()?)
            .map(|i| intersection.segment(i))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((points, segments))
    }
    fn assign_intersections(
        &mut self,
        points: &[IntersectionPoint],
        segments: &[IntersectionSegment],
        reduce: bool,
    ) -> Result<(), Failure> {
        self.point_count = points.len() as i32;
        self.segment_count = segments.len() as i32;
        self.same_domain = self.compute_same_domain()?;
        self.build_records(points, segments)?;
        if reduce && self.has_segment() && !self.same_domain {
            self.reduce_records()?;
        }
        Ok(())
    }
    fn compute_same_domain(&self) -> Result<bool, Failure> {
        if !self.has_segment() {
            return Ok(false);
        }
        let kind = self.curves[0].get_type()?;
        if kind != self.curves[1].get_type()? {
            return Ok(false);
        }
        if kind == CurveType::Line {
            return Ok(true);
        }
        if kind != CurveType::Circle {
            return Ok(false);
        }
        self.same_circle_domain()
    }
    fn same_circle_domain(&self) -> Result<bool, Failure> {
        let (a, b) = (self.curves[0].circle()?, self.curves[1].circle()?);
        if (a.radius() - b.radius())
            .abs()
            .partial_cmp(&precision::confusion())
            != Some(std::cmp::Ordering::Less)
        {
            return Ok(false);
        }
        let (a, b) = (a.location(), b.location());
        let distance = self.surfaces[0]
            .value(a.x(), a.y())?
            .distance(&self.surfaces[0].value(b.x(), b.y())?);
        Ok(distance
            <= BRepTool::edge_tolerance(&self.edges[0])?
                + BRepTool::edge_tolerance(&self.edges[1])?)
    }
    pub(super) fn edge_configuration(
        &self,
    ) -> Result<crate::top_ope_brep_ds::TopOpeBRepDSConfig, Failure> {
        use crate::top_ope_brep_ds::TopOpeBRepDSConfig;
        Ok(
            if TopOpeBRepToolShapeTool::edges_same_oriented(&self.edges[1], &self.edges[0])? {
                TopOpeBRepDSConfig::SameOriented
            } else {
                TopOpeBRepDSConfig::DiffOriented
            },
        )
    }
}
