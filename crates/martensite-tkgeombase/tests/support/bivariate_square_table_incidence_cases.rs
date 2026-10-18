//! Four table roles share actual initialized slots, including callback sample writes.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareOperationFrame as Frame, AdvApp2VarSquareSampleFrame as Samples,
    AdvApp2VarSquareTable as Table,
};
const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
const ROLES: [Table; 4] = [
    Table::SumSum,
    Table::DifferenceSum,
    Table::SumDifference,
    Table::DifferenceDifference,
];
fn table_frame(iso: i32) -> Frame {
    Frame::new(
        vec![1, 3, 3, iso, -1],
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap()
    .with_owned_status_slot(4)
    .unwrap()
}
fn table_request(iso: i32) -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: 3,
        nbpntv: 3,
        urootb: &ROOTS,
        vrootb: &ROOTS,
        isofav: iso,
    }
}
fn run_table_case(iso: i32, shared_samples: bool) {
    let mut samples = Samples::new(vec![7.; 128], 96, if shared_samples { 32 } else { 104 })
        .unwrap()
        .with_table_origins([32; 4])
        .unwrap();
    let mut frame = table_frame(iso);
    let mut request = table_request(iso);
    let traces = std::sync::Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>| {
        let mut traces = traces.lock().unwrap();
        traces.push((
            *call.constant_parameter(),
            [
                call.buffers().parameter(0),
                call.buffers().parameter(1),
                call.buffers().parameter(2),
            ],
        ));
        let count = call
            .argument(martensite_tkgeombase::adv_app2var::AdvApp2VarIntegerField::ParameterCount);
        assert_eq!(count, 3);
        for index in 0..3 {
            call.buffers().set_result(index, 1.);
        }
        if shared_samples {
            for role in ROLES {
                assert_eq!(call.caller_table(role, 0).unwrap(), 1.);
            }
        }
        assert!(call.caller_table(Table::SumSum, usize::MAX).is_err());
        call.set_status(0).unwrap();
    };
    assert_eq!(
        frame
            .sample_square_owned_tables(&mut request, &evaluator, &mut samples)
            .unwrap(),
        0
    );
    assert_eq!(frame.status().unwrap(), 0);
    let expected = if shared_samples {
        [3., 1., 3., 11.]
    } else {
        [8., 9., 9., 11.]
    };
    for role in ROLES {
        let actual: [f64; 4] = std::array::from_fn(|index| samples.table(role, index).unwrap());
        assert_eq!(actual, expected);
        for index in 4..8 {
            assert_eq!(samples.table(role, index).unwrap(), 7.);
        }
    }
    let expected_parameters = ROOTS.map(|root| (1. + root) / 2.);
    let traces = traces.into_inner().unwrap();
    assert_eq!(traces.len(), 3);
    for (index, (constant, parameters)) in traces.iter().enumerate() {
        assert_eq!(*constant, expected_parameters[index]);
        assert_eq!(*parameters, expected_parameters);
    }
    assert_eq!(
        (
            request.ndimen,
            request.nbpntu,
            request.nbpntv,
            request.isofav
        ),
        (1, 3, 3, iso)
    );
}
#[test]
fn iso1_coincident_tables_follow_sequential_source_writes() {
    run_table_case(1, false);
}
#[test]
fn iso2_coincident_tables_follow_sequential_source_writes() {
    run_table_case(2, false);
}
#[test]
fn iso1_sample_writes_are_immediately_visible_in_all_table_roles() {
    run_table_case(1, true);
}
#[test]
fn iso2_sample_writes_are_immediately_visible_in_all_table_roles() {
    run_table_case(2, true);
}
#[test]
fn table_origins_and_entrypoint_contracts_reject_ambiguous_storage() {
    let evaluator =
        |_: &mut Call<'_, '_>| panic!("invalid table contract must fail before evaluation");
    let mut frame = table_frame(2);
    let mut request = table_request(2);
    let mut missing = Samples::new(vec![7.; 128], 96, 104).unwrap();
    assert!(missing.table(Table::SumSum, 0).is_err());
    assert!(
        frame
            .sample_square_owned_tables(&mut request, &evaluator, &mut missing)
            .is_err()
    );
    assert!(
        Samples::new(vec![7.; 128], 96, 104)
            .unwrap()
            .with_table_origins([129; 4])
            .is_err()
    );
    let mut bound = missing.with_table_origins([32; 4]).unwrap();
    let (mut a, mut b, mut c, mut d) = ([7.; 64], [7.; 64], [7.; 64], [7.; 64]);
    let mut tables = SquareTables {
        sosotb: &mut a,
        disotb: &mut b,
        soditb: &mut c,
        diditb: &mut d,
    };
    assert!(
        frame
            .sample_square_owned_samples(&mut request, &evaluator, &mut tables, &mut bound)
            .is_err()
    );
    assert!(
        frame
            .sample_square_owned_roots(&mut request, &evaluator, &mut tables, &mut bound)
            .is_err()
    );
    assert!(bound.table(Table::SumSum, usize::MAX).is_err());
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Table>();
}
#[test]
fn callback_table_writes_and_samples_share_one_allocation_bidirectionally() {
    let mut frame = table_frame(2);
    let mut request = table_request(2);
    let mut samples = Samples::new(vec![7.; 128], 96, 32)
        .unwrap()
        .with_table_origins([32; 4])
        .unwrap();
    let evaluator = |call: &mut Call<'_, '_>| {
        call.set_caller_table(Table::SumSum, 0, 0.25).unwrap();
        assert_eq!(call.buffers().result(0), 0.25);
        for role in ROLES {
            assert_eq!(call.caller_table(role, 0).unwrap(), 0.25);
        }
        for index in 0..3 {
            call.buffers().set_result(index, 1.);
        }
        for role in ROLES {
            assert_eq!(call.caller_table(role, 0).unwrap(), 1.);
        }
        assert!(
            call.set_caller_table(Table::SumSum, usize::MAX, 0.)
                .is_err()
        );
        call.set_status(0).unwrap();
    };
    frame
        .sample_square_owned_tables(&mut request, &evaluator, &mut samples)
        .unwrap();
    assert_eq!(samples.table(Table::SumSum, 0).unwrap(), 3.);
}
