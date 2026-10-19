//! Callback status writes and source error translation use the same initialized slot.
use super::*;
fn callback_status_case(iso: i32, fail: bool) {
    let mut frame = status_frame(iso, 0);
    let mut request = status_request(iso);
    let mut storage = StatusStorage::<512>::new();
    for index in 0..512 {
        storage.sums[index] = (index % 4 + 1) as f64;
        storage.both_difference[index] = (index % 4 + 1) as f64;
        storage.u_difference[index] = (index + 1) as f64;
        storage.v_difference[index] = (index + 1) as f64;
    }
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64]| {
        assert_eq!(call.status().unwrap(), 0);
        assert_eq!(call.argument(Argument::Dimension), 0);
        assert_eq!(call.argument(Argument::ParameterCount), 3);
        assert_eq!(call.argument(Argument::FavoriteIso), iso);
        call.set_status(1).unwrap();
        assert_eq!(call.argument(Argument::Dimension), 1);
        assert_eq!(call.integer(Field::Dimension), 1);
        call.set_argument(Argument::Dimension, 2);
        assert_eq!(call.status().unwrap(), 2);
        call.set_status(i32::from(fail)).unwrap();
        *calls.lock().unwrap() += 1;
    };
    assert_eq!(
        storage.run(&mut frame, &mut request, &evaluator).unwrap(),
        0
    );
    let code = if fail && iso == 2 { 201 } else { 0 };
    assert_eq!(frame.status().unwrap(), code);
    assert_eq!(request.ndimen, code);
    assert_eq!(
        (request.nbpntu, request.nbpntv, request.isofav),
        (3, 3, iso)
    );
    assert_eq!(*calls.lock().unwrap(), if fail { 1 } else { 3 });
    for index in 0..512 {
        let expected = if fail && iso == 1 && index < 404 {
            [1., 3., 2., 4.][index % 4]
        } else {
            (index % 4 + 1) as f64
        };
        assert_eq!(storage.sums[index], expected, "sum index={index}");
        assert_eq!(
            storage.both_difference[index], expected,
            "both-difference index={index}"
        );
        assert_eq!(storage.u_difference[index], (index + 1) as f64);
        assert_eq!(storage.v_difference[index], (index + 1) as f64);
    }
    assert_eq!(storage.samples, [7.; 64]);
    assert_eq!(storage.parameters[3..], [7.; 61]);
}
#[test]
fn first_iso_owned_status_callback_writes_are_immediately_visible_through_dimension() {
    callback_status_case(1, false);
}
#[test]
fn second_iso_owned_status_callback_writes_are_immediately_visible_through_dimension() {
    callback_status_case(2, false);
}
#[test]
fn first_iso_error_clear_retains_captured101_dimension_transposition_loop() {
    callback_status_case(1, true);
}
#[test]
fn second_iso_error_translation_updates_shared_dimension_and_status_to201() {
    callback_status_case(2, true);
}

#[test]
fn owned_square_status_unwind_retains_the_actual_shared_slot() {
    let mut frame = status_frame(2, 0);
    let mut request = status_request(2);
    let mut storage = StatusStorage::<64>::new();
    let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64]| {
        call.set_status(1).unwrap();
        assert_eq!(call.argument(Argument::Dimension), 1);
        panic!("owned square status unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| storage.run(
            &mut frame,
            &mut request,
            &evaluator
        )))
        .is_err()
    );
    assert_eq!(frame.status().unwrap(), 1);
    assert_eq!(frame.integer(Field::Dimension), 1);
    assert_eq!(storage.sums, [0.; 64]);
}
