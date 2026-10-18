//! Caller dimension entries share integer identity without duplicate mutable references.
use super::*;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCombinedBufferEvaluation as IndexedCall,
    AdvApp2VarCombinedEvaluation as CombinedCall, AdvApp2VarIntegerBindings as Bindings,
    AdvApp2VarIntegerField as Integer, AdvApp2VarIsoOperationFrame as Frame,
    AdvApp2VarRealBindings as RealBindings,
};
pub(super) fn dimension_frame(slots: Option<Vec<usize>>) -> Result<Frame, Failure> {
    let bindings = Bindings {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 4,
    };
    let reals = RealBindings {
        domain: [0, 1, 2, 3],
        constant_parameter: 4,
    };
    let frame = Frame::new(vec![0; 6], bindings, vec![0.; 5], reals)?;
    match slots {
        Some(slots) => frame.with_caller_dimension_slots(slots),
        None => Ok(frame),
    }
}
fn dimension_slot_case(iso: i32, failure: i32) {
    dimension_incidence_case(iso, failure, false);
}
pub(super) fn dimension_incidence_case(iso: i32, failure: i32, indexed: bool) {
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
    let mut request = curve_request(iso);
    request.ndimen = 4;
    request.ndimse = &[4];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = -1;
    request.ndgjac = 3;
    request.nbcrmx = 1;
    request.ncflim = 1;
    let mut frame = dimension_frame(Some(vec![0])).unwrap();
    let calls = Mutex::new(0);
    let evaluator = |call: &mut CombinedCall<'_, '_>, values: &mut [f64], error: &mut i32| {
        observe_dimension_identity(call);
        assert_eq!(values.len(), 36);
        values.fill(0.);
        *calls.lock().unwrap() += 1;
        *error = failure;
    };
    let indexed_evaluator = |call: &mut IndexedCall<'_, '_, '_>, error: &mut i32| {
        observe_dimension_identity(call);
        for index in 0..40 {
            call.buffers().set_result(index, 0.);
        }
        for index in 64..72 {
            call.buffers().set_result(index, 1.);
        }
        assert_eq!(call.buffers().result(71), 1.);
        *calls.lock().unwrap() += 1;
        *error = failure;
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7], [7.; 2], [7.], [7.]);
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: curve,
        nbcrbe: -7,
        somtab: sums,
        diftab: differences,
        contr1: first,
        contr2: second,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let mut error = -1;
    if indexed {
        frame
            .approximate_iso_buffers(&mut request, &indexed_evaluator, &mut output, &mut error)
            .unwrap();
    } else {
        frame
            .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
            .unwrap();
    }
    assert_eq!(
        (
            request.ndimen,
            frame.integer_values()[0],
            frame.caller_dimensions()
        ),
        (2, 2, Some(vec![2]))
    );
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(
        (error, output.nbcrbe),
        if failure == 0 { (0, 1) } else { (101, 0) }
    );
    assert_eq!(output.ncoeff, if failure == 0 { [1] } else { [-7] });
    assert_eq!(output.tabdec, [0., 1.]);
    assert_eq!(&output.courbe[..4], &[0.; 4]);
    assert_eq!(&output.courbe[4..], &[7.; 60]);
    assert_eq!(output.errmax, if failure == 0 { [0.] } else { [7.] });
    assert_eq!(output.errmoy, if failure == 0 { [0.] } else { [7.] });
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
}
fn observe_dimension_identity<Parameters>(call: &mut CombinedCall<'_, '_, Parameters>) {
    assert_eq!(
        (
            call.integer(Integer::Dimension),
            call.caller_dimension(0).unwrap()
        ),
        (4, 4)
    );
    assert_eq!(
        (
            call.integer(Integer::ParameterCount),
            call.integer(Integer::UOrder),
            call.integer(Integer::VOrder)
        ),
        (8, 0, 0)
    );
    call.set_integer(Integer::Dimension, 2);
    assert_eq!(call.caller_dimension(0).unwrap(), 2);
    call.set_caller_dimension(0, 4).unwrap();
    assert_eq!(call.integer(Integer::Dimension), 4);
    call.set_caller_dimension(0, 2).unwrap();
}

#[test]
fn first_iso_shared_dimension_entry_completes_with_immediate_bidirectional_visibility() {
    dimension_slot_case(1, 0);
}
#[test]
fn second_iso_shared_dimension_entry_completes_with_immediate_bidirectional_visibility() {
    dimension_slot_case(2, 0);
}
#[test]
fn first_iso_shared_dimension_entry_retains_alias_writeback_on_callback_failure() {
    dimension_slot_case(1, 1);
}
#[test]
fn second_iso_shared_dimension_entry_retains_alias_writeback_on_callback_failure() {
    dimension_slot_case(2, 1);
}
#[test]
fn dimension_entry_bindings_reject_empty_duplicate_private_and_out_of_range_slots() {
    for (slots, expected) in [
        (vec![], "Standard_DomainError"),
        (vec![0, 0], "Standard_DomainError"),
        (vec![2], "Standard_DomainError"),
        (vec![99], "Standard_OutOfRange"),
    ] {
        let error = dimension_frame(Some(slots)).err().unwrap();
        assert_eq!(error.exception_type(), expected);
        assert!(error.message().contains("expected"));
    }
}
#[test]
fn dimension_entry_binding_shape_is_checked_before_evaluation() {
    let mut frame = dimension_frame(Some(vec![0, 5])).unwrap();
    let mut request = curve_request(2);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let evaluator = |_: &mut CombinedCall<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("invalid binding shape must stop before evaluation")
    };
    let failure = frame
        .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(failure.message().contains("len=1; expected 2"));
}
#[test]
fn conflicting_dimension_entry_incidence_is_reported_before_the_callback() {
    // Rebinding another caller field must preserve the array's initial-value checks.
    let mut frame = dimension_frame(Some(vec![1]))
        .unwrap()
        .with_subspace_count_slot(5)
        .unwrap();
    let mut request = curve_request(1);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let evaluator = |_: &mut CombinedCall<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("inconsistent shared initial values must stop before evaluation")
    };
    let failure = frame
        .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("incident SubspaceDimension(0) value=2")
    );
}
#[test]
fn unconfigured_dimension_entries_return_typed_context_failures() {
    let mut frame = dimension_frame(None).unwrap();
    assert_eq!(frame.caller_dimensions(), None);
    let mut request = curve_request(2);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let evaluator = |call: &mut CombinedCall<'_, '_>, _: &mut [f64], error: &mut i32| {
        for failure in [
            call.caller_dimension(0).unwrap_err(),
            call.set_caller_dimension(0, 1).unwrap_err(),
        ] {
            assert_eq!(failure.exception_type(), "Standard_DomainError");
            assert!(failure.message().contains("index=0 unavailable"));
        }
        *error = 1;
    };
    frame
        .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
        .unwrap();
    assert_eq!(error, 101);
}
#[test]
fn distinct_dimension_entry_slots_remain_independent_and_check_entry_indices() {
    let mut frame = dimension_frame(Some(vec![5])).unwrap();
    let mut request = curve_request(2);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let evaluator = |call: &mut CombinedCall<'_, '_>, _: &mut [f64], error: &mut i32| {
        assert_eq!(call.caller_dimension(0).unwrap(), 2);
        call.set_integer(Integer::Dimension, 1);
        assert_eq!(call.caller_dimension(0).unwrap(), 2);
        call.set_caller_dimension(0, 1).unwrap();
        assert_eq!(call.integer(Integer::Dimension), 1);
        assert!(call.caller_dimension(1).is_err());
        assert!(call.set_caller_dimension(1, 7).is_err());
        *error = 1;
    };
    frame
        .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
        .unwrap();
    assert_eq!((error, frame.caller_dimensions()), (101, Some(vec![1])));
}

#[test]
fn default_real_evaluator_dimension_projection_borrows_the_existing_input() {
    use martensite_tkgeombase::adv_app2var::{AdvApp2VarRealEvaluation, AdvApp2VarRealEvaluator};
    let evaluator = |_: &mut AdvApp2VarRealEvaluation<'_, '_>, _: &mut [f64], _: &mut i32| {};
    let dimensions = [2];
    let actual = evaluator.caller_subspace_dimensions(&dimensions);
    assert!(matches!(actual, std::borrow::Cow::Borrowed(_)));
    assert_eq!(actual.as_ref(), dimensions);
}
