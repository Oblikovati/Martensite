//! Original mutable caller arrays are replayed through both combined interfaces.
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerRealArray as Array, AdvApp2VarIntegerBindings as Bindings,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings,
};
pub(super) enum ArrayMode {
    Raw,
    Slice,
    Indexed,
}
pub(super) fn array_frame(array: Array) -> Frame {
    Frame::new(
        vec![0; 7],
        Bindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 4,
        },
        vec![0.; 5],
        AdvApp2VarRealBindings {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap()
    .with_caller_sampling_slots([5, 6])
    .unwrap()
    .with_owned_caller_array(array)
}
#[test]
fn slice_first_iso_caller_roots_drive_quadratic_projection() {
    super::caller_root_cases::caller_roots_mode(1, ArrayMode::Slice);
}
#[test]
fn slice_second_iso_caller_roots_drive_quadratic_projection() {
    super::caller_root_cases::caller_roots_mode(2, ArrayMode::Slice);
}
#[test]
fn indexed_first_iso_caller_roots_drive_quadratic_projection() {
    super::caller_root_cases::caller_roots_mode(1, ArrayMode::Indexed);
}
#[test]
fn indexed_second_iso_caller_roots_drive_quadratic_projection() {
    super::caller_root_cases::caller_roots_mode(2, ArrayMode::Indexed);
}
#[test]
fn slice_first_iso_tighter_tolerance_reports_unmet_precision() {
    super::caller_tolerance_cases::caller_tolerance_mode(1, ArrayMode::Slice);
}
#[test]
fn slice_second_iso_tighter_tolerance_reports_unmet_precision() {
    super::caller_tolerance_cases::caller_tolerance_mode(2, ArrayMode::Slice);
}
#[test]
fn indexed_first_iso_tighter_tolerance_reports_unmet_precision() {
    super::caller_tolerance_cases::caller_tolerance_mode(1, ArrayMode::Indexed);
}
#[test]
fn indexed_second_iso_tighter_tolerance_reports_unmet_precision() {
    super::caller_tolerance_cases::caller_tolerance_mode(2, ArrayMode::Indexed);
}

#[test]
fn real_callback_keeps_owned_input_writes_after_failure_and_reseeds_next_operation() {
    use super::*;
    use martensite_tkgeombase::adv_app2var::{AdvApp2VarIsoRealFrame, AdvApp2VarRealEvaluation};
    for iso in [1, 2] {
        let mut request = curve_request(iso);
        let original_roots = request.rootlg.to_vec();
        let original_limits = request.epsapr.to_vec();
        let mut frame = AdvApp2VarIsoRealFrame::new(
            vec![0.; 5],
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
        .with_owned_caller_array(Array::IsoRoots)
        .with_owned_caller_array(Array::SubspaceTolerances);
        let evaluator =
            |call: &mut AdvApp2VarRealEvaluation<'_, '_>, _: &mut [f64], error: &mut i32| {
                assert_eq!(call.caller_array(Array::IsoRoots).unwrap(), original_roots);
                assert_eq!(
                    call.caller_array(Array::SubspaceTolerances).unwrap(),
                    original_limits
                );
                call.caller_array_mut(Array::IsoRoots).unwrap()[0] = 0.25;
                call.caller_array_mut(Array::SubspaceTolerances).unwrap()[0] = 1e-12;
                *error = 1;
            };
        for _ in 0..2 {
            let mut storage = CurveStorage::new();
            let mut error = -1;
            frame
                .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
                .unwrap();
            assert_eq!(error, 101);
            assert_eq!(frame.caller_array(Array::IsoRoots).unwrap()[0], 0.25);
            assert_eq!(
                frame.caller_array(Array::SubspaceTolerances).unwrap()[0],
                1e-12
            );
            assert_eq!(request.rootlg, original_roots);
            assert_eq!(request.epsapr, original_limits);
        }
    }
}

#[test]
fn first_iso_shared_tolerance_constant_slot_is_immediately_bidirectional() {
    super::caller_tolerance_cases::caller_tolerance_configuration(1, ArrayMode::Indexed, true);
}
#[test]
fn second_iso_shared_tolerance_constant_slot_is_immediately_bidirectional() {
    super::caller_tolerance_cases::caller_tolerance_configuration(2, ArrayMode::Indexed, true);
}
#[test]
fn first_iso_slice_shared_tolerance_constant_slot_is_immediately_bidirectional() {
    super::caller_tolerance_cases::caller_tolerance_configuration(1, ArrayMode::Slice, true);
}
#[test]
fn second_iso_slice_shared_tolerance_constant_slot_is_immediately_bidirectional() {
    super::caller_tolerance_cases::caller_tolerance_configuration(2, ArrayMode::Slice, true);
}

#[test]
fn real_callback_indexed_tolerance_alias_retains_immediate_writes_on_failure() {
    use super::*;
    use martensite_tkgeombase::adv_app2var::{
        AdvApp2VarIsoRealFrame, AdvApp2VarRealEvaluation, AdvApp2VarRealField::ConstantParameter,
    };
    for iso in [1, 2] {
        let mut request = curve_request(iso);
        request.epsapr = &[0.5];
        let mut frame = AdvApp2VarIsoRealFrame::new(
            vec![0.; 5],
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
        .with_caller_array_slots(Array::SubspaceTolerances, vec![4])
        .unwrap();
        let evaluator =
            |call: &mut AdvApp2VarRealEvaluation<'_, '_>, _: &mut [f64], error: &mut i32| {
                assert_eq!(
                    call.caller_array_value(Array::SubspaceTolerances, 0)
                        .unwrap(),
                    0.5
                );
                call.set_caller_array_value(Array::SubspaceTolerances, 0, 1e-12)
                    .unwrap();
                assert_eq!(call.real(ConstantParameter), 1e-12);
                call.set_real(ConstantParameter, 0.5);
                assert_eq!(
                    call.caller_array_value(Array::SubspaceTolerances, 0)
                        .unwrap(),
                    0.5
                );
                call.set_caller_array_value(Array::SubspaceTolerances, 0, 1e-12)
                    .unwrap();
                *error = 1;
            };
        let mut storage = CurveStorage::new();
        let mut error = -1;
        frame
            .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
            .unwrap();
        assert_eq!(error, 101);
        assert_eq!(request.tconst, 1e-12);
        assert_eq!(
            frame
                .caller_array_value(Array::SubspaceTolerances, 0)
                .unwrap(),
            1e-12
        );
        assert_eq!(request.epsapr, [0.5]);
    }
}
