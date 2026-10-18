//! Endpoint constraints and derivative calls publish to their actual initialized identities.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Int, Output, ROOTS8, Reals};
use super::*;
use martensite_tkgeombase::adv_app2var::AdvApp2VarEndpointOutput as Endpoint;
#[path = "bivariate_endpoint_output_model.rs"]
mod model;
fn endpoint_frame(iso: i32, shared: bool) -> Frame {
    let mut owner = vec![7.; 1025];
    owner[16..20].copy_from_slice(&[-1., 1., -1., 1.]);
    owner[32..40].copy_from_slice(&ROOTS8);
    owner[608] = 1e-6;
    owner[1024] = 0.;
    Frame::new(
        vec![1, iso, 0, 0, 0, -7, -7],
        Bindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 4,
        },
        owner,
        Reals {
            domain: [16, 17, 18, 19],
            constant_parameter: 1024,
        },
    )
    .unwrap()
    .with_caller_output_slots([5, 6])
    .unwrap()
    .with_endpoint_output_origins([320, if shared { 320 } else { 384 }])
    .unwrap()
}
fn endpoint_request(iso: i32, order: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [-1., 1., -1., 1.],
        tconst: 0.,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: order,
        ideriv: 0,
        ndgjac: order + 3,
        nbcrmx: 1,
        ncflim: order + 4,
        epsapr: &[1e-6],
    }
}
fn check_values(actual: &[f64], expected: &[f64]) {
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
}
fn write_quadratic(call: &mut Call<'_, '_, '_>, index: usize, derivative: bool) {
    let parameter = call.buffers().parameter(index);
    call.buffers().set_result(
        index,
        if derivative {
            2. * parameter
        } else {
            parameter * parameter
        },
    );
}
fn callback_derivatives(call: &mut Call<'_, '_, '_>, ordinal: i32, shared: bool) {
    let expected = match ordinal {
        1 => [7., 7.],
        2 => [-2., if shared { -2. } else { 7. }],
        _ => [if shared { 2. } else { -2. }, 2.],
    };
    for (role, expected) in [Endpoint::Lower, Endpoint::Upper].into_iter().zip(expected) {
        assert_eq!(call.endpoint_output(role, 1).unwrap(), expected);
    }
}
fn check_endpoint_results(frame: &Frame, output: &IsoApproxOutput<'_>, shared: bool, failure: i32) {
    let lower = if failure == 1 || !shared { -2. } else { 2. };
    let upper = if shared {
        lower
    } else if failure == 1 {
        7.
    } else {
        2.
    };
    for (role, values, derivative) in [
        (Endpoint::Lower, &*output.contr1, lower),
        (Endpoint::Upper, &*output.contr2, upper),
    ] {
        check_values(&values[..2], &[1., derivative]);
        assert_eq!(frame.endpoint_output(role, 1).unwrap(), derivative);
        assert_eq!(&values[2..], &[7.; 30]);
    }
}
fn run_c1(iso: i32, shared: bool, failure: i32) {
    let (mut frame, mut request, mut tables) = (
        endpoint_frame(iso, shared),
        endpoint_request(iso, 1),
        CurveStorage::new(),
    );
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        let mut ordinal = calls.lock().unwrap();
        *ordinal += 1;
        let derivative = *ordinal > 1;
        assert_eq!(
            call.integer(Int::ParameterCount),
            if derivative { 1 } else { 10 }
        );
        assert_eq!(call.integer(Int::UOrder), i32::from(derivative && iso == 2));
        assert_eq!(call.integer(Int::VOrder), i32::from(derivative && iso == 1));
        for index in 0..if derivative { 1 } else { 10 } {
            write_quadratic(call, index, derivative);
        }
        callback_derivatives(call, *ordinal, shared);
        call.set_caller_output(
            Output::Status,
            match (*ordinal, failure) {
                (2, 1) => 5,
                (3, 2) => 6,
                _ => 0,
            },
        )
        .unwrap();
    };
    frame
        .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), if failure == 1 { 2 } else { 3 });
    assert_eq!(
        frame.caller_output(Output::Status).unwrap(),
        if failure == 0 { 0 } else { 104 + failure }
    );
    assert_eq!(output.nbcrbe, i32::from(failure == 0));
    assert_eq!(
        output.ncoeff,
        &[if failure != 0 {
            -7
        } else if shared {
            5
        } else {
            3
        }]
    );
    check_endpoint_results(&frame, &output, shared, failure);
    let (curve, errors) = if failure == 0 {
        model::c1_curve_and_errors(shared)
    } else {
        ([0.; 5], [7.; 2])
    };
    check_values(&output.courbe[..5], &curve);
    check_values(&output.courbe[5..], &[7.; 27]);
    check_values(&[output.errmax[0], output.errmoy[0]], &errors);
}
fn run_c0(iso: i32, shared: bool) {
    let (mut frame, mut request, mut tables) = (
        endpoint_frame(iso, shared),
        endpoint_request(iso, 0),
        CurveStorage::new(),
    );
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(call.integer(Int::ParameterCount), 10);
        for index in 0..10 {
            let t = call.buffers().parameter(index);
            call.buffers().set_result(index, t);
        }
        call.set_caller_output(Output::Status, 0).unwrap();
    };
    frame
        .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[if shared { 4 } else { 2 }]);
    assert_eq!(output.contr1[0], if shared { 1. } else { -1. });
    assert_eq!(output.contr2[0], 1.);
    assert_eq!(&output.contr1[1..], &[7.; 31]);
    assert_eq!(&output.contr2[1..], &[7.; 31]);
    check_values(&output.courbe[..4], &model::c0_curve(shared));
    check_values(&output.courbe[4..], &[7.; 28]);
    if shared {
        check_values(&[output.errmax[0], output.errmoy[0]], &[0., 0.]);
    }
}
macro_rules! endpoint_case {($name:ident,$runner:ident,$($argument:expr),*)=>{#[test]fn $name(){$runner($($argument),*);}};}
endpoint_case!(endpoint_c0_disjoint_iso1, run_c0, 1, false);
endpoint_case!(endpoint_c0_shared_iso1, run_c0, 1, true);
endpoint_case!(endpoint_c0_disjoint_iso2, run_c0, 2, false);
endpoint_case!(endpoint_c0_shared_iso2, run_c0, 2, true);
endpoint_case!(endpoint_c1_disjoint_iso1, run_c1, 1, false, 0);
endpoint_case!(endpoint_c1_shared_iso1, run_c1, 1, true, 0);
endpoint_case!(endpoint_c1_disjoint_iso2, run_c1, 2, false, 0);
endpoint_case!(endpoint_c1_shared_iso2, run_c1, 2, true, 0);
endpoint_case!(endpoint_lower_error_disjoint_iso1, run_c1, 1, false, 1);
endpoint_case!(endpoint_lower_error_shared_iso1, run_c1, 1, true, 1);
endpoint_case!(endpoint_lower_error_disjoint_iso2, run_c1, 2, false, 1);
endpoint_case!(endpoint_lower_error_shared_iso2, run_c1, 2, true, 1);
endpoint_case!(endpoint_upper_error_disjoint_iso1, run_c1, 1, false, 2);
endpoint_case!(endpoint_upper_error_shared_iso1, run_c1, 1, true, 2);
endpoint_case!(endpoint_upper_error_disjoint_iso2, run_c1, 2, false, 2);
endpoint_case!(endpoint_upper_error_shared_iso2, run_c1, 2, true, 2);
#[path = "bivariate_endpoint_output_contracts.rs"]
mod contracts;
