use martensite_tkgeomalgo::int_patch::IntPatchTheSOnBounds;
use martensite_tkmath::gp::Point3;

pub(super) fn circle_points(search: &IntPatchTheSOnBounds) {
    let coordinates = [
        (-0.75, -1.0),
        (0.75, -1.0),
        (1.0, -0.75),
        (1.0, 0.75),
        (0.75, 1.0),
        (-0.75, 1.0),
        (-1.0, 0.75),
        (-1.0, -0.75),
    ];
    for (index, (x, y)) in coordinates.into_iter().enumerate() {
        let point = search.point(index as i32 + 1).unwrap();
        assert_eq!(point.value(), Point3::new(x, y, 0.0));
        assert!(point.is_new());
        // IntStart's native intersection-point record uses TOL=1000*TolBoundary.
        assert_eq!(point.tolerance(), 1000.0 * 1.0e-7);
        assert_eq!(point.parameter(), if index % 2 == 0 { -0.75 } else { 0.75 });
    }
}

pub(super) fn square_segments(search: &IntPatchTheSOnBounds) {
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    for index in 0..4 {
        let segment = search.segment(index as i32 + 1).unwrap();
        let first = segment.first_point().unwrap();
        let last = segment.last_point().unwrap();
        let (x, y) = corners[index];
        assert_eq!(first.value(), Point3::new(x, y, 0.0));
        let (x, y) = corners[(index + 1) % 4];
        assert_eq!(last.value(), Point3::new(x, y, 0.0));
        assert_eq!((first.parameter(), last.parameter()), (-1.0, 1.0));
        assert_eq!((first.tolerance(), last.tolerance()), (1.0e-7, 1.0e-7));
        assert!(!first.is_new() && !last.is_new());
    }
}
