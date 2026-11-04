//! Ordered clip-plane transforms for the explicit rounded-camera/orthographic domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ClipConstantFailure {
    CameraSource,
    ProjectionDomain,
    Nonfinite,
}
fn finite<const N: usize>(values: [f32; N]) -> bool {
    values.into_iter().all(f32::is_finite)
}
fn row_product(plane: [f32; 4], matrix: [f32; 16]) -> Result<[f32; 4], ClipConstantFailure> {
    let result = std::array::from_fn(|column| {
        let terms: [f32; 4] = std::array::from_fn(|row| plane[row] * matrix[column * 4 + row]);
        ((terms[0] + terms[1]) + terms[2]) + terms[3]
    });
    if !finite(result) {
        return Err(ClipConstantFailure::Nonfinite);
    }
    Ok(result)
}
fn rounded_rigid_inverse(view: [f32; 16]) -> Result<[f32; 16], ClipConstantFailure> {
    if !finite(view) || [view[3], view[7], view[11], view[15]] != [0., 0., 0., 1.] {
        return Err(ClipConstantFailure::CameraSource);
    }
    let mut inverse = std::array::from_fn(|index| {
        let (row, column) = (index % 4, index / 4);
        if row < 3 && column < 3 {
            view[row * 4 + column]
        } else {
            0.
        }
    });
    inverse[15] = 1.;
    for row in 0..3 {
        inverse[12 + row] = -((view[12] * inverse[row] + view[13] * inverse[4 + row])
            + view[14] * inverse[8 + row]);
    }
    require_finite_matrix(inverse)
}
fn require_finite_matrix(matrix: [f32; 16]) -> Result<[f32; 16], ClipConstantFailure> {
    if !finite(matrix) {
        return Err(ClipConstantFailure::Nonfinite);
    }
    Ok(matrix)
}

fn require_orthographic(projection: [f32; 16]) -> Result<(), ClipConstantFailure> {
    let diagonal = [0, 5, 10, 12, 13, 14, 15];
    let unexpected = projection
        .iter()
        .enumerate()
        .any(|(i, &v)| !diagonal.contains(&i) && v != 0.);
    if !finite(projection) || unexpected || projection[15] != 1. {
        return Err(ClipConstantFailure::ProjectionDomain);
    }
    Ok(())
}
fn orthographic_inverse(projection: [f32; 16]) -> Result<[f32; 16], ClipConstantFailure> {
    require_orthographic(projection)?;
    let mut inverse = [0.; 16];
    inverse[15] = 1.;
    for (axis, index) in [0, 5, 10].into_iter().enumerate() {
        if projection[index] == 0. {
            return Err(ClipConstantFailure::ProjectionDomain);
        }
        inverse[index] = 1. / projection[index];
        inverse[12 + axis] = -(projection[12 + axis] * inverse[index]);
    }
    if !finite(inverse) {
        return Err(ClipConstantFailure::Nonfinite);
    }
    Ok(inverse)
}
pub(crate) fn clipped_depth_reference_equation(
    world: [f32; 4],
    projection: [f32; 16],
    view: [f32; 16],
    camera_isometry: bool,
) -> Result<[f32; 4], ClipConstantFailure> {
    if !camera_isometry {
        return Err(ClipConstantFailure::CameraSource);
    }
    if !finite(world) {
        return Err(ClipConstantFailure::Nonfinite);
    }
    let eye = row_product(world, rounded_rigid_inverse(view)?)?;
    row_product(eye, orthographic_inverse(projection)?)
}
pub(crate) fn require_incidence_camera(
    projection: [f32; 16],
    view: [f32; 16],
    camera_isometry: bool,
) -> Result<(), ClipConstantFailure> {
    if !camera_isometry
        || !finite(view)
        || [view[3], view[7], view[11], view[15]] != [0., 0., 0., 1.]
    {
        return Err(ClipConstantFailure::CameraSource);
    }
    require_orthographic(projection)
}

#[cfg(test)]
#[path = "clipped_depth_constants_tests.rs"]
mod tests;
