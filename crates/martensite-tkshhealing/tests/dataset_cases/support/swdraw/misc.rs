//! SWscenario `checkoverlapedges` (ShapeAnalysis_Edge::CheckOverlapping), `removeloc`
//! (ShapeUpgrade_RemoveLocations) and `scaleshape` (ShapeCustom::ScaleShape).
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::precision;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
use martensite_tkshhealing::shape_custom::ShapeCustom;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeRemoveLocations;

/// `checkoverlapedges e1 e2 [tol [domain]]`: `Err(text)` for a degenerated edge (the
/// command fails), else the overlap verdict text.
pub fn checkoverlapedges(
    names: (&str, &str),
    e1: &Shape,
    e2: &Shape,
    tolerance: Option<f64>,
    domain: Option<f64>,
) -> Result<String, String> {
    let e1 = topods::edge(e1).expect("checkoverlapedges: an edge");
    let e2 = topods::edge(e2).expect("checkoverlapedges: an edge");
    for (name, edge) in [(names.0, &e1), (names.1, &e2)] {
        if BRepTool::degenerated(edge).unwrap() {
            return Err(format!("{name} is degenerated\n"));
        }
    }
    let mut tolerance = tolerance.unwrap_or_else(precision::confusion);
    let domain = domain.unwrap_or(0.);
    let overlapping = ShapeAnalysisEdge::new()
        .check_overlapping(&e1, &e2, &mut tolerance, domain)
        .unwrap_or_else(|failure| panic!("checkoverlapedges: {failure:?}"));
    if !overlapping {
        return Ok("Edges are not overlapped\n".into());
    }
    if domain == 0. {
        return Ok("Edges are overlapping completely\n".into());
    }
    let g = super::super::printed::g17;
    Ok(format!(
        "Edges are overlapped\nwith tolerance = {}\non segment length = {}\n",
        g(tolerance),
        g(domain)
    ))
}

/// `removeloc res shape`: ShapeUpgrade_RemoveLocations with the default remove level.
pub fn removeloc(shape: &Shape) -> Shape {
    let tool = ShapeUpgradeRemoveLocations::new();
    tool.remove(shape)
        .unwrap_or_else(|failure| panic!("removeloc: {failure:?}"));
    tool.get_result()
}

/// `scaleshape result shape scale`: ShapeCustom::ScaleShape; `None` (and `NO
/// MODIFICATIONS`) when the result is the input; a null result is the `NO RESULT` error.
pub fn scaleshape(shape: &Shape, scale: f64) -> Option<Shape> {
    let result = ShapeCustom::scale_shape(shape, scale)
        .unwrap_or_else(|failure| panic!("scaleshape: {failure:?}"));
    assert!(!result.is_null(), "scaleshape: NO RESULT");
    (result != *shape).then_some(result)
}
