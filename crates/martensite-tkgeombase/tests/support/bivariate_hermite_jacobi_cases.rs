//! A completed sextic pins Hermite/coefficient incidence during constraint addition.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerOrderField as Order, AdvApp2VarCombinedEvaluation as Call,
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
fn hermite_jacobi_frame() -> Frame {
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
        .with_caller_order_slots([5, 6])
        .unwrap()
}
fn assert_sextic_output(output: &IsoApproxOutput<'_>) {
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, [7, -7, -7]);
    assert_eq!(output.tabdec, [0., 1., 7., 7.]);
    let coefficients = [1. / 64., 0., -3. / 64., 0., 3. / 64., 0., -1. / 64.];
    for (actual, expected) in output.courbe[..14]
        .iter()
        .zip(coefficients.into_iter().cycle())
    {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[14..42], &[0.; 28]);
    assert_eq!(&output.courbe[42..], &[7.; 22]);
    assert_eq!(&output.contr1[..6], &[0.; 6]);
    assert_eq!(&output.contr1[6..], &[7.; 58]);
    assert_eq!(&output.contr2[..2], &[0.; 2]);
    assert!(
        output.contr2[2..4]
            .iter()
            .all(|value| value.to_bits() == (-0.0_f64).to_bits())
    );
    assert!(
        output.contr2[4..6]
            .iter()
            .all(|value| value.to_bits() == 0.0_f64.to_bits())
    );
    assert_eq!(&output.contr2[6..], &[7.; 58]);
}
fn evaluate_sextic(call: &mut Call<'_, '_>, values: &mut [f64]) {
    let count = call.integer(Local::ParameterCount) as usize;
    let order = call.integer(Local::UOrder) + call.integer(Local::VOrder);
    for (i, value) in values[..2 * count].iter_mut().enumerate() {
        let parameter = call.parameters()[i / 2];
        let product = parameter * (1. - parameter);
        *value = match order {
            0 => product * product * product,
            1 => 3. * product * product * (1. - 2. * parameter),
            2 => {
                6. * product * (1. - 2. * parameter) * (1. - 2. * parameter)
                    - 6. * product * product
            }
            _ => panic!("derivative order={order}; expected 0..=2"),
        };
    }
}
fn run_hermite_jacobi_case(iso: i32) {
    let mut request = curve_request(iso);
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.ndgjac = 6;
    request.iordre = 0;
    request.ncflim = 7;
    request.nbcrmx = 3;
    let mut frame = hermite_jacobi_frame();
    let trace = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut trace = trace.lock().unwrap();
        trace.push((
            call.caller_order(Order::ContinuityOrder).unwrap(),
            call.integer(Local::ParameterCount),
            call.integer(Local::UOrder),
            call.integer(Local::VOrder),
            call.parameters()[0],
        ));
        if trace.len() == 1 {
            call.set_caller_order(Order::ContinuityOrder, 2).unwrap();
        }
        assert_eq!(call.caller_order(Order::CrossDerivativeOrder).unwrap(), 0);
        evaluate_sextic(call, values);
        *error = 0;
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 3], [7.; 3]);
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: curve,
        nbcrbe: 0,
        somtab: sums,
        diftab: differences,
        contr1: first,
        contr2: second,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!((request.iordre, request.ideriv, error), (2, 0, 0));
    let first = if iso == 1 { (0, 1) } else { (1, 0) };
    let second = if iso == 1 { (0, 2) } else { (2, 0) };
    assert_eq!(
        *trace.lock().unwrap(),
        [
            (0, 10, 0, 0, 0.),
            (2, 1, first.0, first.1, 0.),
            (2, 1, second.0, second.1, 0.),
            (2, 1, first.0, first.1, 1.),
            (2, 1, second.0, second.1, 1.)
        ]
    );
    assert_sextic_output(&output);
}
#[test]
fn continuity_growth_completes_closed_form_sextic_on_first_iso_path() {
    run_hermite_jacobi_case(1);
}
#[test]
fn continuity_growth_completes_closed_form_sextic_on_second_iso_path() {
    run_hermite_jacobi_case(2);
}
