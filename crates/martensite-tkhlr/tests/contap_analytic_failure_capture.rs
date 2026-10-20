//! Legacy diagnostics retain the process-wide exception capture policy.
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_patch::IntPatchTheIWLineOfTheIWalking;
use martensite_tkgeomalgo::int_surf::IntSurfLineOn2S;
use martensite_tkhlr::contap::{
    ContapContAna, ContapLine, ContapPoint, ContapTheIWLineOfTheIWalking,
};
use martensite_tkmath::gp::{Axis2, Axis3, Circle3, Cylinder3, Direction3, Line3, Point3, Sphere3};
use tangent_output_fixture::geometry_records;

struct CaptureReset(i32);
impl Drop for CaptureReset {
    fn drop(&mut self) {
        Failure::set_default_stack_trace_length(self.0);
    }
}

fn assert_failure<T>(result: Result<T, Failure>, expected: &str, captured: bool) {
    let failure = result.err().expect("analytic accessor must fail");
    assert_eq!(failure.exception_type(), expected);
    assert_eq!(failure.message_bytes(), b"");
    assert_eq!(!failure.stack_bytes().is_empty(), captured);
}

fn check_fresh(captured: bool) {
    let contour = ContapContAna::new();
    assert_failure(contour.nb_contours(), "StdFail_NotDone", captured);
    assert_failure(contour.type_contour(), "StdFail_NotDone", captured);
    assert_failure(contour.circle(), "StdFail_NotDone", captured);
    assert_failure(contour.line(0), "StdFail_NotDone", captured);
}

fn check_empty(captured: bool) {
    let mut contour = ContapContAna::new();
    let sphere = Sphere3::new(Axis3::default(), 2.).unwrap();
    contour
        .perform_sphere_eye(&sphere, Point3::default())
        .unwrap();
    assert_eq!(contour.nb_contours().unwrap(), 0);
    assert_failure(contour.type_contour(), "Standard_DomainError", captured);
    assert_failure(contour.circle(), "Standard_DomainError", captured);
    assert_failure(contour.line(0), "Standard_DomainError", captured);
}

fn check_lines(captured: bool) {
    let mut contour = ContapContAna::new();
    let cylinder = Cylinder3::new(Axis3::default(), 2.).unwrap();
    contour.perform_cylinder(&cylinder, Direction3::X).unwrap();
    assert_eq!(contour.nb_contours().unwrap(), 2);
    assert_failure(contour.circle(), "Standard_DomainError", captured);
    for index in [i32::MIN, 0, 3, i32::MAX] {
        assert_failure(contour.line(index), "Standard_OutOfRange", captured);
    }
}

fn check_circle(captured: bool) {
    let mut contour = ContapContAna::new();
    let sphere = Sphere3::new(Axis3::default(), 2.).unwrap();
    contour.perform_sphere(&sphere, Direction3::Z).unwrap();
    assert_failure(contour.line(0), "Standard_DomainError", captured);
}

#[test]
fn accessor_failures_keep_empty_native_messages_and_capture_policy() {
    let _restore = CaptureReset(Failure::default_stack_trace_length());
    for length in [0, 4, 0] {
        Failure::set_default_stack_trace_length(length);
        // The native and Rust trace backends both provide traces in the recorded Linux run.
        let captured = length > 0;
        check_fresh(captured);
        check_empty(captured);
        check_lines(captured);
        check_circle(captured);
        check_point_record(captured);
        check_line_record(captured);
        check_contour_walking_endpoints(captured);
        check_patch_walking_endpoints(captured);
        check_passing_bounds(captured);
        check_stored_point_bounds(captured);
        check_search_record_failures(captured);
        check_tangent_directions(captured);
        check_arc_saved_points(captured);
        check_explicit_context_failure_capture(captured);
        check_empty_arc_query_failures(captured);
        check_projected_vertical_tangent(captured);
        check_output_reference_tangents(captured);
        intstart_search::check(captured);
        intstart_populated::check(captured);
        iwalking_search::check(captured);
    }
}

fn check_point_record(captured: bool) {
    let point = ContapPoint::new();
    assert_failure(point.arc(), "Standard_DomainError", captured);
    assert_failure(point.parameter_on_arc(), "Standard_DomainError", captured);
    assert_failure(point.transition_on_line(), "Standard_DomainError", captured);
    assert_failure(point.transition_on_arc(), "Standard_DomainError", captured);
    assert_failure(point.vertex(), "Standard_DomainError", captured);
}

fn check_line_record(captured: bool) {
    let mut line = ContapLine::new();
    line.set_line(Line3::new(Point3::default(), Direction3::X));
    assert_failure(line.circle(), "Standard_DomainError", captured);
    assert_failure(line.arc(), "Standard_DomainError", captured);
    assert_failure(line.nb_pnts(), "Standard_DomainError", captured);
    assert_failure(line.point(0), "Standard_DomainError", captured);
    check_vertex_bounds(&line, captured);
    line.set_circle(Circle3::new(Axis2::default(), 1.).unwrap());
    assert_failure(line.line(), "Standard_DomainError", captured);
}

fn check_vertex_bounds(line: &ContapLine, captured: bool) {
    for index in [i32::MIN, 0, 1, i32::MAX] {
        let failure = line.vertex(index).err().expect("empty vertex sequence");
        assert_eq!(failure.exception_type(), "Standard_OutOfRange");
        assert_eq!(
            failure.message_bytes(),
            b"NCollection_Sequence::ChangeValue"
        );
        assert_eq!(!failure.stack_bytes().is_empty(), captured);
    }
}

fn check_contour_walking_endpoints(captured: bool) {
    let line = ContapTheIWLineOfTheIWalking::new(None);
    assert_empty_endpoints(
        [
            line.first_point().map(|_| ()),
            line.first_point_index().map(|_| ()),
            line.last_point().map(|_| ()),
            line.last_point_index().map(|_| ()),
        ],
        captured,
    );
}

fn check_patch_walking_endpoints(captured: bool) {
    let line = IntPatchTheIWLineOfTheIWalking::new(None);
    assert_empty_endpoints(
        [
            line.first_point().map(|_| ()),
            line.first_point_index().map(|_| ()),
            line.last_point().map(|_| ()),
            line.last_point_index().map(|_| ()),
        ],
        captured,
    );
}

fn assert_empty_endpoints(results: [Result<(), Failure>; 4], captured: bool) {
    for result in results {
        assert_failure(result, "Standard_DomainError", captured);
    }
}

fn check_passing_bounds(captured: bool) {
    let contour = ContapTheIWLineOfTheIWalking::new(None);
    let patch = IntPatchTheIWLineOfTheIWalking::new(None);
    for index in [i32::MIN, 0, 1, i32::MAX] {
        for result in [contour.passing_point(index), patch.passing_point(index)] {
            let failure = result.expect_err("empty passing sequence");
            assert_eq!(failure.exception_type(), "Standard_OutOfRange");
            assert_eq!(failure.message_bytes(), b"NCollection_Sequence::Value");
            assert_eq!(!failure.stack_bytes().is_empty(), captured);
        }
    }
}

fn check_stored_point_bounds(captured: bool) {
    let contour = ContapTheIWLineOfTheIWalking::new(None);
    let patch = IntPatchTheIWLineOfTheIWalking::new(None);
    let stored = IntSurfLineOn2S::new(None);
    for index in [i32::MIN, 0, 1, i32::MAX] {
        for result in [
            contour.value(index),
            patch.value(index),
            stored.value(index),
        ] {
            assert_sequence_value_failure(result, captured);
        }
        assert_failure(contour.cut(index), "Standard_OutOfRange", captured);
        assert_failure(patch.cut(index), "Standard_OutOfRange", captured);
        assert_failure(stored.split(index), "Standard_OutOfRange", captured);
    }
}

fn assert_sequence_value_failure<T>(result: Result<T, Failure>, captured: bool) {
    let failure = result.err().expect("empty point sequence");
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(failure.message_bytes(), b"NCollection_Sequence::Value");
    assert_eq!(!failure.stack_bytes().is_empty(), captured);
}

fn check_search_record_failures(captured: bool) {
    use martensite_tkhlr::contap::{
        ContapThePathPointOfTheSearch as Point, ContapTheSegmentOfTheSearch as Segment,
    };
    check_one_search_record(&mut Point::new(), &mut Segment::new(), captured);
    use martensite_tkgeomalgo::int_patch::{
        IntPatchThePathPointOfTheSOnBounds as PatchPoint,
        IntPatchTheSegmentOfTheSOnBounds as PatchSegment,
    };
    check_one_search_record(&mut PatchPoint::new(), &mut PatchSegment::new(), captured);
}

fn check_one_search_record(
    point: &mut martensite_tkgeomalgo::int_patch::IntPatchThePathPointOfTheSOnBounds,
    segment: &mut martensite_tkgeomalgo::int_patch::IntPatchTheSegmentOfTheSOnBounds,
    captured: bool,
) {
    assert_failure(point.vertex(), "Standard_DomainError", captured);
    assert_failure(segment.first_point(), "Standard_DomainError", captured);
    assert_failure(segment.last_point(), "Standard_DomainError", captured);
    point.set_value_on_arc(Point3::new(1., 2., 3.), 0.25, &Default::default(), 4.);
    assert_failure(point.vertex(), "Standard_DomainError", captured);
    segment.set_limit_point(point, true);
    segment.set_limit_point(point, false);
    segment.set_value(&Default::default());
    assert_failure(segment.first_point(), "Standard_DomainError", captured);
    assert_failure(segment.last_point(), "Standard_DomainError", captured);
}

fn check_tangent_directions(captured: bool) {
    use geometry_records::contour_surface_fixture;
    use martensite_tkhlr::contap::ContapSurfFunction;
    use martensite_tkmath::math::Vector;
    let surface =
        contour_surface_fixture::from_fields(0, Axis3::default(), 1., 0., || (0., 1., 0., 1.))
            .unwrap();
    let mut function = ContapSurfFunction::new();
    function.set_surface(&surface).unwrap();
    let x = Vector::filled(1, 2, 0.25).unwrap();
    let mut value = Vector::filled(1, 1, 17.).unwrap();
    function.value(&x, &mut value).unwrap();
    for _ in 0..2 {
        assert_failure(
            function.direction3d(),
            "StdFail_UndefinedDerivative",
            captured,
        );
        assert_failure(
            function.direction2d(),
            "StdFail_UndefinedDerivative",
            captured,
        );
    }
}

fn check_arc_saved_points(captured: bool) {
    use martensite_tkhlr::contap::ContapArcFunction;
    let mut function = ContapArcFunction::new();
    for index in [i32::MIN, -1, 0, 1, i32::MAX] {
        assert_arc_bounds(&function, index, captured);
    }
    assert_eq!(function.get_state_number(), 1);
    for index in [i32::MIN, -1, 0, 2, i32::MAX] {
        assert_arc_bounds(&function, index, captured);
    }
    assert_eq!(function.valpoint(1).unwrap(), Point3::default());
    function.set_arc(&Default::default());
    assert_arc_bounds(&function, 1, captured);
}

fn assert_arc_bounds(
    function: &martensite_tkhlr::contap::ContapArcFunction,
    index: i32,
    captured: bool,
) {
    let failure = function.valpoint(index).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    let expected = if index < 0 {
        "NCollection_Sequence::Value: negative index"
    } else {
        "NCollection_Sequence::Value"
    };
    assert_eq!(failure.message_bytes(), expected.as_bytes());
    assert_eq!(!failure.stack_bytes().is_empty(), captured);
}

fn check_explicit_context_failure_capture(captured: bool) {
    use martensite_tkhlr::contap::{ContapArcFunction, ContapSamplingContext, ContapSurfFunction};
    let mut context = ContapSamplingContext::new();
    let mut function = ContapSurfFunction::new();
    let mut arc = ContapArcFunction::new();
    let surface = Default::default();
    for result in [
        function.set_surface_with_context(&surface, &mut context),
        arc.set_surface_with_context(&surface, &mut context),
    ] {
        let failure = result.unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_NullObject");
        assert_eq!(
            failure.message_bytes(),
            b"Contap_HContTool: null surface; expected live Adaptor3d_Surface"
        );
        assert_eq!(!failure.stack_bytes().is_empty(), captured);
    }
}

fn check_empty_arc_query_failures(captured: bool) {
    for arc in contour_query_arcs() {
        check_both_empty_arc_tools(&arc, captured);
    }
}

fn contour_query_arcs() -> [martensite_tkgeomalgo::int_start::TheArc; 2] {
    use martensite_tkernel::standard::Handle;
    use martensite_tkg2d::{geom2d::Geom2dLine, geom2d_adaptor::Geom2dAdaptorCurve};
    use martensite_tkmath::gp::{Direction2, Point2};
    let curve = Handle::new(Geom2dLine::from_point(Point2::default(), Direction2::X)).cast();
    let arc = Handle::new(Geom2dAdaptorCurve::with_curve(&curve).unwrap()).cast();
    [Default::default(), arc]
}

fn check_both_empty_arc_tools(arc: &martensite_tkgeomalgo::int_start::TheArc, captured: bool) {
    use martensite_tkgeomalgo::int_patch::IntPatchHInterTool as P;
    use martensite_tkhlr::contap::ContapHContTool as C;
    for index in [i32::MIN, -1, 0, 1, i32::MAX] {
        assert_failure(C::value(arc, index), "Standard_OutOfRange", captured);
        assert_failure(C::vertex(arc, index), "Standard_OutOfRange", captured);
        assert_failure(
            C::has_first_point(arc, index),
            "Standard_OutOfRange",
            captured,
        );
        assert_failure(
            C::has_last_point(arc, index),
            "Standard_OutOfRange",
            captured,
        );
        assert_failure(P::value(arc, index), "Standard_OutOfRange", captured);
        assert_failure(P::vertex(arc, index), "Standard_OutOfRange", captured);
        assert_failure(
            P::has_first_point(arc, index),
            "Standard_OutOfRange",
            captured,
        );
        assert_failure(
            P::has_last_point(arc, index),
            "Standard_OutOfRange",
            captured,
        );
    }
}

use tangent_output_fixture::tangent_curve_fixture as capture_curve_fixture;

fn projected_vertical_curve() -> martensite_tkhlr::hlr_brep::Curve {
    use geometry_records::common::Reader;
    use martensite_tkhlr::{hlr_algo::init_min_max, hlr_brep::Curve};
    let projector = capture_curve_fixture::projector(&mut Reader::new(&[2.])).unwrap();
    let edge = capture_curve_fixture::edge(&mut Reader::new(&[0., 1., 2., 3., 0., 0., 1., 0., 2.]))
        .unwrap();
    let mut curve = Curve::new();
    curve.set_projector(Some(std::sync::Arc::new(projector)));
    curve.set_curve(&edge).unwrap();
    let (mut low, mut high) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut low, &mut high);
    curve.update(&mut low, &mut high).unwrap();
    curve
}

fn check_projected_vertical_tangent(captured: bool) {
    let curve = projected_vertical_curve();
    for start in [true, false] {
        let parameter = if start { 0. } else { 2. };
        assert_eq!(
            curve.value(parameter).unwrap(),
            martensite_tkmath::gp::Point2::new(1., 2.)
        );
        let failure = curve
            .tangent(start)
            .expect_err("vertical projection has no tangent");
        assert_eq!(failure.exception_type(), "StdFail_UndefinedDerivative");
        assert_eq!(failure.message_bytes(), b"HLRBRep_Curve::Tangent");
        assert_eq!(!failure.stack_bytes().is_empty(), captured);
    }
}

#[path = "tangent_outputs/fixture.rs"]
mod tangent_output_fixture;

fn check_output_reference_tangents(captured: bool) {
    for label in ["top_z", "type0_normal"] {
        let curve = tangent_output_fixture::projected_fixture(label);
        for start in [true, false] {
            check_output_reference_tangent(&curve, label, start, captured);
        }
    }
}

fn check_output_reference_tangent(
    curve: &martensite_tkhlr::hlr_brep::Curve,
    label: &str,
    start: bool,
    captured: bool,
) {
    use martensite_tkmath::gp::{Direction2, Point2};
    let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
    let error = curve
        .tangent_into(start, &mut point, &mut direction)
        .unwrap_err();
    assert_eq!(error.exception_type(), "StdFail_UndefinedDerivative");
    assert_eq!(error.message_bytes(), b"HLRBRep_Curve::Tangent");
    assert_eq!(!error.stack_bytes().is_empty(), captured);
    let expected = if label == "top_z" {
        Point2::new(1., 2.)
    } else {
        Point2::new(7., 8.)
    };
    assert_eq!(point, expected);
    assert_eq!(direction, Direction2::Y);
}

#[path = "contap_analytic_failure_capture/intstart_search.rs"]
mod intstart_search;

#[path = "contap_analytic_failure_capture/iwalking_search.rs"]
mod iwalking_search;

#[path = "contap_analytic_failure_capture/intstart_populated.rs"]
mod intstart_populated;
