//! Additive signed callback access keeps live owner writes on error and unwind.
use super::*;
fn contract_integer_frame() -> Frame {
    Frame::new(
        vec![1, 1, 1, 2, -7],
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap()
    .with_owned_status_slot(4)
    .unwrap()
}
fn contract_request() -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: 1,
        nbpntv: 1,
        urootb: &[0.],
        vrootb: &[0.],
        isofav: 2,
    }
}
fn contract_frames() -> (Frame, AdvApp2VarSquareSampleFrame, SampledSquare<'static>) {
    let samples = AdvApp2VarSquareSampleFrame::new(vec![7.; 16], 8, 10)
        .unwrap()
        .with_table_origins([4; 4])
        .unwrap();
    (contract_integer_frame(), samples, contract_request())
}

fn write_signed_prefix(call: &mut Call<'_, '_>) {
    assert_eq!(call.caller_table_signed(Table::SumSum, -1).unwrap(), 7.);
    call.set_caller_table_signed(Table::SumSum, -1, 9.).unwrap();
    assert_eq!(
        call.caller_table_signed(Table::DifferenceSum, -1).unwrap(),
        9.
    );
    let failure = call
        .set_caller_table_signed(Table::SumSum, -5, 42.)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert!(failure.message().contains("offset=-5"));
    assert!(call.caller_table(Table::SumSum, usize::MAX).is_err());
}
#[test]
fn signed_callback_prefix_access_preserves_writes_on_positive_error() {
    let (mut frame, mut samples, mut request) = contract_frames();
    let evaluator = |call: &mut Call<'_, '_>| {
        write_signed_prefix(call);
        call.set_status(1).unwrap();
    };
    frame
        .sample_square_owned_tables(&mut request, &evaluator, &mut samples)
        .unwrap();
    assert_eq!(frame.status().unwrap(), 201);
    assert_eq!(samples.values[3], 9.);
    assert_eq!(samples.table_signed(Table::SumSum, -1).unwrap(), 9.);
    assert_eq!(samples.values[2], 7.);
}
#[test]
fn signed_callback_prefix_access_restores_frame_after_unwind() {
    let (mut frame, mut samples, mut request) = contract_frames();
    let evaluator = |call: &mut Call<'_, '_>| {
        write_signed_prefix(call);
        call.set_status(7).unwrap();
        panic!("named signed table callback unwind");
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        frame.sample_square_owned_tables(&mut request, &evaluator, &mut samples)
    }));
    assert!(result.is_err());
    assert_eq!(frame.status().unwrap(), 7);
    assert_eq!(samples.values[3], 9.);
    assert_eq!(
        samples
            .table_signed(Table::DifferenceDifference, -1)
            .unwrap(),
        9.
    );
}
