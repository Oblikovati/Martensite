//! The compatibility cylinder mapping preserves its original coefficient and datum arithmetic.
use super::*;
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::gp::Axis3;
pub(super) struct Cylinder<'a>(pub(super) &'a Request);
impl CarrierBuilder for Cylinder<'_> {
    fn handedness(&self) -> DevelopedMapHandedness {
        DevelopedMapHandedness::Preserving
    }
    fn caps(&self) -> Result<[Face; 2], Failure> {
        let request = self.0;
        let caps = request
            .depths
            .map(|d| cap(request.radius + d, request.tolerance / 4.));
        Ok([caps[0].clone()?, caps[1].clone()?])
    }
    fn vertex_pair(&self, point: [f64; 2]) -> Result<[Vertex; 2], Failure> {
        vertex_pair(point, self.0)
    }
    fn curves(
        &self,
        source: &AuthoredPiece,
        ends: [&[Vertex; 2]; 2],
    ) -> Result<[Handle<GeomBezierCurve>; 2], Failure> {
        boundary_curves(self.0, source, ends)
    }
    fn attach(
        &self,
        source: &AuthoredPiece,
        edges: &[Edge; 2],
        caps: &[Face; 2],
    ) -> Result<(), Failure> {
        attach_cap_curves(self.0, source, edges, caps)
    }
    fn check_caps(&self, pieces: &[NativePiece], caps: &[Face; 2]) -> Result<(), Failure> {
        retained_cap_parameters(self.0, pieces, caps)
    }
}

fn retained_cap_parameters(
    request: &Request,
    pieces: &[NativePiece],
    caps: &[Face; 2],
) -> Result<(), Failure> {
    for (source, piece) in request.loops.iter().flatten().zip(pieces) {
        for (depth, cap) in caps.iter().enumerate() {
            let captured = BRepTool::curve_on_face(&piece.edges[depth], cap)?;
            let curve: Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve> =
                captured.pcurve.downcast();
            let expected: Vec<_> = cap_coordinates::angular_controls(&source.poles, request.radius);
            retained_poles(&curve, &expected)?;
        }
    }
    Ok(())
}

fn retained_poles(
    curve: &Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve>,
    expected: &[[f64; 2]],
) -> Result<(), Failure> {
    let Some(curve) = curve.get() else {
        return Err(Failure::new(
            "native conformance replaced an authored Bezier pcurve",
        ));
    };
    let poles = curve.poles();
    let actual: Vec<_> = poles.iter().map(|p| [p.x(), p.y()]).collect();
    if actual != expected {
        return Err(Failure::new(&format!(
            "native pcurve poles={actual:?}; expected unchanged authored poles={expected:?}"
        )));
    }
    Ok(())
}

fn cap(radius: f64, tolerance: f64) -> Result<Face, Failure> {
    let surface = Handle::new(GeomCylindricalSurface::new(Axis3::default(), radius)?).into_upcast();
    let mut face = Face::default();
    BRepBuilder::default().make_face_on_surface(&mut face, &surface, tolerance)?;
    Ok(face)
}

fn vertex_pair(point: [f64; 2], request: &Request) -> Result<[Vertex; 2], Failure> {
    let make = |depth| -> Result<Vertex, Failure> {
        let [x, y, z] = angular::endpoint(point, request.radius, request.radius + depth);
        let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()?;
        BRepBuilder::default().update_vertex_tolerance(&vertex, request.tolerance / 2.)?;
        Ok(vertex)
    };
    Ok([make(request.depths[0])?, make(request.depths[1])?])
}

fn depth_curve(
    coefficients: &[[f64; 4]],
    radius: f64,
    ends: [&Vertex; 2],
) -> Result<Handle<martensite_tkg3d::geom::GeomBezierCurve>, Failure> {
    let mut poles = angular::at_radius(coefficients, radius);
    let last = poles.points.len() - 1;
    for (index, vertex) in [(0, ends[0]), (last, ends[1])] {
        let point = BRepTool::pnt(vertex)?;
        poles.points[index] = [point.x(), point.y(), point.z()];
    }
    curve3(&poles)
}

fn boundary_curves(
    request: &Request,
    source: &AuthoredPiece,
    ends: [&[Vertex; 2]; 2],
) -> Result<[Handle<GeomBezierCurve>; 2], Failure> {
    let coefficients = angular::coefficients(&source.poles, request.radius, source.chart_center);
    let make = |i| {
        depth_curve(
            &coefficients,
            request.radius + request.depths[i],
            [&ends[0][i], &ends[1][i]],
        )
    };
    Ok([make(0)?, make(1)?])
}

fn attach_cap_curves(
    request: &Request,
    source: &AuthoredPiece,
    edges: &[Edge; 2],
    caps: &[Face; 2],
) -> Result<(), Failure> {
    let pcurve = curve2(
        &source
            .poles
            .iter()
            .map(|[s, z]| [s / request.radius, *z])
            .collect::<Vec<_>>(),
    )?;
    attach(&edges[0], &pcurve, &caps[0], request.tolerance / 2.)?;
    attach(&edges[1], &pcurve, &caps[1], request.tolerance / 2.)
}
