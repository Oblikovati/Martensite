//! Located owner equality, not coordinates, must realize the two endpoint classes.
use super::{boundary::Boundary, budget::*, decode::*};
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
) -> CheckResult<()> {
    let mut endpoints = Vec::new();
    let mut counts = [0; 4];
    for (edge_index, edge) in strip.edges.iter().enumerate() {
        if edge.vertices.len() != 2 || edge.vertices[0].side == edge.vertices[1].side {
            return Err(invalid(
                edge.owner,
                "two intrinsic range ends",
                "missing or repeated endpoint side",
            ));
        }
        for vertex in &edge.vertices {
            let corner = boundary.corners[edge_index][vertex.side];
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
    for left in 0..endpoints.len() {
        for right in left + 1..endpoints.len() {
            budget.query()?;
            let (a, ca) = endpoints[left];
            let (b, cb) = endpoints[right];
            let same = a.owner == b.owner && a.location == b.location;
            if same != ((ca < 2) == (cb < 2)) {
                return Err(invalid(
                    a.owner,
                    "located endpoint quotient",
                    format!("pair=[{left},{right}], same={same}, corners=[{ca},{cb}]"),
                ));
            }
        }
    }
    Ok(())
}
