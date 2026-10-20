//! Caller-only subspace count captured through actual operation-local identity.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCombinedEvaluation as Call, AdvApp2VarIntegerBindings as Integers,
    AdvApp2VarIntegerField as Integer, AdvApp2VarIsoOperationFrame as Frame,
    AdvApp2VarRealBindings as Reals, AdvApp2VarRealField as Real,
};

struct SubspaceStorage {
    curve: CurveStorage,
    maximum: [f64; 2],
    mean: [f64; 2],
}
impl SubspaceStorage {
    fn new() -> Self {
        Self {
            curve: CurveStorage::new(),
            maximum: [7.; 2],
            mean: [7.; 2],
        }
    }
    fn output(&mut self) -> IsoApproxOutput<'_> {
        let mut output = self.curve.output();
        output.errmax = &mut self.maximum;
        output.errmoy = &mut self.mean;
        output
    }
}
fn subspace_request() -> IsoApproxRequest<'static> {
    let mut request = curve_request(2);
    request.nbsesp = 2;
    request.ndimse = &[1, 1];
    request.epsapr = &[1e-6, 1e-6];
    request
}
fn count_frame(slot: Option<usize>) -> Frame {
    let integers = Integers {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 4,
    };
    let reals = Reals {
        domain: [0, 1, 2, 3],
        constant_parameter: 4,
    };
    let frame = Frame::new(vec![0; 6], integers, vec![0.; 5], reals).unwrap();
    match slot {
        Some(slot) => frame.with_subspace_count_slot(slot).unwrap(),
        None => frame,
    }
}
fn captured_count_failure(call: &mut Call<'_, '_>, first_count: i32, error: &mut i32) {
    assert_eq!(call.subspace_count(), 2);
    call.set_integer(Integer::Dimension, 1);
    assert_eq!(call.integer(Integer::FavoriteIso), 2);
    assert_eq!(call.subspace_count(), first_count);
    call.set_integer(Integer::FavoriteIso, 1);
    assert_eq!(
        (call.integer(Integer::Dimension), call.subspace_count()),
        (1, 1)
    );
    call.set_integer(Integer::ParameterCount, 2);
    call.set_integer(Integer::UOrder, 7);
    assert_eq!(call.integer(Integer::VOrder), 0);
    call.set_integer(Integer::VOrder, 9);
    assert_eq!(call.integer(Integer::UOrder), 7);
    call.set_real(Real::ConstantParameter, 0.75);
    call.u_bounds()[0] = -0.5;
    call.v_bounds()[1] = 0.75;
    *error = 1;
}
#[test]
fn caller_count_shared_with_dimension_or_iso_matches_both_retained_failures() {
    for (slot, first_count) in [(0, 1), (1, 2)] {
        let mut frame = count_frame(Some(slot));
        let mut request = subspace_request();
        let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
            captured_count_failure(call, first_count, error)
        };
        let mut storage = SubspaceStorage::new();
        let mut output = storage.output();
        let mut error = -1;
        frame
            .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
            .unwrap();
        assert_eq!(
            (
                request.ndimen,
                request.isofav,
                request.nbsesp,
                request.tconst,
                error
            ),
            (1, 1, 1, 0.75, 101)
        );
        assert_eq!(request.uvfonc, [0., 1., 0., 1.]);
        assert_eq!(output.nbcrbe, 0);
        assert_eq!(&output.courbe[..6], &[0.; 6]);
        assert_eq!(&output.courbe[6..], &[7.; 26]);
        assert_eq!(output.errmax, &[7.; 2]);
        assert_eq!(output.errmoy, &[7.; 2]);
        assert_eq!(&frame.integer_values()[..5], &[1, 1, 2, 7, 9]);
    }
}
#[test]
fn writes_through_caller_count_are_immediately_visible_through_each_callback_field() {
    for (slot, changed) in [(0, (1, 2)), (1, (2, 1))] {
        let mut frame = count_frame(Some(slot));
        let mut request = subspace_request();
        let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
            call.set_subspace_count(1);
            assert_eq!(
                (
                    call.integer(Integer::Dimension),
                    call.integer(Integer::FavoriteIso)
                ),
                changed
            );
            call.set_subspace_count(2);
            assert_eq!(
                (
                    call.integer(Integer::Dimension),
                    call.integer(Integer::FavoriteIso)
                ),
                (2, 2)
            );
            *error = 1;
        };
        let mut error = -1;
        frame
            .approximate_iso(
                &mut request,
                &evaluator,
                &mut SubspaceStorage::new().output(),
                &mut error,
            )
            .unwrap();
        assert_eq!(
            (request.ndimen, request.isofav, request.nbsesp, error),
            (2, 2, 2, 101)
        );
    }
}
#[test]
fn standalone_caller_count_stays_distinct_unbound_or_in_its_own_slot() {
    for slot in [None, Some(5)] {
        let mut frame = count_frame(slot);
        let mut request = subspace_request();
        let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
            assert_eq!(call.subspace_count(), 2);
            call.set_subspace_count(1);
            assert_eq!(
                (
                    call.integer(Integer::Dimension),
                    call.integer(Integer::FavoriteIso)
                ),
                (2, 2)
            );
            *error = 1;
        };
        let mut error = -1;
        frame
            .approximate_iso(
                &mut request,
                &evaluator,
                &mut SubspaceStorage::new().output(),
                &mut error,
            )
            .unwrap();
        assert_eq!(
            (request.ndimen, request.isofav, request.nbsesp, error),
            (2, 2, 1, 101)
        );
        if slot.is_some() {
            assert_eq!(frame.integer_values()[5], 1);
        }
    }
}
#[test]
fn count_shrink_survives_the_next_endpoint_callback_before_failure() {
    let mut frame = count_frame(Some(0));
    let mut request = subspace_request();
    request.iordre = 1;
    request.ndgjac = 3;
    request.ncflim = 4;
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut count = calls.lock().unwrap();
        *count += 1;
        if *count == 1 {
            assert_eq!(call.subspace_count(), 2);
            values.fill(1.);
            call.set_integer(Integer::Dimension, 1);
            *error = 0;
            return;
        }
        assert_eq!(*count, 2);
        assert_eq!(call.subspace_count(), 1);
        assert_eq!(
            (
                call.integer(Integer::Dimension),
                call.integer(Integer::ParameterCount)
            ),
            (1, 1)
        );
        assert_eq!(
            (call.integer(Integer::UOrder), call.integer(Integer::VOrder)),
            (1, 0)
        );
        *error = 1;
    };
    let mut storage = SubspaceStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 2);
    assert_eq!((request.ndimen, request.nbsesp, error), (1, 1, 101));
    assert_eq!(&output.courbe[..8], &[0.; 8]);
    assert_eq!(&output.courbe[8..], &[7.; 24]);
}
#[test]
fn successful_count_shrink_reaches_the_projection_and_error_steps_with_the_live_count() {
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
    let mut frame = count_frame(Some(0));
    let mut request = subspace_request();
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        values.fill(1.);
        call.set_integer(Integer::Dimension, 1);
        assert_eq!(call.subspace_count(), 1);
        *error = 0;
    };
    let mut storage = SubspaceStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(
        (
            request.ndimen,
            request.nbsesp,
            output.nbcrbe,
            output.ncoeff[0],
            error
        ),
        (1, 1, 1, 1, 0)
    );
    assert!((output.courbe[0] - 1.).abs() <= 8. * f64::EPSILON);
    assert_eq!(&output.courbe[1..6], &[0.; 5]);
    assert_eq!(&output.courbe[6..], &[7.; 26]);
    assert!(output.errmax[0] <= 8. * f64::EPSILON);
    assert_eq!(output.errmax[1], 7.);
    assert_eq!(output.errmoy[1], 7.);
}
#[test]
fn invalid_count_slot_and_conflicting_initial_incidence_are_typed_errors() {
    let frame = count_frame(None);
    let failure = frame.with_subspace_count_slot(6).err().unwrap();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert!(
        failure
            .message()
            .contains("SubspaceCount slot=6; expected slot in 0..6")
    );
    let mut frame = count_frame(Some(0));
    let mut storage = SubspaceStorage::new();
    let mut error = -1;
    let evaluator = |_: &mut Call<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("inconsistent field identities must not reach callback")
    };
    let failure = frame
        .approximate_iso(
            &mut curve_request(2),
            &evaluator,
            &mut storage.output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("incident SubspaceCount value=1; expected shared input value=2")
    );
    assert_eq!(error, 101);
    assert_eq!(&storage.curve.curve[..6], &[0.; 6]);
}

struct CountPieceStorage {
    curve: CurveStorage,
    counts: [i32; 3],
    cuts: [f64; 4],
    maximum: [f64; 6],
    mean: [f64; 6],
}
impl CountPieceStorage {
    fn new() -> Self {
        Self {
            curve: CurveStorage::new(),
            counts: [-7; 3],
            cuts: [7.; 4],
            maximum: [7.; 6],
            mean: [7.; 6],
        }
    }
    fn output(&mut self) -> IsoApproxOutput<'_> {
        let mut output = self.curve.output();
        output.ncoeff = &mut self.counts;
        output.tabdec = &mut self.cuts;
        output.errmax = &mut self.maximum;
        output.errmoy = &mut self.mean;
        output
    }
}
#[test]
fn count_shrink_retains_initial_error_table_strides_across_three_pieces() {
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
    let mut frame = count_frame(Some(0));
    let mut request = subspace_request();
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.ndgjac = 3;
    request.nbcrmx = 3;
    request.epsapr = &[1e-3, 1e-3];
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let dimension = call.integer(Integer::Dimension) as usize;
        let count = call.integer(Integer::ParameterCount) as usize;
        for (sample, parameter) in values[..dimension * count]
            .chunks_mut(dimension)
            .zip(&call.parameters()[..count])
        {
            sample.fill(martensite_cmath::powi(*parameter, 3));
        }
        call.set_integer(Integer::Dimension, 1);
        *error = 0;
    };
    let mut storage = CountPieceStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(
        (request.ndimen, request.nbsesp, output.nbcrbe, error),
        (1, 1, 3, -1)
    );
    assert_eq!(output.ncoeff, &[3, 3, 3]);
    assert_eq!(output.tabdec, &[0., 0.25, 0.5, 1.]);
    for (piece, (middle, half_span)) in [(0.125_f64, 0.125_f64), (0.375, 0.125), (0.75, 0.25)]
        .into_iter()
        .enumerate()
    {
        let expected = [
            martensite_cmath::powi(middle, 3),
            3. * martensite_cmath::powi(middle, 2) * half_span
                + 0.6 * martensite_cmath::powi(half_span, 3),
            3. * middle * martensite_cmath::powi(half_span, 2),
        ];
        for (actual, expected) in output.courbe[piece * 6..piece * 6 + 3].iter().zip(expected) {
            assert!((actual - expected).abs() <= 8. * f64::EPSILON);
        }
        assert_eq!(&output.courbe[piece * 6 + 3..piece * 6 + 6], &[0.; 3]);
        let maximum = martensite_cmath::powi(2. * half_span, 3) / 20.;
        assert!((output.errmax[piece * 2] - maximum).abs() <= 8. * f64::EPSILON);
        assert!((output.errmoy[piece * 2] - maximum / 7_f64.sqrt()).abs() <= 8. * f64::EPSILON);
        assert_eq!(output.errmax[piece * 2 + 1], 7.);
        assert_eq!(output.errmoy[piece * 2 + 1], 7.);
    }
    assert_eq!(&output.courbe[18..], &[7.; 14]);
}

#[test]
fn caller_count_cannot_acquire_identity_with_private_sampling_locals() {
    for slot in [2, 3, 4] {
        let failure = count_frame(None)
            .with_subspace_count_slot(slot)
            .err()
            .unwrap();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(
            failure
                .message()
                .contains(&format!("SubspaceCount slot={slot} names a callback local"))
        );
        assert!(
            failure.message().contains(
                "expected a forwarded caller argument slot or a distinct owned caller slot"
            )
        );
    }
}
