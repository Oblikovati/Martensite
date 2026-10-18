//! Ordered maximum/RMS publication, with independent linear projection arithmetic.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Output, ROOTS8, Reals};
use super::*;
use martensite_tkgeombase::adv_app2var::AdvApp2VarErrorOutput as ErrorOutput;
fn error_frame(iso: i32, shared: bool) -> Frame {
    error_frame_with_domain(iso, shared, [-1., 1., -1., 1.], 0.)
}
fn error_frame_with_domain(iso: i32, shared: bool, domain: [f64; 4], constant: f64) -> Frame {
    let mut owner = vec![7.; 1025];
    owner[16..20].copy_from_slice(&domain);
    owner[32..40].copy_from_slice(&ROOTS8);
    owner[608] = 1e-6;
    owner[1024] = constant;
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
    .with_error_output_origins([512, if shared { 512 } else { 576 }])
    .unwrap()
}
fn error_request(iso: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [-1., 1., -1., 1.],
        tconst: 0.,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 1,
        epsapr: &[1e-6],
    }
}
fn independent_linear_errors() -> [f64; 2] {
    let weights = [
        0.08148077003260405,
        0.20191512550812227,
        0.21698010683678565,
        0.11905590159029178,
    ];
    let mut coefficient = 0.;
    for (root, weight) in ROOTS8[4..].iter().zip(weights) {
        coefficient += (root - (-root)) * weight;
    }
    let upper = coefficient.abs() * 1.5_f64.sqrt();
    [
        (upper * upper).sqrt(),
        ((coefficient * coefficient) / 2.).sqrt(),
    ]
}
fn run_error_outputs(iso: i32, shared: bool) {
    let mut frame = error_frame(iso, shared);
    let mut request = error_request(iso);
    let mut tables = CurveStorage::new();
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        for (index, root) in ROOTS8.iter().enumerate() {
            assert_eq!(call.buffers().parameter(index).to_bits(), root.to_bits());
            call.buffers().set_result(index, *root);
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
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -1);
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 1);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[1]);
    assert_eq!(output.courbe[0].to_bits(), 0);
    assert_eq!(&output.courbe[1..], &[7.; 31]);
    let [maximum, mean] = independent_linear_errors();
    let expected = [if shared { mean } else { maximum }, mean];
    for (role, value, expected) in [
        (ErrorOutput::Maximum, output.errmax[0], expected[0]),
        (ErrorOutput::RootMeanSquare, output.errmoy[0], expected[1]),
    ] {
        assert_eq!(value.to_bits(), expected.to_bits());
        assert_eq!(
            frame.error_output(role, 0).unwrap().to_bits(),
            expected.to_bits()
        );
        assert_eq!(frame.error_output(role, 1).unwrap(), 7.);
    }
}
#[test]
fn error_output_disjoint_iso1() {
    run_error_outputs(1, false);
}
#[test]
fn error_output_shared_iso1() {
    run_error_outputs(1, true);
}
#[test]
fn error_output_disjoint_iso2() {
    run_error_outputs(2, false);
}
#[test]
fn error_output_shared_iso2() {
    run_error_outputs(2, true);
}
#[path = "bivariate_error_output_contracts.rs"]
mod contracts;
#[path = "bivariate_error_threshold_cases.rs"]
mod threshold;
