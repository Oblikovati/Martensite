//! CPU references screen-space UV interpolation on the shader's bounded 12-bit lattice.
const FRACTION_BITS: u32 = 12;
const MAX_WINDOW_COORDINATE: f32 = 4096.0;

pub(super) fn snap_window_point(point: [f32; 2]) -> Option<[f32; 2]> {
    if point.iter().any(|value| !valid_window_coordinate(*value)) {
        return None;
    }
    Some(point.map(|value| (value * scale()).round_ties_even() / scale()))
}

fn valid_window_coordinate(value: f32) -> bool {
    value.is_finite() && value.abs() <= MAX_WINDOW_COORDINATE
}

fn scale() -> f32 {
    (1_u32 << FRACTION_BITS) as f32
}

pub(super) fn triangle_area(points: [[f32; 2]; 3]) -> Option<f32> {
    if !shader_area_is_safe(points) {
        return None;
    }
    let [a, b, c] = points;
    let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    (area.is_finite() && area != 0.0).then_some(area)
}

fn shader_area_is_safe(points: [[f32; 2]; 3]) -> bool {
    let Some([[ax, ay], [bx, by], [cx, cy]]) = lattice_points(points) else {
        return false;
    };
    let dx1 = i128::from(bx - ax);
    let dy1 = i128::from(by - ay);
    let dx2 = i128::from(cx - ax);
    let dy2 = i128::from(cy - ay);
    let first = dx1 * dy2;
    let second = dy1 * dx2;
    let determinant = first - second;
    // The shader's two products and subtraction have total error below 4*f32::EPSILON
    // times the product magnitudes; this strict integer test keeps the float area nonzero.
    determinant != 0 && determinant.abs() * (1_i128 << 21) > first.abs() + second.abs()
}

fn lattice_points(points: [[f32; 2]; 3]) -> Option<[[i64; 2]; 3]> {
    points
        .map(|point| {
            snap_window_point(point).map(|snapped| {
                snapped.map(|coordinate| (coordinate * scale()).round_ties_even() as i64)
            })
        })
        .into_iter()
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()
}

#[cfg(test)]
pub(super) fn interpolate_uv(
    points: [[f32; 2]; 3],
    uv: [[f32; 2]; 3],
    clip_w: [f32; 3],
    sample: [f32; 2],
) -> Option<[f32; 2]> {
    if uv.iter().flatten().any(|value| !value.is_finite())
        || clip_w.iter().any(|value| {
            let reciprocal = 1.0 / value;
            !value.is_finite() || *value <= 0.0 || !reciprocal.is_normal()
        })
        || sample.iter().any(|value| !valid_window_coordinate(*value))
    {
        return None;
    }
    let snapped = points
        .map(snap_window_point)
        .into_iter()
        .collect::<Option<Vec<_>>>()?;
    interpolate_snapped_uv(snapped.try_into().ok()?, uv, clip_w, sample)
}

#[cfg(test)]
fn interpolate_snapped_uv(
    [a, b, c]: [[f32; 2]; 3],
    uv: [[f32; 2]; 3],
    clip_w: [f32; 3],
    sample: [f32; 2],
) -> Option<[f32; 2]> {
    let area = triangle_area([a, b, c])?;
    let barycentric = barycentric_weights([a, b, c], sample, area);
    let weights = std::array::from_fn(|i| barycentric[i] / clip_w[i]);
    let sum = (weights[0] + weights[1]) + weights[2];
    if weights.iter().any(|weight| !weight.is_finite()) || !sum.is_finite() || sum <= 0.0 {
        return None;
    }
    let result = blend_uv(uv, weights, sum);
    result
        .iter()
        .all(|value| value.is_finite())
        .then_some(result)
}

#[cfg(test)]
fn barycentric_weights([a, b, c]: [[f32; 2]; 3], p: [f32; 2], area: f32) -> [f32; 3] {
    [
        cross(p, b, c) / area,
        cross(a, p, c) / area,
        cross(a, b, p) / area,
    ]
}

#[cfg(test)]
fn cross(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

#[cfg(test)]
fn blend_uv(uv: [[f32; 2]; 3], weights: [f32; 3], sum: f32) -> [f32; 2] {
    std::array::from_fn(|axis| {
        ((weights[0] * uv[0][axis] + weights[1] * uv[1][axis]) + weights[2] * uv[2][axis]) / sum
    })
}

#[cfg(test)]
#[path = "text_grid_interpolation_math_tests.rs"]
mod tests;
