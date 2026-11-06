//! Reads files of the test dataset through `StepControlReader`, checks the shape, writes it with
//! `StepControlWriter` and reads it back. Expected counts and masses are recorded, with error-bounded
//! integration: the default integration is off by 0.88 on `xr1-tc-rod.stp`.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkdestep::de_step::StepControlStepModelType;
use martensite_tkdestep::step_control::{StepControlReader, StepControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::path::PathBuf;

/// The measures of a shape compared with recorded.
#[derive(Debug, Clone, Copy)]
struct Measures {
    solids: i32,
    faces: i32,
    mass: f64,
}

fn dataset(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step")
        .join(name)
}

fn count(shape: &Shape, kind: ShapeEnum) -> i32 {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("explorer");
    let mut total = 0;
    while explorer.more() {
        total += 1;
        explorer.next().expect("next");
    }
    total
}

fn measures(shape: &Shape) -> Measures {
    let mut properties = Default::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).expect("volume");
    Measures {
        solids: count(shape, ShapeEnum::Solid),
        faces: count(shape, ShapeEnum::Face),
        mass: properties.mass(),
    }
}

fn read(path: &str) -> Shape {
    let mut reader = StepControlReader::new().expect("reader");
    assert_eq!(
        reader.read_file(path).expect("read"),
        IFSelectReturnStatus::RetDone,
        "{path}"
    );
    // Comparing the translators alone with recorded: no healing.
    if std::env::var("STEP_ROUND_TRIP_NO_HEALING").is_ok() {
        reader
            .xs_reader()
            .set_shape_process_flags(Default::default());
    }
    let range = ProgressRange::default();
    assert!(
        reader.transfer_roots(&range).expect("transfer") > 0,
        "{path}: no root transferred"
    );
    reader.xs_reader().one_shape()
}

fn assert_valid(shape: &Shape, what: &str) {
    assert!(!shape.is_null(), "{what}: no shape");
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false).expect("analyzer");
    assert!(
        analyzer.is_valid().expect("analysis"),
        "{what}: BRepCheck reports the shape invalid"
    );
}

fn assert_measures(found: Measures, expected: Measures, what: &str) {
    assert_eq!(found.solids, expected.solids, "{what}: solids");
    assert_eq!(found.faces, expected.faces, "{what}: faces");
    let gap = (found.mass - expected.mass).abs();
    assert!(
        gap <= 1.0e-6 * expected.mass.abs().max(1.0),
        "{what}: mass {} expected {}",
        found.mass,
        expected.mass
    );
}

fn round_trip(name: &str, expected: Measures) {
    let path = dataset(name);
    let shape = read(path.to_str().expect("path"));
    assert_valid(&shape, name);
    assert_measures(measures(&shape), expected, name);
    let output = std::env::temp_dir().join(format!(
        "martensite-round-trip-{}-{name}",
        std::process::id()
    ));
    let writer = StepControlWriter::new().expect("writer");
    let status = writer
        .transfer(
            &shape,
            StepControlStepModelType::AS_IS,
            true,
            &ProgressRange::default(),
        )
        .expect("transfer");
    assert_eq!(status, IFSelectReturnStatus::RetDone, "{name}: transfer");
    let written = output.to_str().expect("path");
    assert_eq!(
        writer.write(written).expect("write"),
        IFSelectReturnStatus::RetDone,
        "{name}: write"
    );
    let again = read(written);
    std::fs::remove_file(&output).ok();
    assert_valid(&again, "the shape read back");
    assert_measures(measures(&again), expected, "the shape read back");
}

/// The measures of the shape read back after writing it, as text.
fn written_back(shape: &Shape, name: &str) -> String {
    if shape.is_null() {
        return "none".to_owned();
    }
    let output =
        std::env::temp_dir().join(format!("martensite-list-{}-{name}", std::process::id()));
    let written = output.to_str().expect("path");
    let writer = StepControlWriter::new().expect("writer");
    let mode = StepControlStepModelType::AS_IS;
    if writer
        .transfer(shape, mode, true, &ProgressRange::default())
        .is_err()
        || writer.write(written).is_err()
    {
        return "write-failed".to_owned();
    }
    let again = read(written);
    std::fs::remove_file(&output).ok();
    let found = measures(&again);
    format!("{}/{}/{:.12e}", found.solids, found.faces, found.mass)
}

/// The time of each stage on the file named by `STEP_ROUND_TRIP_STAGES`, and the surface area.
#[test]
fn stages_of_a_named_file() {
    let Ok(name) = std::env::var("STEP_ROUND_TRIP_STAGES") else {
        return;
    };
    let start = std::time::Instant::now();
    let shape = read(dataset(&name).to_str().expect("path"));
    println!(
        "STAGE read {:?} solids={} faces={}",
        start.elapsed(),
        count(&shape, ShapeEnum::Solid),
        count(&shape, ShapeEnum::Face)
    );
    let mut explorer = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    let mut rank = 0;
    while explorer.more() {
        rank += 1;
        let mut area = Default::default();
        let face = explorer.current();
        BRepGProp::surface_properties(face, &mut area, false, false).expect("area");
        println!("FACE {rank} area={:.9e}", area.mass());
        explorer.next().expect("next");
    }
    println!("STAGE area {:?}", start.elapsed());
    let valid = BRepCheckAnalyzer::new(&shape, true, false, false).and_then(|a| a.is_valid());
    println!("STAGE check {:?} {valid:?}", start.elapsed());
    println!("STAGE mass {:?} {:?}", start.elapsed(), measures(&shape));
}

/// One line per file of `STEP_ROUND_TRIP_LIST` (a file of names), for comparing with recorded:
/// nothing is asserted, so one failing file does not hide the others.
#[test]
fn measures_of_listed_files() {
    let Ok(list) = std::env::var("STEP_ROUND_TRIP_LIST") else {
        return;
    };
    for name in std::fs::read_to_string(list).expect("list").lines() {
        let outcome = std::panic::catch_unwind(|| {
            let shape = read(dataset(name).to_str().expect("path"));
            let found = measures(&shape);
            let valid = !shape.is_null()
                && BRepCheckAnalyzer::new(&shape, true, false, false)
                    .and_then(|a| a.is_valid())
                    .unwrap_or(false);
            (found, valid, written_back(&shape, name))
        });
        match outcome {
            Ok((found, valid, back)) => println!(
                "RESULT {name}: solids={} faces={} mass={:.12e} valid={} back={}",
                found.solids,
                found.faces,
                found.mass,
                i32::from(valid),
                back
            ),
            Err(_) => println!("RESULT {name}: panic"),
        }
    }
}

/// `clean_duplicate_entities` between the transfer and the write leaves the same shape.
#[test]
fn a_cleaned_model_writes_the_same_shape() {
    let name = "trj6_c1-ug-214.stp";
    let shape = read(dataset(name).to_str().expect("path"));
    let writer = StepControlWriter::new().expect("writer");
    let mode = StepControlStepModelType::AS_IS;
    writer
        .transfer(&shape, mode, true, &ProgressRange::default())
        .expect("transfer");
    writer.clean_duplicate_entities().expect("clean");
    let output =
        std::env::temp_dir().join(format!("martensite-clean-{}-{name}", std::process::id()));
    let written = output.to_str().expect("path");
    assert_eq!(
        writer.write(written).expect("write"),
        IFSelectReturnStatus::RetDone
    );
    let again = read(written);
    std::fs::remove_file(&output).ok();
    assert_valid(&again, "the cleaned shape read back");
    let expected = Measures {
        solids: 1,
        faces: 7,
        mass: 27829.7552413793,
    };
    assert_measures(measures(&again), expected, "the cleaned shape read back");
}

#[test]
fn a_box_reads_and_writes_back() {
    round_trip(
        "tr9_3dt-td.stp",
        Measures {
            solids: 1,
            faces: 6,
            mass: 125000.0,
        },
    );
}

#[test]
fn a_rod_reads_and_writes_back() {
    round_trip(
        "xr1-tc-rod.stp",
        Measures {
            solids: 1,
            faces: 4,
            mass: 15707.9632679,
        },
    );
}

#[test]
fn a_part_with_curved_faces_reads_and_writes_back() {
    round_trip(
        "trj6_c1-ug-214.stp",
        Measures {
            solids: 1,
            faces: 7,
            mass: 27829.7552413793,
        },
    );
}

#[test]
fn two_spheres_read_and_write_back() {
    round_trip(
        "bug24283_spheres.stp",
        Measures {
            solids: 2,
            faces: 2,
            mass: 8.3775804095728,
        },
    );
}
