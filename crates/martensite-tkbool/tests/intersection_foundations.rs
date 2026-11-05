//! Independent record boundaries and analytic rectangle classification.
use martensite_tkbool::top_ope_brep::{
    TopOpeBRep, TopOpeBRepBipoint, TopOpeBRepP2DStatus, TopOpeBRepPointClassifier,
    TopOpeBRepTypeLineCurve,
};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::{Plane3, Point2};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::io::{self, Write};

fn assert_endpoint_failure(result: Result<i32, Failure>, endpoint: &str) {
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert_eq!(failure.message(), format!("TopOpeBRepBipoint {endpoint}=0"));
}

#[test]
fn endpoint_records_retain_default_zero_errors_and_positive_boundaries() {
    let empty = TopOpeBRepBipoint::default();
    assert_endpoint_failure(empty.i1(), "I1");
    assert_endpoint_failure(empty.i2(), "I2");
    for index in [1, 2, i32::MAX] {
        let pair = TopOpeBRepBipoint::new(index, i32::MAX);
        assert_eq!(pair.i1().unwrap(), index);
        assert_eq!(pair.i2().unwrap(), i32::MAX);
    }
}

#[test]
fn nonpositive_endpoint_errors_keep_the_case_zero_literal() {
    for index in [0, -1, i32::MIN] {
        let first = TopOpeBRepBipoint::new(index, 7);
        assert_endpoint_failure(first.i1(), "I1");
        assert_eq!(first.i2().unwrap(), 7);
        let second = TopOpeBRepBipoint::new(7, index);
        assert_eq!(second.i1().unwrap(), 7);
        assert_endpoint_failure(second.i2(), "I2");
    }
}

#[test]
fn intersection_status_discriminants_are_stable() {
    use TopOpeBRepP2DStatus::*;
    for (value, expected) in [
        (Unknown, 0),
        (Intersection, 1),
        (SegmentFirst, 2),
        (SegmentLast, 3),
        (New, 4),
    ] {
        assert_eq!(value as i32, expected);
    }
    assert_eq!(TopOpeBRepP2DStatus::default(), Unknown);
}

const LINE_KINDS: &[(TopOpeBRepTypeLineCurve, &str)] = &[
    (TopOpeBRepTypeLineCurve::Analytic, "ANALYTIC"),
    (TopOpeBRepTypeLineCurve::Restriction, "RESTRICTION"),
    (TopOpeBRepTypeLineCurve::Walking, "WALKING"),
    (TopOpeBRepTypeLineCurve::Line, "LINE"),
    (TopOpeBRepTypeLineCurve::Circle, "CIRCLE"),
    (TopOpeBRepTypeLineCurve::Ellipse, "ELLIPSE"),
    (TopOpeBRepTypeLineCurve::Parabola, "PARABOLA"),
    (TopOpeBRepTypeLineCurve::Hyperbola, "HYPERBOLA"),
    (TopOpeBRepTypeLineCurve::OtherType, "OTHERTYPE"),
];

#[test]
fn line_category_names_and_discriminants_are_exact() {
    for (index, &(kind, expected)) in LINE_KINDS.iter().enumerate() {
        assert_eq!(kind as usize, index);
        let mut output = Vec::new();
        TopOpeBRep::print(kind, &mut output).unwrap();
        assert_eq!(output, expected.as_bytes());
    }
    assert_eq!(
        TopOpeBRepTypeLineCurve::default(),
        TopOpeBRepTypeLineCurve::Analytic
    );
}

#[test]
fn unnamed_line_discriminants_use_the_unknown_print_branch() {
    for raw in [9_i32, 15] {
        let mut output = Vec::new();
        TopOpeBRep::print(raw, &mut output).unwrap();
        assert_eq!(output, b"UNKNOWN");
    }
}

#[derive(Default)]
struct RefuseCategoryOutput {
    flushed: bool,
}

impl Write for RefuseCategoryOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        Err(io::Error::other(format!(
            "received {} category bytes; expected writable output",
            bytes.len()
        )))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushed = true;
        Ok(())
    }
}

#[test]
fn category_output_reports_write_failure_without_adding_a_flush() {
    let mut output = RefuseCategoryOutput::default();
    let error = TopOpeBRep::print(TopOpeBRepTypeLineCurve::Line, &mut output).unwrap_err();
    assert_eq!(
        error.to_string(),
        "received 4 category bytes; expected writable output"
    );
    assert!(!output.flushed);
    output.flush().unwrap();
    assert!(output.flushed);
}

fn rectangle(bounds: (f64, f64, f64, f64)) -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), bounds)
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn rectangular_face_classification_has_independent_in_out_on_results() {
    let face = rectangle((-1., 1., -1., 1.));
    let mut classifier = TopOpeBRepPointClassifier::new();
    assert_eq!(classifier.state(), State::Unknown);
    classifier.load(&face).unwrap();
    assert_eq!(classifier.state(), State::Unknown);
    for (u, expected) in [
        (0., State::In),
        (2., State::Out),
        (1., State::On),
        (1.0001, State::Out),
    ] {
        assert_eq!(
            classifier
                .classify(&face, Point2::new(u, 0.), 1e-7)
                .unwrap(),
            expected
        );
        assert_eq!(classifier.state(), expected);
    }
}

#[test]
fn face_cache_keeps_distinct_trim_domains_and_load_preserves_state() {
    let first = rectangle((-1., 1., -1., 1.));
    let second = rectangle((10., 12., 10., 12.));
    let mut classifier = TopOpeBRepPointClassifier::new();
    let point = Point2::new(0., 0.);
    assert_eq!(classifier.classify(&first, point, 1e-7).unwrap(), State::In);
    classifier.load(&second).unwrap();
    assert_eq!(classifier.state(), State::In);
    assert_eq!(
        classifier.classify(&second, point, 1e-7).unwrap(),
        State::Out
    );
    assert_eq!(classifier.classify(&first, point, 1e-7).unwrap(), State::In);
}

#[test]
fn init_resets_state_and_rebuilds_the_first_tolerance_cache() {
    let face = rectangle((-1., 1., -1., 1.));
    let point = Point2::new(1.0001, 0.);
    let mut classifier = TopOpeBRepPointClassifier::default();
    assert_eq!(classifier.classify(&face, point, 1e-7).unwrap(), State::Out);
    assert_eq!(classifier.classify(&face, point, 1e-3).unwrap(), State::Out);
    classifier.init();
    assert_eq!(classifier.state(), State::Unknown);
    assert_eq!(classifier.classify(&face, point, 1e-3).unwrap(), State::On);
}

fn require_send_sync<T: Send + Sync>(_: &T) {}

#[test]
fn intersection_foundation_public_types_are_send_and_sync() {
    require_send_sync(&TopOpeBRepBipoint::default());
    require_send_sync(&TopOpeBRepPointClassifier::new());
    require_send_sync(&TopOpeBRepP2DStatus::default());
    require_send_sync(&TopOpeBRepTypeLineCurve::default());
    require_send_sync(&TopOpeBRep);
}
