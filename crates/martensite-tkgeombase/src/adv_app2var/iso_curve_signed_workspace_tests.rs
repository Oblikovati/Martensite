//! Source status/degree incidence retains signed region totals before early callback failure.
use super::*;
fn signed_workspace_fixture(
    iso: i32,
) -> (
    AdvApp2VarIsoOperationFrame,
    crate::adv_app2var::AdvApp2VarCallerCurveSlots,
    IsoApproxRequest<'static>,
) {
    let integers = signed_workspace_integers(iso);
    let (frame, curve) = fixture::configured_frame(integers, 38);
    let mut request = fixture::request(iso);
    (request.iordre, request.ndgjac, request.ncflim) = (1, 4, 5);
    (frame, curve, request)
}
fn signed_workspace_integers(iso: i32) -> Vec<i32> {
    let mut integers = fixture::integer_owner(iso);
    integers[36] = 1;
    integers[38] = 4;
    integers[40] = 5;
    integers
}
fn check_signed_workspace_owners(
    frame: &mut AdvApp2VarIsoOperationFrame,
    iso: i32,
    mode: ResourceMode,
) {
    let allocated = mode != ResourceMode::Capacity;
    let mut integers = signed_workspace_integers(iso);
    integers[38] = if mode == ResourceMode::None { 101 } else { 13 };
    if allocated {
        integers[41] = 0;
    }
    assert_eq!(&frame.integer_values()[32..70], &integers[32..70]);
    let mut expected = fixture::real_owner();
    expected[64..69].fill(0.);
    if allocated {
        expected[448..450].copy_from_slice(&[0., 1.]);
    }
    let mut owner = frame.caller_scalar_curve_slots(0, 1100).unwrap();
    owner.with_curve_values(|values| {
        for (index, (actual, expected)) in values.iter().zip(&expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "iso={iso},mode={mode:?},slot={index}"
            );
        }
    });
}
fn run_signed_workspace_case(iso: i32, mode: ResourceMode) {
    let (mut frame, curve, mut request) = signed_workspace_fixture(iso);
    let mut buffers = fixture::ResourceBuffers::new();
    let mut output = buffers.output(curve);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(call.integer(AdvApp2VarIntegerField::ParameterCount), 10);
        assert_eq!(call.buffers().allocation_capacity().unwrap(), 44);
        assert_eq!(call.buffers().parameter(0).to_bits(), 0_f64.to_bits());
        assert_eq!(call.buffers().parameter(9).to_bits(), 1_f64.to_bits());
        for (index, root) in fixture::ROOTS.iter().enumerate() {
            assert_eq!(
                call.buffers().parameter(index + 1).to_bits(),
                (0.5 * root + 0.5).to_bits()
            );
        }
        for index in 0..10 {
            call.buffers().set_result(index, 1.);
        }
        call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status, 1)
            .unwrap();
    };
    let mut workspace = RegistryWorkspace::new(mode);
    frame
        .run_owned_integer_outputs_with_workspace(
            &mut request,
            &evaluator,
            &mut output,
            &mut workspace,
        )
        .unwrap();
    assert_eq!(
        *calls.lock().unwrap(),
        i32::from(mode != ResourceMode::Capacity)
    );
    assert_eq!(
        workspace.events,
        if mode == ResourceMode::Capacity {
            vec![(44, 1)]
        } else {
            vec![
                (44, 0),
                (44, if mode == ResourceMode::Retired { 3 } else { 0 }),
            ]
        }
    );
    assert_eq!(
        request.ndgjac,
        if mode == ResourceMode::None { 101 } else { 13 }
    );
    assert_eq!(
        output.nbcrbe,
        if mode == ResourceMode::Capacity {
            -7
        } else {
            0
        }
    );
    assert_eq!(output.ncoeff, &[-7]);
    check_signed_workspace_owners(&mut frame, iso, mode);
}
#[test]
fn signed_private_region_total_matches_all_six_defined_retained_early_error_operations() {
    for iso in [1, 2] {
        for mode in [
            ResourceMode::None,
            ResourceMode::Capacity,
            ResourceMode::Retired,
        ] {
            run_signed_workspace_case(iso, mode);
        }
    }
}
