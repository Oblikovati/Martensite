//! The whole regression cases `bugs_modalg_5::bug23706_10` to `_15`. No display
//! assertions or external inputs.
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeomalgo::{geom_api, geom2d_api};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::{Point2, Point3};
#[path = "repeated_pole_extrema/fixtures.rs"]
mod fixtures;
#[path = "support/spatial_extrema_output.rs"]
mod spatial_output;
use fixtures::*;
use spatial_output::{extrema_segment, spatial_result_words};

struct SplineInput {
    poles: &'static [[f64; 2]],
    knots: &'static [f64],
    multiplicities: &'static [i32],
}

fn array<T: Default + Copy>(items: &[T]) -> Array1<'static, T> {
    let mut result = Array1::with_bounds(1, items.len() as i32).unwrap();
    result.as_mut_slice().copy_from_slice(items);
    result
}

impl SplineInput {
    fn consumed_poles(&self) -> &[[f64; 2]] {
        // GeomliteTest computes np from multiplicities and ignores trailing arguments.
        let count = self.multiplicities.iter().sum::<i32>() - 2 - 1;
        &self.poles[..count as usize]
    }

    fn planar(&self) -> Handle<dyn Geom2dCurve> {
        let poles: Vec<Point2> = self
            .consumed_poles()
            .iter()
            .map(|p| Point2::new(p[0], p[1]))
            .collect();
        let curve = Geom2dBSplineCurve::with_weights(
            &array(&poles),
            &array(&vec![1.0; poles.len()]),
            &array(self.knots),
            &array(self.multiplicities),
            2,
            false,
        )
        .unwrap();
        Handle::new(curve).cast()
    }

    fn spatial(&self) -> Handle<dyn GeomCurve> {
        let poles: Vec<Point3> = self
            .consumed_poles()
            .iter()
            .map(|p| Point3::new(p[0], p[1], 3.0))
            .collect();
        let curve = GeomBSplineCurve::with_weights(
            &array(&poles),
            &array(&vec![1.0; poles.len()]),
            &array(self.knots),
            &array(self.multiplicities),
            2,
            false,
        )
        .unwrap();
        Handle::new(curve).cast()
    }
}

fn spatial_extrema(a: &SplineInput, b: &SplineInput) -> geom_api::ExtremaCurveCurve {
    let (a, b) = (a.spatial(), b.spatial());
    geom_api::ExtremaCurveCurve::with_ranges(
        &a,
        &b,
        (a.first_parameter(), a.last_parameter()),
        (b.first_parameter(), b.last_parameter()),
    )
    .unwrap()
}

fn planar_first_distance(a: &SplineInput, b: &SplineInput) -> f64 {
    let (a, b) = (a.planar(), b.planar());
    let extrema = geom2d_api::ExtremaCurveCurve::new(
        &a,
        &b,
        a.first_parameter(),
        a.last_parameter(),
        b.first_parameter(),
        b.last_parameter(),
    )
    .unwrap();
    assert!(
        extrema.nb_extrema() > 0,
        "original dist 1 regex must capture a value"
    );
    extrema.distance(1).unwrap()
}

fn assert_first_segment(a: &SplineInput, b: &SplineInput, limit: f64) {
    let extrema = spatial_extrema(a, b);
    assert!(!extrema.is_parallel().unwrap());
    assert!(extrema.nb_extrema() > 0, "original ext_1 must exist");
    let (first, second) = extrema.points(1).unwrap();
    let length = first.distance(&second);
    assert!(
        length >= 1.0e-16,
        "native point branch does not publish ext_1"
    );
    let segment = extrema_segment(first, second);
    let adaptor = GeomAdaptorCurve::with_curve(&segment).unwrap();
    let measured = AbscissaPoint::length_with_tolerance(&adaptor, precision::confusion()).unwrap();
    assert!(
        measured <= limit,
        "length {measured} exceeds original {limit}"
    );
}

#[test]
fn bug23706_10_original_spatial_first_segment() {
    assert_first_segment(&C10_1, &C10_2, 5.0e-11);
}

#[test]
fn bug23706_11_original_spatial_first_segment() {
    assert_first_segment(&C11_1, &C11_2, 1.0e-8);
}

#[test]
fn bug23706_12_original_spatial_result_list_length() {
    let extrema = spatial_extrema(&C12_1, &C12_2);
    assert!(!extrema.is_parallel().unwrap());
    let mut words = Vec::new();
    for index in 1..=extrema.nb_extrema() {
        let (a, b) = extrema.points(index).unwrap();
        // Native point output has eight words; segment output has one.
        words.extend(spatial_result_words(index, a, b));
    }
    if extrema.nb_extrema() == 0 {
        words.extend(["No".into(), "solutions!".into()]);
    }
    assert_eq!(words.len(), 1, "original llength result: {words:?}");
}

#[test]
fn bug23706_13_original_planar_first_distance() {
    let distance = planar_first_distance(&C13_1, &C13_2);
    assert!(distance <= 1.0e-7, "original dist 1: {distance}");
}

#[test]
fn bug23706_14_original_planar_first_distance() {
    let distance = planar_first_distance(&C14_1, &C14_2);
    assert!(
        (distance - 3.671_053_417_126_128_4).abs() <= 1.0e-7,
        "original dist 1: {distance}"
    );
}

#[test]
fn bug23706_15_original_planar_rejected_distance() {
    let distance = planar_first_distance(&C15_1, &C15_2);
    // The original rejects near-equality; reversing this would alter its assertion.
    assert!(
        (distance - 2.324_677_740_972_722_5).abs() >= 1.0e-7,
        "original deliberately rejected dist 1: {distance}"
    );
}

#[test]
fn original_command_consumes_only_derived_pole_count() {
    assert_eq!((C11_2.poles.len(), C11_2.consumed_poles().len()), (7, 6));
    assert_eq!((C15_2.poles.len(), C15_2.consumed_poles().len()), (6, 6));
}

#[test]
fn spatial_point_branch_is_not_a_single_result_list_word() {
    assert_eq!(
        spatial_result_words(1, Point3::default(), Point3::default()),
        ["Extrema", "1", "is", "point", ":", "0", "0", "0"]
    );
    assert_eq!(
        spatial_result_words(1, Point3::default(), Point3::new(1.0e-16, 0.0, 0.0)),
        ["ext_1"]
    );
}
