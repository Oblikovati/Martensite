//! Bounded homogeneous clipping for the explicit framebuffer-origin depth contract.
//! Input vertices and plane order are owned scene operands; no image-derived correction.
const MAX_PLANES: usize = 14;
const MAX_VERTICES: usize = 3 + MAX_PLANES;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ClipDepthFailure {
    InvalidVertex { index: usize },
    InvalidPlane { index: usize },
    UnionChain { index: usize, length: usize },
    PlaneLimit { actual: usize, maximum: usize },
    Arithmetic,
    VertexLimit,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ClipDepthVertex {
    pub clip: [f32; 4],
    pub weights: [f32; 3],
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct ClipDepthHalfspace {
    pub equation: [f32; 4],
    pub chain_length: usize,
}
fn finite<const N: usize>(values: [f32; N]) -> bool {
    values.into_iter().all(f32::is_finite)
}
fn validate(
    vertices: &[[f32; 4]; 3],
    planes: &[ClipDepthHalfspace],
) -> Result<(), ClipDepthFailure> {
    if planes.len() > MAX_PLANES {
        return Err(ClipDepthFailure::PlaneLimit {
            actual: planes.len(),
            maximum: MAX_PLANES,
        });
    }
    for (index, vertex) in vertices.iter().enumerate() {
        if !finite(*vertex) || vertex[3] <= 0. {
            return Err(ClipDepthFailure::InvalidVertex { index });
        }
    }
    validate_planes(planes)
}
fn validate_planes(planes: &[ClipDepthHalfspace]) -> Result<(), ClipDepthFailure> {
    for (index, plane) in planes.iter().enumerate() {
        if !finite(plane.equation) {
            return Err(ClipDepthFailure::InvalidPlane { index });
        }
        if plane.chain_length != 1 {
            return Err(ClipDepthFailure::UnionChain {
                index,
                length: plane.chain_length,
            });
        }
    }
    Ok(())
}
fn distance(vertex: ClipDepthVertex, plane: [f32; 4]) -> Result<f32, ClipDepthFailure> {
    let products = std::array::from_fn::<_, 4, _>(|i| vertex.clip[i] * plane[i]);
    let result = ((products[0] + products[1]) + products[2]) + products[3];
    if !result.is_finite() {
        return Err(ClipDepthFailure::Arithmetic);
    }
    Ok(result)
}
fn initial_vertices(clip: [[f32; 4]; 3]) -> Vec<ClipDepthVertex> {
    clip.into_iter()
        .enumerate()
        .map(|(index, clip)| ClipDepthVertex {
            clip,
            weights: std::array::from_fn(|component| f32::from(component == index)),
        })
        .collect()
}
pub(crate) fn clipped_depth_polygon(
    clip: [[f32; 4]; 3],
    planes: &[ClipDepthHalfspace],
) -> Result<Vec<ClipDepthVertex>, ClipDepthFailure> {
    validate(&clip, planes)?;
    let mut polygon = initial_vertices(clip);
    for plane in planes {
        polygon = clip_halfspace(&polygon, plane.equation)?;
        if polygon.is_empty() {
            break;
        }
    }
    Ok(polygon)
}
fn append(
    output: &mut Vec<ClipDepthVertex>,
    vertex: ClipDepthVertex,
) -> Result<(), ClipDepthFailure> {
    if output.len() == MAX_VERTICES {
        return Err(ClipDepthFailure::VertexLimit);
    }
    output.push(vertex);
    Ok(())
}
fn clip_halfspace(
    input: &[ClipDepthVertex],
    plane: [f32; 4],
) -> Result<Vec<ClipDepthVertex>, ClipDepthFailure> {
    let mut output = Vec::with_capacity(input.len() + 1);
    for (index, &first) in input.iter().enumerate() {
        let second = input[(index + 1) % input.len()];
        let a = distance(first, plane)?;
        let b = distance(second, plane)?;
        if a >= 0. {
            append(&mut output, first)?;
        }
        if (a < 0.) != (b < 0.) {
            append(&mut output, intersection(first, second, a, b)?)?;
        }
    }
    Ok(output)
}
pub(crate) fn intersection(
    a: ClipDepthVertex,
    b: ClipDepthVertex,
    da: f32,
    db: f32,
) -> Result<ClipDepthVertex, ClipDepthFailure> {
    let denominator = db - da;
    if !denominator.is_finite() || denominator == 0. {
        return Err(ClipDepthFailure::Arithmetic);
    }
    let (base, other, t) = if db.abs() < da.abs() {
        (b, a, db / denominator)
    } else {
        (a, b, -da / denominator)
    };
    if !t.is_finite() || !(0. ..=1.).contains(&t) {
        return Err(ClipDepthFailure::Arithmetic);
    }
    interpolate(base, other, t)
}
fn interpolate(
    base: ClipDepthVertex,
    other: ClipDepthVertex,
    t: f32,
) -> Result<ClipDepthVertex, ClipDepthFailure> {
    let clip = std::array::from_fn(|i| base.clip[i] + t * (other.clip[i] - base.clip[i]));
    let weights =
        std::array::from_fn(|i| base.weights[i] + t * (other.weights[i] - base.weights[i]));
    if !finite(clip) || !finite(weights) || clip[3] <= 0. {
        return Err(ClipDepthFailure::Arithmetic);
    }
    Ok(ClipDepthVertex { clip, weights })
}
pub(crate) fn clipped_depth_last_fan(polygon: &[ClipDepthVertex]) -> Vec<[ClipDepthVertex; 3]> {
    if polygon.len() < 3 {
        return Vec::new();
    }
    (2..polygon.len())
        .map(|i| [polygon[i - 1], polygon[i], polygon[0]])
        .collect()
}
#[cfg(test)]
#[path = "clipped_depth_polygon_tests.rs"]
mod tests;
