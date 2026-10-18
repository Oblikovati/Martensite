//! Bivariate iso integer output contract cases behavior and supporting operations.
use super::*;
#[test]
fn caller_output_slot_contracts_reject_missing_private_and_mismatched_storage() {
    let values = vec![0; 64];
    let bare = Frame::new(
        values,
        Bindings {
            dimension: 32,
            favorite_iso: 36,
            parameter_count: 0,
            u_order: 1,
            v_order: 2,
        },
        vec![0., 1., 0., 1., 0.5],
        Reals {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap();
    assert!(bare.caller_output(Output::Status).is_err());
    assert!(bare.with_caller_output_slots([0, 34]).is_err());
    assert!(
        output_frame(1, 3)
            .with_caller_output_slots([64, 34])
            .is_err()
    );
    let mut frame = output_frame(1, 3);
    let mut request = output_request(1, true);
    let mut tables = OutputTables::new();
    let mut output = tables.output(0);
    let evaluator = |_: &mut Call<'_, '_, '_>| {
        panic!("invalid integer output contract must fail before callback")
    };
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    output.nbcrbe = -7;
    request.ndimen = 2;
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert!(output.courbe.iter().all(|value| *value == 7.));
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -7);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Output>();
}
#[test]
fn borrowed_entrypoints_reject_configured_owned_output_fields() {
    let mut frame = output_frame(2, 3);
    let mut request = output_request(2, true);
    let mut tables = OutputTables::new();
    let mut output = tables.output(-7);
    let mut code = -7;
    let evaluator = |_: &mut Call<'_, '_, '_>, _: &mut i32| {
        panic!("ambiguous output entrypoint must fail before evaluation")
    };
    assert!(
        frame
            .approximate_iso_buffers(&mut request, &evaluator, &mut output, &mut code)
            .is_err()
    );
    let slice =
        |_: &mut martensite_tkgeombase::adv_app2var::AdvApp2VarCombinedEvaluation<'_, '_>,
         _: &mut [f64],
         _: &mut i32| panic!("ambiguous slice entrypoint must fail before evaluation");
    assert!(
        frame
            .approximate_iso(&mut request, &slice, &mut output, &mut code)
            .is_err()
    );
    assert_eq!(code, -7);
    assert!(output.courbe.iter().all(|value| *value == 7.));
}
#[test]
fn caller_piece_and_dimension_writes_are_immediately_bidirectional_during_callback() {
    let mut frame = output_frame(1, 1);
    let mut request = output_request(1, false);
    let mut tables = OutputTables::new();
    let mut output = tables.output(1);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        call.set_caller_output(Output::CompletedPieces, 2).unwrap();
        assert_eq!(call.integer(Int::Dimension), 2);
        call.set_integer(Int::Dimension, 3);
        assert_eq!(call.caller_output(Output::CompletedPieces).unwrap(), 3);
        call.set_caller_output(Output::CompletedPieces, 0).unwrap();
        call.set_caller_output(Output::Status, 1).unwrap();
    };
    frame
        .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(
        (
            output.nbcrbe,
            request.ndimen,
            frame.caller_output(Output::Status).unwrap()
        ),
        (0, 0, 101)
    );
}
#[test]
fn owned_integer_outputs_and_actual_curve_owner_use_the_same_pipeline() {
    for iso in [1, 2] {
        let mut integers = vec![-7; 64];
        integers[32] = 1;
        integers[33] = 8;
        integers[36] = iso;
        integers[37] = 2;
        let mut reals = vec![7.; 69];
        reals[..5].copy_from_slice(&[0., 1., 0., 1., 0.5]);
        let mut frame = Frame::new(
            integers,
            Bindings {
                dimension: 32,
                favorite_iso: 36,
                parameter_count: 0,
                u_order: 1,
                v_order: 2,
            },
            reals,
            Reals {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
        .with_caller_sampling_slots([33, 37])
        .unwrap()
        .with_caller_output_slots([35, 34])
        .unwrap();
        let curve = frame.caller_curve_slots(5, 64).unwrap();
        let mut request = output_request(iso, true);
        let mut tables = OutputTables::new();
        let slices = tables.output(-7);
        let IsoApproxOutput {
            ncoeff,
            nbcrbe,
            somtab,
            diftab,
            contr1,
            contr2,
            tabdec,
            errmax,
            errmoy,
            ..
        } = slices;
        let mut output = martensite_tkgeombase::adv_app2var::approx_f2var::IsoApproxCallerOutput {
            ncoeff,
            courbe: curve,
            nbcrbe,
            somtab,
            diftab,
            contr1,
            contr2,
            tabdec,
            errmax,
            errmoy,
        };
        let evaluator = |call: &mut Call<'_, '_, '_>| {
            assert_eq!(call.caller_curve_value(0).unwrap(), 0.);
            for index in 0..8 {
                call.buffers().set_result(index, 1.);
            }
            call.set_caller_output(Output::Status, 0).unwrap();
        };
        frame
            .approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
            .unwrap();
        assert_eq!(
            (output.nbcrbe, frame.caller_output(Output::Status).unwrap()),
            (1, 0)
        );
        assert!((frame.caller_curve_value(0).unwrap() - 1.).abs() <= 32. * f64::EPSILON);
        assert_eq!(
            (
                frame.caller_curve_value(1).unwrap(),
                frame.caller_curve_value(2).unwrap()
            ),
            (0., 0.)
        );
        assert_eq!(frame.caller_curve_value(3).unwrap(), 7.);
    }
}
#[test]
fn callback_unwind_retains_actual_owned_output_and_input_identity() {
    let mut frame = output_frame(1, 1);
    let mut request = output_request(1, false);
    let mut tables = OutputTables::new();
    let mut output = tables.output(1);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        call.set_caller_output(Output::CompletedPieces, 2).unwrap();
        assert_eq!(call.integer(Int::Dimension), 2);
        panic!("owned integer output unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        ))
        .is_err()
    );
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 2);
    assert_eq!(frame.integer_values()[32], 2);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(&output.courbe[..3], &[0., 0., 0.]);
}
#[test]
fn initialized_subspace_dimension_slots_are_validated_before_any_output_clear() {
    let mut frame = output_frame(1, 0)
        .with_caller_dimension_slots(vec![32])
        .unwrap();
    let mut request = output_request(1, false);
    let mut tables = OutputTables::new();
    let mut output = tables.output(-7);
    let evaluator = |_: &mut Call<'_, '_, '_>| {
        panic!("mismatched initialized dimension must fail before callback")
    };
    request.ndimse = &[2];
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    request.ndimse = &[];
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 1);
    assert!(output.courbe.iter().all(|value| *value == 7.));
    request.ndimse = &[1];
    let stop = |call: &mut Call<'_, '_, '_>| {
        assert_eq!(call.caller_dimension(0).unwrap(), 0);
        call.set_caller_output(Output::Status, 1).unwrap();
    };
    frame
        .approximate_iso_owned_outputs(&mut request, &stop, &mut output)
        .unwrap();
    assert_eq!(frame.caller_dimensions().unwrap(), vec![101]);
    assert_eq!(request.ndimen, 101);
}
#[test]
fn old_unbound_callback_reports_missing_output_identity_instead_of_fabricating_slots() {
    let mut values = vec![0; 64];
    values[32] = 1;
    values[36] = 2;
    let mut frame = Frame::new(
        values,
        Bindings {
            dimension: 32,
            favorite_iso: 36,
            parameter_count: 0,
            u_order: 1,
            v_order: 2,
        },
        vec![0., 1., 0., 1., 0.5],
        Reals {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap();
    let mut request = output_request(2, true);
    let mut tables = OutputTables::new();
    let mut output = tables.output(-7);
    let unused =
        |_: &mut Call<'_, '_, '_>| panic!("missing output binding must fail before callback");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &unused, &mut output)
            .is_err()
    );
    let stop = |call: &mut Call<'_, '_, '_>, error: &mut i32| {
        assert!(call.caller_output(Output::Status).is_err());
        assert!(call.set_caller_output(Output::CompletedPieces, 1).is_err());
        *error = 1;
    };
    let mut status = -7;
    frame
        .approximate_iso_buffers(&mut request, &stop, &mut output, &mut status)
        .unwrap();
    assert_eq!(status, 101);
}
