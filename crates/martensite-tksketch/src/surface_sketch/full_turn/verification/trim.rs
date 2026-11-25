// SPDX-License-Identifier: GPL-2.0-only
//! Distinguish one native edge from its two directed uses on a periodic face.
use super::*;
use martensite_tkbrep::brep::EdgePCurve;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkmath::{geom_abs::CurveType, gp::Point2};
use std::f64::consts::{FRAC_PI_2, PI};

mod aperture;
mod partition;

pub(super) use aperture::{
    increasing, verify_aperture_loop, verify_aperture_wall, verify_aperture_walls,
};

pub(super) fn verify(
    section: &BandSection,
    bindings: &Bindings,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    rims: [SurfaceCurveId; 2],
    index: usize,
    aperture: Option<[SurfaceCurveId; 4]>,
) -> Checked<()> {
    let periodic = carrier_type(section.cylinder, index) != SurfaceType::Plane;
    let names = trim_names(section, bindings, rims, index, periodic, aperture)?;
    let trims = partition::classify(face, names)?;
    for rim in &trims.rims {
        verify_rim_trim(section, face, surface, rim, periodic)?;
    }
    if let Some([first, second]) = trims.seam {
        verify_seam(section, face, surface, first, second, index)?;
    }
    if let Some(hole) = trims.hole {
        aperture::verify_wire(
            section,
            face,
            surface,
            &trims.rims[usize::from(index == 3)],
            &hole,
        )?;
    }
    Ok(())
}

fn trim_names<'a>(
    section: &BandSection,
    bindings: &'a Bindings,
    rims: [SurfaceCurveId; 2],
    index: usize,
    periodic: bool,
    aperture: Option<[SurfaceCurveId; 4]>,
) -> Checked<partition::Names<'a>> {
    let sweep = &section.sweep;
    let [lower, upper] = rim_keys(rims, sweep.first_depth, index).map(|rim| lookup(bindings, rim));
    let seam = if periodic {
        let key = face_key(sweep.region, rims, sweep.first_depth, index);
        Some(lookup(bindings, seam_key(key))?)
    } else {
        None
    };
    let hole = hole_bindings(bindings, aperture, index, sweep.first_depth)?;
    Ok(partition::Names {
        rims: [lower?, upper?],
        seam,
        hole,
    })
}

fn hole_bindings(
    bindings: &Bindings,
    aperture: Option<[SurfaceCurveId; 4]>,
    index: usize,
    first_depth: usize,
) -> Checked<Option<[&topods::Shape; 4]>> {
    let Some(curves) = aperture.filter(|_| !index.is_multiple_of(2)) else {
        return Ok(None);
    };
    let depth = if index == 3 {
        first_depth
    } else {
        1 - first_depth
    };
    let [a, b, c, d] = curves.map(|curve| {
        lookup(
            bindings,
            HistoryKey {
                source: HistorySource::Curve(curve),
                role: edge_role(depth),
            },
        )
    });
    Ok(Some([a?, b?, c?, d?]))
}

pub(in crate::surface_sketch::full_turn) fn verify_rim_trim(
    section: &BandSection,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    edge: &topods::Edge,
    periodic: bool,
) -> Checked<()> {
    let stored =
        BRepTool::curve_on_face(edge, face).map_err(|e| native("full-turn rim pcurve", e))?;
    if !rim_carrier(&stored.pcurve, periodic) {
        return Err(incomplete(
            "analytic closed rim trim",
            format!("periodic: {periodic}"),
        ));
    }
    close(
        stored.last - stored.first,
        TAU,
        angular(section),
        "closed rim trim span",
    )?;
    verify_representation(section, surface, edge, &stored)
}

fn rim_carrier(curve: &Handle<dyn Geom2dCurve>, periodic: bool) -> bool {
    if periodic {
        curve
            .downcast_ref::<Geom2dLine>()
            .is_some_and(|line| line.direction().y() == 0. && line.direction().x().abs() == 1.)
    } else {
        curve.downcast_ref::<Geom2dCircle>().is_some()
    }
}

fn verify_seam(
    section: &BandSection,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    first: topods::Edge,
    second: topods::Edge,
    index: usize,
) -> Checked<()> {
    let points = section.sweep.points;
    verify_seam_fragment(
        section,
        face,
        surface,
        [&first, &second],
        [points[index], points[(index + 1) % 4]],
    )
}

pub(in crate::surface_sketch::full_turn) fn verify_seam_fragment(
    section: &BandSection,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    [first, second]: [&topods::Edge; 2],
    endpoints: [[f64; 3]; 2],
) -> Checked<()> {
    opposite(first, second)?;
    verify_seam_line(section, first, endpoints)?;
    let left = seam_trim(face, first)?;
    let right = seam_trim(face, second)?;
    verify_period_pair(section, surface, &left, &right)?;
    verify_representation(section, surface, first, &left)?;
    verify_representation(section, surface, second, &right)
}

fn opposite(first: &topods::Edge, second: &topods::Edge) -> Checked<()> {
    let pair = [first.orientation(), second.orientation()];
    if !matches!(
        pair,
        [Orientation::Forward, Orientation::Reversed]
            | [Orientation::Reversed, Orientation::Forward]
    ) {
        return Err(incomplete(
            "opposite coedges on one periodic seam",
            format!("{pair:?}"),
        ));
    }
    Ok(())
}

fn verify_seam_line(
    section: &BandSection,
    edge: &topods::Edge,
    [a, b]: [[f64; 3]; 2],
) -> Checked<()> {
    let curve = BRepAdaptorCurve::with_edge(edge).map_err(|e| native("full-turn seam curve", e))?;
    check_curve_kind(&curve, false)?;
    let middle = std::array::from_fn(|i| a[i] * 0.5 + b[i] * 0.5);
    check_edge_samples(&curve, [a, b, middle], tolerance(section))
}

fn seam_trim(face: &topods::Face, edge: &topods::Edge) -> Checked<EdgePCurve> {
    let trim =
        BRepTool::curve_on_face(edge, face).map_err(|e| native("full-turn seam pcurve", e))?;
    let line = trim.pcurve.downcast_ref::<Geom2dLine>();
    if !trim.is_stored
        || !line.is_some_and(|line| line.direction().x() == 0. && line.direction().y().abs() == 1.)
    {
        return Err(incomplete(
            "stored iso-U seam trim",
            format!("{:?}", edge.orientation()),
        ));
    }
    Ok(trim)
}

fn verify_period_pair(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    first: &EdgePCurve,
    second: &EdgePCurve,
) -> Checked<()> {
    let u0 = surface
        .first_u_parameter()
        .map_err(|e| native("first full-turn U", e))?;
    let u1 = surface
        .last_u_parameter()
        .map_err(|e| native("last full-turn U", e))?;
    close(u1 - u0, TAU, angular(section), "full-turn face period")?;
    for parameter in [first.first, first.last] {
        let a = trim_uv(first, parameter)?;
        let b = trim_uv(second, parameter)?;
        close(a.y(), b.y(), tolerance(section), "paired seam V")?;
        close(a.x().min(b.x()), u0, angular(section), "lower seam lift")?;
        close(a.x().max(b.x()), u1, angular(section), "upper seam lift")?;
    }
    Ok(())
}

pub(in crate::surface_sketch::full_turn) fn verify_representation(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    edge: &topods::Edge,
    pcurve: &EdgePCurve,
) -> Checked<()> {
    let curve = BRepAdaptorCurve::with_edge(edge).map_err(|e| native("full-turn trim edge", e))?;
    let first = curve
        .first_parameter()
        .map_err(|e| native("first trim parameter", e))?;
    let last = curve
        .last_parameter()
        .map_err(|e| native("last trim parameter", e))?;
    if [first, last] != [pcurve.first, pcurve.last] {
        return Err(incomplete(
            "equal 3D and pcurve intervals",
            format!("{first}..{last} vs {}..{}", pcurve.first, pcurve.last),
        ));
    }
    let residual = |t| trim_residual(&curve, surface, pcurve, t);
    let bound = if curve
        .get_type()
        .map_err(|e| native("full-turn trim edge type", e))?
        == CurveType::Circle
    {
        first_harmonic_bound(
            residual(first)?,
            residual(first + FRAC_PI_2)?,
            residual(first + PI)?,
        )
    } else {
        norm(residual(first)?).max(norm(residual(last)?))
    };
    close(
        bound + section.sweep.coordinate_error,
        0.,
        section.sweep.options.resolution.weld,
        "whole analytic edge/trim correspondence",
    )
}

fn trim_residual(
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
    pcurve: &EdgePCurve,
    parameter: f64,
) -> Checked<[f64; 3]> {
    let value = curve
        .value(parameter)
        .map_err(|e| native("full-turn edge evaluation", e))?;
    let uv = trim_uv(pcurve, parameter)?;
    let lifted = surface
        .value(uv.x(), uv.y())
        .map_err(|e| native("full-turn trim evaluation", e))?;
    Ok([
        value.x() - lifted.x(),
        value.y() - lifted.y(),
        value.z() - lifted.z(),
    ])
}

fn first_harmonic_bound(start: [f64; 3], quarter: [f64; 3], half: [f64; 3]) -> f64 {
    // The checked circle/iso-V compositions contain only one harmonic.
    // These parameters recover its constant, cosine and sine coefficients.
    let center = std::array::from_fn(|i| (start[i] + half[i]) / 2.);
    let cosine = std::array::from_fn(|i| (start[i] - half[i]) / 2.);
    let sine = std::array::from_fn(|i| quarter[i] - center[i]);
    norm(center) + norm(cosine) + norm(sine)
}

fn norm(x: [f64; 3]) -> f64 {
    frame::dot(x, x).sqrt()
}

pub(super) fn trim_uv(trim: &EdgePCurve, parameter: f64) -> Checked<Point2> {
    trim.pcurve
        .get()
        .ok_or_else(|| incomplete("analytic full-turn trim", "null pcurve"))?
        .value(parameter)
        .map_err(|e| native("full-turn pcurve evaluation", e))
}
