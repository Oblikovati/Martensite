//! Reads dataset files through `STEPCAFControlReader` and compares each value of the document with
//! the values recorded from the same file: dimensions, tolerances, datums, views, properties, colours, layers, names.
#[path = "support/xde_values.rs"]
mod xde_values;

use martensite_tkdestep::de_step::DeStepWriteModeStepSchema;
use martensite_tkdestep::stepcaf_control::{
    STEPCAFControlController, STEPCAFControlReader, STEPCAFControlWriter,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxsbase::interface::InterfaceStatic;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};
use xde_values::{differences, dump_document};

/// Relative agreement of each number: the reference prints 12 significant digits.
const AGREEMENT: f64 = 1e-9;

fn new_document() -> Handle<TDocStdDocument> {
    STEPCAFControlController::init().expect("controller");
    let application = XCAFAppApplication::get_application();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("document");
    document
}

fn dataset_path(name: &str) -> PathBuf {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step");
    let path = ["stp", "step", "STEP"]
        .iter()
        .map(|extension| directory.join(format!("{name}.{extension}")))
        .find(|path| path.exists())
        .expect("a file of that name");
    path
}

/// Reads a file into a new document.
fn read_document(path: &str) -> (Handle<TDocStdDocument>, bool) {
    let document = new_document();
    let mut reader = STEPCAFControlReader::new().expect("reader");
    let ok = reader
        .perform(path, &document, &ProgressRange::default())
        .expect("perform");
    (document, ok)
}

fn records_of(document: &Handle<TDocStdDocument>, ok: bool) -> Vec<String> {
    let main = document.get().expect("document").main().expect("main");
    let mut records = vec![format!("OK {}", i32::from(ok))];
    records.extend(dump_document(&main));
    records
}

fn read_records(name: &str) -> Vec<String> {
    let (document, ok) = read_document(&dataset_path(name).to_string_lossy());
    records_of(&document, ok)
}

/// The write actor belongs to the controller, of which a process has one:, two writers
/// cannot translate at the same time.
static ONE_WRITER_AT_A_TIME: Mutex<()> = Mutex::new(());

/// Reads a file, writes the document in the schema, reads what was written.
fn written_records(name: &str, schema: DeStepWriteModeStepSchema) -> Vec<String> {
    let _writing = ONE_WRITER_AT_A_TIME
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let (document, _) = read_document(&dataset_path(name).to_string_lossy());
    let output = std::env::temp_dir().join(format!(
        "martensite-xde-{}-{name}-{}.stp",
        std::process::id(),
        schema.as_raw()
    ));
    InterfaceStatic::set_i_val("write.step.schema", schema.as_raw()).expect("schema");
    let mut writer = STEPCAFControlWriter::new().expect("writer");
    let written = writer
        .perform(
            &document,
            &output.to_string_lossy(),
            &ProgressRange::default(),
        )
        .expect("write");
    let (back, ok) = read_document(&output.to_string_lossy());
    if let Ok(keep) = std::env::var("XDE_VALUES_KEEP") {
        std::fs::copy(&output, format!("{keep}/{name}-{}.stp", schema.as_raw())).expect("keep");
    }
    std::fs::remove_file(&output).expect("remove");
    records_of(&back, written && ok)
}

/// Compares records with those of the fixture of that name.
fn compare(fixture: &str, here: &[String]) {
    compare_records(fixture, here, <[String]>::to_vec);
}

/// `prepare` changes the records as `here` was.
fn compare_records(fixture: &str, here: &[String], prepare: impl Fn(&[String]) -> Vec<String>) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/xde_values")
        .join(format!("{fixture}.txt"));
    let recorded: Vec<String> = std::fs::read_to_string(&path)
        .expect("fixture")
        .lines()
        .map(str::to_owned)
        .collect();
    let recorded = prepare(&recorded);
    if let Ok(out) = std::env::var("XDE_VALUES_OUT") {
        std::fs::write(format!("{out}/{fixture}.txt"), here.join("\n") + "\n").expect("write");
    }
    let name = fixture;
    let found = differences(&recorded, here, AGREEMENT);
    for (left, right) in found.iter().take(6) {
        eprintln!("recorded: {left}\nhere:   {right}");
    }
    assert!(
        found.is_empty(),
        "{name}: {} records differ of {}",
        found.len(),
        recorded.len()
    );
}

fn check(name: &str) {
    compare(name, &read_records(name));
}

/// The library writes the styles of the top-level shapes in the order of a map of shapes by
/// address, which changes from run to run, and the labels of the colours follow it. The colours are
/// compared by value, each once.
fn with_colours_by_value(records: &[String]) -> Vec<String> {
    let mut values = Vec::new();
    for record in records {
        let mut fields = record.splitn(3, ' ');
        if let (Some("COLOR"), Some(entry), Some(value)) =
            (fields.next(), fields.next(), fields.next())
        {
            values.push((format!("c{entry} "), format!("c{value} ")));
        }
    }
    let mut colours: Vec<String> = values
        .iter()
        .map(|(_, value)| format!("COLOR {value}"))
        .collect();
    colours.sort();
    let mut result = Vec::new();
    for record in records {
        if record.starts_with("COLOR ") || record.starts_with("NAME 0:1:2:") {
            continue;
        }
        let mut changed = format!("{record} ");
        for (entry, value) in &values {
            changed = changed.replace(entry, value);
        }
        result.push(changed.trim_end().to_owned());
    }
    result.extend(colours);
    result
}

fn check_written(name: &str, schema: DeStepWriteModeStepSchema) {
    compare_records(
        &format!("{name}.written-{}", schema.as_raw()),
        &with_colours_by_value(&written_records(name, schema)),
        with_colours_by_value,
    );
}

macro_rules! value_cases {
    ($($test:ident: $file:literal,)*) => { $(#[test] fn $test() { check($file); })* };
}

value_cases! {
    tolerances_datums_and_dimensions_with_views: "bug33569",
    pmi_with_a_section_plane_and_metadata: "bug32745_pmi1",
    ap242_dimensions_of_a_nist_model: "nist_ftc_08_asme1_ap242-2",
    the_tessellated_nist_model: "bug33661_nist_ftc_08_asme1_ap242-e1-tg",
    dimensions_with_views: "bug29241",
    datums_of_the_first_version: "trj12_gd1-id-214",
    views_of_an_ap203_model: "trj1_d1-tc",
    an_assembly_with_a_layer: "tr9_asd-db",
    an_assembly_with_materials: "as1-oc-214-mat",
    an_assembly_of_external_files: "trj4_xr1-tc-214",
    a_transparent_colour: "bug33100_window",
    names_in_other_alphabets_and_on_several_lines: "bug28454_directives",
}

macro_rules! written_cases {
    ($($test:ident: $file:literal, $schema:ident,)*) => {
        $(#[test] fn $test() { check_written($file, DeStepWriteModeStepSchema::$schema); })*
    };
}

written_cases! {
    written_in_ap242_tolerances_datums_and_dimensions: "bug33569", AP242_DIS,
    written_in_ap242_pmi_and_metadata: "bug32745_pmi1", AP242_DIS,
    written_in_ap242_dimensions_of_a_nist_model: "nist_ftc_08_asme1_ap242-2", AP242_DIS,
    written_in_ap214_datums_of_the_first_version: "trj12_gd1-id-214", AP214_IS,
    written_in_ap214_an_assembly_with_a_layer: "tr9_asd-db", AP214_IS,
    written_in_ap214_an_assembly_with_materials: "as1-oc-214-mat", AP214_IS,
    written_in_ap203_an_assembly_with_materials: "as1-oc-214-mat", AP203,
    written_in_ap214_a_transparent_colour: "bug33100_window", AP214_IS,
    written_in_ap214_names_in_other_alphabets: "bug28454_directives", AP214_IS,
}

/// Compares each file named by a dump in the directory `XDE_VALUES_SWEEP`; for finding
/// which files to keep as cases, as it needs the dumps of the whole dataset.
#[test]
#[ignore = "needs XDE_VALUES_SWEEP"]
fn sweep_of_recorded_dumps() {
    let directory = std::env::var("XDE_VALUES_SWEEP").expect("XDE_VALUES_SWEEP");
    let mut failed = 0;
    for dump in std::fs::read_dir(directory).expect("directory") {
        let dump = dump.expect("entry").path();
        let name = dump
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();
        let recorded: Vec<String> = std::fs::read_to_string(&dump)
            .expect("dump")
            .lines()
            .map(str::to_owned)
            .collect();
        let task = name.clone();
        let here = std::thread::spawn(move || read_records(&task)).join();
        let Ok(here) = here else {
            eprintln!("PANIC {name}");
            failed += 1;
            continue;
        };
        let found = differences(&recorded, &here, AGREEMENT);
        if !found.is_empty() {
            failed += 1;
            eprintln!("DIFF {name}: {} of {}", found.len(), recorded.len());
            for (left, right) in found.iter().take(3) {
                eprintln!("  recorded: {left}\n  here:   {right}");
            }
        }
    }
    assert_eq!(failed, 0);
}
