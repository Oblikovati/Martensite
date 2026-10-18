//! Caller integer outputs act at source writes, before clear lengths and callback setup.
use super::*;
pub(super) use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerSamplingField as Sample, AdvApp2VarCombinedBufferEvaluation as Call,
    AdvApp2VarIntegerBindings as Bindings, AdvApp2VarIntegerField as Int,
    AdvApp2VarIsoIntegerOutput as Output, AdvApp2VarIsoOperationFrame as Frame,
    AdvApp2VarRealBindings as Reals,
};
const ROOTS3: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
pub(super) const ROOTS8: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
fn output_request(iso: i32, ordinary: bool) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: if ordinary { 8 } else { 3 },
        rootlg: if ordinary { &ROOTS8 } else { &ROOTS3 },
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    }
}
fn output_frame(iso: i32, alias: usize) -> Frame {
    let mut values = vec![-7; 64];
    values[32] = 1;
    values[33] = if alias == 3 { 8 } else { 3 };
    values[36] = iso;
    values[37] = 2;
    if alias == 1 {
        values[35] = -1;
    }
    let slots = [
        if alias == 0 {
            32
        } else if alias == 2 {
            33
        } else {
            35
        },
        if alias == 1 { 32 } else { 34 },
    ];
    Frame::new(
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
    .unwrap()
    .with_caller_sampling_slots([33, 37])
    .unwrap()
    .with_caller_output_slots(slots)
    .unwrap()
}
#[derive(Clone)]
struct OutputTables {
    values: [f64; 128],
    counts: [i32; 3],
}
impl OutputTables {
    fn new() -> Self {
        Self {
            values: [7.; 128],
            counts: [-7; 3],
        }
    }
    fn output(&mut self, made: i32) -> IsoApproxOutput<'_> {
        let (rest, mean) = self.values.split_at_mut(112);
        let (rest, max) = rest.split_at_mut(104);
        let (rest, cuts) = rest.split_at_mut(96);
        let (rest, second) = rest.split_at_mut(80);
        let (rest, first) = rest.split_at_mut(64);
        let (rest, differences) = rest.split_at_mut(48);
        let (rest, sums) = rest.split_at_mut(32);
        let (_, curve) = rest.split_at_mut(16);
        IsoApproxOutput {
            ncoeff: &mut self.counts,
            courbe: curve,
            nbcrbe: made,
            somtab: sums,
            diftab: differences,
            contr1: first,
            contr2: second,
            tabdec: cuts,
            errmax: max,
            errmoy: mean,
        }
    }
}
fn run_error_case(iso: i32, alias: usize) {
    let mut frame = output_frame(iso, alias);
    let mut request = output_request(iso, false);
    let mut tables = OutputTables::new();
    let made = if alias == 1 { 1 } else { -7 };
    let mut output = tables.output(made);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(call.integer(Int::Dimension), if alias <= 1 { 0 } else { 1 });
        assert_eq!(
            call.integer(Int::ParameterCount),
            if alias == 2 { 0 } else { 3 }
        );
        assert_eq!(call.integer(Int::FavoriteIso), iso);
        assert_eq!(*call.u_bounds(), [0., 1.]);
        assert_eq!(*call.v_bounds(), [0., 1.]);
        assert_eq!(call.caller_output(Output::Status).unwrap(), 0);
        assert_eq!(call.caller_output(Output::CompletedPieces).unwrap(), 0);
        for (index, root) in ROOTS3
            .iter()
            .enumerate()
            .take(if alias == 2 { 0 } else { 3 })
        {
            assert_eq!(call.buffers().parameter(index), (1. + root) / 2.);
        }
        call.set_caller_output(Output::Status, 1).unwrap();
        assert_eq!(call.caller_output(Output::Status).unwrap(), 1);
        if alias == 0 {
            assert_eq!(call.integer(Int::Dimension), 1);
        }
        if alias == 2 {
            assert_eq!(call.caller_sampling(Sample::RootCount).unwrap(), 1);
        }
    };
    assert_eq!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .unwrap(),
        0
    );
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(output.nbcrbe, 0);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 101);
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 0);
    assert_eq!(
        request.ndimen,
        if alias == 0 {
            101
        } else if alias == 1 {
            0
        } else {
            1
        }
    );
    assert_eq!(request.nbroot, if alias == 2 { 101 } else { 3 });
    assert_eq!(
        &output.courbe[..8],
        if alias == 0 {
            &[7.; 8]
        } else {
            &[0., 0., 0., 7., 7., 7., 7., 7.]
        }
    );
    assert_eq!(output.ncoeff, &[-7, -7, -7]);
    assert_eq!(&output.tabdec[..4], &[0., 1., 7., 7.]);
    for values in [
        output.somtab,
        output.diftab,
        output.contr1,
        output.contr2,
        output.errmax,
        output.errmoy,
    ] {
        assert!(values.iter().all(|value| *value == 7.));
    }
}
#[test]
fn first_iso_status_clear_changes_dimension_before_curve_clear() {
    run_error_case(1, 0);
}
#[test]
fn second_iso_status_clear_changes_dimension_before_curve_clear() {
    run_error_case(2, 0);
}
#[test]
fn first_iso_piece_count_clear_changes_dimension_after_curve_clear() {
    run_error_case(1, 1);
}
#[test]
fn second_iso_piece_count_clear_changes_dimension_after_curve_clear() {
    run_error_case(2, 1);
}
#[test]
fn first_iso_status_clear_changes_root_count_before_sampling() {
    run_error_case(1, 2);
}
#[test]
fn second_iso_status_clear_changes_root_count_before_sampling() {
    run_error_case(2, 2);
}
fn run_ordinary_case(iso: i32) {
    let mut frame = output_frame(iso, 3);
    let mut request = output_request(iso, true);
    let mut tables = OutputTables::new();
    let mut baseline = OutputTables::new();
    let mut output = tables.output(-7);
    let mut previous = baseline.output(-7);
    let mut old_request = request;
    let mut old_status = -7;
    let legacy = |call: &AdvApp2VarEvaluation<'_>, values: &mut [f64], error: &mut i32| {
        for value in values
            .iter_mut()
            .take((call.dimension * call.nb_params) as usize)
        {
            *value = 1.;
        }
        *error = 0;
    };
    mma2fnc_mut_(&mut old_request, &legacy, &mut previous, &mut old_status);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        *calls.lock().unwrap() += 1;
        assert_eq!(call.integer(Int::Dimension), 1);
        assert_eq!(call.integer(Int::ParameterCount), 8);
        for (index, root) in ROOTS8.iter().enumerate() {
            assert_eq!(call.buffers().parameter(index), (1. + root) / 2.);
            call.buffers().set_result(index, 1.);
        }
        call.set_caller_output(Output::Status, 0).unwrap();
    };
    frame
        .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(
        (output.nbcrbe, frame.caller_output(Output::Status).unwrap()),
        (1, 0)
    );
    assert_eq!(output.nbcrbe, previous.nbcrbe);
    assert_eq!(old_status, 0);
    assert_eq!(output.ncoeff, &[1, -7, -7]);
    assert!((output.courbe[0] - 1.).abs() <= 32. * f64::EPSILON);
    assert_eq!(&output.courbe[1..3], &[0., 0.]);
    assert_eq!(
        tables.values.map(f64::to_bits),
        baseline.values.map(f64::to_bits)
    );
    assert_eq!(tables.counts, baseline.counts);
}
#[test]
fn first_iso_owned_integer_outputs_complete_constant_curve_unchanged() {
    run_ordinary_case(1);
}
#[test]
fn second_iso_owned_integer_outputs_complete_constant_curve_unchanged() {
    run_ordinary_case(2);
}

#[path = "bivariate_iso_integer_output_contract_cases.rs"]
mod contracts;
