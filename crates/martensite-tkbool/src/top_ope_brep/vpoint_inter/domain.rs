use super::{TopOpeBRepVPointInter, domain_error, source_domain_error};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorCurve2d,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorHVertex;
use std::io::Write;

impl TopOpeBRepVPointInter<'_> {
    /// Reads the first domain's topology edge; `view.arc_on_s1()?`.
    pub fn arc_on_s1(&self) -> Result<Shape, Failure> {
        let arc = self.source()?.arc_on_s1().map_err(source_domain_error)?;
        let curve = Handle::<BRepAdaptorCurve2d>::down_cast(&arc);
        checked_curve(&curve).map(|curve| curve.edge().into_shape())
    }
    /// Reads the second domain's topology edge; `view.arc_on_s2()?`.
    pub fn arc_on_s2(&self) -> Result<Shape, Failure> {
        let arc = self.source()?.arc_on_s2().map_err(source_domain_error)?;
        let curve = Handle::<BRepAdaptorCurve2d>::down_cast(&arc);
        checked_curve(&curve).map(|curve| curve.edge().into_shape())
    }
    /// Reads a first-domain vertex or the native domain error; `view.vertex_on_s1()?`.
    pub fn vertex_on_s1(&self) -> Result<Shape, Failure> {
        if !self.is_vertex_on_s1()? {
            return Err(domain_error("VertexOnS1"));
        }
        let vertex = self.source()?.vertex_on_s1()?;
        checked_vertex(&Handle::<BRepTopAdaptorHVertex>::down_cast(&vertex))
    }
    /// Reads a second-domain vertex or the native domain error; `view.vertex_on_s2()?`.
    pub fn vertex_on_s2(&self) -> Result<Shape, Failure> {
        if !self.is_vertex_on_s2()? {
            return Err(domain_error("VertexOnS2"));
        }
        let vertex = self.source()?.vertex_on_s2()?;
        checked_vertex(&Handle::<BRepTopAdaptorHVertex>::down_cast(&vertex))
    }
    /// Reads a restriction edge, returning a null shape when absent; `view.edge(1)?`.
    pub fn edge(&self, index: i32) -> Result<Shape, Failure> {
        match index {
            1 if self.is_on_dom_s1()? => self.arc_on_s1(),
            2 if self.is_on_dom_s2()? => self.arc_on_s2(),
            _ => Ok(Shape::default()),
        }
    }
    /// Reads a restriction parameter, returning zero when absent; `view.edge_parameter(1)?`.
    pub fn edge_parameter(&self, index: i32) -> Result<f64, Failure> {
        match index {
            1 if self.is_on_dom_s1()? => self.parameter_on_arc1(),
            2 if self.is_on_dom_s2()? => self.parameter_on_arc2(),
            _ => Ok(0.),
        }
    }
    /// Reads surface UVs, returning the origin for other indices; `view.surface_parameters(1)?`.
    pub fn surface_parameters(&self, index: i32) -> Result<Point2, Failure> {
        let (u, v) = match index {
            1 => self.parameters_on_s1()?,
            2 => self.parameters_on_s2()?,
            _ => (0., 0.),
        };
        Ok(Point2::new(u, v))
    }
    /// Checks the requested domain's vertex flag; other indices return false; `view.is_vertex(1)?`.
    pub fn is_vertex(&self, index: i32) -> Result<bool, Failure> {
        match index {
            1 => self.is_vertex_on_s1(),
            2 => self.is_vertex_on_s2(),
            _ => Ok(false),
        }
    }
    /// Reads a domain vertex or a null shape when absent; `view.vertex(1)?`.
    pub fn vertex(&self, index: i32) -> Result<Shape, Failure> {
        match index {
            1 if self.is_vertex_on_s1()? => self.vertex_on_s1(),
            2 if self.is_vertex_on_s2()? => self.vertex_on_s2(),
            _ => Ok(Shape::default()),
        }
    }
    /// Compares line parameters strictly and positions within the larger tolerance; `view.equal_pp(&other)?`.
    pub fn equal_pp(&self, other: &Self) -> Result<bool, Failure> {
        let same_parameter = (self.parameter_on_line()? - other.parameter_on_line()?).abs()
            < precision::p_confusion();
        let first = self.tolerance()?;
        let second = other.tolerance()?;
        let tolerance = if first > second { first } else { second };
        Ok(same_parameter && self.value()?.distance(&other.value()?) <= tolerance)
    }
    /// Finds an edge parameter, leaving the supplied output unchanged on failure; `view.par_on_edge(&edge, &mut parameter)?`.
    pub fn par_on_edge(&self, edge: &Edge, parameter: &mut f64) -> Result<bool, Failure> {
        for index in [1, 2] {
            if self.on_domain(index)? && edge.is_same(&self.edge(index)?) {
                *parameter = self.edge_parameter(index)?;
                return Ok(true);
            }
        }
        self.par_on_classified_edge(edge, parameter)
    }
    fn par_on_classified_edge(&self, edge: &Edge, parameter: &mut f64) -> Result<bool, Failure> {
        for index in [1, 2] {
            if self.state(index)? == State::On && self.edge_on(index)?.is_same(edge) {
                *parameter = self.edge_on_parameter(index)?;
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn on_domain(&self, index: i32) -> Result<bool, Failure> {
        if index == 1 {
            self.is_on_dom_s1()
        } else {
            self.is_on_dom_s2()
        }
    }
    /// Performs release-mode edge diagnostics without writing debug output; `view.dump_on_surface(1, &face, &mut output)?`.
    pub fn dump_on_surface(
        &self,
        index: i32,
        face: &Face,
        _output: &mut impl Write,
    ) -> Result<(), Failure> {
        let edge = topods::edge(&self.edge(index)?)?;
        self.edge_parameter(index)?;
        edge.orientation();
        TopOpeBRepToolShapeTool::closed(&edge, face)?;
        Ok(())
    }
    /// The release-mode two-face dump writes nothing; `view.dump(&face1, &face2, &mut output)`.
    pub fn dump(&self, _first: &Face, _second: &Face, _output: &mut impl Write) {}
}

fn checked_curve(curve: &Handle<BRepAdaptorCurve2d>) -> Result<&BRepAdaptorCurve2d, Failure> {
    if curve.is_null() {
        return Err(domain_error("Arc: expected a non-null BRepAdaptor_Curve2d"));
    }
    Ok(curve)
}
fn checked_vertex(vertex: &Handle<BRepTopAdaptorHVertex>) -> Result<Shape, Failure> {
    if vertex.is_null() {
        return Err(domain_error(
            "Vertex: expected a non-null BRepTopAdaptor_HVertex",
        ));
    }
    Ok(vertex.vertex().clone().into_shape())
}
