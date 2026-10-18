//! A negative logical root offset can still name initialized original owner storage.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareIntegerField as Field, AdvApp2VarSquareOperationFrame as Frame,
    AdvApp2VarSquareRootAxis as Axis, AdvApp2VarSquareSampleFrame as Samples,
};
const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
fn signed_root_frame() -> Frame {
    Frame::new(
        vec![1, 3, 3, 2, -1],
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
fn signed_root_request() -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: 3,
        nbpntv: 3,
        urootb: &[],
        vrootb: &[],
        isofav: 2,
    }
}
#[test]
fn changed_count_reads_initialized_prefix_before_root_origin() {
    let mut values = vec![7.; 128];
    values[15] = 0.25;
    values[16..19].copy_from_slice(&ROOTS);
    let mut samples = Samples::new(values, 96, 80)
        .unwrap()
        .with_root_origin(Axis::U, 16)
        .unwrap()
        .with_root_origin(Axis::V, 16)
        .unwrap();
    let mut frame = signed_root_frame();
    let mut request = signed_root_request();
    let (mut sums, mut u, mut v, mut both) = ([0.; 32], [0.; 32], [0.; 32], [0.; 32]);
    let mut tables = SquareTables {
        sosotb: &mut sums,
        disotb: &mut u,
        soditb: &mut v,
        diditb: &mut both,
    };
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>| {
        let mut calls = calls.lock().unwrap();
        let index = *calls;
        *calls += 1;
        let constants = [(1. + ROOTS[0]) / 2., 0.5, (1. + 0.25) / 2.];
        assert_eq!(*call.constant_parameter(), constants[index]);
        for (index, root) in ROOTS.iter().enumerate() {
            assert_eq!(call.buffers().parameter(index), (1. + root) / 2.);
        }
        assert_eq!(
            call.integer(Field::VRootCount),
            if index == 0 { 3 } else { -3 }
        );
        if index == 0 {
            call.set_integer(Field::VRootCount, -3);
            assert_eq!(call.integer(Field::VRootCount), -3);
        }
        for index in 0..3 {
            call.buffers().set_result(index, 1.);
        }
        call.set_status(0).unwrap();
    };
    frame
        .sample_square_owned_roots(&mut request, &evaluator, &mut tables, &mut samples)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 3);
    assert_eq!(frame.status().unwrap(), 0);
    assert_eq!(
        (
            request.ndimen,
            request.nbpntu,
            request.nbpntv,
            request.isofav
        ),
        (1, 3, -3, 2)
    );
    assert_eq!(&sums[..4], &[1., 2., 2., 4.]);
    assert!(sums[4..].iter().all(|value| *value == 0.));
    for table in [u, v, both] {
        assert!(table.iter().all(|value| *value == 0.));
    }
    assert_eq!(samples.root(Axis::V, 0).unwrap(), ROOTS[0]);
}
