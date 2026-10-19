//! Complete constant operations with a live precision/error identity.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerRealArray as Array, AdvApp2VarRealField as Real,
};
fn threshold_frame(iso: i32, shared: bool) -> Frame {
    error_frame_with_domain(iso, false, [0., 1., 0., 1.], 0.5)
        .with_error_output_origins([if shared { 608 } else { 512 }, 576])
        .unwrap()
        .with_caller_array_slots(Array::SubspaceTolerances, vec![608])
        .unwrap()
}
fn independent_constant_projection(shared: bool) -> ([f64; 3], [f64; 2]) {
    let constant_weights = [
        0.2564561626532326,
        0.221824096603541,
        0.15724713746926033,
        0.07157938446051357,
    ];
    let quadratic_weights = [
        -0.25778313522636853,
        -0.04252004995346777,
        0.1589358116831741,
        0.1413673734966622,
    ];
    let (mut constant, mut quadratic) = (0., 0.);
    for (zero, two) in constant_weights.into_iter().zip(quadratic_weights) {
        constant += 2. * zero;
        quadratic += 2. * two;
    }
    let component = f64::abs(quadratic) * 2.5_f64.sqrt();
    let errors = if shared {
        [0., 0.]
    } else {
        [
            (component * component).sqrt(),
            ((quadratic * quadratic) / 2.).sqrt(),
        ]
    };
    let tail = if shared { quadratic } else { 0. };
    let curve = [
        (0. + constant * std::f64::consts::FRAC_1_SQRT_2) + tail * (-0.625_f64.sqrt()),
        0.,
        0. + tail * 5.625_f64.sqrt(),
    ];
    (curve, errors)
}
fn run_threshold_output(iso: i32, shared: bool) {
    let mut frame = threshold_frame(iso, shared);
    let mut request = error_request(iso);
    request.uvfonc = [0., 1., 0., 1.];
    request.tconst = 0.5;
    request.ncflim = 3;
    let mut tables = CurveStorage::new();
    tables.maximum[0] = frame.error_output(ErrorOutput::Maximum, 0).unwrap();
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(call.real(Real::ConstantParameter), 0.5);
        for (index, root) in ROOTS8.iter().enumerate() {
            assert_eq!(
                call.buffers().parameter(index).to_bits(),
                (0.5 + 0.5 * root).to_bits()
            );
            call.buffers().set_result(index, 1.);
        }
        call.set_caller_output(Output::Status, 0).unwrap();
    };
    assert_eq!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .unwrap(),
        0
    );
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[if shared { 3 } else { 1 }]);
    let (curve, errors) = independent_constant_projection(shared);
    assert_eq!(
        output.courbe[..3]
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        curve.map(f64::to_bits)
    );
    assert_eq!(&output.courbe[3..], &[7.; 29]);
    assert_eq!(output.errmax[0].to_bits(), errors[0].to_bits());
    assert_eq!(output.errmoy[0].to_bits(), errors[1].to_bits());
    assert_eq!(
        frame
            .caller_array_value(Array::SubspaceTolerances, 0)
            .unwrap()
            .to_bits(),
        if shared { 0_f64 } else { 1e-6_f64 }.to_bits()
    );
}
#[test]
fn threshold_error_disjoint_iso1() {
    run_threshold_output(1, false);
}
#[test]
fn threshold_error_shared_iso1() {
    run_threshold_output(1, true);
}
#[test]
fn threshold_error_disjoint_iso2() {
    run_threshold_output(2, false);
}
#[test]
fn threshold_error_shared_iso2() {
    run_threshold_output(2, true);
}
