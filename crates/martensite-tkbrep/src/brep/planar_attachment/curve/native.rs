// SPDX-License-Identifier: GPL-2.0-only
//! Create and read back a bounded native edge with two synchronized curve representations.
use super::{DetachedPlanarCurve, FaceAttachmentCurveError as Error, PlanarSupport};
use crate::{
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::{Edge, Face, Vertex},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::{
    geom::{GeomBezierCurve, GeomCurve, GeomPlane, GeomSurface},
    top_abs::Orientation,
};
use martensite_tkmath::{
    gp::{Point2, Point3},
    toploc::Location,
};

type CurveControls<const N: usize> = ([[f64; 3]; N], [[f64; 2]; N]);

pub(in crate::brep::planar_attachment) fn build<const N: usize>(
    chart: PlanarSupport,
    controls: CurveControls<N>,
    distance: f64,
    limit: f64,
) -> Result<(DetachedPlanarCurve, f64), Error> {
    let native = materialize(chart, controls, distance)?;
    verify_controls(&native, controls)?;
    let tolerance = native.verify_boundary(boundary(controls.0)?, chart)?;
    if tolerance > limit {
        return Err(Error::Tolerance {
            effective: tolerance,
            limit,
        });
    }
    Ok((native, tolerance))
}

fn verify_controls<const N: usize>(
    native: &DetachedPlanarCurve,
    expected: CurveControls<N>,
) -> Result<(), Error> {
    let actual = native.controls::<N>()?;
    if actual.0.map(bits) != expected.0.map(bits) || actual.1.map(bits) != expected.1.map(bits) {
        return Err(Error::Representation);
    }
    Ok(())
}

fn materialize<const N: usize>(
    chart: PlanarSupport,
    (world, uv): CurveControls<N>,
    distance: f64,
) -> Result<DetachedPlanarCurve, Error> {
    let points = world.map(point);
    let parameters = uv.map(|[u, v]| Point2::new(u, v));
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomBezierCurve::new(&Array1::from_shared(&points, 1))?).into_upcast();
    let pcurve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dBezierCurve::new(
        &Array1::from_shared(&parameters, 1),
    )?)
    .into_upcast();
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    let support = make_support(&builder, chart, distance)?;
    builder.make_edge_with_curve(&mut edge, &curve, distance)?;
    builder.update_edge_pcurve_on_face(&edge, &pcurve, &support, distance)?;
    builder.range(&edge, 0., 1., false)?;
    add_ends(&builder, &mut edge, boundary(points)?, distance)?;
    Ok(DetachedPlanarCurve { edge, support })
}

fn make_support(
    builder: &BRepBuilder,
    chart: PlanarSupport,
    tolerance: f64,
) -> Result<Face, Error> {
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(chart.frame)).into_upcast();
    let location = Location::new(chart.position);
    let mut support = Face::default();
    builder.make_face_on_located_surface(&mut support, &surface, &location, tolerance)?;
    Ok(support)
}

fn add_ends(
    builder: &BRepBuilder,
    edge: &mut Edge,
    points: [Point3; 2],
    tolerance: f64,
) -> Result<(), Error> {
    for (point, orientation, parameter) in [
        (points[0], Orientation::Forward, 0.),
        (points[1], Orientation::Reversed, 1.),
    ] {
        let mut vertex = Vertex::default();
        builder.make_vertex_at(&mut vertex, point, tolerance)?;
        vertex.set_orientation(orientation);
        builder.add(edge, &vertex)?;
        builder.update_vertex_on_edge(&vertex, parameter, edge, tolerance)?;
    }
    Ok(())
}

impl DetachedPlanarCurve {
    pub(super) fn controls<const N: usize>(&self) -> Result<CurveControls<N>, Error> {
        let world = BRepTool::curve_with_location(&self.edge)?;
        let planar = BRepTool::curve_on_face(&self.edge, &self.support)?;
        if !world.location.is_identity()
            || !planar.is_stored
            || [world.first, world.last] != [0., 1.]
            || [planar.first, planar.last] != [0., 1.]
        {
            return Err(Error::Representation);
        }
        let degree = N
            .checked_sub(1)
            .and_then(|degree| i32::try_from(degree).ok())
            .ok_or(Error::Representation)?;
        let curve = world.curve.downcast::<GeomBezierCurve>();
        let pcurve = planar.pcurve.downcast::<Geom2dBezierCurve>();
        let curve = curve
            .get()
            .filter(|c| c.degree() == degree && !c.is_rational())
            .ok_or(Error::Representation)?;
        let pcurve = pcurve
            .get()
            .filter(|c| c.degree() == degree && !c.is_rational())
            .ok_or(Error::Representation)?;
        let mut world = [[0.; 3]; N];
        let mut uv = [[0.; 2]; N];
        for (i, (xyz, xy)) in world.iter_mut().zip(&mut uv).enumerate() {
            let pole = i32::try_from(i + 1).map_err(|_| Error::Representation)?;
            *xyz = *curve.pole(pole)?.coordinates();
            *xy = *pcurve.pole(pole)?.xy().coordinates();
        }
        Ok((world, uv))
    }

    fn verify_boundary(&self, world: [[f64; 3]; 2], chart: PlanarSupport) -> Result<f64, Error> {
        let [first, last] = vertices(&self.edge);
        if first.is_null()
            || last.is_null()
            || !BRepTool::same_range(&self.edge)?
            || !BRepTool::same_parameter(&self.edge)?
            || BRepTool::degenerated(&self.edge)?
        {
            return Err(Error::Representation);
        }
        let parameters = [
            BRepTool::parameter(&first, &self.edge)?,
            BRepTool::parameter(&last, &self.edge)?,
        ];
        let actual = [BRepTool::pnt(&first)?, BRepTool::pnt(&last)?];
        if parameters != [0., 1.] || actual.map(|p| bits(*p.coordinates())) != world.map(bits) {
            return Err(Error::Representation);
        }
        self.verify_support(chart)?;
        Ok(BRepTool::edge_tolerance(&self.edge)?
            .max(BRepTool::vertex_tolerance(&first)?)
            .max(BRepTool::vertex_tolerance(&last)?))
    }

    fn verify_support(&self, chart: PlanarSupport) -> Result<(), Error> {
        let (surface, location) = BRepTool::surface_with_location(&self.support)?;
        let surface = surface.downcast::<GeomPlane>();
        let axis = surface.get().ok_or(Error::Representation)?.pln().position();
        if frame_bits(axis) != frame_bits(chart.frame)
            || placement_bits(location.transformation())? != placement_bits(chart.position)?
        {
            return Err(Error::Representation);
        }
        Ok(())
    }
}

fn boundary<T: Copy, const N: usize>(controls: [T; N]) -> Result<[T; 2], Error> {
    let first = controls.first().ok_or(Error::Representation)?;
    let last = controls.last().ok_or(Error::Representation)?;
    Ok([*first, *last])
}

fn vertices(edge: &Edge) -> [Vertex; 2] {
    let (first, last) = topexp::vertices(edge, true);
    [first, last]
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn bits<const N: usize>(values: [f64; N]) -> [u64; N] {
    values.map(f64::to_bits)
}

fn frame_bits(axis: martensite_tkmath::gp::Axis3) -> [[u64; 3]; 4] {
    [
        *axis.location().coordinates(),
        *axis.x_direction().coordinates(),
        *axis.y_direction().coordinates(),
        *axis.direction().coordinates(),
    ]
    .map(bits)
}

fn placement_bits(position: martensite_tkmath::gp::Transform3) -> Result<[[u64; 4]; 3], Error> {
    position
        .checked_unit_placement_rows()
        .map(|rows| rows.map(bits))
        .map_err(|_| Error::Representation)
}

#[cfg(test)]
mod cubic_tests {
    use super::*;
    use martensite_tkmath::gp::{Axis3, Transform3};

    const UV: [[f64; 2]; 4] = [[-3., 2.75], [-1., 0.75], [1., 0.75], [3., 2.75]];

    fn attached() -> (DetachedPlanarCurve, CurveControls<4>) {
        let chart = PlanarSupport {
            frame: Axis3::default(),
            position: Transform3::default(),
        };
        let controls = (UV.map(|[u, v]| [u, v, 0.]), UV);
        let (result, _) = build(chart, controls, 0., 1e-6).unwrap();
        (result, controls)
    }

    #[test]
    fn retained_cubic_control_and_range_witness_are_independent() {
        let (cubic, controls) = attached();
        let trace = BRepTool::curve_on_face(&cubic.edge, &cubic.support).unwrap();
        let trace = trace.pcurve.downcast::<Geom2dBezierCurve>();
        trace
            .get()
            .unwrap()
            .set_pole(2, Point2::new(-1., 0.))
            .unwrap();
        assert!(matches!(
            verify_controls(&cubic, controls),
            Err(Error::Representation)
        ));

        let (cubic, controls) = attached();
        BRepBuilder::new()
            .range(&cubic.edge, 0., 0.5, false)
            .unwrap();
        assert!(matches!(
            verify_controls(&cubic, controls),
            Err(Error::Representation)
        ));
    }
}
