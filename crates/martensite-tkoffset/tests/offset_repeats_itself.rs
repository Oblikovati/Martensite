//! The same offset gives the same shape, written out byte for byte: in repeats within one process
//! and in processes of their own, where every address differs.
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPEATS: usize = 3;
/// Set in a child process: the folder it leaves its results in.
const RESULT_FOLDER: &str = "MARTENSITE_OFFSET_REPEAT_FOLDER";
const CHILD_TEST: &str = "writes_its_offsets_for_the_parent_process";

fn test_box() -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0).unwrap();
    maker.shape().unwrap().clone()
}

fn test_cylinder() -> Shape {
    let mut maker = BRepPrimAPIMakeCylinder::new(10.0, 20.0);
    maker.shape().unwrap().clone()
}

/// The offset by arcs of `shape`, as the bytes of its written form.
fn offset_bytes(shape: &Shape, value: f64, scratch: &Path) -> Vec<u8> {
    let mut maker = BRepOffsetMakeOffset::new();
    maker.initialize(
        shape,
        value,
        1.0e-7,
        MakeOffsetOptions {
            mode: BRepOffsetMode::Skin,
            intersection: false,
            self_inter: false,
            join: JoinType::Arc,
            thickening: false,
            remove_int_edges: false,
        },
    );
    maker.make_offset_shape(&ProgressRange::default()).unwrap();
    assert!(maker.is_done());
    let file = scratch.join("offset.brep");
    let name = AsciiString::from(file.to_string_lossy().as_ref());
    assert!(brep_tools::write_file(maker.shape(), &name, &ProgressRange::default()).unwrap());
    fs::read(&file).unwrap()
}

/// Every offset of the test, in order.
fn all_offsets(scratch: &Path) -> Vec<Vec<u8>> {
    [
        (test_box(), 1.0),
        (test_box(), -1.0),
        (test_cylinder(), 1.0),
    ]
    .iter()
    .map(|(shape, value)| offset_bytes(shape, *value, scratch))
    .collect()
}

fn scratch_folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "martensite-offset-repeats-{name}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&folder).unwrap();
    folder
}

#[test]
fn repeats_in_one_process_give_the_same_bytes() {
    let scratch = scratch_folder("same-process");
    let first = all_offsets(&scratch);
    for _ in 1..REPEATS {
        assert!(all_offsets(&scratch) == first);
    }
    fs::remove_dir_all(&scratch).unwrap();
}

/// Not a check of its own: what the child processes of the next test run.
#[test]
fn writes_its_offsets_for_the_parent_process() {
    let Some(folder) = std::env::var_os(RESULT_FOLDER) else {
        return;
    };
    let folder = PathBuf::from(folder);
    for (rank, bytes) in all_offsets(&folder).iter().enumerate() {
        fs::write(folder.join(format!("{rank}.bytes")), bytes).unwrap();
    }
}

#[test]
fn processes_of_their_own_give_the_same_bytes() {
    let parent = scratch_folder("own-process");
    let here = all_offsets(&parent);
    let program = std::env::current_exe().unwrap();
    for run in 0..REPEATS {
        let folder = parent.join(run.to_string());
        fs::create_dir_all(&folder).unwrap();
        let status = Command::new(&program)
            .args([CHILD_TEST, "--exact", "--test-threads=1"])
            .env(RESULT_FOLDER, &folder)
            .status()
            .unwrap();
        assert!(status.success());
        for (rank, bytes) in here.iter().enumerate() {
            let there = fs::read(folder.join(format!("{rank}.bytes"))).unwrap();
            assert!(&there == bytes, "offset {rank} differs in process {run}");
        }
    }
    fs::remove_dir_all(&parent).unwrap();
}
