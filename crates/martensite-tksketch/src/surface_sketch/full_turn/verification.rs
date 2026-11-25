// SPDX-License-Identifier: GPL-2.0-only
//! Bind each analytic carrier, closed rim and directed seam to its meridional source.
use super::{admission::BandSection, history::*};
use crate::surface_sketch::{admission as cylinder, semantic};
use crate::surface_sketch::{failure::*, frame, history::*, request::*, semantic::*};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topexp::{self, Explorer},
    topods,
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis1, Point3},
};
use std::f64::consts::TAU;

mod periodic_cut;
mod trim;

pub(super) use periodic_cut::unique_kind;
pub(super) use periodic_cut::validate_crossed;
pub(super) use trim::{verify_rim_trim, verify_seam_fragment};

pub(super) fn validate(
    section: &BandSection,
    rims: [SurfaceCurveId; 2],
    bindings: &Bindings,
) -> Checked<()> {
    verify_band(section, rims, bindings, None)
}

pub(super) fn validate_aperture(
    section: &BandSection,
    rims: [SurfaceCurveId; 2],
    aperture: &cylinder::Admitted,
    bindings: &Bindings,
) -> Checked<()> {
    let rectangle = &aperture.rectangle;
    let hole: Bindings = bindings
        .iter()
        .filter(|(key, _)| match key.source {
            HistorySource::Region(_) => false,
            HistorySource::Curve(curve) => rectangle.boundaries.contains(&curve),
            HistorySource::Vertex(vertex) => rectangle.corners.contains(&vertex),
        })
        .cloned()
        .collect();
    semantic::validate(aperture, &hole)?;
    trim::verify_aperture_walls(section, &hole)?;
    verify_band(section, rims, bindings, Some(rectangle.boundaries))
}

fn verify_band(
    section: &BandSection,
    rims: [SurfaceCurveId; 2],
    bindings: &Bindings,
    aperture: Option<[SurfaceCurveId; 4]>,
) -> Checked<()> {
    for index in 0..4 {
        verify_face(section, rims, bindings, index, aperture)?;
        let rim = rim_keys(rims, section.sweep.first_depth, index)[0];
        verify_rim(section, bindings, rim, index)?;
    }
    Ok(())
}

pub(super) fn verify_face(
    section: &BandSection,
    rims: [SurfaceCurveId; 2],
    bindings: &Bindings,
    index: usize,
    aperture: Option<[SurfaceCurveId; 4]>,
) -> Checked<()> {
    let key = face_key(section.sweep.region, rims, section.sweep.first_depth, index);
    let face = topods::face(lookup(bindings, key)?).map_err(|e| native("full-turn face", e))?;
    let surface =
        BRepAdaptorSurface::with_face(&face, true).map_err(|e| native("full-turn surface", e))?;
    let carrier = carrier_type(section.cylinder, index);
    require_surface(&surface, carrier)?;
    verify_carrier(section, &surface, carrier, index)?;
    trim::verify(section, bindings, &face, &surface, rims, index, aperture)
}

fn carrier_type(cylinder: bool, index: usize) -> SurfaceType {
    if !cylinder {
        SurfaceType::Cone
    } else if index.is_multiple_of(2) {
        SurfaceType::Plane
    } else {
        SurfaceType::Cylinder
    }
}

fn verify_carrier(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    carrier: SurfaceType,
    index: usize,
) -> Checked<()> {
    match carrier {
        SurfaceType::Plane => verify_plane(section, surface, index),
        SurfaceType::Cylinder => verify_cylinder(section, surface, index),
        SurfaceType::Cone => verify_cone(section, surface, index),
        _ => Err(incomplete(
            "analytic meridian carrier",
            format!("{carrier:?}"),
        )),
    }
}

fn verify_plane(section: &BandSection, surface: &BRepAdaptorSurface, index: usize) -> Checked<()> {
    let plane = surface.plane().map_err(|e| native("full-turn plane", e))?;
    axis_direction(section, plane.axis())?;
    close(
        plane_distance(plane, section.sweep.points[index]),
        0.,
        tolerance(section),
        "rim plane",
    )
}

pub(super) fn verify_cylinder(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    index: usize,
) -> Checked<()> {
    let cylinder = surface
        .cylinder()
        .map_err(|e| native("full-turn cylinder", e))?;
    coaxial(section, cylinder.axis())?;
    close(
        cylinder.radius(),
        section.meridian[index][0],
        tolerance(section),
        "depth cylinder radius",
    )
}

pub(super) fn verify_cone(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    index: usize,
) -> Checked<()> {
    let cone = surface.cone().map_err(|e| native("full-turn cone", e))?;
    coaxial(section, cone.axis())?;
    let [r0, z0] = section.meridian[index];
    let [r1, z1] = section.meridian[(index + 1) % 4];
    let slope = (z1 - z0) / (r1 - r0);
    let angle = martensite_cmath::atan2((r1 - r0).abs(), (z1 - z0).abs());
    close(
        cone.semi_angle().abs(),
        angle,
        angular(section),
        "cone carrier semi-angle",
    )?;
    let apex = axis_point(section, z0 - r0 * slope);
    close(
        distance(cone.apex(), apex) * martensite_cmath::sin(angle),
        0.,
        tolerance(section),
        "cone apex contribution to carrier offset",
    )
}

pub(super) fn verify_rim(
    section: &BandSection,
    bindings: &Bindings,
    key: HistoryKey,
    index: usize,
) -> Checked<()> {
    let edge = topods::edge(lookup(bindings, key)?).map_err(|e| native("full-turn rim", e))?;
    let curve = BRepAdaptorCurve::with_edge(&edge).map_err(|e| native("full-turn rim curve", e))?;
    check_curve_kind(&curve, true)?;
    close(
        curve_span(&curve)?,
        TAU,
        angular(section),
        "closed rim parameter span",
    )?;
    let circle = curve.circle().map_err(|e| native("full-turn circle", e))?;
    axis_direction(section, circle.axis())?;
    let [radius, axial] = section.meridian[index];
    close(circle.radius(), radius, tolerance(section), "rim radius")?;
    close(
        distance(circle.location(), axis_point(section, axial)),
        0.,
        tolerance(section),
        "rim centre",
    )?;
    verify_anchor(section, &edge, bindings, key, index)
}

fn verify_anchor(
    section: &BandSection,
    edge: &topods::Edge,
    bindings: &Bindings,
    key: HistoryKey,
    index: usize,
) -> Checked<()> {
    let anchor_key = HistoryKey {
        source: key.source,
        role: vertex_role(usize::from(key.role == HistoryRole::LastDepthEdge)),
    };
    let anchor =
        topods::vertex(lookup(bindings, anchor_key)?).map_err(|e| native("rim anchor", e))?;
    if !anchor.is_same(&topexp::first_vertex(edge, true))
        || !anchor.is_same(&topexp::last_vertex(edge, true))
    {
        return Err(incomplete(
            "same closed rim anchor at both ends",
            format!("{key:?}"),
        ));
    }
    let found = BRepTool::pnt(&anchor).map_err(|e| native("rim anchor point", e))?;
    close(
        distance(found, section.sweep.points[index]),
        0.,
        tolerance(section),
        "rim anchor point",
    )
}

fn coaxial(section: &BandSection, axis: Axis1) -> Checked<()> {
    axis_direction(section, axis)?;
    let delta = delta(axis.location(), section.sweep.origin);
    let perpendicular = frame::cross(delta, section.sweep.axis);
    close(
        frame::dot(perpendicular, perpendicular).sqrt(),
        0.,
        tolerance(section),
        "carrier axis location",
    )
}

fn axis_direction(section: &BandSection, axis: Axis1) -> Checked<()> {
    let angle = unoriented_angle(*axis.direction().coordinates(), section.sweep.axis)?;
    close(angle, 0., angular(section), "carrier axis direction")
}

fn axis_point(section: &BandSection, axial: f64) -> [f64; 3] {
    std::array::from_fn(|i| section.sweep.origin[i] + axial * section.sweep.axis[i])
}

fn delta(point: Point3, origin: [f64; 3]) -> [f64; 3] {
    [
        point.x() - origin[0],
        point.y() - origin[1],
        point.z() - origin[2],
    ]
}

pub(super) fn tolerance(section: &BandSection) -> f64 {
    section.sweep.options.resolution.weld + section.sweep.coordinate_error
}

pub(super) fn angular(section: &BandSection) -> f64 {
    section.sweep.options.resolution.angular
}
