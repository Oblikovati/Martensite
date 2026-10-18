//! Each upper-end derivative call selects its live caller-root parameter again.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerSamplingField as Sampling, AdvApp2VarCombinedEvaluation as Call,
    AdvApp2VarIntegerBindings as Integers, AdvApp2VarIntegerField as Local,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings as Reals,
};
const ROOTS: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
#[derive(Debug, PartialEq)]
struct EndpointRecord {
    roots: i32,
    degree: i32,
    count: i32,
    u: i32,
    v: i32,
    parameter: f64,
}
fn endpoint_frame(complete: bool) -> Frame {
    let integers = Integers {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 4,
    };
    let reals = Reals {
        domain: [0, 1, 2, 3],
        constant_parameter: 4,
    };
    Frame::new(vec![0; 7], integers, vec![0.; 5], reals)
        .unwrap()
        .with_caller_sampling_slots(if complete { [5, 6] } else { [5, 5] })
        .unwrap()
}
fn record_endpoint(call: &mut Call<'_, '_>) -> EndpointRecord {
    EndpointRecord {
        roots: call.caller_sampling(Sampling::RootCount).unwrap(),
        degree: call.caller_sampling(Sampling::JacobiDegree).unwrap(),
        count: call.integer(Local::ParameterCount),
        u: call.integer(Local::UOrder),
        v: call.integer(Local::VOrder),
        parameter: call.parameters()[0],
    }
}
fn update_endpoint_caller(mode: i32, index: usize, call: &mut Call<'_, '_>) {
    if index == 4 {
        call.set_caller_sampling(
            Sampling::RootCount,
            if mode == 1 {
                2
            } else if mode == 2 {
                3
            } else {
                1
            },
        )
        .unwrap();
        assert_eq!(
            call.caller_sampling(Sampling::JacobiDegree).unwrap(),
            if mode == 2 {
                6
            } else if mode == 1 {
                2
            } else {
                1
            }
        );
    }
    if index == 5 && mode == 2 {
        call.set_caller_sampling(Sampling::RootCount, 8).unwrap();
    }
}
fn endpoint_evaluation(
    mode: i32,
    trace: &Mutex<Vec<EndpointRecord>>,
    call: &mut Call<'_, '_>,
    values: &mut [f64],
    error: &mut i32,
) {
    let mut trace = trace.lock().unwrap();
    trace.push(record_endpoint(call));
    let index = trace.len();
    update_endpoint_caller(mode, index, call);
    if index == 5 && mode != 2 {
        *error = 1;
        return;
    }
    let count = call.integer(Local::ParameterCount) as usize;
    let derivative = call.integer(Local::UOrder) != 0 || call.integer(Local::VOrder) != 0;
    let parameters = call.parameters();
    for (i, value) in values[..2 * count].iter_mut().enumerate() {
        *value = if mode == 2 {
            if derivative { 0. } else { 1. }
        } else {
            parameters[i / 2]
        };
    }
    *error = 0;
}
fn assert_endpoint_trace(trace: &[EndpointRecord], mode: i32, iso: i32) {
    assert_eq!(trace.len(), 5);
    let initial = if mode == 2 { 8 } else { 3 };
    let replacement = if mode == 2 {
        3
    } else if mode == 1 {
        2
    } else {
        1
    };
    let final_parameter = if mode == 2 {
        (1. + ROOTS[3]) / 2.
    } else if mode == 1 {
        (1. + 0.7745966692414834) / 2.
    } else {
        0.5
    };
    for (index, record) in trace.iter().enumerate() {
        let order = [0, 1, 2, 1, 2][index];
        assert_eq!(
            (record.u, record.v),
            if iso == 1 { (0, order) } else { (order, 0) }
        );
        assert_eq!(record.roots, if index == 4 { replacement } else { initial });
        assert_eq!(record.degree, if mode == 2 { 6 } else { record.roots });
        assert_eq!(record.count, if index == 0 { initial + 2 } else { 1 });
        assert_eq!(record.parameter, [0., 0., 0., 1., final_parameter][index]);
    }
}
fn assert_endpoint_outputs(output: &IsoApproxOutput<'_>, complete: bool) {
    let extent = if complete { 14 } else { 18 };
    let mut expected = vec![0.; extent];
    if complete {
        expected[0] = 1.;
        expected[7] = 1.;
    }
    for (actual, expected) in output.courbe[..extent].iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert!(output.courbe[extent..].iter().all(|value| *value == 7.));
    assert_eq!(&output.contr1[6..], &[7.; 26]);
    assert_eq!(&output.contr2[6..], &[7.; 26]);
    assert_eq!(output.nbcrbe, i32::from(complete));
    assert_eq!(output.ncoeff, if complete { [1, -7, -7] } else { [-7; 3] });
    assert_eq!(
        &output.contr1[..6],
        if complete {
            &[1., 1., 0., 0., 0., 0.]
        } else {
            &[0.; 6]
        }
    );
    assert_eq!(
        &output.contr2[..6],
        if complete {
            &[1., 1., 0., 0., 0., 0.]
        } else {
            &[1., 1., 1., 1., 7., 7.]
        }
    );
}
fn run_endpoint_case(mode: i32, iso: i32) {
    let complete = mode == 2;
    let mut request = curve_request(iso);
    request.iordre = 2;
    request.ndgjac = if complete { 6 } else { 3 };
    request.nbcrmx = if complete { 1 } else { 3 };
    if complete {
        request.nbroot = 8;
        request.rootlg = &ROOTS;
        request.ncflim = 7;
    }
    let mut frame = endpoint_frame(complete);
    let trace = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        endpoint_evaluation(mode, &trace, call, values, error)
    };
    let mut storage = CurveStorage::new();
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 3], [7.; 3]);
    let mut output = storage.output();
    output.ncoeff = &mut counts;
    output.tabdec = &mut cuts;
    output.errmax = &mut maximum;
    output.errmoy = &mut mean;
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(error, if complete { 0 } else { 101 });
    assert_eq!(
        request.nbroot,
        if complete {
            8
        } else if mode == 1 {
            2
        } else {
            1
        }
    );
    assert_eq!(request.ndgjac, if complete { 6 } else { request.nbroot });
    assert_endpoint_trace(&trace.lock().unwrap(), mode, iso);
    assert_endpoint_outputs(&output, complete);
}
#[test]
fn upper_endpoint_rereads_shared_root_count_one_between_derivative_orders() {
    for iso in [1, 2] {
        run_endpoint_case(0, iso);
    }
}
#[test]
fn upper_endpoint_rereads_shared_root_count_two_between_derivative_orders() {
    for iso in [1, 2] {
        run_endpoint_case(1, iso);
    }
}
#[test]
fn transient_root_count_between_upper_derivatives_keeps_constant_approximation() {
    for iso in [1, 2] {
        run_endpoint_case(2, iso);
    }
}
