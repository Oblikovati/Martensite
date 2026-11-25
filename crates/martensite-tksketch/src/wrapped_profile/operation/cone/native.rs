//! Analytic caps and rational ruled boundaries reuse the common incidence constructor.
use super::*;
use crate::surface_sketch::wrapped_profile::{boundaries::*, native::CarrierBuilder};
use martensite_cmath::{atan2, cos, hypot, sin};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::Vertex,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomConicalSurface};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
struct Cone<'a> {
    input: &'a Prepared,
    support: ConeSupport,
}
pub(super) fn construct(input: &Prepared, support: ConeSupport) -> Result<Constructed, Failure> {
    super::super::super::native::construct_mapped(&input.source, &Cone { input, support })
}
impl CarrierBuilder for Cone<'_> {
    fn handedness(&self) -> super::super::topology::DevelopedMapHandedness {
        super::super::topology::DevelopedMapHandedness::Reversing
    }
    fn caps(&self) -> Result<[Face; 2], Failure> {
        Ok([self.cap(0)?, self.cap(1)?])
    }
    fn vertex_pair(&self, point: [f64; 2]) -> Result<[Vertex; 2], Failure> {
        Ok([self.vertex(point, 0)?, self.vertex(point, 1)?])
    }
    fn vertex_pair_at(&self, source: &AuthoredPiece) -> Result<[Vertex; 2], Failure> {
        let Some(table) = &self.input.final_endpoints else {
            return self.vertex_pair(source.poles[0]);
        };
        let points = table
            .lookup(
                self.input.source.region,
                source.span,
                source.start,
                source.poles[0],
            )
            .map_err(|cause| Failure::new(&cause.to_string()))?;
        Ok([self.vertex_at(points[0])?, self.vertex_at(points[1])?])
    }
    fn curves(
        &self,
        source: &AuthoredPiece,
        ends: [&[Vertex; 2]; 2],
    ) -> Result<[Handle<GeomBezierCurve>; 2], Failure> {
        let fit = self.fit(source)?;
        Ok([
            depth_curve(&fit.poles[0], [&ends[0][0], &ends[1][0]])?,
            depth_curve(&fit.poles[1], [&ends[0][1], &ends[1][1]])?,
        ])
    }
    fn attach(
        &self,
        source: &AuthoredPiece,
        edges: &[Edge; 2],
        caps: &[Face; 2],
    ) -> Result<(), Failure> {
        let uv = curve2(&self.fit(source)?.uv)?;
        attach(&edges[0], &uv, &caps[0], self.input.source.tolerance / 2.)?;
        attach(&edges[1], &uv, &caps[1], self.input.source.tolerance / 2.)
    }
    fn check_caps(&self, pieces: &[NativePiece], caps: &[Face; 2]) -> Result<(), Failure> {
        for (source, piece) in self.input.source.loops.iter().flatten().zip(pieces) {
            check_cap_poles(&self.fit(source)?.uv, &piece.edges, caps)?;
        }
        Ok(())
    }
}
impl Cone<'_> {
    fn fit(&self, source: &AuthoredPiece) -> Result<&SpanFit, Failure> {
        self.input.fits.get(&source.span).ok_or_else(|| {
            Failure::new(&format!(
                "cone span={:?}; expected admitted fit",
                source.span
            ))
        })
    }
    fn cap(&self, depth: usize) -> Result<Face, Failure> {
        let CapCarrier { datum, axis } = cap_carrier(self.input, self.support.semi_angle, depth)?;
        let surface = Handle::new(GeomConicalSurface::new(
            axis,
            datum.semi_angle,
            datum.radius,
        )?)
        .into_upcast();
        let mut face = Face::default();
        BRepBuilder::default().make_face_on_surface(
            &mut face,
            &surface,
            self.input.source.tolerance / 4.,
        )?;
        Ok(face)
    }
    fn vertex(&self, point: [f64; 2], depth: usize) -> Result<Vertex, Failure> {
        let point = endpoint(
            point,
            self.support.semi_angle,
            self.support.chart_origin,
            self.input.wedge,
            self.input.source.depths[depth],
        );
        self.vertex_at(point)
    }
    fn vertex_at(&self, [x, y, z]: [f64; 3]) -> Result<Vertex, Failure> {
        let p = Point3::new(x, y, z);
        let vertex = BRepBuilderAPIMakeVertex::new(p)?.vertex()?;
        BRepBuilder::default()
            .update_vertex_tolerance(&vertex, self.input.source.tolerance / 2.)?;
        Ok(vertex)
    }
}
fn depth_curve(
    poles: &angular::RationalPoles,
    ends: [&Vertex; 2],
) -> Result<Handle<GeomBezierCurve>, Failure> {
    let mut points = poles.points.clone();
    let last = points.len() - 1;
    for (index, vertex) in [(0, ends[0]), (last, ends[1])] {
        let p = BRepTool::pnt(vertex)?;
        points[index] = [p.x(), p.y(), p.z()];
    }
    curve3(&angular::RationalPoles {
        points,
        weights: poles.weights.clone(),
    })
}
fn check_cap_poles(
    expected: &[[f64; 2]],
    edges: &[Edge; 2],
    caps: &[Face; 2],
) -> Result<(), Failure> {
    for (edge, cap) in edges.iter().zip(caps) {
        let capture = BRepTool::curve_on_face(edge, cap)?;
        let curve: Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve> = capture.pcurve.downcast();
        let Some(curve) = curve.get() else {
            return Err(Failure::new(
                "cone cap pcurve lost polynomial Bezier carrier",
            ));
        };
        let actual: Vec<_> = curve.poles().iter().map(|p| [p.x(), p.y()]).collect();
        if actual != expected {
            return Err(Failure::new(&format!(
                "cone pcurve={actual:?}; expected retained coefficients={expected:?}"
            )));
        }
    }
    Ok(())
}
pub(super) fn developed_angle([x, y]: [f64; 2], wedge: ConicalSectorWedge) -> f64 {
    let angle = atan2(if x < 0. && y == 0. { 0. } else { y }, x);
    if wedge == ConicalSectorWedge::NegativeX && angle < 0. {
        angle + std::f64::consts::TAU
    } else {
        angle
    }
}

fn cap_axis(z: f64) -> Result<Axis3, Failure> {
    Axis3::new(
        Point3::new(0., 0., z),
        Direction3::new(0., 0., 1.)?,
        Direction3::new(1., 0., 0.)?,
    )
}

pub(super) fn endpoint(
    point: [f64; 2],
    semi_angle: f64,
    chart_origin: f64,
    wedge: ConicalSectorWedge,
    depth: f64,
) -> [f64; 3] {
    let l = hypot(point[0], point[1]);
    let angle = chart_origin + developed_angle(point, wedge) / sin(semi_angle);
    let d = depth;
    let r = l * sin(semi_angle) + d * cos(semi_angle);
    [
        r * cos(angle),
        r * sin(angle),
        l * cos(semi_angle) - d * sin(semi_angle),
    ]
}

pub(super) fn cap_datum(input: &Prepared, semi_angle: f64, depth: usize) -> ConicalTrimCarrier {
    let (s, c) = (sin(semi_angle), cos(semi_angle));
    let l0 = input.reference_slant;
    let d = input.source.depths[depth];
    ConicalTrimCarrier {
        radius: l0 * s + d * c,
        axial_origin: l0 * c - d * s,
        semi_angle,
    }
}

pub(super) struct CapCarrier {
    pub datum: ConicalTrimCarrier,
    pub axis: Axis3,
}
impl CapCarrier {
    pub(super) fn frame(&self) -> [[f64; 3]; 4] {
        let origin = self.axis.location();
        let mut frame = [[origin.x(), origin.y(), origin.z()]; 4];
        for (row, direction) in frame[1..].iter_mut().zip([
            self.axis.x_direction(),
            self.axis.y_direction(),
            self.axis.direction(),
        ]) {
            *row = [direction.x(), direction.y(), direction.z()];
        }
        frame
    }
}
pub(super) fn cap_carrier(
    input: &Prepared,
    semi_angle: f64,
    depth: usize,
) -> Result<CapCarrier, Failure> {
    let datum = cap_datum(input, semi_angle, depth);
    let axis = cap_axis(datum.axial_origin)?;
    Ok(CapCarrier { datum, axis })
}
