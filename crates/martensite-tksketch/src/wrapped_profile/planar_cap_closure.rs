//! Closure checks use authored corner identity and exact real endpoint equality.
use super::*;
pub(super) fn certify(request: &Request) -> Result<(), ConeFailure> {
    for contour in &request.loops {
        if contour.is_empty() {
            return Err(ConeFailure::Binding {
                spans: vec![],
                field: "nonempty cap contour",
                actual: 0,
                expected: 1,
            });
        }
        for (index, piece) in contour.iter().enumerate() {
            let next = &contour[(index + 1) % contour.len()];
            join(piece, next, request.radius)?;
        }
    }
    Ok(())
}
fn join(first: &AuthoredPiece, next: &AuthoredPiece, radius: f64) -> Result<(), ConeFailure> {
    let a = first
        .poles
        .last()
        .copied()
        .map(|point| angular(point, radius));
    let b = next
        .poles
        .first()
        .copied()
        .map(|point| angular(point, radius));
    if first.end != next.start || !equal_endpoints(a, b) {
        return Err(ConeFailure::CapJoin {
            spans: [first.span, next.span],
            owners: [first.end, next.start],
            points: Box::new([a, b]),
        });
    }
    Ok(())
}
fn angular([s, z]: [f64; 2], radius: f64) -> [f64; 2] {
    [s / radius, z]
}
fn equal_endpoints(a: Option<[f64; 2]>, b: Option<[f64; 2]>) -> bool {
    let (Some(a), Some(b)) = (a, b) else {
        return false;
    };
    a.iter().chain(&b).all(|value| value.is_finite()) && a == b
}
#[cfg(test)]
#[path = "planar_cap_closure_tests.rs"]
mod tests;
