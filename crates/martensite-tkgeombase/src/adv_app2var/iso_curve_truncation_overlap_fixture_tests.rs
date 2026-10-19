//! Native422-cell payload, with a live dimension sharing its subspace slot.
use super::*;
use crate::adv_app2var::{AdvApp2VarCallerCurveSlots, AdvApp2VarCallerRealArray as Array};
pub(super) fn request(iso: i32, warning: bool) -> IsoApproxRequest<'static> {
    let mut request = super::super::fixture::request(iso);
    (request.ndimen, request.ndgjac, request.ncflim) = (9, 22, 1);
    (request.ndimse, request.epsapr) = (&[9], &[1.]);
    if warning {
        (
            request.ndimen,
            request.ndgjac,
            request.ndimse,
            request.epsapr,
        ) = (20, 40, &[20], &[-1.]);
    }
    request
}

fn integer_owner(iso: i32, warning: bool) -> Vec<i32> {
    let mut values = vec![0; 128];
    values[32..43].copy_from_slice(&[9, 1, iso, 8, -1, 0, 22, 1, 1, -7, -7]);
    if warning {
        values[32] = 20;
        values[38] = 40;
    }
    values[64] = -7;
    values
}
fn real_owner(warning: bool) -> Vec<f64> {
    let mut values = vec![0.; 65_536];
    values[16..20].copy_from_slice(&[0., 1., 0., 1.]);
    values[32..40].copy_from_slice(&super::super::fixture::ROOTS);
    values[96] = if warning { -1. } else { 1. };
    values[128] = 0.5;
    values
}
fn bind_integer_roles(frame: AdvApp2VarIsoOperationFrame) -> AdvApp2VarIsoOperationFrame {
    frame
        .with_subspace_count_slot(33)
        .unwrap()
        .with_caller_order_slots([36, 37])
        .unwrap()
        .with_caller_sampling_slots([35, 38])
        .unwrap()
        .with_caller_limit_slots([39, 40])
        .unwrap()
        .with_caller_dimension_slots(vec![32])
        .unwrap()
        .with_caller_output_slots([42, 41])
        .unwrap()
        .with_coefficient_count_origin(64)
        .unwrap()
}
fn bind_real_roles(frame: AdvApp2VarIsoOperationFrame) -> AdvApp2VarIsoOperationFrame {
    frame
        .with_caller_array_slots(Array::IsoRoots, (32..40).collect())
        .unwrap()
        .with_caller_array_slots(Array::SubspaceTolerances, vec![96])
        .unwrap()
        .with_folded_output_origins([10_240, 18_432])
        .unwrap()
        .with_endpoint_output_origins([26_624, 34_816])
        .unwrap()
        .with_cut_parameter_origin(43_008)
        .unwrap()
        .with_error_output_origins([51_200, 59_392])
        .unwrap()
}
pub(super) fn frame(
    iso: i32,
    warning: bool,
) -> (AdvApp2VarIsoOperationFrame, AdvApp2VarCallerCurveSlots) {
    let owner = AdvApp2VarIsoOperationFrame::new(
        integer_owner(iso, warning),
        overlap_integer_bindings(),
        real_owner(warning),
        AdvApp2VarRealBindings {
            domain: [16, 17, 18, 19],
            constant_parameter: 128,
        },
    )
    .unwrap();
    let mut owner = bind_real_roles(bind_integer_roles(owner));
    let curve = owner.caller_curve_slots(2048, 8192).unwrap();
    (owner, curve)
}
pub(super) fn output(
    curve: AdvApp2VarCallerCurveSlots,
) -> IsoApproxCallerOutput<'static, AdvApp2VarCallerCurveSlots> {
    IsoApproxCallerOutput {
        ncoeff: &mut [],
        courbe: curve,
        nbcrbe: -7,
        somtab: &mut [],
        diftab: &mut [],
        contr1: &mut [],
        contr2: &mut [],
        tabdec: &mut [],
        errmax: &mut [],
        errmoy: &mut [],
    }
}

pub(super) fn assert_real_owner(frame: &mut AdvApp2VarIsoOperationFrame, warning: bool) {
    let mut expected = real_owner(warning);
    expected[43_009] = 1.;
    let mut values = frame.caller_scalar_curve_slots(0, 65_536).unwrap();
    values.with_curve_values(|actual| {
        for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "caller real slot={index}"
            );
        }
    });
}

fn overlap_integer_bindings() -> AdvApp2VarIntegerBindings {
    AdvApp2VarIntegerBindings {
        dimension: 32,
        favorite_iso: 34,
        parameter_count: 0,
        u_order: 1,
        v_order: 2,
    }
}
