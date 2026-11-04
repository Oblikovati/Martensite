//! CPU reference of ordered shared-incidence clipping; never supplies rendered geometry.
use crate::clipped_depth_polygon::{self as polygon, ClipDepthFailure as Failure, ClipDepthVertex};
#[derive(Clone, Copy, Debug)]
pub(crate) struct SharedIncidenceVertex {
    pub source: ClipDepthVertex,
    pub incidences: [f32; 8],
}
fn initial(
    clip: [[f32; 4]; 3],
    incidences: [[f32; 8]; 3],
) -> Result<Vec<SharedIncidenceVertex>, Failure> {
    if incidences.into_iter().flatten().any(|v| !v.is_finite()) {
        return Err(Failure::Arithmetic);
    }
    Ok(polygon::clipped_depth_polygon(clip, &[])?
        .into_iter()
        .enumerate()
        .map(|(i, source)| SharedIncidenceVertex {
            source,
            incidences: incidences[i],
        })
        .collect())
}
fn append(
    output: &mut Vec<SharedIncidenceVertex>,
    vertex: SharedIncidenceVertex,
) -> Result<(), Failure> {
    if output.len() == 17 {
        return Err(Failure::VertexLimit);
    }
    output.push(vertex);
    Ok(())
}
fn intersection(
    a: SharedIncidenceVertex,
    b: SharedIncidenceVertex,
    plane: usize,
) -> Result<SharedIncidenceVertex, Failure> {
    let (da, db) = (a.incidences[plane], b.incidences[plane]);
    let source = polygon::intersection(a.source, b.source, da, db)?;
    let incidences = interpolate_incidences(a, b, da, db)?;
    Ok(SharedIncidenceVertex { source, incidences })
}
fn interpolate_incidences(
    a: SharedIncidenceVertex,
    b: SharedIncidenceVertex,
    da: f32,
    db: f32,
) -> Result<[f32; 8], Failure> {
    let (base, other, t) = if db.abs() < da.abs() {
        (b, a, db / (db - da))
    } else {
        (a, b, -da / (db - da))
    };
    let values = std::array::from_fn(|i| {
        base.incidences[i] + t * (other.incidences[i] - base.incidences[i])
    });
    if values.into_iter().any(|v| !v.is_finite()) {
        return Err(Failure::Arithmetic);
    }
    Ok(values)
}

fn halfspace(
    input: &[SharedIncidenceVertex],
    plane: usize,
) -> Result<Vec<SharedIncidenceVertex>, Failure> {
    let mut output = Vec::new();
    for (index, &a) in input.iter().enumerate() {
        let b = input[(index + 1) % input.len()];
        if a.incidences[plane] >= 0. {
            append(&mut output, a)?;
        }
        if (a.incidences[plane] < 0.) != (b.incidences[plane] < 0.) {
            append(&mut output, intersection(a, b, plane)?)?;
        }
    }
    Ok(output)
}
pub(crate) fn clipped_shared_incidence(
    clip: [[f32; 4]; 3],
    incidences: [[f32; 8]; 3],
    planes: usize,
) -> Result<Vec<SharedIncidenceVertex>, Failure> {
    require_plane_count(planes)?;
    let mut output = initial(clip, incidences)?;
    for plane in 0..planes {
        output = halfspace(&output, plane)?;
        if output.is_empty() {
            break;
        }
    }
    Ok(output)
}
fn require_plane_count(planes: usize) -> Result<(), Failure> {
    if !(1..=8).contains(&planes) {
        return Err(Failure::PlaneLimit {
            actual: planes,
            maximum: 8,
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "clipped_depth_shared_reference_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "clipped_depth_shared_captured_tests.rs"]
mod captured_tests;
