//! Every checkreal retains its source order, absolute epsilon and zero relative tolerance.
use super::observations::Observed;
use crate::shape_mesh::check_real;
fn scalar(name: &str, value: f64, expected: f64, tolerance: f64) -> bool {
    check_real(name, value, expected, tolerance, 0.);
    (value - expected).abs() <= tolerance
}
fn coordinates(
    prefix: &str,
    axes: &[&str],
    values: &[f64],
    expected: &[f64],
    tolerance: f64,
) -> bool {
    coordinate_lengths(axes.len(), values.len(), expected.len());
    let mut passed = true;
    for ((axis, value), expected) in axes.iter().zip(values).zip(expected) {
        passed &= scalar(&format!("{prefix}, {axis}"), *value, *expected, tolerance);
    }
    passed
}
pub(super) fn first(value: &Observed) -> bool {
    let mut passed = standard(value, 1, [1., 1.], [0.5, 100.5, -0.5], 1.0e-7, 1.);
    passed &= point_window(value, 1);
    passed
}
pub(super) fn second(value: &Observed) -> bool {
    let mut passed = standard(value, 2, [0.05, 19.], [25., 4.95, 125.], 0.05, 0.05);
    passed &= pixel_grid(value);
    passed &= point_window(value, 2);
    passed &= world_grid(value);
    passed
}
fn standard(
    value: &Observed,
    index: u8,
    scalars: [f64; 2],
    model: [f64; 3],
    scalar_tol: f64,
    model_tol: f64,
) -> bool {
    let mut passed = scalar_pair(value, index, scalars, scalar_tol);
    passed &= origin_window(value, index);
    passed &= center_view(value, index);
    passed &= ray_direction(value, index);
    passed &= model_point(value, index, model, model_tol);
    passed
}
fn scalar_pair(value: &Observed, index: u8, expected: [f64; 2], tolerance: f64) -> bool {
    let mut passed = scalar(
        &format!("vconvert 1.0 view, view {index}"),
        value.base.scalar_view,
        expected[0],
        tolerance,
    );
    passed &= scalar(
        &format!("vconvert 1.0 window, view {index}"),
        f64::from(value.base.scalar_pixel),
        expected[1],
        tolerance,
    );
    passed
}
fn origin_window(value: &Observed, index: u8) -> bool {
    let (x, y) = value.base.origin_pixel;
    coordinates(
        &format!("vconvert 0.0 0.0 window, view {index}"),
        &["Xp", "Yp"],
        &[f64::from(x), f64::from(y)],
        &[200.; 2],
        1.,
    )
}
fn center_view(value: &Observed, index: u8) -> bool {
    let (x, y) = value.base.center_view;
    coordinates(
        &format!("vconvert 200 200 view, view {index}"),
        &["Xv", "Yv"],
        &[x, y],
        &[0.; 2],
        1.0e-7,
    )
}
fn ray_direction(value: &Observed, index: u8) -> bool {
    let expected = if index == 1 {
        [0., -1., 0.]
    } else {
        [0., 0., -1.]
    };
    coordinates(
        &format!("vconvert 0.5 0.5 ray, view {index}"),
        &["DX", "DY", "DZ"],
        &value.base.ray[3..6],
        &expected,
        1.0e-7,
    )
}
fn model_point(value: &Observed, index: u8, expected: [f64; 3], tolerance: f64) -> bool {
    let (x, y, z) = value.base.center_world;
    coordinates(
        &format!("vconvert 200 200, view {index}"),
        &["X", "Y", "Z"],
        &[x, y, z],
        &expected,
        tolerance,
    )
}
fn point_window(value: &Observed, index: u8) -> bool {
    let (x, y) = value.point_pixel;
    let expected = if index == 1 { [200., 199.] } else { [200.; 2] };
    // The view-2 source label intentionally names the earlier point despite querying (25,4.94,35).
    coordinates(
        &format!("vconvert 0.5 100 0.5 window, view {index}"),
        &["Xp", "Yp"],
        &[f64::from(x), f64::from(y)],
        &expected,
        1.,
    )
}
fn pixel_grid(value: &Observed) -> bool {
    let (x, y, z) = value
        .grid_world
        .expect("actual grid query=None; expected view-2 pixel-to-grid coordinates");
    coordinates(
        "vconvert 200 200 grid, view 2",
        &["X", "Y", "Z"],
        &[x, y, z],
        &[25.486, 4.856, 0.],
        0.05,
    )
}
fn world_grid(value: &Observed) -> bool {
    let (x, y, z) = value
        .point_grid
        .expect("actual grid query=None; expected view-2 world-to-grid coordinates");
    coordinates(
        "vconvert 25 4.94 35 grid, view 2",
        &["X", "Y", "Z"],
        &[x, y, z],
        &[25.486, 4.856, 0.],
        0.05,
    )
}

fn coordinate_lengths(axes: usize, values: usize, references: usize) {
    assert_eq!(
        axes, values,
        "axes/values lengths; expected one value per source axis"
    );
    assert_eq!(
        axes, references,
        "axes/references lengths; expected one reference per source axis"
    );
}
