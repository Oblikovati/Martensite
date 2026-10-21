use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::Edge,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomParabola, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{Axis2, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

pub(super) struct ConicCase {
    pub kind: usize,
    pub modes: usize,
    wrapped: bool,
    reversed: bool,
}
impl ConicCase {
    pub fn from_id(id: usize) -> Self {
        Self {
            kind: id / 16,
            modes: (id / 4) % 4,
            wrapped: (id / 2) % 2 == 1,
            reversed: id % 2 == 1,
        }
    }
    fn basis(&self) -> Handle<dyn GeomCurve> {
        match self.kind {
            0 => Handle::new(GeomCircle::new(Axis2::default(), 2.).unwrap()).cast(),
            1 => Handle::new(GeomEllipse::new(Axis2::default(), 3., 2.).unwrap()).cast(),
            2 => Handle::new(GeomHyperbola::new(Axis2::default(), 3., 2.).unwrap()).cast(),
            3 => Handle::new(GeomParabola::new(Axis2::default(), 2.).unwrap()).cast(),
            value => panic!("conic kind {value}; expected conic index in 0..4"),
        }
    }
    pub fn edge(&self) -> Edge {
        let basis = self.basis();
        let mut edge = BRepBuilderAPIMakeEdge::from_curve_params(&basis, 0.125, 0.875)
            .unwrap()
            .edge()
            .unwrap();
        if self.wrapped {
            install_trim(&edge, &basis);
        }
        if self.reversed {
            edge.reverse();
        }
        self.assert_input_endpoints(&edge);
        edge
    }
    pub fn converts_to_bezier(&self) -> bool {
        self.wrapped || (self.modes & 2 != 0 && (self.kind != 0 || self.modes & 1 != 0))
    }
    fn point(&self, parameter: f64) -> Point3 {
        match self.kind {
            0 => Point3::new(2. * parameter.cos(), 2. * parameter.sin(), 0.),
            1 => Point3::new(3. * parameter.cos(), 2. * parameter.sin(), 0.),
            2 => Point3::new(3. * parameter.cosh(), 2. * parameter.sinh(), 0.),
            3 => Point3::new(parameter * parameter / 8., parameter, 0.),
            value => panic!("conic kind {value}; expected conic index in 0..4"),
        }
    }
    fn assert_input_endpoints(&self, edge: &Edge) {
        let (first, last) = topexp::vertices(edge, false);
        for (vertex, parameter) in [(first, 0.125), (last, 0.875)] {
            let error = BRepTool::pnt(&vertex)
                .unwrap()
                .distance(&self.point(parameter));
            assert!(
                error <= 1e-14,
                "kind{} parameter{parameter} endpoint error{error}",
                self.kind
            );
        }
    }
}

fn install_trim(edge: &Edge, basis: &Handle<dyn GeomCurve>) {
    let trimmed =
        Handle::new(GeomTrimmedCurve::new(basis, 0.125, 0.875, true, true).unwrap()).cast();
    BRepBuilder::default()
        .update_edge_curve(edge, &trimmed, 0.)
        .unwrap();
}
