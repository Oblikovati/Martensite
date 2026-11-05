//! Preserve the original parameter chart and the geometry-only dispatch of plain rotate.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkmath::gp::{Axis1, Axis3, Cone3, Direction3, Point3};
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeFace, brep_lib::BRepLibMakeEdge};
use std::f64::consts::{FRAC_PI_4, PI};

fn conical_face(v_min: f64, v_max: f64) -> Result<Shape, Failure> {
    let cone = Cone3::new(Axis3::default(), FRAC_PI_4, 0.)?;
    Ok(
        BRepBuilderAPIMakeFace::from_cone_bounds(cone, (0., 2. * PI, v_min, v_max))?
            .face()?
            .into_shape(),
    )
}

/// Keep the native origin and trim range instead of rebuilding from endpoints.
/// Example: `case_line_edge_28054(Point3::default(), Direction3::Z)`.
pub(super) fn case_line_edge_28054(point: Point3, direction: Direction3) -> Result<Shape, Failure> {
    let basis: Handle<dyn GeomCurve> =
        Handle::new(GeomLine::new(Axis1::new(point, direction))).cast();
    let trimmed: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&basis, -100., 100., true, true)?).cast();
    Ok(BRepLibMakeEdge::from_curve(&trimmed)?.edge()?.into_shape())
}

/// Preserve face→line-edge→copy order and actual topology-command copy identity; `case_inputs_28054()`.
pub(super) fn case_inputs_28054() -> Result<([Shape; 4], [Shape; 2]), Failure> {
    let first = conical_face(5., 10.)?;
    let second = conical_face(-10., -5.)?;
    let edges = [
        case_line_edge_28054(Point3::default(), Direction3::new(1., 0., 1.)?)?,
        case_line_edge_28054(Point3::default(), Direction3::new(0., -1., -1.)?)?,
    ];
    // TKBOOL-28054-ROTATE: plain rotate resolves geometry-command geometry, never topology-command shapes.
    // Thus the literal32/81degree requests leave copy f1→f3 and copy f2→f4 unchanged.
    let faces = [first.clone(), second.clone(), first, second];
    Ok((faces, edges))
}
