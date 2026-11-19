// SPDX-License-Identifier: GPL-2.0-only
//! Every native row and ruling curve equals its actual surface boundary.
use super::*;
use martensite_tkg3d::geom::GeomBezierCurve;
pub(super) fn verify(side: &SideGeometry) -> Result<(), Failure> {
    for row in 0..2 {
        let (poles, weights) = capture(&side.edges[row])?;
        if bits(&poles) != bits(&side.rows[row])
            || weights.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                != side.weights.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
        {
            return Err(rejected(
                "native row curve differs from exact actual side boundary",
            ));
        }
    }
    verify_rulings(side)?;
    Ok(())
}
fn bits(poles: &[[f64; 3]]) -> Vec<[u64; 3]> {
    poles.iter().map(|p| p.map(f64::to_bits)).collect()
}
fn capture(shape: &Shape) -> Result<(Vec<[f64; 3]>, Vec<f64>), Failure> {
    let edge = topods::edge(shape)?;
    let captured = BRepTool::curve_with_location(&edge)?;
    if !captured.location.is_identity() || [captured.first, captured.last] != [0., 1.] {
        return Err(rejected(
            "native curve requires identity placement and full unit range",
        ));
    }
    let handle: Handle<GeomBezierCurve> = captured.curve.cast();
    let curve = handle
        .get()
        .ok_or_else(|| rejected("actual boundary requires rational Bezier curve"))?;
    if curve.has_eval_representation() {
        return Err(rejected("actual boundary evaluator override unsupported"));
    }
    let (poles, weights) = controls(curve)?;
    Ok((poles, weights))
}

fn verify_rulings(side: &SideGeometry) -> Result<(), Failure> {
    for (slot, column) in [(2, 0), (3, side.weights.len() - 1)] {
        let (poles, weights) = capture(&side.edges[slot])?;
        let expected = [side.rows[0][column], side.rows[1][column]];
        if weights != [1., 1.]
            || (bits(&poles) != bits(&expected)
                && bits(&poles) != bits(&[expected[1], expected[0]]))
        {
            return Err(rejected(
                "native ruling differs from actual two-pole unit-weight boundary",
            ));
        }
    }
    Ok(())
}

fn controls(curve: &GeomBezierCurve) -> Result<(Vec<[f64; 3]>, Vec<f64>), Failure> {
    let mut poles = Vec::new();
    let mut weights = Vec::new();
    for i in 1..=curve.nb_poles() {
        let p = curve.pole(i)?;
        poles.push([p.x(), p.y(), p.z()]);
        weights.push(curve.weight(i)?);
    }
    Ok((poles, weights))
}
