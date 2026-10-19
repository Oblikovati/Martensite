//! Exact initialized caller-domain/cut-table chronology, with closed-form constant results.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Output, ROOTS8, Reals};
use super::*;
use martensite_tkgeombase::adv_app2var::AdvApp2VarRealField as Real;
fn domain_stages(iso: i32, shared: bool) -> ([f64; 4], [f64; 4], [f64; 2]) {
    let mut caller = [0., 4., 3., 6.];
    if iso == 1 {
        let u = [caller[0], caller[1]];
        let cuts = [caller[2], caller[3]];
        if shared {
            caller[0] = cuts[0];
            caller[1] = cuts[1];
        }
        return (caller, [u[0], u[1], cuts[0], cuts[1]], cuts);
    }
    let cuts = [caller[0], caller[1]];
    if shared {
        caller[2] = cuts[0];
        caller[3] = cuts[1];
    }
    (caller, [cuts[0], cuts[1], caller[2], caller[3]], cuts)
}
fn run_cut_domain(iso: i32, shared: bool) {
    let origin = if shared {
        if iso == 1 { 16 } else { 18 }
    } else {
        448
    };
    let mut owner = vec![7.; 1025];
    owner[16..20].copy_from_slice(&[0., 4., 3., 6.]);
    owner[32..40].copy_from_slice(&ROOTS8);
    owner[608] = 1e-6;
    owner[1024] = 3.5;
    let mut cuts = owner[origin..origin + 12].to_vec();
    let mut frame = Frame::new(
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
    .with_cut_parameter_origin(origin)
    .unwrap();
    let mut request = IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [0., 4., 3., 6.],
        tconst: 3.5,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    };
    let (mut counts, mut curve, mut sums, mut diff, mut left, mut right, mut maximum, mut mean) = (
        [-7; 3], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64],
    );
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: &mut curve,
        nbcrbe: -7,
        somtab: &mut sums,
        diftab: &mut diff,
        contr1: &mut left,
        contr2: &mut right,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let (expected_caller, expected_callback, expected_cuts) = domain_stages(iso, shared);
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(
            [
                call.u_bounds()[0],
                call.u_bounds()[1],
                call.v_bounds()[0],
                call.v_bounds()[1]
            ],
            expected_callback
        );
        assert_eq!(
            [Real::UStart, Real::UEnd, Real::VStart, Real::VEnd].map(|field| call.real(field)),
            expected_caller
        );
        assert_eq!(call.real(Real::ConstantParameter), 3.5);
        assert_eq!(
            [
                call.cut_parameter(0).unwrap(),
                call.cut_parameter(1).unwrap()
            ],
            expected_cuts
        );
        let varying = if iso == 1 {
            &expected_callback[2..4]
        } else {
            &expected_callback[..2]
        };
        for (index, root) in ROOTS8.iter().enumerate() {
            let parameter = (varying[0] + varying[1]) / 2. + (varying[1] - varying[0]) / 2. * root;
            assert_eq!(
                call.buffers().parameter(index).to_bits(),
                parameter.to_bits()
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
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 1);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[1, -7, -7]);
    assert_eq!(request.uvfonc, expected_caller);
    assert_eq!(&output.tabdec[..2], &expected_cuts);
    assert_eq!(
        [Real::UStart, Real::UEnd, Real::VStart, Real::VEnd].map(|field| frame.real(field)),
        expected_caller
    );
    super::coefficient_count_cases::check_count_curve(&output, "none", 1, 1);
}
#[test]
fn cut_domain_disjoint_iso1() {
    run_cut_domain(1, false);
}
#[test]
fn cut_domain_shared_iso1() {
    run_cut_domain(1, true);
}
#[test]
fn cut_domain_disjoint_iso2() {
    run_cut_domain(2, false);
}
#[test]
fn cut_domain_shared_iso2() {
    run_cut_domain(2, true);
}

#[path = "bivariate_cut_output_contracts.rs"]
mod contracts;
