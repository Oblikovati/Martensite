//! Caller layout belongs to original arguments, preceding the operation's first status write.
use super::*;
fn expected_layout_owner(prefix: bool) -> Vec<f64> {
    let mut owner = initial_reals();
    owner[64..67].fill(0.);
    owner[448..450].copy_from_slice(&[0., 1.]);
    for index in if prefix { &[0, -1, 0][..] } else { &[0][..] } {
        let folded = (*index * 5) as isize;
        owner[(192 + folded) as usize..(197 + folded) as usize].fill(0.);
        owner[(256 + folded) as usize..(261 + folded) as usize].fill(0.);
        owner[(320 + *index) as usize] = 1.;
        owner[(384 + *index) as usize] = 1.;
        owner[(512 + *index) as usize] = 0.;
        owner[(576 + *index) as usize] = 0.;
        owner[(64 + *index * 3) as usize] = 1.;
    }
    owner
}
fn evaluate_entry_layout(call: &mut Call<'_, '_, '_>, ordinal: i32, prefix: bool) {
    assert_eq!(call.integer(Int::Dimension), 1);
    assert_eq!(call.integer(Int::ParameterCount), 10);
    let (low, high) = if ordinal == 2 { (7., 0.) } else { (0., 1.) };
    let expected = [low]
        .into_iter()
        .chain(ROOTS8.map(|root| (high - low) / 2. * root + (high + low) / 2.))
        .chain([high]);
    for (index, parameter) in expected.enumerate() {
        assert_eq!(
            call.buffers().parameter(index).to_bits(),
            parameter.to_bits()
        );
        call.buffers().set_result(index, 1.);
    }
    if prefix && ordinal == 1 {
        call.set_caller_output(Output::CompletedPieces, -2).unwrap();
    }
    call.set_caller_output(Output::Status, 0).unwrap();
}
fn check_entry_layout(frame: &Frame, prefix: bool) {
    let owner = expected_layout_owner(prefix);
    expectations::check_curve(frame, &owner);
    expectations::check_window(&owner, 64, |i| frame.caller_curve_value_signed(i).unwrap());
    expectations::check_window(&owner, 192, |i| {
        frame.folded_output(Folded::SymmetricSum, i).unwrap()
    });
    expectations::check_window(&owner, 256, |i| {
        frame
            .folded_output(Folded::AntisymmetricDifference, i)
            .unwrap()
    });
    expectations::check_window(&owner, 320, |i| {
        frame.endpoint_output(Endpoint::Lower, i).unwrap()
    });
    expectations::check_window(&owner, 384, |i| {
        frame.endpoint_output(Endpoint::Upper, i).unwrap()
    });
    expectations::check_window(&owner, 448, |i| frame.cut_parameter(i).unwrap());
    expectations::check_window(&owner, 512, |i| {
        frame.error_output(ErrorRole::Maximum, i).unwrap()
    });
    expectations::check_window(&owner, 576, |i| {
        frame.error_output(ErrorRole::RootMeanSquare, i).unwrap()
    });
}
fn run_entry_layout(iso: i32, prefix: bool) {
    let (frame, curve) = signed_piece_frame(iso);
    let mut frame = frame.with_caller_output_slots([36, 41]).unwrap();
    let (mut request, mut tables) = (signed_piece_request(iso), CurveStorage::new());
    let mut output = signed_piece_output(&mut tables, curve);
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        let mut ordinal = calls.lock().unwrap();
        *ordinal += 1;
        assert!(*ordinal <= 3);
        evaluate_entry_layout(call, *ordinal, prefix);
    };
    frame
        .approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), if prefix { 3 } else { 1 });
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[1]);
    assert_eq!(request.iordre, 0);
    let mut expected = initial_integers(iso);
    expected[36] = 0;
    expected[41] = 1;
    expected[64] = 1;
    if prefix {
        expected[63] = 1;
    }
    assert_eq!(&frame.integer_values()[32..70], &expected[32..70]);
    check_entry_layout(&frame, prefix);
}
#[test]
fn entry_layout_control_iso1() {
    run_entry_layout(1, false);
}
#[test]
fn entry_layout_control_iso2() {
    run_entry_layout(2, false);
}
#[test]
fn entry_layout_prefix_iso1() {
    run_entry_layout(1, true);
}
#[test]
fn entry_layout_prefix_iso2() {
    run_entry_layout(2, true);
}
