const NET: [[i128; 3]; 4] = [[0, 0, 0], [4, 0, 1], [5, 3, 2], [0, 2, -1]];
fn numerator([u, v]: [i128; 2], axis: usize) -> i128 {
    NET[0][axis] * (8 - u) * (8 - v)
        + NET[1][axis] * u * (8 - v)
        + NET[2][axis] * u * v
        + NET[3][axis] * (8 - u) * v
}
fn midpoint_derivatives(p: [i128; 2], q: [i128; 2], axis: usize) -> [i128; 2] {
    let [s, t] = [p[0] + q[0], p[1] + q[1]];
    [
        (NET[1][axis] - NET[0][axis]) * (16 - t) + (NET[2][axis] - NET[3][axis]) * t,
        (NET[3][axis] - NET[0][axis]) * (16 - s) + (NET[2][axis] - NET[1][axis]) * s,
    ]
}
fn assert_exact_identity(p: [i128; 2], q: [i128; 2]) {
    for axis in 0..3 {
        let [du, dv] = midpoint_derivatives(p, q, axis);
        assert_eq!(
            2 * (numerator(p, axis) - numerator(q, axis)),
            du * (p[0] - q[0]) + dv * (p[1] - q[1])
        );
    }
}
#[test]
fn integer_midpoint_identity_control_is_exact_for_all_grid_pairs() {
    let points: Vec<_> = (0..=8).flat_map(|u| (0..=8).map(move |v| [u, v])).collect();
    for &first in &points {
        for &second in &points {
            assert_exact_identity(first, second);
        }
    }
}
