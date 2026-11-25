// SPDX-License-Identifier: GPL-2.0-only
//! Distinguish one native edge from its two directed uses on a periodic face.
use super::*;
use martensite_tkbrep::brep::EdgePCurve;
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkmath::{geom_abs::CurveType, gp::Point2};
use std::f64::consts::{FRAC_PI_2, PI};

struct FaceTrims {
    rims: [topods::Edge; 2],
    seam: Option<[topods::Edge; 2]>,
}

pub(super) fn verify(
    section: &BandSection,
    bindings: &Bindings,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    rims: [SurfaceCurveId; 2],
    index: usize,
) -> Checked<()> {
    let periodic = carrier_type(section.cylinder, index) != SurfaceType::Plane;
    let sweep = &section.sweep;
    let [lower, upper] = rim_keys(rims, sweep.first_depth, index).map(|rim| lookup(bindings, rim));
    let seam = if periodic {
        let key = face_key(sweep.region, rims, sweep.first_depth, index);
        Some(lookup(bindings, seam_key(key))?)
    } else {
        None
    };
    let trims = occurrences(face, [lower?, upper?], seam)?;
    for rim in &trims.rims {
        verify_rim_trim(section, face, surface, rim, periodic)?;
    }
    if let Some([first, second]) = trims.seam {
        verify_seam(section, face, surface, first, second, index)?;
    }
    Ok(())
}

fn occurrences(
    face: &topods::Face,
    rims: [&topods::Shape; 2],
    seam: Option<&topods::Shape>,
) -> Checked<FaceTrims> {
    verify_wire_partition(face, seam.is_some())?;
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("full-turn coedges", e))?;
    let mut found = [Vec::new(), Vec::new(), Vec::new()];
    while explorer.more() {
        let edge = topods::edge(explorer.current()).map_err(|e| native("full-turn coedge", e))?;
        let index = occurrence_owner(&edge, rims, seam)?;
        found[index].push(edge);
        explorer
            .next()
            .map_err(|e| native("next full-turn coedge", e))?;
    }
    let [lower, upper, paired] = found;
    finish_occurrences(lower, upper, paired, seam.is_some())
}

fn verify_wire_partition(face: &topods::Face, periodic: bool) -> Checked<()> {
    let mut wires = ShapeIterator::new(face, false, false);
    let mut count = 0;
    while wires.more() {
        let wire = wires
            .value()
            .map_err(|e| native("full-turn face wire", e))?;
        verify_wire(wire, if periodic { 4 } else { 1 })?;
        wires.next();
        count += 1;
    }
    if count != if periodic { 1 } else { 2 } {
        return Err(incomplete(
            "one seam-cut loop or two planar rim loops",
            format!("{count} loops"),
        ));
    }
    Ok(())
}

fn verify_wire(wire: &topods::Shape, expected_uses: usize) -> Checked<()> {
    let kind = wire
        .shape_type()
        .map_err(|e| native("full-turn wire kind", e))?;
    if kind != ShapeEnum::Wire {
        return Err(incomplete("full-turn trim wire", format!("{kind:?}")));
    }
    let mut uses = Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("full-turn wire edges", e))?;
    let mut count = 0;
    while uses.more() {
        uses.next()
            .map_err(|e| native("next full-turn wire edge", e))?;
        count += 1;
    }
    if count != expected_uses {
        return Err(incomplete(
            "full-turn wire edge uses",
            format!("{count} uses, expected {expected_uses}"),
        ));
    }
    Ok(())
}

fn occurrence_owner(
    edge: &topods::Edge,
    rims: [&topods::Shape; 2],
    seam: Option<&topods::Shape>,
) -> Checked<usize> {
    if let Some(index) = rims.into_iter().position(|rim| rim.is_same(edge)) {
        return Ok(index);
    }
    if seam.is_some_and(|same| same.is_same(edge)) {
        return Ok(2);
    }
    Err(incomplete(
        "bound rim or periodic seam",
        format!("{:?}", edge.orientation()),
    ))
}

fn finish_occurrences(
    lower: Vec<topods::Edge>,
    upper: Vec<topods::Edge>,
    paired: Vec<topods::Edge>,
    periodic: bool,
) -> Checked<FaceTrims> {
    let [lower] = cardinality(lower, "one use of the first rim")?;
    let [upper] = cardinality(upper, "one use of the second rim")?;
    let seam = if periodic {
        Some(cardinality(paired, "two uses of the periodic seam")?)
    } else {
        let [] = cardinality(paired, "zero planar seam uses")?;
        None
    };
    Ok(FaceTrims {
        rims: [lower, upper],
        seam,
    })
}

fn cardinality<const N: usize>(
    items: Vec<topods::Edge>,
    expected: &'static str,
) -> Checked<[topods::Edge; N]> {
    items
        .try_into()
        .map_err(|items: Vec<_>| incomplete(expected, format!("{} uses", items.len())))
}

fn verify_rim_trim(
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
    opposite(&first, &second)?;
    verify_seam_line(section, &first, index)?;
    let left = seam_trim(face, &first)?;
    let right = seam_trim(face, &second)?;
    verify_period_pair(section, surface, &left, &right)?;
    verify_representation(section, surface, &first, &left)?;
    verify_representation(section, surface, &second, &right)
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

fn verify_seam_line(section: &BandSection, edge: &topods::Edge, index: usize) -> Checked<()> {
    let curve = BRepAdaptorCurve::with_edge(edge).map_err(|e| native("full-turn seam curve", e))?;
    check_curve_kind(&curve, false)?;
    let a = section.sweep.points[index];
    let b = section.sweep.points[(index + 1) % 4];
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

fn verify_representation(
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

fn trim_uv(trim: &EdgePCurve, parameter: f64) -> Checked<Point2> {
    trim.pcurve
        .get()
        .ok_or_else(|| incomplete("analytic full-turn trim", "null pcurve"))?
        .value(parameter)
        .map_err(|e| native("full-turn pcurve evaluation", e))
}
