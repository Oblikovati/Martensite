use super::{
    ContapHContTool, ContapSamplingContext,
    fake_surface::{CallbackTrace, make_spline_surface},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::{ContapArcFunction, ContapSurfFunction};
use martensite_tkmath::math::Vector;

type ObservedFailure = Option<(String, Vec<u8>)>;
fn observed(result: Result<(), Failure>) -> ObservedFailure {
    result
        .err()
        .map(|error| (error.exception_type().into(), error.message_bytes().into()))
}

#[test]
fn historical_and_explicit_setters_keep_callbacks_partial_points_and_mean() {
    for (spline, callbacks) in [
        (None, 20),
        (Some([2, 3, 2, 3]), 56),
        (Some([4, 3, 3, 3]), 119),
    ] {
        check_complete_callback_trace(spline);
        for failure in std::iter::once(None).chain((1..=callbacks).map(Some)) {
            compare_surface_setters(spline, failure);
            compare_arc_setters(spline, failure);
        }
    }
    compare_partial_bound_storage();
    check_historical_range_is_not_modified_by_explicit_context();
}

fn fixture(
    spline: Option<[i32; 4]>,
    failure: Option<usize>,
) -> (Adaptor3dSurfaceHandle, CallbackTrace) {
    make_spline_surface([1., 2., 9., 18.], failure, spline)
}

fn compare_surface_setters(spline: Option<[i32; 4]>, failure: Option<usize>) {
    let (old_surface, old_trace) = fixture(spline, failure);
    let (new_surface, new_trace) = fixture(spline, failure);
    let mut old = ContapSurfFunction::new();
    let mut new = ContapSurfFunction::new();
    let mut context = ContapSamplingContext::new();
    assert_eq!(
        observed(old.set_surface(&old_surface)),
        observed(new.set_surface_with_context(&new_surface, &mut context))
    );
    assert_eq!(*old_trace.lock().unwrap(), *new_trace.lock().unwrap());
    assert_eq!(old.point(), new.point());
    assert_eq!(old.surface(), old_surface);
    assert_eq!(new.surface(), new_surface);
    // Unit normal makes post-failure value expose the retained partial mean.
    assert_eq!(evaluated_bits(&mut old), evaluated_bits(&mut new));
}

fn evaluated_bits(function: &mut ContapSurfFunction) -> (ObservedFailure, u64) {
    let x = Vector::filled(1, 2, 0.25).unwrap();
    let mut value = Vector::filled(1, 1, 17.).unwrap();
    let failure = observed(function.value(&x, &mut value).map(|_| ()));
    (failure, value[1].to_bits())
}

fn compare_arc_setters(spline: Option<[i32; 4]>, failure: Option<usize>) {
    let (old_surface, old_trace) = fixture(spline, failure);
    let (new_surface, new_trace) = fixture(spline, failure);
    let mut old = ContapArcFunction::new();
    let mut new = ContapArcFunction::new();
    let mut context = ContapSamplingContext::new();
    assert_eq!(
        observed(old.set_surface(&old_surface)),
        observed(new.set_surface_with_context(&new_surface, &mut context))
    );
    assert_eq!(*old_trace.lock().unwrap(), *new_trace.lock().unwrap());
    assert_eq!(old.last_computed_point(), new.last_computed_point());
    assert_eq!(old.surface(), old_surface);
    assert_eq!(new.surface(), new_surface);
    assert_eq!(old.get_state_number(), new.get_state_number());
    assert_eq!(old.valpoint(1).unwrap(), new.valpoint(1).unwrap());
}

fn check_historical_range_is_not_modified_by_explicit_context() {
    let (historical, _) = make_spline_surface([1., 2., 9., 18.], None, None);
    ContapHContTool::nb_sample_points(&historical).unwrap();
    let (different, _) = make_spline_surface([101., 202., 109., 218.], None, None);
    let mut context = ContapSamplingContext::new();
    context.nb_sample_points(&different).unwrap();
    assert_eq!(
        ContapHContTool::sample_point(&different, 1).unwrap(),
        (3., 6.)
    );
    assert_eq!(context.sample_point(&historical, 1).unwrap(), (103., 206.));
}

fn check_complete_callback_trace(spline: Option<[i32; 4]>) {
    let (surface, trace) = fixture(spline, None);
    let mut function = ContapSurfFunction::new();
    function
        .set_surface_with_context(&surface, &mut ContapSamplingContext::new())
        .unwrap();
    let mut expected = vec!["first_u", "last_u", "first_v", "last_v", "kind"];
    if spline.is_some() {
        expected.extend(SPLINE_QUERIES);
    }
    let samples = if spline == Some([4, 3, 3, 3]) { 12 } else { 5 };
    for _ in 0..samples {
        expected.push("kind");
        if spline.is_some() {
            expected.extend(SPLINE_QUERIES);
        }
        expected.extend(["kind", "normal"]);
    }
    assert_eq!(*trace.lock().unwrap(), expected);
}
const SPLINE_QUERIES: [&str; 6] = ["kind", "u_knots", "u_degree", "kind", "v_knots", "v_degree"];

fn compare_partial_bound_storage() {
    for position in 1..=5 {
        let (seed, _) = make_spline_surface([17., 19., 23., 29.], None, None);
        let mut context = ContapSamplingContext::new();
        ContapHContTool::nb_sample_points(&seed).unwrap();
        context.nb_sample_points(&seed).unwrap();
        let (historical, _) = make_spline_surface([9., 18., 1., 2.], Some(position), None);
        let (local, _) = make_spline_surface([9., 18., 1., 2.], Some(position), None);
        assert_eq!(
            observed(ContapHContTool::nb_sample_points(&historical).map(|_| ())),
            observed(context.nb_sample_points(&local).map(|_| ()))
        );
        for index in 1..=5 {
            assert_eq!(
                ContapHContTool::sample_point(&seed, index).unwrap(),
                context.sample_point(&seed, index).unwrap()
            );
        }
    }
}
