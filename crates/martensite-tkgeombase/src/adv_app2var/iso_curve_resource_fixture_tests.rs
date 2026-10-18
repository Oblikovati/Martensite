//! Initialized resource fixtures retain one actual caller owner for every real role.
use super::*;
use crate::adv_app2var::{AdvApp2VarCallerCurveSlots, AdvApp2VarCallerRealArray as Array};
pub(super) const ROOTS: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
pub(super) fn integer_owner(iso: i32) -> Vec<i32> {
    let mut values = vec![-7; 128];
    values[32..43].copy_from_slice(&[1, 1, iso, 8, -1, 0, 2, 1, 3, -7, -7]);
    values[48..50].copy_from_slice(&[1, 1]);
    values
}
pub(super) fn real_owner() -> Vec<f64> {
    let mut values = vec![7.; 1100];
    values[16..20].copy_from_slice(&[0., 1., 0., 1.]);
    values[32..40].copy_from_slice(&ROOTS);
    values[608..612].fill(1e-6);
    values[1024] = 0.5;
    values
}
pub(super) fn frame(
    iso: i32,
    incident: bool,
) -> (AdvApp2VarIsoOperationFrame, AdvApp2VarCallerCurveSlots) {
    configured_frame(integer_owner(iso), if incident { 41 } else { 42 })
}
pub(super) fn configured_frame(
    integers: Vec<i32>,
    status_slot: usize,
) -> (AdvApp2VarIsoOperationFrame, AdvApp2VarCallerCurveSlots) {
    let mut owner = AdvApp2VarIsoOperationFrame::new(
        integers,
        AdvApp2VarIntegerBindings {
            dimension: 32,
            favorite_iso: 34,
            parameter_count: 0,
            u_order: 1,
            v_order: 2,
        },
        real_owner(),
        AdvApp2VarRealBindings {
            domain: [16, 17, 18, 19],
            constant_parameter: 1024,
        },
    )
    .unwrap()
    .with_subspace_count_slot(33)
    .unwrap()
    .with_caller_order_slots([36, 37])
    .unwrap()
    .with_caller_sampling_slots([35, 38])
    .unwrap()
    .with_caller_limit_slots([39, 40])
    .unwrap()
    .with_caller_dimension_slots(vec![48, 49])
    .unwrap()
    .with_caller_output_slots([status_slot, 41])
    .unwrap()
    .with_coefficient_count_origin(64)
    .unwrap()
    .with_caller_array_slots(Array::IsoRoots, (32..40).collect())
    .unwrap()
    .with_caller_array_slots(Array::SubspaceTolerances, (608..612).collect())
    .unwrap()
    .with_cut_parameter_origin(448)
    .unwrap()
    .with_folded_output_origins([192, 256])
    .unwrap()
    .with_endpoint_output_origins([320, 384])
    .unwrap()
    .with_error_output_origins([512, 576])
    .unwrap();
    let curve = owner.caller_curve_slots(64, 128).unwrap();
    (owner, curve)
}
pub(super) fn request(iso: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1, 1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6; 4],
    }
}
pub(super) fn expected_owner(allocated: bool, completed: bool) -> Vec<f64> {
    let mut expected = real_owner();
    expected[64..67].fill(0.);
    if allocated {
        expected[448..450].copy_from_slice(&[0., 1.]);
    }
    if !completed {
        return expected;
    }
    let mut constant = 0.;
    for weight in [
        0.2564561626532326,
        0.221824096603541,
        0.15724713746926033,
        0.07157938446051357,
    ] {
        constant += 2. * weight;
    }
    let mut quadratic: f64 = 0.;
    for weight in [
        -0.25778313522636853,
        -0.04252004995346777,
        0.1589358116831741,
        0.1413673734966622,
    ] {
        quadratic += 2. * weight;
    }
    expected[64] = constant * std::f64::consts::FRAC_1_SQRT_2;
    expected[192..197].copy_from_slice(&[0., 2., 2., 2., 2.]);
    expected[256..261].fill(0.);
    expected[512] = quadratic.abs() * 2.5_f64.sqrt();
    expected[576] = ((quadratic * quadratic) / 2.).sqrt();
    expected
}
pub(super) struct ResourceBuffers {
    counts: [i32; 1],
    sums: [f64; 5],
    differences: [f64; 5],
    left: [f64; 2],
    right: [f64; 2],
    cuts: [f64; 2],
    maximum: [f64; 1],
    mean: [f64; 1],
}
impl ResourceBuffers {
    pub(super) fn new() -> Self {
        Self {
            counts: [-7],
            sums: [7.; 5],
            differences: [7.; 5],
            left: [7.; 2],
            right: [7.; 2],
            cuts: [7.; 2],
            maximum: [7.],
            mean: [7.],
        }
    }
    pub(super) fn output(
        &mut self,
        curve: AdvApp2VarCallerCurveSlots,
    ) -> IsoApproxCallerOutput<'_, AdvApp2VarCallerCurveSlots> {
        IsoApproxCallerOutput {
            ncoeff: &mut self.counts,
            courbe: curve,
            nbcrbe: -7,
            somtab: &mut self.sums,
            diftab: &mut self.differences,
            contr1: &mut self.left,
            contr2: &mut self.right,
            tabdec: &mut self.cuts,
            errmax: &mut self.maximum,
            errmoy: &mut self.mean,
        }
    }
}
