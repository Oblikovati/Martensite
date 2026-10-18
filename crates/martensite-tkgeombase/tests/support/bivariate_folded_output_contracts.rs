//! Bivariate folded output contracts behavior and supporting operations.
use super::*;
#[test]
fn folded_origin_and_signed_access_require_initialized_owner() {
    let frame = folded_frame(1, true);
    assert_eq!(frame.folded_output(Folded::SymmetricSum, -1).unwrap(), 7.);
    assert!(frame.folded_output(Folded::SymmetricSum, -193).is_err());
    assert!(frame.folded_output(Folded::SymmetricSum, i32::MAX).is_err());
    assert!(
        folded_frame(1, true)
            .with_folded_output_origins([1025, 192])
            .is_err()
    );
}
#[test]
fn folded_callback_stores_share_inputs_and_survive_unwind_without_a_guard() {
    let mut frame = folded_frame(1, true)
        .with_folded_output_origins([16, 16])
        .unwrap();
    let mut request = folded_request(1);
    let mut tables = CurveStorage::new();
    for index in 0..32 {
        let value = frame
            .folded_output(Folded::SymmetricSum, index as i32)
            .unwrap();
        tables.sums[index] = value;
        tables.differences[index] = value;
    }
    let mut output = tables.output();
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        call.set_folded_output(Folded::SymmetricSum, 0, 0.25)
            .unwrap();
        assert_eq!(
            call.folded_output(Folded::AntisymmetricDifference, 0)
                .unwrap(),
            0.25
        );
        assert_eq!(
            call.real(martensite_tkgeombase::adv_app2var::AdvApp2VarRealField::UStart),
            0.25
        );
        call.set_real(
            martensite_tkgeombase::adv_app2var::AdvApp2VarRealField::UStart,
            0.75,
        );
        assert_eq!(call.folded_output(Folded::SymmetricSum, 0).unwrap(), 0.75);
        panic!("folded initialized owner unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        }))
        .is_err()
    );
    assert_eq!(frame.folded_output(Folded::SymmetricSum, 0).unwrap(), 0.75);
    assert_eq!(
        frame
            .folded_output(Folded::AntisymmetricDifference, 0)
            .unwrap(),
        0.75
    );
}
#[test]
fn folded_mirror_mismatch_rejects_before_clearing_or_callback() {
    let mut frame = folded_frame(1, true)
        .with_folded_output_origins([16, 16])
        .unwrap();
    let mut request = folded_request(1);
    let mut tables = CurveStorage::new();
    for index in 0..32 {
        let value = frame
            .folded_output(Folded::SymmetricSum, index as i32)
            .unwrap();
        tables.sums[index] = value;
        tables.differences[index] = value;
    }
    assert_eq!(tables.differences[0].to_bits(), 0_f64.to_bits());
    tables.differences[0] = -0.;
    let mut output = tables.output();
    let evaluator = |_: &mut Call<'_, '_, '_>| panic!("unexpected mismatched folded callback");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(output.courbe, &[7.; 32]);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -7);
}
