// SPDX-License-Identifier: GPL-2.0-only
//! Positive common rows and a complete actual unit-rectangle boundary.
use super::*;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;

pub(super) fn capture(id: u64, face: &Face) -> Result<SideGeometry, Failure> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let handle: Handle<GeomBezierSurface> = surface.cast();
    let surface = handle
        .get()
        .ok_or_else(|| rejected("side carrier is not Bezier"))?;
    if !location.is_identity() || surface.has_eval_representation() || surface.nb_v_poles() != 2 {
        return Err(rejected(
            "side requires identity placement, two rows and no evaluator override",
        ));
    }
    super::cap::selected_trim_domain(face, "side")?;
    let rows = [row(surface, 1)?, row(surface, 2)?];
    let weights = weights(surface)?;
    let side = side_value(id, face, rows, weights)?;
    curves::verify(&side)?;
    Ok(side)
}

fn row(surface: &GeomBezierSurface, v: i32) -> Result<Vec<[f64; 3]>, Failure> {
    (1..=surface.nb_u_poles())
        .map(|u| {
            let p = surface.pole(u, v)?;
            let xyz = [p.x(), p.y(), p.z()];
            if xyz.iter().any(|value| !value.is_finite()) {
                return Err(rejected("nonfinite actual side control"));
            }
            Ok(xyz)
        })
        .collect()
}
fn weights(surface: &GeomBezierSurface) -> Result<Vec<f64>, Failure> {
    (1..=surface.nb_u_poles())
        .map(|u| {
            let a = surface.weight(u, 1)?;
            let b = surface.weight(u, 2)?;
            if !a.is_finite() || a <= 0. || a.to_bits() != b.to_bits() {
                return Err(rejected(&format!(
                    "side weights={a},{b}; expected positive bit-identical rows"
                )));
            }
            Ok(a)
        })
        .collect()
}
fn rectangle(face: &Face) -> Result<[Shape; 4], Failure> {
    if children(face, ShapeEnum::Wire)?.len() != 1 {
        return Err(rejected(
            "actual side requires exactly one unit-rectangle wire",
        ));
    }
    let mut slots: [Option<Shape>; 4] = std::array::from_fn(|_| None);
    for shape in children(face, ShapeEnum::Edge)? {
        let edge = topods::edge(&shape)?;
        let role = role(&edge, face)?;
        if slots[role].replace(shape).is_some() {
            return Err(rejected("duplicate actual unit rectangle boundary"));
        }
    }
    let found: Vec<_> = slots
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| rejected("missing actual unit rectangle boundary"))?;
    found
        .try_into()
        .map_err(|_| rejected("expected four actual rectangle edges"))
}
fn role(edge: &topods::Edge, face: &Face) -> Result<usize, Failure> {
    let captured = BRepTool::curve_on_face(edge, face)?;
    if !captured.is_stored || [captured.first, captured.last] != [0., 1.] {
        return Err(rejected("side requires stored full unit-range pcurve"));
    }
    let handle: Handle<Geom2dBezierCurve> = captured.pcurve.cast();
    let curve = handle
        .get()
        .ok_or_else(|| rejected("side pcurve requires affine Bezier"))?;
    if curve.has_eval_representation() || curve.degree() != 1 {
        return Err(rejected(
            "side pcurve requires degree one without evaluator override",
        ));
    }
    let endpoints = endpoints(curve)?;
    classify(endpoints)
}
fn classify(mut points: [[f64; 2]; 2]) -> Result<usize, Failure> {
    if points[0][0] > points[1][0] || points[0][1] > points[1][1] {
        points.swap(0, 1);
    }
    let expected = [
        [[0., 0.], [1., 0.]],
        [[0., 1.], [1., 1.]],
        [[0., 0.], [0., 1.]],
        [[1., 0.], [1., 1.]],
    ];
    expected
        .iter()
        .position(|candidate| *candidate == points)
        .ok_or_else(|| {
            rejected(&format!(
                "actual pcurve endpoints={points:?}; expected unit rectangle side"
            ))
        })
}

mod curves;
pub(in crate::body_check::certified_conical) mod source;

fn endpoints(curve: &Geom2dBezierCurve) -> Result<[[f64; 2]; 2], Failure> {
    let mut endpoints = [[0.; 2]; 2];
    for i in 1..=2 {
        if curve.weight(i)?.to_bits() != 1_f64.to_bits() {
            return Err(rejected("side pcurve requires unit weights"));
        }
        let p = curve.pole(i)?;
        endpoints[i as usize - 1] = [p.x(), p.y()];
    }
    Ok(endpoints)
}

fn side_value(
    id: u64,
    face: &Face,
    rows: [Vec<[f64; 3]>; 2],
    weights: Vec<f64>,
) -> Result<SideGeometry, Failure> {
    let side = SideGeometry {
        id,
        orientation: orientation(face)?,
        rows,
        weights,
        edges: rectangle(face)?,
    };
    Ok(side)
}
