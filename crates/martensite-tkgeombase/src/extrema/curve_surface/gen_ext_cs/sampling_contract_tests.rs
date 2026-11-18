//! Sampling and failure receipts retained by the move-only grid change (#496).
use super::*;
use martensite_tkernel::{impl_transient, standard::StandardTransient};
use std::sync::Mutex;

#[derive(Default)]
pub(super) struct RecordingGridSurface {
    parent: StandardTransient,
    samples: Mutex<Vec<[f64; 2]>>,
    fail_at: Option<usize>,
}
impl_transient!(
    RecordingGridSurface,
    "RecordingGridSurface496",
    StandardTransient,
    parent
);

impl Adaptor3dSurface for RecordingGridSurface {
    fn get_type(&self) -> Result<SurfaceType, Failure> {
        Ok(SurfaceType::OtherSurface)
    }

    fn u_resolution(&self, resolution: f64) -> Result<f64, Failure> {
        Ok(resolution)
    }

    fn v_resolution(&self, resolution: f64) -> Result<f64, Failure> {
        Ok(resolution)
    }

    fn eval_d0(&self, u: f64, v: f64) -> Result<Point3, Failure> {
        let mut samples = self.samples.lock().unwrap();
        let index = samples.len();
        samples.push([u, v]);
        if self.fail_at == Some(index) {
            return Err(Failure::new(
                "grid fixture failed at sample 5; expected complete grid",
            ));
        }
        Ok(Point3::new(u, v, u + 2. * v))
    }
}

pub(super) fn initialize<'a>(
    extrema: &mut GenExtCS<'a>,
    surface: &'a RecordingGridSurface,
    counts: (i32, i32),
) -> Result<(), Failure> {
    extrema.initialize_range(surface, counts, (0., 10000.), (0., 20000.), 1e-9)
}

fn expected_samples() -> Vec<[f64; 2]> {
    [1., 5000., 9999.]
        .into_iter()
        .flat_map(|u| {
            [2., 5001., 10000., 14999., 19998.]
                .into_iter()
                .map(move |v| [u, v])
        })
        .collect()
}

#[test]
fn asymmetric_grid_evaluates_each_point_in_u_then_v_order() {
    let surface = RecordingGridSurface::default();
    let mut extrema = GenExtCS::new();
    initialize(&mut extrema, &surface, (2, 4)).unwrap();
    assert_eq!(*surface.samples.lock().unwrap(), expected_samples());
    assert_eq!((extrema.usample, extrema.vsample), (2, 4));
    assert!(!extrema.is_done());
}

#[test]
fn zero_sample_counts_keep_the_single_inner_parameter() {
    let surface = RecordingGridSurface::default();
    let mut extrema = GenExtCS::new();
    initialize(&mut extrema, &surface, (0, 0)).unwrap();
    assert_eq!(*surface.samples.lock().unwrap(), [[1., 2.]]);
    assert_eq!((extrema.usample, extrema.vsample), (0, 0));
}

#[test]
fn minus_one_count_keeps_empty_grid_initialization() {
    for counts in [(-1, 4), (2, -1), (-1, -1)] {
        let surface = RecordingGridSurface::default();
        let mut extrema = GenExtCS::new();
        initialize(&mut extrema, &surface, counts).unwrap();
        assert!(surface.samples.lock().unwrap().is_empty());
        assert_eq!((extrema.usample, extrema.vsample), counts);
    }
}

#[test]
fn negative_extent_fails_before_surface_evaluation_and_retains_metadata() {
    for counts in [(-2, 4), (2, -2)] {
        let surface = RecordingGridSurface::default();
        let mut extrema = GenExtCS::new();
        extrema.done = true;
        let failure = initialize(&mut extrema, &surface, counts).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_RangeError");
        assert!(surface.samples.lock().unwrap().is_empty());
        assert_eq!((extrema.usample, extrema.vsample), counts);
        assert!(extrema.is_done());
    }
}

#[test]
fn evaluation_failure_stops_at_first_failed_point_without_resetting_done() {
    let surface = RecordingGridSurface {
        fail_at: Some(5),
        ..Default::default()
    };
    let mut extrema = GenExtCS::new();
    extrema.done = true;
    let failure = initialize(&mut extrema, &surface, (2, 4)).unwrap_err();
    assert_eq!(
        failure.message(),
        "grid fixture failed at sample 5; expected complete grid"
    );
    assert_eq!(*surface.samples.lock().unwrap(), expected_samples()[..6]);
    assert_eq!((extrema.usample, extrema.vsample), (2, 4));
    assert!(extrema.is_done());
}
