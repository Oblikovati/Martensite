//! Public IGES files read into shapes, written back, and read again with the same geometry.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{DeProvider, new_configuration_node};
use martensite_tkdeiges::de_iges::{DeIgesConfigurationNode, DeIgesProvider};
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkdeiges::iges_geom::IgesGeomLine;
use martensite_tkdeiges::iges_to_brep::IgesToBRepReader;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxsbase::if_select::{IFSelectPrintCount, IFSelectPrintFail, IFSelectReturnStatus};
use martensite_tkxsbase::interface::{InterfaceModelEntities, InterfaceStatic};
use martensite_tkxsbase::transfer::TransferSimpleBinderOfTransient;
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// The actor of the format is process-wide: one transfer at a time.
fn locked() -> MutexGuard<'static, ()> {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner)
}

fn dataset_file(name: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/iges");
    root.join(name).to_string_lossy().into_owned()
}

fn temporary_file(name: &str) -> String {
    let directory = std::env::temp_dir().join(format!("iges-round-trip-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("a directory for the written files");
    directory.join(name).to_string_lossy().into_owned()
}

fn read_shape(path: &str) -> Shape {
    let mut reader = IgesControlReader::new().expect("the norm");
    assert_eq!(
        reader.read_file(path).expect(path),
        IFSelectReturnStatus::RetDone,
        "{path}"
    );
    assert!(
        reader.nb_roots_for_transfer().expect("roots") > 0,
        "{path} has no root"
    );
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    reader.xs_reader().one_shape()
}

fn count(shape: &Shape, kind: ShapeEnum) -> i32 {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().expect("the next shape");
    }
    found
}

fn area(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(shape, &mut properties, false, false).expect("the area");
    properties.mass()
}

fn write_shape(shape: &Shape, path: &str, mode: i32) -> String {
    let mut writer = IgesControlWriter::with_unit(b"MM", mode).expect("the writer");
    assert!(
        writer
            .add_shape(shape, &ProgressRange::default())
            .expect("the translation")
    );
    assert!(
        writer.write_file(path, false).expect("the writing"),
        "{path}"
    );
    std::fs::read_to_string(path).expect(path)
}

fn assert_round_trip(name: &str, mode: i32, nb_faces: i32) {
    let shape = read_shape(&dataset_file(name));
    assert!(!shape.is_null(), "{name} gave no shape");
    assert_eq!(count(&shape, ShapeEnum::Face), nb_faces, "faces of {name}");
    let read_area = area(&shape);
    assert!(read_area > 0.0, "area of {name}: {read_area}");
    let written_name = format!("{mode}-{name}");
    let path = temporary_file(&written_name);
    let text = write_shape(&shape, &path, mode);
    assert!(
        text.contains("Martensite"),
        "the header of {written_name} names the system"
    );
    assert!(
        !text.contains("Open CASCADE"),
        "the header of {written_name} names another system"
    );
    assert!(
        !text.contains("CAS.CADE"),
        "the header of {written_name} names another system"
    );
    let again = read_shape(&path);
    assert_eq!(
        count(&again, ShapeEnum::Face),
        nb_faces,
        "faces of {written_name}"
    );
    let written_area = area(&again);
    assert!(
        (written_area - read_area).abs() <= 1.0e-3 * read_area,
        "area of {written_name}: {written_area}, was {read_area}"
    );
}

#[test]
fn brep_solid_file_reads_and_writes_back_as_faces() {
    let _guard = locked();
    assert_round_trip("OCC8842_cone_Ideas.igs", 0, 2);
}

#[test]
fn brep_solid_file_reads_and_writes_back_as_brep_entities() {
    let _guard = locked();
    assert_round_trip("OCC8842_cone_Ideas.igs", 1, 2);
}

#[test]
fn trimmed_surface_file_reads_and_writes_back() {
    let _guard = locked();
    assert_round_trip("OCC131-3.igs", 0, 3);
}

#[test]
fn provider_reads_and_writes_shapes_by_the_settings_of_its_node() {
    let _guard = locked();
    let node = new_configuration_node(DeIgesConfigurationNode::default());
    let provider = DeIgesProvider::with_node(&node.cast());
    assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
    let mut shape = Shape::default();
    let input = AsciiString::new(&dataset_file("OCC8842_cone_Ideas.igs"));
    assert!(
        provider
            .read_shape(&input, &mut shape, &ProgressRange::default())
            .expect("the reading")
    );
    assert_eq!(count(&shape, ShapeEnum::Face), 2);
    let path = temporary_file("provider-OCC8842_cone_Ideas.igs");
    let output = AsciiString::new(&path);
    assert!(
        provider
            .write_shape(&output, &shape, &ProgressRange::default())
            .expect("the writing")
    );
    let again = read_shape(&path);
    assert_eq!(count(&again, ShapeEnum::Face), 2);
    assert!((area(&again) - area(&shape)).abs() <= 1.0e-3 * area(&shape));
}

/// The curves of shared edges stay in the model in BRep mode.
#[test]
fn shared_curves_of_a_box_are_in_the_model_in_brep_mode() {
    let _guard = locked();
    let solid = BRepPrimAPIMakeBox::at_point(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0)
        .expect("a box")
        .solid()
        .expect("its solid");
    let mut writer = IgesControlWriter::with_unit(b"MM", 1).expect("the writer");
    assert!(
        writer
            .add_shape(&solid, &ProgressRange::default())
            .expect("the translation")
    );
    let process_handle = writer.transfer_process();
    let process = process_handle.get().expect("the process");
    let model_handle = writer.model();
    let model = model_handle.get().expect("the model");
    let mut nb_lines = 0;
    for rank in 1..=process.nb_mapped() {
        let binder = process
            .map_item(rank)
            .expect("a binder")
            .cast::<TransferSimpleBinderOfTransient>();
        let Some(live) = binder.get() else {
            continue;
        };
        let line = live.result().cast::<IgesGeomLine>();
        if line.is_some() {
            nb_lines += 1;
            assert_ne!(
                model.d_num(&line),
                0,
                "line entity at rank {rank} is not in the model"
            );
        }
    }
    assert!(
        nb_lines > 0,
        "no line entity among {} results",
        process.nb_mapped()
    );
}

/// A file of the pinned test data outside the dataset's folder.
fn source_file(name: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/occt-source-data/iges");
    root.join(name).to_string_lossy().into_owned()
}

/// A file through both ways, its results printed: the one `IGES_TRANSFER_FILE` names, or else
/// the public files of the pinned test data.
#[test]
fn a_file_reads_and_writes_back() {
    let _guard = locked();
    let paths = std::env::var("IGES_TRANSFER_FILE").map_or_else(
        |_| vec![source_file("bearing.iges"), source_file("hammer.iges")],
        |path| vec![path],
    );
    if let Ok(settings) = std::env::var("IGES_SET") {
        let _reader = IgesControlReader::new().expect("the settings");
        for setting in settings.split(',') {
            let (key, value) = setting.split_once('=').expect("key=value");
            InterfaceStatic::set_i_val(key, value.parse().expect("a number")).expect("the setting");
        }
    }
    let line = |shape: &Shape| -> String {
        if shape.is_null() {
            return " 0 0.000000 0 0.000000".to_string();
        }
        let mut properties = GProps::new();
        BRepGProp::linear_properties(shape, &mut properties, false, false).expect("the length");
        format!(
            " {} {:.6} {} {:.6}",
            count(shape, ShapeEnum::Face),
            area(shape),
            count(shape, ShapeEnum::Edge),
            properties.mass()
        )
    };
    for path in paths {
        let shape = read_shape(&path);
        let mut result = line(&shape);
        for mode in [0, 1] {
            if shape.is_null() {
                break;
            }
            let written = std::env::var("IGES_TRANSFER_OUT").map_or_else(
                |_| temporary_file(&format!("named-{mode}.igs")),
                |prefix| format!("{prefix}.{mode}.igs"),
            );
            let mut writer = IgesControlWriter::with_unit(b"MM", mode).expect("the writer");
            let _ = writer
                .add_shape(&shape, &ProgressRange::default())
                .expect("the translation");
            let _ = writer.write_file(&written, false).expect("the writing");
            let mut reader = IgesControlReader::new().expect("the norm");
            let _ = reader.read_file(&written).expect("the reading");
            reader
                .transfer_roots(&ProgressRange::default())
                .expect("the transfer");
            result += &line(&reader.xs_reader().one_shape());
        }
        println!(
            "
RESULT{result}"
        );
    }
}

/// The direct reader, without a session: the roots of a file and one entity of it.
#[test]
fn the_direct_reader_gives_the_shapes_of_a_file() {
    let _guard = locked();
    let path = dataset_file("OCC8842_cone_Ideas.igs");
    let mut reader = IgesToBRepReader::new().expect("the set-up");
    assert_eq!(reader.load_file(&path).expect("the reading"), 0);
    assert!(!reader.is_done());
    reader
        .transfer_roots(false, &ProgressRange::default())
        .expect("the transfer");
    assert!(reader.is_done());
    assert!(reader.nb_shapes() > 0);
    let all = reader.one_shape().expect("the shapes");
    assert_eq!(count(&all, ShapeEnum::Face), 2);
    assert!(reader.shape(reader.nb_shapes() + 1).is_null());
    assert!(reader.check(false).is_ok());
    reader.clear();
    assert_eq!(reader.nb_shapes(), 0);
    assert!(
        !reader
            .transfer(0, &ProgressRange::default())
            .expect("the status")
    );
    let model = reader.model();
    let last = model.get().expect("a model").nb_entities();
    assert!(
        reader
            .transfer(last, &ProgressRange::default())
            .expect("the status")
    );
    assert!(
        !reader
            .transfer(last + 1, &ProgressRange::default())
            .expect("the status")
    );
    assert_eq!(
        reader.load_file("no-such-file.igs").expect("the status"),
        -1
    );
}

/// Every kind of report on a transfer runs on a file.
#[test]
fn a_transfer_can_be_reported_in_every_way() {
    let _guard = locked();
    let mut reader = IgesControlReader::new().expect("the norm");
    let path = dataset_file("OCC131-3.igs");
    assert_eq!(
        reader.read_file(&path).expect(&path),
        IFSelectReturnStatus::RetDone
    );
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    for fails in [IFSelectPrintFail::FailOnly, IFSelectPrintFail::FailAndWarn] {
        for mode in [
            IFSelectPrintCount::ItemsByEntity,
            IFSelectPrintCount::CountByItem,
            IFSelectPrintCount::ShortByItem,
            IFSelectPrintCount::ListByItem,
            IFSelectPrintCount::EntitiesByItem,
            IFSelectPrintCount::CountSummary,
            IFSelectPrintCount::GeneralInfo,
            IFSelectPrintCount::Mapping,
            IFSelectPrintCount::ResultCount,
        ] {
            reader.print_transfer_info(fails, mode).expect("the report");
        }
    }
}

/// A shape written through a session of the norm, in both modes.
#[test]
fn a_session_writes_a_shape_in_both_modes() {
    let _guard = locked();
    let solid = BRepPrimAPIMakeBox::at_point(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0)
        .expect("a box")
        .solid()
        .expect("its solid");
    for mode in [0, 1] {
        let _norm = IgesControlReader::new().expect("the norm");
        let session = XSControlWorkSession::new();
        let live = session.get().expect("a session");
        assert!(live.select_norm("IGES").expect("the norm"));
        let _ = live.new_model().expect("a model");
        live.transfer_writer()
            .get()
            .expect("the writer")
            .set_transfer_mode(mode);
        let status = live
            .transfer_write_shape(&solid, true, &ProgressRange::default())
            .expect("the translation");
        assert_eq!(status, IFSelectReturnStatus::RetDone);
        assert!(live.model().is_some());
    }
}
