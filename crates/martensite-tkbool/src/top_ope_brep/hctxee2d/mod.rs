//! Edge intersection contexts in the first face's parameter space.
use super::hctxff2d::context_slot;
use crate::top_ope_brep_tool::{PCurveOnFace, fc2d_curve_on_surface};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorSurface,
    topods::{Edge, Face},
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, StandardTransient},
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkgeomalgo::int_res2d::Domain;
use martensite_tkmath::geom_abs::SurfaceType;
mod projection;

/// Two edges and their 2D intersection domains; `TopOpeBRepHctxee2d::new()`.
#[derive(Default)]
pub struct TopOpeBRepHctxee2d {
    parent: StandardTransient,
    edges: [Edge; 2],
    curves: [Geom2dAdaptorCurve; 2],
    domains: [Domain; 2],
}
impl_transient!(
    TopOpeBRepHctxee2d,
    "TopOpeBRepHctxee2d",
    StandardTransient,
    parent
);
impl TopOpeBRepHctxee2d {
    /// Starts with null edges and unbounded domains; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Loads both edges in the first face's parameter space; `context.set_edges(&e1, &e2, &s1, &s2)?`.
    pub fn set_edges(
        &mut self,
        first: &Edge,
        second: &Edge,
        s1: &BRepAdaptorSurface,
        s2: &BRepAdaptorSurface,
    ) -> Result<(), Failure> {
        let (f1, f2) = (s1.face(), s2.face());
        let surface_type = s1.get_type()?;
        self.edges = [first.clone(), second.clone()];
        let pc1 = self.load_first_on_face(first, &f1)?;
        if surface_type == SurfaceType::Plane || f1.is_same(&f2) || same_support(&f1, &f2)? {
            let pc2 = fc2d_curve_on_surface(second, &f1, false)?;
            return self.load_on_face(1, &f1, &pc2);
        }
        self.load_projected(&f1, (pc1.1, pc1.2))
    }
    fn load_first_on_face(&mut self, edge: &Edge, face: &Face) -> Result<PCurveOnFace, Failure> {
        let curve = fc2d_curve_on_surface(edge, face, false)?;
        if curve.0.is_null() {
            return Err(Failure::new("TopOpeBRepHctxee2d::SetEdges : no 2d curve"));
        }
        self.load_on_face(0, face, &curve)?;
        Ok(curve)
    }
    fn load_on_face(
        &mut self,
        index: usize,
        face: &Face,
        curve: &PCurveOnFace,
    ) -> Result<(), Failure> {
        self.curves[index].load(&curve.0)?;
        let (first, last) = BRepTool::uv_points_on_face(&self.edges[index], face)?;
        let tolerance = BRepTool::edge_tolerance(&self.edges[index])?;
        self.domains[index].set_values(first, curve.1, tolerance, last, curve.2, tolerance);
        Ok(())
    }
    /// Reads an input edge, including its original location/orientation; `context.edge(1)?`.
    pub fn edge(&self, index: i32) -> Result<&Edge, Failure> {
        Ok(&self.edges[context_slot(index, "TopOpeBRepHctxee2d::Edge")?])
    }
    /// Reads a 2D curve adaptor; `context.curve(2)?`.
    pub fn curve(&self, index: i32) -> Result<&Geom2dAdaptorCurve, Failure> {
        Ok(&self.curves[context_slot(index, "TopOpeBRepHctxee2d::Curve")?])
    }
    /// Reads endpoint parameters, UV values and edge tolerances; `context.domain(1)?`.
    pub fn domain(&self, index: i32) -> Result<&Domain, Failure> {
        Ok(&self.domains[context_slot(index, "TopOpeBRepHctxee2d::Domain")?])
    }
}
fn same_support(first: &Face, second: &Face) -> Result<bool, Failure> {
    let (s1, l1) = BRepTool::surface_with_location(first)?;
    let (s2, l2) = BRepTool::surface_with_location(second)?;
    Ok(s1 == s2 && l1 == l2)
}

use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
