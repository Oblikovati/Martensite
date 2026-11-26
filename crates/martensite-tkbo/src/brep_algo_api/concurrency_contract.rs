// SPDX-License-Identifier: GPL-2.0-only
//! Regression contracts for non-destructive input preservation (#461), `Send` operation
//! objects (#64), and multi-threaded determinism vs single-threaded execution (#66).
use super::{BRepAlgoAPIBuilderAlgo, BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::operation::OperationResult;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::io::{Read, Seek, SeekFrom};

fn assert_send<T: Send>() {}

fn make_box(x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Shape {
    let mut maker =
        BRepPrimAPIMakeBox::from_points(Point3::new(x, y, z), Point3::new(x + dx, y + dy, z + dz))
            .expect("make box");
    maker.shape().expect("box shape").clone()
}

fn serialize_shape(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    brep_tools::write(shape, &mut stream, &ProgressRange::default()).expect("serialize shape");
    let mut bytes = Vec::new();
    let mut device = stream.into_device().expect("memory device");
    device.seek(SeekFrom::Start(0)).expect("rewind stream");
    device.read_to_end(&mut bytes).expect("read stream");
    bytes
}

#[test]
fn public_boolean_operation_objects_are_send() {
    assert_send::<BRepAlgoAPIFuse>();
    assert_send::<BRepAlgoAPICut>();
    assert_send::<BRepAlgoAPICommon>();
    assert_send::<BRepAlgoAPIBuilderAlgo>();
    assert_send::<BRepPrimAPIMakeBox>();
    assert_send::<OperationResult>();
}

#[test]
fn non_destructive_disjoint_fuse_preserves_input_free_flag_and_bytes() {
    let left = make_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let right = make_box(5.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    assert!(left.free().expect("left free before"));
    assert!(right.free().expect("right free before"));

    let left_before = serialize_shape(&left);
    let right_before = serialize_shape(&right);

    let mut fuse = BRepAlgoAPIFuse::new();
    fuse.set_non_destructive(true);
    let mut args = martensite_tkernel::ncollection::List::default();
    args.append(left.clone());
    let mut tools = martensite_tkernel::ncollection::List::default();
    tools.append(right.clone());
    fuse.set_arguments(&args);
    fuse.set_tools(&tools);
    fuse.build().expect("fuse disjoint boxes");
    assert!(fuse.is_done());

    assert!(left.free().expect("left free after"));
    assert!(right.free().expect("right free after"));
    assert_eq!(serialize_shape(&left), left_before);
    assert_eq!(serialize_shape(&right), right_before);
}

#[test]
fn multithreaded_booleans_match_single_threaded_output_byte_for_byte() {
    let run_pair = |offset: f64| -> Vec<u8> {
        let a = make_box(offset, 0.0, 0.0, 2.0, 2.0, 2.0);
        let b = make_box(offset + 1.0, 0.5, 0.5, 2.0, 1.0, 1.0);
        let fuse = BRepAlgoAPIFuse::from_shapes(&a, &b).expect("fuse");
        assert!(fuse.is_done());
        serialize_shape(fuse.shape())
    };

    let offsets = [0.0, 10.0, 20.0, 30.0];
    let single_thread: Vec<Vec<u8>> = offsets.iter().map(|&o| run_pair(o)).collect();

    let handles: Vec<_> = offsets
        .iter()
        .map(|&o| std::thread::spawn(move || run_pair(o)))
        .collect();
    let multi_thread: Vec<Vec<u8>> = handles
        .into_iter()
        .map(|h| h.join().expect("worker thread"))
        .collect();

    assert_eq!(multi_thread, single_thread);
}
