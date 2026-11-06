//! The dataset files holding `COMPLEX_TRIANGULATED_FACE` records whose normals are not a square
//! array. Each writes and reads
//! back, and the faces read back hold what the source text of the file holds.
use martensite_tkdestep::rw_step_ap214::RwStepAP214;
use martensite_tkdestep::step_ap214::StepAP214;
use martensite_tkdestep::step_data::{StepDataProtocolOps, StepDataStepModel, StepDataStepWriter};
use martensite_tkdestep::step_file::step_file_read;
use martensite_tkdestep::step_visual::StepVisualComplexTriangulatedFace;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use std::io::Cursor;
use std::path::PathBuf;

const FACE_KEYWORD: &str = "COMPLEX_TRIANGULATED_FACE(";

/// What a face holds, whichever way it was obtained.
#[derive(Debug, PartialEq)]
struct FaceValues {
    pnmax: i32,
    normals: Vec<Vec<f64>>,
    pnindex: Vec<i32>,
    strips: Vec<Vec<i32>>,
    fans: Vec<Vec<i32>>,
}

fn protocol() -> Handle<dyn StepDataProtocolOps> {
    RwStepAP214::init();
    StepAP214::protocol().cast()
}

fn model_of(bytes: Vec<u8>) -> Handle<StepDataStepModel> {
    let model = StepDataStepModel::new();
    let mut stream = Cursor::new(bytes);
    let status = step_file_read("dataset", Some(&mut stream), &model, &protocol()).unwrap();
    assert_eq!(status, 0, "the text does not parse");
    model
}

fn written(model: &Handle<StepDataStepModel>) -> Vec<u8> {
    let mut writer = StepDataStepWriter::new(model);
    writer.send_model(&protocol(), false).unwrap();
    let mut out = Vec::new();
    writer.print(&mut out).unwrap();
    out
}

/// The arguments of the record whose text starts at `text`, cut at the commas outside parentheses.
fn arguments_of(text: &str) -> Vec<&str> {
    let (mut depth, mut start) = (1, 0);
    let mut found = Vec::new();
    for (at, byte) in text.bytes().enumerate() {
        depth += i32::from(byte == b'(') - i32::from(byte == b')');
        if depth == 0 || (depth == 1 && byte == b',') {
            found.push(text[start..at].trim());
            start = at + 1;
        }
        if depth == 0 {
            break;
        }
    }
    found
}

fn numbers_of<T: std::str::FromStr>(list: &str) -> Vec<T> {
    list.split(|c: char| "(), \r\n".contains(c))
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().ok().expect("a number"))
        .collect()
}

/// `((1,2),(3))` as its rows; `()` has none.
fn rows_of<T: std::str::FromStr>(list: &str) -> Vec<Vec<T>> {
    let inner = list
        .trim()
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    arguments_of(&format!("{inner})"))
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| numbers_of(row))
        .collect()
}

/// The faces of a STEP text, read from the text alone.
fn faces_in_text(text: &str) -> Vec<FaceValues> {
    text.match_indices(FACE_KEYWORD)
        .map(|(at, _)| {
            let arguments = arguments_of(&text[at + FACE_KEYWORD.len()..]);
            FaceValues {
                pnmax: arguments[2].parse().unwrap(),
                normals: rows_of(arguments[3]),
                pnindex: numbers_of(arguments[5]),
                strips: rows_of(arguments[6]),
                fans: rows_of(arguments[7]),
            }
        })
        .collect()
}

fn lists_of(lists: &Handle<HArray1<Handle<StandardTransient>>>) -> Vec<Vec<i32>> {
    let lists = lists.get().unwrap().array1();
    (1..=lists.length())
        .map(|rank| {
            let list = lists.value(rank).unwrap().cast::<HArray1<i32>>();
            list.get().unwrap().array1().iter().copied().collect()
        })
        .collect()
}

fn values_of(face: &StepVisualComplexTriangulatedFace) -> FaceValues {
    let normals = face.normals();
    let normals = normals.get().unwrap().array2();
    FaceValues {
        pnmax: face.pnmax(),
        normals: (1..=normals.col_length())
            .map(|row| {
                (1..=normals.row_length())
                    .map(|column| *normals.value(row, column).unwrap())
                    .collect()
            })
            .collect(),
        pnindex: face
            .pnindex()
            .get()
            .unwrap()
            .array1()
            .iter()
            .copied()
            .collect(),
        strips: lists_of(&face.triangle_strips()),
        fans: lists_of(&face.triangle_fans()),
    }
}

/// The faces of a model, in the order of its entities.
fn faces_in_model(model: &Handle<StepDataStepModel>) -> Vec<FaceValues> {
    let live = model.get().unwrap();
    (1..=live.nb_entities())
        .filter_map(|num| {
            let entity = live.value(num).unwrap();
            entity
                .is_kind(StepVisualComplexTriangulatedFace::static_type())
                .then(|| {
                    values_of(
                        entity
                            .cast::<StepVisualComplexTriangulatedFace>()
                            .get()
                            .unwrap(),
                    )
                })
        })
        .collect()
}

fn assert_written_file_keeps_its_faces(name: &str, faces: usize, entities: i32) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step")
        .join(name);
    let bytes = std::fs::read(&path).unwrap();
    let expected = faces_in_text(&String::from_utf8_lossy(&bytes));
    assert_eq!(expected.len(), faces, "{name}: faces in the source text");
    let model = model_of(bytes);
    assert_eq!(model.get().unwrap().nb_entities(), entities, "{name}");
    assert!(
        expected
            .iter()
            .any(|face| face.normals.len() != face.normals[0].len()),
        "{name}"
    );
    let back = model_of(written(&model));
    assert_eq!(
        back.get().unwrap().nb_entities(),
        entities,
        "{name}: entities read back"
    );
    assert_eq!(faces_in_model(&back), expected, "{name}: faces read back");
}

#[test]
fn bug33638_writes_and_keeps_its_faces() {
    assert_written_file_keeps_its_faces("bug33638.stp", 6, 69);
}

#[test]
fn bug33661_writes_and_keeps_its_faces() {
    assert_written_file_keeps_its_faces("bug33661_nist_ftc_08_asme1_ap242-e1-tg.stp", 273, 12630);
}

#[test]
fn nist_ftc_08_writes_and_keeps_its_faces() {
    assert_written_file_keeps_its_faces("nist_ftc_08_asme1_ap242-2.stp", 273, 12630);
}
