#[path = "../common/geometry_records/mod.rs"]
mod geometry_records;
use super::{ContapSamplingContext, fake_surface::make_surface};

#[test]
fn every_bound_failure_keeps_only_prior_writes_in_callback_order() {
    let seed = [17., 19., 23., 29.];
    for position in 1..=5 {
        let mut context = ContapSamplingContext::new();
        let (initial, _) = make_surface(seed, None);
        context.nb_sample_points(&initial).unwrap();
        let (failing, trace) = make_surface([9., 18., 1., 2.], Some(position));
        let error = context.nb_sample_points(&failing).unwrap_err();
        assert_eq!(error.exception_type(), "SamplingCallbackFailure");
        assert_eq!(trace.lock().unwrap().as_slice(), &CALLBACKS[..position]);
        assert_eq!(context.bounds(), expected_partial_bounds(position));
    }
}
const CALLBACKS: [&str; 5] = ["first_u", "last_u", "first_v", "last_v", "kind"];
fn expected_partial_bounds(position: usize) -> [f64; 4] {
    match position {
        1 => [17., 19., 23., 29.],
        2 => [9., 19., 23., 29.],
        3 => [9., 19., 1., 29.],
        4 => [9., 18., 1., 29.],
        5 => [1., 2., 9., 18.],
        _ => panic!("position {position}: expected callback1..=5"),
    }
}

#[test]
fn null_capture_and_null_sample_leave_the_context_unchanged() {
    let mut context = ContapSamplingContext::new();
    let (surface, _) = make_surface([1., 2., 9., 18.], None);
    context.nb_sample_points(&surface).unwrap();
    for error in [
        context.nb_sample_points(&Default::default()).unwrap_err(),
        context.sample_point(&Default::default(), 1).unwrap_err(),
    ] {
        assert_eq!(error.exception_type(), "Standard_NullObject");
        assert_eq!(
            error.message_bytes(),
            b"Contap_HContTool: null surface; expected live Adaptor3d_Surface"
        );
        assert_eq!(context.bounds(), [1., 2., 9., 18.]);
    }
}

#[test]
fn failed_local_surface_setup_retains_tangent_cache_and_saved_arc_points() {
    use martensite_tkhlr::contap::ContapArcFunction;
    use martensite_tkmath::gp::Point3;
    let mut context = ContapSamplingContext::new();
    let mut function = tangent_plane_function(&mut context);
    check_failed_surface_cache(&mut function, &mut context);
    let mut arc = ContapArcFunction::new();
    assert_eq!(arc.get_state_number(), 1);
    let (failing, _) = make_surface([1., 2., 9., 18.], Some(1));
    assert!(
        arc.set_surface_with_context(&failing, &mut context)
            .is_err()
    );
    assert_eq!(arc.valpoint(1).unwrap(), Point3::default());
}

fn check_failed_surface_cache(
    function: &mut martensite_tkhlr::contap::ContapSurfFunction,
    context: &mut ContapSamplingContext,
) {
    let (failing, trace) = make_surface([1., 2., 9., 18.], Some(1));
    let point = function.point();
    assert!(
        function
            .set_surface_with_context(&failing, context)
            .is_err()
    );
    assert_eq!(function.surface(), failing);
    assert_eq!(function.point(), point);
    assert!(function.is_tangent().unwrap());
    assert_eq!(trace.lock().unwrap().as_slice(), ["first_u"]);
}

fn tangent_plane_function(
    context: &mut ContapSamplingContext,
) -> martensite_tkhlr::contap::ContapSurfFunction {
    use geometry_records::contour_surface_fixture;
    use martensite_tkhlr::contap::ContapSurfFunction;
    use martensite_tkmath::{gp::Axis3, math::Vector};
    let surface =
        contour_surface_fixture::from_fields(0, Axis3::default(), 1., 0., || (0., 1., 0., 1.))
            .unwrap();
    let mut function = ContapSurfFunction::new();
    function
        .set_surface_with_context(&surface, context)
        .unwrap();
    let mut value = Vector::filled(1, 1, 17.).unwrap();
    function
        .value(&Vector::filled(1, 2, 0.25).unwrap(), &mut value)
        .unwrap();
    assert!(function.is_tangent().unwrap());
    function
}
