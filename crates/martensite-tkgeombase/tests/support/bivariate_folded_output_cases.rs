//! Same-owner folded stores are observed before projection, with independent constant results.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Output, ROOTS8, Reals};
use super::*;
use martensite_tkgeombase::adv_app2var::AdvApp2VarFoldedOutput as Folded;
fn folded_frame(iso: i32, shared: bool) -> Frame {
    let mut owner = vec![7.; 1025];
    owner[16..20].copy_from_slice(&[0., 1., 0., 1.]);
    owner[32..40].copy_from_slice(&ROOTS8);
    owner[608] = 1e-6;
    owner[1024] = 0.5;
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
    .with_folded_output_origins([192, if shared { 192 } else { 256 }])
    .unwrap()
}
fn folded_request(iso: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    }
}
fn run_folded_outputs(iso: i32, shared: bool) {
    let mut frame = folded_frame(iso, shared);
    let mut request = folded_request(iso);
    let mut tables = CurveStorage::new();
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(
            call.real(martensite_tkgeombase::adv_app2var::AdvApp2VarRealField::ConstantParameter),
            0.5
        );
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
    assert_eq!(output.ncoeff, &[1]);
    // Source-order sum then difference stores hit one owner slot when identities coincide.
    let expected_sum: [f64; 5] = if shared {
        [0.; 5]
    } else {
        [0., 2., 2., 2., 2.]
    };
    for (table, values, expected) in [
        (Folded::SymmetricSum, &*output.somtab, expected_sum),
        (Folded::AntisymmetricDifference, &*output.diftab, [0.; 5]),
    ] {
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(values[index].to_bits(), value.to_bits());
            assert_eq!(
                frame.folded_output(table, index as i32).unwrap().to_bits(),
                value.to_bits()
            );
        }
        assert_eq!(&values[5..], &[7.; 27]);
    }
    if shared {
        assert_eq!(
            output.courbe[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            vec![0; 3]
        );
        assert_eq!(&output.courbe[3..], &[7.; 29]);
        assert_eq!(output.errmax[0].to_bits(), 0);
        assert_eq!(output.errmoy[0].to_bits(), 0);
    } else {
        super::coefficient_count_cases::check_count_curve(&output, "none", 1, 1);
    }
}
#[test]
fn folded_output_disjoint_iso1() {
    run_folded_outputs(1, false);
}
#[test]
fn folded_output_shared_iso1() {
    run_folded_outputs(1, true);
}
#[test]
fn folded_output_disjoint_iso2() {
    run_folded_outputs(2, false);
}
#[test]
fn folded_output_shared_iso2() {
    run_folded_outputs(2, true);
}
#[path = "bivariate_folded_output_contracts.rs"]
mod contracts;
