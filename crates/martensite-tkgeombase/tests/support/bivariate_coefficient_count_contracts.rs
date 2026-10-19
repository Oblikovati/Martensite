//! Bivariate coefficient count contracts behavior and supporting operations.
use super::*;
#[test]
fn signed_count_offsets_require_initialized_physical_slots_and_keep_private_locals_distinct() {
    let (slots, origin) = initial_count_slots(1, "none");
    assert!(
        count_frame(slots.clone(), origin, false)
            .with_coefficient_count_origin(0)
            .is_err()
    );
    assert!(
        count_frame(slots.clone(), origin, false)
            .with_coefficient_count_origin(128)
            .is_err()
    );
    let frame = count_frame(slots, origin, false);
    assert_eq!(frame.coefficient_count(-1).unwrap(), -7);
    assert!(frame.coefficient_count(-65).is_err());
    assert!(frame.coefficient_count(i32::MAX).is_err());
    assert!(frame.coefficient_count(i32::MIN).is_err());
    assert!(frame.coefficient_count(-64).is_err());
}
#[test]
fn count_callback_writes_are_bidirectional_and_the_frame_is_restored_on_unwind() {
    let (slots, origin) = initial_count_slots(2, "dimension");
    let mut frame = count_frame(slots.clone(), origin, false);
    let mut request = IsoApproxRequest {
        ndimen: 2,
        nbsesp: 1,
        ndimse: &[2, 1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: 2,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6; 4],
    };
    let mut count = slots[origin..origin + 6].to_vec();
    let (mut curve, mut sums, mut diff, mut left, mut right, mut cuts, mut max, mut mean) = (
        [7.; 128], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64],
    );
    let mut output = IsoApproxOutput {
        ncoeff: &mut count,
        courbe: &mut curve,
        nbcrbe: -7,
        somtab: &mut sums,
        diftab: &mut diff,
        contr1: &mut left,
        contr2: &mut right,
        tabdec: &mut cuts,
        errmax: &mut max,
        errmoy: &mut mean,
    };
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        assert!(call.coefficient_count(-33).is_err());
        assert!(call.set_coefficient_count(-32, 9).is_err());
        call.set_coefficient_count(0, 3).unwrap();
        assert_eq!(call.integer(Int::Dimension), 3);
        call.set_integer(Int::Dimension, 2);
        assert_eq!(call.coefficient_count(0).unwrap(), 2);
        panic!("owned coefficient callback unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        }))
        .is_err()
    );
    assert_eq!(frame.coefficient_count(0).unwrap(), 2);
    assert_eq!(frame.integer_values()[32], 2);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(&output.courbe[..6], &[0.; 6]);
}
#[test]
fn count_mirror_mismatch_fails_before_clear_or_callback() {
    let (slots, origin) = initial_count_slots(1, "none");
    let mut frame = count_frame(slots, origin, false);
    let mut request = IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1, 1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: 1,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6; 4],
    };
    let (
        mut count,
        mut curve,
        mut sums,
        mut diff,
        mut left,
        mut right,
        mut cuts,
        mut max,
        mut mean,
    ) = (
        [0; 6], [7.; 128], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64], [7.; 64],
    );
    let mut output = IsoApproxOutput {
        ncoeff: &mut count,
        courbe: &mut curve,
        nbcrbe: -7,
        somtab: &mut sums,
        diftab: &mut diff,
        contr1: &mut left,
        contr2: &mut right,
        tabdec: &mut cuts,
        errmax: &mut max,
        errmoy: &mut mean,
    };
    let evaluator =
        |_: &mut Call<'_, '_, '_>| panic!("unexpected evaluator on mismatched count mirror");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(output.courbe, &[7.; 128]);
    assert_eq!(frame.coefficient_count(0).unwrap(), -7);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -7);
}
