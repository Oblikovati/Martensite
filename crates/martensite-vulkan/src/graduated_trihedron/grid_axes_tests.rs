use super::*;

fn bounds() -> (Vec3<f32>, Vec3<f32>) {
    (Vec3::new(0.0, 0.0, 0.0), Vec3::new(40.0, 30.0, 20.0))
}

#[test]
fn corner_indices_select_min_or_max_per_axis() {
    let (min, max) = bounds();
    assert_eq!(box_corner(0, min, max), min);
    assert_eq!(box_corner(7, min, max), max);
    assert_eq!(box_corner(5, min, max), Vec3::new(40.0, 0.0, 20.0));
    assert_eq!(box_corner(2, min, max), Vec3::new(0.0, 30.0, 0.0));
}

#[test]
fn no_positive_corner_keeps_the_minimum_corner_grid() {
    let (min, max) = bounds();
    let (axes, edges) = grid_axes(&[-1.0; 8], min, max);
    assert_eq!(axes.origin, min);
    assert_eq!(axes.axes[0], Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(
        axes.ticks,
        [
            Vec3::new(0.0, 0.0, 20.0),
            Vec3::new(40.0, 0.0, 0.0),
            Vec3::new(40.0, 0.0, 0.0)
        ]
    );
    assert_eq!(
        edges,
        OOZ_XOZ | OYO_XYO | XOO_XYO | OOZ_OYZ | XOO_XOZ | OYO_OYZ
    );
}

#[test]
fn farthest_corner_flips_its_axes_and_shows_nine_edges_for_the_max_corner() {
    let (min, max) = bounds();
    let mut corners = [0.5; 8];
    corners[7] = 2.0;
    corners[3] = 2.0; // ties keep the first maximum
    let (axes, edges) = grid_axes(&corners, min, max);
    assert_eq!(axes.origin, Vec3::new(0.0, 30.0, 20.0));
    assert_eq!(axes.axes[1], Vec3::new(0.0, -1.0, 0.0));
    assert_eq!(axes.axes[2], Vec3::new(0.0, 0.0, -1.0));
    assert_eq!(edges.count_ones(), 7);
    corners[3] = 0.0;
    let (axes, edges) = grid_axes(&corners, min, max);
    assert_eq!(axes.origin, max);
    assert_eq!(edges.count_ones(), 9);
    assert_eq!(axes.ticks[2], Vec3::new(40.0, 0.0, 20.0));
}

#[test]
fn corner_distance_is_the_normal_projection_from_the_center() {
    let normal = Vec3::new(0.0, 0.0, 1.0);
    let center = Vec3::new(20.0, 15.0, 10.0);
    assert_eq!(
        distance_to_corner(normal, center, Vec3::new(0.0, 0.0, 20.0)),
        10.0
    );
    assert_eq!(
        distance_to_corner(normal, center, Vec3::new(0.0, 0.0, 0.0)),
        -10.0
    );
}

#[test]
fn orthographic_front_view_normal_points_away_from_the_viewer_with_unit_pixels() {
    // A 100x100 viewport over world [-50, 50]^2 looking down -Z: one world unit per pixel.
    let mut projection = Mat4::<f32>::identity();
    projection.set_value(0, 0, 0.02);
    projection.set_value(1, 1, 0.02);
    projection.set_value(2, 2, -0.02);
    let (normal, dpix) = view_normal(&Mat4::identity(), &projection, [0, 0, 100, 100]);
    assert_eq!(dpix, 1.0);
    assert!(
        normal.z < 0.0 && normal.x == 0.0 && normal.y == 0.0,
        "{normal:?}"
    );
}
