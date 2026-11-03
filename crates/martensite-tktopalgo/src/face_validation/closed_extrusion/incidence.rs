//! Located owner equality, not coordinates, must realize the two endpoint classes.
use super::{boundary::Boundary, budget::*, decode::*};
type Endpoint<'a> = (&'a VertexUse, usize);
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let endpoints = collect_endpoints(strip, boundary)?;
    for left in 0..endpoints.len() {
        for right in left + 1..endpoints.len() {
            budget.query()?;
            compare_endpoints(endpoints[left], endpoints[right], [left, right])?;
        }
    }
    Ok(())
}
fn collect_endpoints<'a>(strip: &'a Strip, boundary: &Boundary) -> CheckResult<Vec<Endpoint<'a>>> {
    let mut endpoints = Vec::new();
    let mut counts = [0; 4];
    for (index, edge) in strip.edges.iter().enumerate() {
        require_endpoint_sides(edge)?;
        for vertex in &edge.vertices {
            let corner = boundary.corners[index][vertex.side];
            counts[corner] += 1;
            endpoints.push((vertex, corner));
        }
    }
    if counts != [2; 4] {
        return Err(invalid(
            0,
            "two uses per strip corner",
            format!("{counts:?}"),
        ));
    }
    Ok(endpoints)
}
fn require_endpoint_sides(edge: &EdgeUse) -> CheckResult<()> {
    if edge.vertices.len() != 2 || edge.vertices[0].side == edge.vertices[1].side {
        return Err(invalid(
            edge.owner,
            "two intrinsic range ends",
            "missing or repeated endpoint side",
        ));
    }
    Ok(())
}
fn compare_endpoints(
    first: Endpoint<'_>,
    second: Endpoint<'_>,
    indices: [usize; 2],
) -> CheckResult<()> {
    let (a, ca) = first;
    let (b, cb) = second;
    let same = a.owner == b.owner && a.location == b.location;
    if same != ((ca < 2) == (cb < 2)) {
        return Err(invalid(
            a.owner,
            "located endpoint quotient",
            format!("pair={indices:?}, same={same}, corners=[{ca},{cb}]"),
        ));
    }
    Ok(())
}
