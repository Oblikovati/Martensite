use super::TopOpeBRepEdgesIntersector;
use crate::top_ope_brep::TopOpeBRepHctxee2d;
use crate::top_ope_brep_tool::{PCurveOnFace, fc2d_curve_on_surface, fc2d_make_curve_on_surface};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeomalgo::int_res2d::Domain;
use martensite_tkmath::geom_abs::SurfaceType;
impl TopOpeBRepEdgesIntersector {
    pub(super) fn load_curves(&mut self) -> Result<Option<[Domain; 2]>, Failure> {
        let spherical = self.surfaces[0].get_type()? == SurfaceType::Sphere
            && self.surfaces[1].get_type()? == SurfaceType::Sphere;
        if spherical && !BRepTool::degenerated(&self.edges[1])? {
            return self.load_regular_spherical().map(Some);
        }
        if spherical {
            fc2d_make_curve_on_surface(&self.edges[0], &self.faces[0], 0., true)?;
        }
        let context = self.edge_context()?;
        let mut domains = [*context.domain(1)?, *context.domain(2)?];
        self.curves[0].load(&context.curve(1)?.curve())?;
        if context.curve(2)?.curve().is_null() {
            return Ok(None);
        }
        self.curves[1].load(&context.curve(2)?.curve())?;
        if spherical {
            self.load_spherical_curves(&mut domains)?;
        }
        Ok(Some(domains))
    }
    fn edge_context(&self) -> Result<TopOpeBRepHctxee2d, Failure> {
        let mut context = TopOpeBRepHctxee2d::new();
        context
            .set_edges(
                &self.edges[0],
                &self.edges[1],
                &self.surfaces[0],
                &self.surfaces[1],
            )
            .map_err(context_failure)?;
        Ok(context)
    }
    fn load_spherical_curves(&mut self, domains: &mut [Domain; 2]) -> Result<(), Failure> {
        let curve = fc2d_make_curve_on_surface(&self.edges[0], &self.faces[0], 0., true)?;
        domains[0] = self.spherical_domain(0, &curve, true)?;
        Ok(())
    }
    fn load_regular_spherical(&mut self) -> Result<[Domain; 2], Failure> {
        let same = self.same_support()?;
        let first = fc2d_make_curve_on_surface(&self.edges[0], &self.faces[0], 0., true)?;
        let second = if same {
            fc2d_curve_on_surface(&self.edges[1], &self.faces[0], false)?
        } else {
            fc2d_make_curve_on_surface(&self.edges[1], &self.faces[0], 0., true)?
        };
        let mut domains = [Domain::default(); 2];
        for (index, curve) in [first, second].iter().enumerate() {
            domains[index] = self.spherical_domain(index, curve, index == 0 || same)?;
        }
        Ok(domains)
    }
    fn spherical_domain(
        &mut self,
        index: usize,
        curve: &PCurveOnFace,
        uv_points: bool,
    ) -> Result<Domain, Failure> {
        self.curves[index].load(&curve.0)?;
        let tolerance = BRepTool::edge_tolerance(&self.edges[index])?;
        let (start, end) = if uv_points {
            BRepTool::uv_points_on_face(&self.edges[index], &self.faces[0])?
        } else {
            (curve.0.value(curve.1)?, curve.0.value(curve.2)?)
        };
        Ok(Domain::bounded(
            start, curve.1, tolerance, end, curve.2, tolerance,
        ))
    }
    fn same_support(&self) -> Result<bool, Failure> {
        if self.faces[0].is_same(&self.faces[1]) {
            return Ok(true);
        }
        let (a, la) = BRepTool::surface_with_location(&self.faces[0])?;
        let (b, lb) = BRepTool::surface_with_location(&self.faces[1])?;
        Ok(a == b && la == lb)
    }
}

fn context_failure(error: Failure) -> Failure {
    if error.message() == "TopOpeBRepHctxee2d::SetEdges : no 2d curve" {
        return Failure::new("EdgesIntersector::Perform : no 2d curve");
    }
    error
}
