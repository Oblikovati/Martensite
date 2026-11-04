//! CPU admission for the declared V2 numerical domain; this is not a GPU error bound.
pub(super) fn window_point(clip: [f32; 4], viewport: [f32; 4]) -> Option<[f32; 2]> {
    if !clip.into_iter().all(f32::is_finite) || clip[3] <= 0. || !(1. / clip[3]).is_finite() {
        return None;
    }
    let point =
        std::array::from_fn(|i| viewport[i] + (clip[i] / clip[3] + 1.) * viewport[i + 2] * 0.5);
    point.into_iter().all(f32::is_finite).then_some(point)
}
pub(super) fn signed_area(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ac = [c[0] - a[0], c[1] - a[1]];
    ab[0] * ac[1] - ab[1] * ac[0]
}
pub(super) fn admitted_triangle(clips: [[f32; 4]; 3], viewport: [f32; 4]) -> bool {
    let [Some(a), Some(b), Some(c)] = clips.map(|p| window_point(p, viewport)) else {
        return false;
    };
    let area = signed_area(a, b, c);
    area != 0. && area.is_finite() && (1. / area).is_finite()
}
#[cfg(test)]
#[path = "phong_interpolation_math_tests.rs"]
mod tests;

// V3 admits the new class only without clipping, after the same finite window screen.
pub(super) fn admitted_triangle_v3(clips: [[f32; 4]; 3], viewport: [f32; 4]) -> bool {
    if !viewport.into_iter().all(f32::is_finite) || viewport[2] == 0. || viewport[3] == 0. {
        return false;
    }
    if clips
        .map(|p| window_point(p, viewport))
        .iter()
        .any(Option::is_none)
    {
        return false;
    }
    if super::phong_projected_axis::constant_axis(clips) {
        return super::phong_projected_axis::inside_clip(clips);
    }
    admitted_triangle(clips, viewport) && super::phong_projected_axis::nonzero_determinant(clips)
}
