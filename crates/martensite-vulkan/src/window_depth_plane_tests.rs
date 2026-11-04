use super::*;

fn rational_triangle(positions: [[f64; 2]; 3]) -> [[f64; 3]; 3] {
    positions.map(|[x, y]| [x, y, 0.2 + 0.001 * x + 0.002 * y])
}

#[test]
fn rational_plane_and_slope_survive_physical_grid_compensation() {
    let vertices = rational_triangle([[1.001, 2.003], [10.002, 2.001], [1.004, 12.002]]);
    let plane = WindowDepthPlane::through(vertices).unwrap();
    let compensated = plane
        .compensated_vertices(vertices, RenderViewportGridPrecision::Fractional8)
        .unwrap();
    let reconstructed = WindowDepthPlane::through(compensated).unwrap();
    assert!((reconstructed.evaluate([4., 5.]) - 0.214).abs() < 1e-14);
    assert!((reconstructed.maximum_slope() - 0.002).abs() < 1e-14);
}

#[test]
fn reference_grid_and_physical_grid_are_declared_independently() {
    let vertices = rational_triangle([[-100.001, -20.003], [-10.002, -20.001], [-100.004, 80.002]]);
    let reference =
        snapped_window_vertices(vertices, RenderViewportGridPrecision::Fractional12).unwrap();
    let plane = WindowDepthPlane::through(reference).unwrap();
    let compensated = plane
        .compensated_vertices(vertices, RenderViewportGridPrecision::Fractional8)
        .unwrap();
    let reconstructed = WindowDepthPlane::through(compensated).unwrap();
    assert!((reconstructed.evaluate([-30., 10.]) - plane.evaluate([-30., 10.])).abs() < 1e-14);
    assert!((reconstructed.maximum_slope() - plane.maximum_slope()).abs() < 1e-14);
}

#[test]
fn nearest_even_ties_preserve_depth_and_distinguish_adjacent_cells() {
    let vertices = [
        [0.5 / 256., 1.5 / 256., 0.2],
        [2.5 / 256., -0.5 / 256., 0.3],
        [1., 1., 0.4],
    ];
    let snapped =
        snapped_window_vertices(vertices, RenderViewportGridPrecision::Fractional8).unwrap();
    assert_eq!(snapped[0], [0., 2. / 256., 0.2]);
    assert_eq!(snapped[1], [2. / 256., -0., 0.3]);
}

#[test]
fn physical_grid_collapse_is_a_typed_refusal() {
    let vertices = [[0., 0., 0.2], [0.0001, 0., 0.3], [0., 0.0001, 0.4]];
    let plane = WindowDepthPlane::through(vertices).unwrap();
    assert!(matches!(
        plane.compensated_vertices(vertices, RenderViewportGridPrecision::Fractional8),
        Err(WindowDepthPlaneFailure::Degenerate { .. })
    ));
}

#[test]
fn nonfinite_inputs_refuse() {
    let mut vertices = rational_triangle([[0., 0.], [1., 0.], [0., 1.]]);
    vertices[1][2] = f64::INFINITY;
    assert!(matches!(
        WindowDepthPlane::through(vertices),
        Err(WindowDepthPlaneFailure::NonFinite {
            vertex: 1,
            component: 2,
            ..
        })
    ));
}

#[test]
fn arithmetic_overflow_is_incomplete_instead_of_a_published_plane() {
    let vertices = [[0., 0., 0.], [1e-200, 0., 1e200], [0., 1e200, 0.]];
    assert!(matches!(
        WindowDepthPlane::through(vertices),
        Err(WindowDepthPlaneFailure::NonFiniteCoefficient { .. })
    ));
    let huge = [[f64::MAX, 0., 0.], [0., 1., 0.], [1., 0., 0.]];
    assert!(matches!(
        snapped_window_vertices(huge, RenderViewportGridPrecision::Fractional12),
        Err(WindowDepthPlaneFailure::Grid(_))
    ));
}

#[test]
fn plane_reference_is_send_and_sync() {
    fn require_traits<T: Send + Sync>() {}
    require_traits::<WindowDepthPlane>();
    require_traits::<WindowDepthPlaneFailure>();
}
