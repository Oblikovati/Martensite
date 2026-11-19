// SPDX-License-Identifier: GPL-2.0-only
//! Native cap loops bind issued selected material without outer-wire heuristics.
use super::*;
use crate::profile_extrusion::{LoopUse, ProfileCurve, ProfileLoop};
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;

pub(super) fn capture(
    id: u64,
    face: &Face,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
) -> Result<Cap, Failure> {
    let (handle, location) = BRepTool::surface_with_location(face)?;
    let cone: Handle<GeomConicalSurface> = handle.cast();
    let cone = cone
        .get()
        .ok_or_else(|| rejected("cap requires analytic cone"))?;
    if !location.is_identity() {
        return Err(rejected("cap requires identity placement"));
    }
    selected_trim_domain(face, "cap")?;
    let (frame, datums) = carrier_values(cone);
    let forward = topods::face(&face.oriented(Orientation::Forward))?;
    let contours = contours(&forward, sides, material, curves)?;
    cap_result(id, face, frame, datums, contours)
}
fn frame(cone: &GeomConicalSurface) -> [[f64; 3]; 4] {
    let position = cone.position();
    let p = position.location();
    let directions = [
        position.x_direction(),
        position.y_direction(),
        position.direction(),
    ];
    let mut frame = [[p.x(), p.y(), p.z()]; 4];
    for (row, d) in frame[1..].iter_mut().zip(directions) {
        *row = [d.x(), d.y(), d.z()];
    }
    frame
}
fn contour(
    face: &Face,
    wire: &topods::Wire,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
) -> Result<Contour, Failure> {
    let (mut edges, uses) = contour_edges(face, wire, sides, material, curves)?;
    let role = material
        .loop_role(&ProfileLoop { uses })
        .ok_or_else(|| rejected("actual directed cap loop differs from selected material"))?;
    let first = edges
        .iter()
        .enumerate()
        .min_by_key(|(_, edge)| edge.side)
        .map(|(index, _)| index)
        .ok_or_else(|| rejected("empty actual cap contour"))?;
    edges.rotate_left(first);
    Ok(Contour {
        role,
        orientation: orientation(wire)?,
        edges,
    })
}
fn unique_edge(actual: &[Shape], edge: &topods::Edge) -> Result<usize, Failure> {
    let found: Vec<_> = actual
        .iter()
        .enumerate()
        .filter(|(_, candidate)| candidate.is_same(edge))
        .map(|(i, _)| i)
        .collect();
    if found.len() != 1 {
        return Err(rejected("ambiguous actual native cap edge occurrence"));
    }
    Ok(found[0])
}
fn curve_id(curves: &[(ProfileCurveId, u64)], side: u64) -> Result<ProfileCurveId, Failure> {
    let found: Vec<_> = curves
        .iter()
        .filter(|(_, candidate)| *candidate == side)
        .map(|(curve, _)| *curve)
        .collect();
    if found.len() != 1 {
        return Err(rejected("missing/ambiguous source curve-to-side binding"));
    }
    Ok(found[0])
}
fn edge_uv(edge: &topods::Edge, face: &Face, sides: &[Side]) -> Result<UvEdge, Failure> {
    let captured = BRepTool::curve_on_face(edge, face)?;
    if !captured.is_stored || [captured.first, captured.last] != [0., 1.] {
        return Err(rejected(
            "cap requires full unit-range stored polynomial pcurve",
        ));
    }
    let handle: Handle<Geom2dBezierCurve> = captured.pcurve.cast();
    let curve = handle
        .get()
        .ok_or_else(|| rejected("cap requires Bezier pcurve"))?;
    if curve.has_eval_representation() || !(1..=3).contains(&curve.degree()) {
        return Err(rejected(
            "cap requires polynomial degree1..3 without evaluator override",
        ));
    }
    let (poles, weights) = uv_controls(curve)?;
    let links = links(edge, sides)?;
    uv_result(edge, poles, weights, links, [captured.first, captured.last])
}
fn match_curve(
    material: &CertifiedPlanarMaterial,
    id: ProfileCurveId,
    edge: &UvEdge,
) -> Result<(), Failure> {
    let curve = material
        .profile()
        .curves
        .iter()
        .find(|curve| curve.id() == id)
        .ok_or_else(|| rejected("selected material curve missing"))?;
    match_polynomial(curve, id, edge)
}

fn contours(
    forward: &Face,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
) -> Result<Vec<Contour>, Failure> {
    let mut contours = Vec::new();
    for shape in children(forward, ShapeEnum::Wire)? {
        contours.push(contour(
            forward,
            &topods::wire(&shape)?,
            sides,
            material,
            curves,
        )?);
    }
    contours.sort_by_key(|contour| contour.role);
    contour_roles(&contours, material)?;
    Ok(contours)
}

fn contour_edges(
    face: &Face,
    wire: &topods::Wire,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
) -> Result<(Vec<UvEdge>, Vec<LoopUse>), Failure> {
    let actual = children(wire, ShapeEnum::Edge)?;
    let mut visited = vec![false; actual.len()];
    let mut explorer = WireExplorer::with_wire_on_face(wire, face)?;
    let (mut edges, mut uses) = (Vec::new(), Vec::new());
    while explorer.more() {
        let edge = explorer.current();
        let slot = unique_edge(&actual, edge)?;
        if std::mem::replace(&mut visited[slot], true) {
            return Err(rejected("repeated actual cap wire edge"));
        }
        let captured = captured_use(edge, face, sides, material, curves, &mut uses)?;
        edges.push(captured);
        explorer.next()?;
    }
    if visited.iter().any(|seen| !seen) {
        return Err(rejected("native wire explorer omitted cap edge"));
    }
    Ok((edges, uses))
}

fn captured_use(
    edge: &topods::Edge,
    face: &Face,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
    uses: &mut Vec<LoopUse>,
) -> Result<UvEdge, Failure> {
    let captured = edge_uv(edge, face, sides)?;
    let curve = curve_id(curves, captured.side)?;
    match_curve(material, curve, &captured)?;
    uses.push(LoopUse {
        curve,
        reversed: captured.reversed,
    });
    Ok(captured)
}

fn carrier_values(cone: &GeomConicalSurface) -> ([[f64; 3]; 4], [f64; 3]) {
    let frame = frame(cone);
    let datums = [
        cone.ref_radius(),
        cone.position().location().z(),
        cone.semi_angle(),
    ];
    (frame, datums)
}

fn links(edge: &topods::Edge, sides: &[Side]) -> Result<Vec<(u64, usize)>, Failure> {
    let links: Vec<_> = sides
        .iter()
        .flat_map(|side| {
            (0..2).filter_map(move |row| side.edges[row].is_same(edge).then_some((side.id, row)))
        })
        .collect();
    if links.len() != 1 {
        return Err(rejected(
            "cap edge requires exactly one actual side-row incidence",
        ));
    }
    Ok(links)
}

fn uv_controls(curve: &Geom2dBezierCurve) -> Result<(Vec<[f64; 2]>, Vec<f64>), Failure> {
    let (mut poles, mut weights) = (Vec::new(), Vec::new());
    for i in 1..=curve.degree() + 1 {
        let p = curve.pole(i)?;
        poles.push([p.x(), p.y()]);
        weights.push(curve.weight(i)?);
    }
    Ok((poles, weights))
}

fn polynomial_matches(
    id: ProfileCurveId,
    edge: &UvEdge,
    degree: u32,
    poles: &[[f64; 2]],
    knots: &[f64],
) -> Result<(), Failure> {
    let actual: Vec<_> = edge
        .poles
        .iter()
        .map(|p| [p[1].to_bits(), p[0].to_bits()])
        .collect();
    let expected: Vec<_> = poles.iter().map(|p| p.map(f64::to_bits)).collect();
    let expected_knots: Vec<_> = std::iter::repeat_n(0_f64.to_bits(), poles.len())
        .chain(std::iter::repeat_n(1_f64.to_bits(), poles.len()))
        .collect();
    if degree as usize + 1 != poles.len()
        || actual != expected
        || edge.weights.iter().any(|w| w.to_bits() != 1_f64.to_bits())
        || knots.iter().map(|x| x.to_bits()).collect::<Vec<_>>() != expected_knots
    {
        return Err(rejected(&format!(
            "actual cap curve={id:?} differs from issued polynomial material"
        )));
    }
    Ok(())
}

fn cap_result(
    id: u64,
    face: &Face,
    frame: [[f64; 3]; 4],
    datums: [f64; 3],
    contours: Vec<Contour>,
) -> Result<Cap, Failure> {
    Ok(Cap {
        id,
        orientation: orientation(face)?,
        frame,
        datums,
        contours,
    })
}

fn uv_result(
    edge: &topods::Edge,
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    links: Vec<(u64, usize)>,
    range: [f64; 2],
) -> Result<UvEdge, Failure> {
    Ok(UvEdge {
        side: links[0].0,
        row: links[0].1,
        reversed: orientation(edge)? < 0,
        poles,
        weights,
        range,
    })
}

fn match_polynomial(
    curve: &ProfileCurve,
    id: ProfileCurveId,
    edge: &UvEdge,
) -> Result<(), Failure> {
    let ProfileCurve::Spline {
        degree,
        poles,
        weights: None,
        knots,
        ..
    } = curve
    else {
        return Err(rejected(
            "selected material requires polynomial Bezier source",
        ));
    };
    polynomial_matches(id, edge, *degree, poles, knots)?;
    Ok(())
}

fn contour_roles(contours: &[Contour], material: &CertifiedPlanarMaterial) -> Result<(), Failure> {
    let mut roles: Vec<_> = contours.iter().map(|entry| entry.role).collect();
    roles.sort_unstable();
    if roles != (0..material.loops().count()).collect::<Vec<_>>() {
        return Err(rejected(
            "actual cap omits/duplicates selected bounded material loops",
        ));
    }
    Ok(())
}

pub(super) fn selected_trim_domain(face: &Face, kind: &str) -> Result<(), Failure> {
    let actual = BRepTool::natural_restriction(face)?;
    if actual {
        return Err(rejected(&format!(
            "actual natural_restriction={actual}; expected false for selected trimmed {kind} domain"
        )));
    }
    Ok(())
}
