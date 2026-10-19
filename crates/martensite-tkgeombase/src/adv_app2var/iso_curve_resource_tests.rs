//! Source resource failures bracket the same owned callback and numerical pipeline.
use super::*;
use crate::adv_app2var::AdvApp2VarCombinedBufferEvaluation as Call;
use crate::adv_app2var::approx_f2var::{
    IsoApproxCallerOutput, curve_output::CurveCoefficientStorage,
};
#[path = "iso_curve_resource_fixture_tests.rs"]
mod fixture;
#[path = "iso_curve_resource_workspace_tests.rs"]
mod workspace;
use workspace::{RegistryWorkspace, ResourceMode};
#[derive(Clone, Copy, Debug)]
struct ResourceCase {
    iso: i32,
    mode: ResourceMode,
    code: i32,
    incident: bool,
}
impl ResourceCase {
    fn allocated(self) -> bool {
        self.mode != ResourceMode::Capacity
    }
    fn completed(self) -> bool {
        self.allocated() && self.code == 0
    }
    fn status(self) -> i32 {
        if self.mode != ResourceMode::None {
            return 13;
        }
        if self.code != 0 {
            return 100 + self.code;
        }
        0
    }
}
fn evaluate_resource_constant(call: &mut Call<'_, '_, '_>, code: i32) {
    assert_eq!(call.integer(AdvApp2VarIntegerField::ParameterCount), 8);
    for (index, root) in fixture::ROOTS.iter().enumerate() {
        assert_eq!(
            call.buffers().parameter(index).to_bits(),
            (0.5 * root + 0.5).to_bits()
        );
        call.buffers().set_result(index, 1.);
    }
    call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status, code)
        .unwrap();
}
fn expected_integer_owner(case: ResourceCase) -> Vec<i32> {
    let mut integers = fixture::integer_owner(case.iso);
    integers[41] = if case.completed() {
        1
    } else if case.allocated() {
        0
    } else {
        -7
    };
    integers[if case.incident { 41 } else { 42 }] = case.status();
    if case.completed() {
        integers[64] = 1;
    }
    integers
}
fn check_resource_owner(owner: &mut AdvApp2VarIsoOperationFrame, case: ResourceCase) {
    let expected = fixture::expected_owner(case.allocated(), case.completed());
    let mut full_owner = owner.caller_scalar_curve_slots(0, 1100).unwrap();
    full_owner.with_curve_values(|values| {
        for (index, (actual, expected)) in values.iter().zip(&expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "case={case:?},slot={index}"
            );
        }
    });
}
fn check_resource_events(resource: &RegistryWorkspace, case: ResourceCase) {
    let events = if case.allocated() {
        vec![
            (38, 0),
            (
                38,
                if case.mode == ResourceMode::Retired {
                    3
                } else {
                    0
                },
            ),
        ]
    } else {
        vec![(38, 1)]
    };
    assert_eq!(resource.events, events);
}
fn run_resource_case(case: ResourceCase) {
    let (mut owner, curve) = fixture::frame(case.iso, case.incident);
    let (mut request, mut buffers) = (fixture::request(case.iso), fixture::ResourceBuffers::new());
    let mut output = buffers.output(curve);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        evaluate_resource_constant(call, case.code);
    };
    let mut resource = RegistryWorkspace::new(case.mode);
    assert_eq!(
        owner
            .run_owned_integer_outputs_with_workspace(
                &mut request,
                &evaluator,
                &mut output,
                &mut resource
            )
            .unwrap(),
        0
    );
    assert_eq!(*calls.lock().unwrap(), i32::from(case.allocated()));
    let integers = expected_integer_owner(case);
    assert_eq!(&owner.integer_values()[32..70], &integers[32..70]);
    assert_eq!(
        (output.nbcrbe, &*output.ncoeff),
        (integers[41], &integers[64..65])
    );
    check_resource_owner(&mut owner, case);
    check_resource_events(&resource, case);
}
#[test]
fn outer_curve_resource_boundaries_match_all_eighteen_defined_retained_operations() {
    for iso in [1, 2] {
        for mode in [
            ResourceMode::None,
            ResourceMode::Capacity,
            ResourceMode::Retired,
        ] {
            for (code, incident) in [(0, false), (5, false), (5, true)] {
                run_resource_case(ResourceCase {
                    iso,
                    mode,
                    code,
                    incident,
                });
            }
        }
    }
}
#[test]
fn outer_curve_workspace_unwind_restores_frame_and_preserves_callback_writes() {
    let (mut owner, curve) = fixture::frame(2, false);
    let (mut request, mut buffers) = (fixture::request(2), fixture::ResourceBuffers::new());
    let mut output = buffers.output(curve);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status, 7)
            .unwrap();
        call.set_caller_output(AdvApp2VarIsoIntegerOutput::CompletedPieces, 3)
            .unwrap();
        panic!("named resource callback unwind");
    };
    let mut resource = RegistryWorkspace::new(ResourceMode::None);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        owner.run_owned_integer_outputs_with_workspace(
            &mut request,
            &evaluator,
            &mut output,
            &mut resource,
        )
    }));
    assert!(result.is_err());
    assert_eq!(
        owner
            .caller_output(AdvApp2VarIsoIntegerOutput::Status)
            .unwrap(),
        7
    );
    assert_eq!(
        owner
            .caller_output(AdvApp2VarIsoIntegerOutput::CompletedPieces)
            .unwrap(),
        3
    );
    assert_eq!(resource.events, [(38, 0)]);
    assert_eq!(
        owner.caller_curve_value(0).unwrap().to_bits(),
        0_f64.to_bits()
    );
}

#[path = "iso_curve_signed_workspace_tests.rs"]
mod signed_workspace;

#[path = "iso_curve_truncation_overlap_tests.rs"]
mod truncation_overlap;

#[path = "iso_curve_endpoint_span_tests.rs"]
mod endpoint_span;
