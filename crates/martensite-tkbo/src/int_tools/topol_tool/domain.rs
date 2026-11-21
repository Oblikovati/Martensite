// Native virtual dispatch for intersection algorithms must select IntTools sampling.
use super::*;
use martensite_tkbrep::topods::Edge;
use martensite_tkg3d::top_abs::{Orientation, State};
use martensite_tkgeomalgo::int_start::{TheArc, TheTopolTool, TheVertex};
impl TheTopolTool for TopolTool {
    fn nb_samples_u(&self) -> Result<i32, Failure> {
        TopolTool::nb_samples_u(self)
    }
    fn nb_samples_v(&self) -> Result<i32, Failure> {
        TopolTool::nb_samples_v(self)
    }
    fn nb_samples(&self) -> Result<i32, Failure> {
        TopolTool::nb_samples(self)
    }
    fn sample_point(&self, index: i32) -> Result<(Point2, Point3), Failure> {
        TopolTool::sample_point(self, index)
    }
    fn classify(&self, point: Point2, tol: f64, reframe: bool) -> Result<State, Failure> {
        Ok(Adaptor3dTopolTool::classify(
            &self.topology,
            point,
            tol,
            reframe,
        ))
    }
    fn init(&self) {
        Adaptor3dTopolTool::init(&self.topology);
    }
    fn more(&self) -> bool {
        Adaptor3dTopolTool::more(&self.topology)
    }
    fn value(&self) -> Result<TheArc, Failure> {
        Adaptor3dTopolTool::value(&self.topology)
    }
    fn next(&self) {
        Adaptor3dTopolTool::next(&self.topology);
    }
    fn initialize_arc(&self, arc: &TheArc) -> Result<(), Failure> {
        self.initialize_curve(arc)
    }
    fn init_vertex_iterator(&self) -> Result<(), Failure> {
        Adaptor3dTopolTool::init_vertex_iterator(&self.topology);
        Ok(())
    }
    fn more_vertex(&self) -> bool {
        Adaptor3dTopolTool::more_vertex(&self.topology)
    }
    fn vertex(&self) -> Result<TheVertex, Failure> {
        Adaptor3dTopolTool::vertex(&self.topology)
    }
    fn next_vertex(&self) -> Result<(), Failure> {
        Adaptor3dTopolTool::next_vertex(&self.topology);
        Ok(())
    }
    fn identical(&self, first: &TheVertex, second: &TheVertex) -> Result<bool, Failure> {
        Adaptor3dTopolTool::identical(&self.topology, first, second)
    }
    // The base class has no edge: its `Edge()` returns a null pointer.
    fn edge(&self) -> Result<Option<Edge>, Failure> {
        Ok(None)
    }
    fn is_the_point_on(&self, point: Point2, tol: f64, reframe: bool) -> Result<bool, Failure> {
        Ok(Adaptor3dTopolTool::is_the_point_on(
            &self.topology,
            point,
            tol,
            reframe,
        ))
    }
    fn domain_is_infinite(&self) -> Result<bool, Failure> {
        Ok(Adaptor3dTopolTool::domain_is_infinite(&self.topology))
    }
    fn sample_pnts(&self, deflection: f64, nu_min: i32, nv_min: i32) -> Result<(), Failure> {
        TopolTool::sample_pnts(self, deflection, nu_min, nv_min)
    }
    fn u_parameters(&self) -> Result<Array1<'static, f64>, Failure> {
        TopolTool::u_parameters(self)
    }
    fn v_parameters(&self) -> Result<Array1<'static, f64>, Failure> {
        TopolTool::v_parameters(self)
    }
    fn is_uniform_sampling(&self) -> Result<bool, Failure> {
        Adaptor3dTopolTool::is_uniform_sampling(&self.topology)
    }
    fn has_3d(&self) -> bool {
        Adaptor3dTopolTool::has_3d(&self.topology)
    }
    fn tol_3d_arc(&self, arc: &TheArc) -> Result<f64, Failure> {
        self.curve_tol_3d(arc)
    }
    fn tol_3d_vertex(&self, vertex: &TheVertex) -> Result<f64, Failure> {
        Adaptor3dTopolTool::vertex_tol_3d(&self.topology, vertex)
    }
    fn arc_orientation(&self, arc: &TheArc) -> Result<Orientation, Failure> {
        Ok(self.curve_orientation(arc))
    }
    fn vertex_orientation(&self, vertex: &TheVertex) -> Result<Orientation, Failure> {
        Adaptor3dTopolTool::vertex_orientation(&self.topology, vertex)
    }
}
