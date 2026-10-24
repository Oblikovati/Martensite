//! Combined coordinate reads share one point snapshot without changing offsets (#220).
use martensite_tkernel::{
    impl_transient,
    ncollection::Array1,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkgeomalgo::{
    approx_int::{ApproxIntLine, ApproxIntMultiLine, ApproxIntMultiLineLayout, ApproxIntOffsets},
    int_patch::IntPatchWLine,
    int_surf::{IntSurfLineOn2S, IntSurfPntOn2S},
};
use martensite_tkgeombase::app_par_curves::LineTool;
use martensite_tkmath::gp::{Point2, Point3};
use std::sync::atomic::{AtomicUsize, Ordering};

struct SnapshotCountingLine {
    transient: StandardTransient,
    walking: IntPatchWLine,
    point_reads: AtomicUsize,
}
impl_transient!(
    SnapshotCountingLine,
    "SnapshotCountingLine",
    StandardTransient,
    transient
);

impl ApproxIntLine for SnapshotCountingLine {
    fn nb_pnts(&self) -> Result<i32, Failure> {
        ApproxIntLine::nb_pnts(&self.walking)
    }

    fn point(&self, index: i32) -> Result<IntSurfPntOn2S, Failure> {
        self.point_reads.fetch_add(1, Ordering::Relaxed);
        ApproxIntLine::point(&self.walking, index)
    }

    fn from_points(line: &Handle<IntSurfLineOn2S>, tang: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            walking: ApproxIntLine::from_points(line, tang),
            point_reads: AtomicUsize::new(0),
        }
    }
}

fn sample_points() -> Handle<IntSurfLineOn2S> {
    let points = Handle::new(IntSurfLineOn2S::new(None));
    let mut point = IntSurfPntOn2S::new();
    point.set_value_all(Point3::new(2.0, 3.0, 5.0), 7.0, 11.0, 13.0, 17.0);
    points.add(&point);
    points
}

fn sample_layout(curves: i32, on_first: bool) -> ApproxIntMultiLineLayout {
    ApproxIntMultiLineLayout {
        nb_p3d: 1,
        nb_p2d: curves,
        p2d_on_first: on_first,
        offsets: sample_offsets(),
        ind_min: 1,
        ind_max: 1,
        ..Default::default()
    }
}

fn sample_offsets() -> ApproxIntOffsets {
    ApproxIntOffsets {
        xo: -1.0,
        yo: 2.0,
        zo: 3.0,
        u1o: 5.0,
        v1o: -7.0,
        u2o: 11.0,
        v2o: -13.0,
    }
}

fn check_combined_snapshot(curves: i32, first: bool, capacity: i32) {
    let walking = Handle::new(SnapshotCountingLine::from_points(&sample_points(), false));
    let line = ApproxIntMultiLine::with_line(walking.clone(), sample_layout(curves, first));
    let mut spatial = Array1::<Point3>::with_bounds(1, 1).unwrap();
    let mut parameters = Array1::<Point2>::with_bounds(1, capacity).unwrap();
    LineTool::value(&line, 1, &mut spatial, &mut parameters).unwrap();
    assert_eq!(
        walking.point_reads.load(Ordering::Relaxed),
        1,
        "combined read (#220)"
    );
    assert_eq!(spatial[1], Point3::new(1.0, 5.0, 8.0));
    check_shifted_parameters(&parameters, curves, first);
    check_separate_coordinates(&line, &walking, spatial, parameters);
}

fn check_shifted_parameters(parameters: &Array1<'_, Point2>, curves: i32, first: bool) {
    let expected = if curves == 1 && !first {
        Point2::new(24.0, 4.0)
    } else {
        Point2::new(12.0, 4.0)
    };
    assert_eq!(parameters[1], expected);
    if parameters.length() == 2 {
        assert_eq!(parameters[2], Point2::new(24.0, 4.0));
    }
}

fn check_separate_coordinates(
    line: &ApproxIntMultiLine<SnapshotCountingLine>,
    walking: &Handle<SnapshotCountingLine>,
    mut spatial: Array1<'_, Point3>,
    mut parameters: Array1<'_, Point2>,
) {
    let combined = (spatial.clone(), parameters.clone());
    LineTool::value_3d(line, 1, &mut spatial).unwrap();
    LineTool::value_2d(line, 1, &mut parameters).unwrap();
    assert_eq!(walking.point_reads.load(Ordering::Relaxed), 3);
    assert_eq!(spatial.as_slice(), combined.0.as_slice());
    assert_eq!(parameters.as_slice(), combined.1.as_slice());
}

#[test]
fn combined_read_snapshots_once_for_each_parameter_layout() {
    for (curves, first, capacity) in [(1, true, 1), (1, false, 1), (2, true, 1), (2, false, 2)] {
        check_combined_snapshot(curves, first, capacity);
    }
}

#[test]
#[ignore = "manual identical-workload timing for issue #220; no wall-clock CI threshold"]
fn measure_combined_walking_reads() {
    let walking = Handle::new(IntPatchWLine::from_points(&sample_points(), false));
    let line = ApproxIntMultiLine::with_line(walking, sample_layout(2, true));
    let mut spatial = Array1::<Point3>::with_bounds(1, 1).unwrap();
    let mut parameters = Array1::<Point2>::with_bounds(1, 2).unwrap();
    for _ in 0..5 {
        let elapsed = time_read_batch(&line, &mut spatial, &mut parameters);
        println!(
            "combined_walking_reads iterations=200000 elapsed_ns={}",
            elapsed.as_nanos()
        );
    }
}

fn time_read_batch(
    line: &ApproxIntMultiLine<IntPatchWLine>,
    spatial: &mut Array1<'_, Point3>,
    parameters: &mut Array1<'_, Point2>,
) -> std::time::Duration {
    let started = std::time::Instant::now();
    for _ in 0..200_000 {
        LineTool::value(std::hint::black_box(line), 1, spatial, parameters).unwrap();
        std::hint::black_box((&spatial, &parameters));
    }
    started.elapsed()
}
