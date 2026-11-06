//! Independent closed form for the tiny half-space probe; samples remain device-observed.
const QUAD: [[f64; 2]; 4] = [[0.0625, 0.0625], [0.5, 0.0625], [0.5, 0.5], [0.0625, 0.5]];
const CUT_X: f64 = 0.25;
const WINDOW_PIXELS_PER_WORLD_UNIT: f64 = 64. / 20.;

#[test]
fn plane_cut_intersections_match_the_camera_pixel_bounds() {
    let cut = cut_intersections();
    assert_eq!(cut, [[CUT_X, QUAD[0][1]], [CUT_X, QUAD[3][1]]]);
    assert_eq!(clipped_area(), 7. / 64.);
    assert_pixel_bounds(QUAD);
    assert_eq!(window_point(cut[0]), [32.8, 31.8]);
    assert_eq!(window_point(cut[1]), [32.8, 30.4]);
}

fn cut_intersections() -> [[f64; 2]; 2] {
    [[CUT_X, QUAD[0][1]], [CUT_X, QUAD[3][1]]]
}

fn clipped_area() -> f64 {
    (QUAD[2][0] - CUT_X) * (QUAD[2][1] - QUAD[1][1])
}

fn assert_pixel_bounds(points: [[f64; 2]; 4]) {
    for [x, y] in points {
        let [window_x, window_y] = window_point([x, y]);
        assert!((32. ..34.).contains(&window_x));
        assert!((30. ..32.).contains(&window_y));
    }
}

fn window_point([x, y]: [f64; 2]) -> [f64; 2] {
    [
        32. + WINDOW_PIXELS_PER_WORLD_UNIT * x,
        32. - WINDOW_PIXELS_PER_WORLD_UNIT * y,
    ]
}
