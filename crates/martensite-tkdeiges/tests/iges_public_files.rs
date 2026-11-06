//! Every public IGES file of the test data: its shapes, written back in both modes, and its document.
#[path = "support/public_file_path.rs"]
mod public_file_path;
#[path = "support/public_file_results.rs"]
mod public_file_results;

use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkdeiges::igescaf_control::IgesCafControlReader;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use public_file_path::public_file_path;
use public_file_results::{PUBLIC_FILES, PublicFile};
use std::path::Path;
use std::sync::{Mutex, PoisonError};

/// How far an area may be, relatively, from the one of the table.
const AREA_TOLERANCE: f64 = 1.0e-5;

type Row = PublicFile;

fn failure(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

fn read_shape(path: &str) -> Result<Shape, String> {
    let mut reader = IgesControlReader::new().map_err(failure)?;
    let status = reader.read_file(path).map_err(failure)?;
    if status != IFSelectReturnStatus::RetDone {
        return Err(format!("status {status:?}"));
    }
    reader
        .transfer_roots(&ProgressRange::default())
        .map_err(failure)?;
    Ok(reader.xs_reader().one_shape())
}

fn faces(shape: &Shape) -> Result<i32, String> {
    if shape.is_null() {
        return Ok(0);
    }
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).map_err(failure)?;
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().map_err(failure)?;
    }
    Ok(found)
}

fn area(shape: &Shape) -> Result<f64, String> {
    if shape.is_null() {
        return Ok(0.0);
    }
    let mut properties = GProps::new();
    BRepGProp::surface_properties(shape, &mut properties, false, false).map_err(failure)?;
    Ok(properties.mass())
}

fn check_shape(shape: &Shape, rank: usize, row: &Row) -> Result<(), String> {
    let (found, measured) = (faces(shape)?, area(shape)?);
    if found != row.1[rank] {
        return Err(format!(
            "step {rank}: {found} faces, expected {}",
            row.1[rank]
        ));
    }
    let expected = row.2[rank];
    if (measured - expected).abs() > AREA_TOLERANCE * expected.abs() {
        return Err(format!("step {rank}: area {measured}, expected {expected}"));
    }
    Ok(())
}

fn written_back(shape: &Shape, path: &Path, mode: i32) -> Result<Shape, String> {
    let mut writer = IgesControlWriter::with_unit(b"MM", mode).map_err(failure)?;
    let done = writer
        .add_shape(shape, &ProgressRange::default())
        .map_err(failure)?;
    let stored = writer
        .write_file(&path.to_string_lossy(), false)
        .map_err(failure)?;
    if !(done && stored) {
        return Err(format!("mode {mode}: not written"));
    }
    read_shape(&path.to_string_lossy())
}

/// What goes wrong with the shapes of one file, if anything.
fn check_shapes(row: &Row, directory: &Path) -> Result<(), String> {
    let shape = read_shape(&public_file_path(row.0).to_string_lossy())?;
    check_shape(&shape, 0, row)?;
    if shape.is_null() {
        return Ok(());
    }
    for mode in [0, 1] {
        let written = directory.join(format!("{mode}-{}", row.0));
        check_shape(
            &written_back(&shape, &written, mode)?,
            mode as usize + 1,
            row,
        )?;
    }
    Ok(())
}

fn count_names(label: &TdfLabel) -> Result<usize, String> {
    let mut name = Handle::<TDataStdName>::default();
    let found = label
        .find_attribute_as(&TDataStdName::get_id(), &mut name)
        .map_err(failure)?;
    let mut total = usize::from(found);
    let mut children = TdfChildIterator::new(label, false).map_err(failure)?;
    while children.more() {
        total += count_names(&children.value())?;
        children.next();
    }
    Ok(total)
}

/// What goes wrong with the document of one file, if anything.
fn check_document(row: &Row) -> Result<(), String> {
    let application = XCAFAppApplication::get_application();
    let mut document = Handle::<TDocStdDocument>::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .map_err(failure)?;
    let mut reader = IgesCafControlReader::new().map_err(failure)?;
    let path = public_file_path(row.0);
    let range = ProgressRange::default();
    reader
        .perform(&path.to_string_lossy(), &document, &range)
        .map_err(failure)?;
    let main = document
        .get()
        .ok_or("no document")?
        .main()
        .map_err(failure)?;
    let shapes = XCAFDocDocumentTool::shape_tool(&main).map_err(failure)?;
    let shapes = shapes.get().ok_or("no shapes")?;
    let colors = XCAFDocDocumentTool::color_tool(&main).map_err(failure)?;
    let layers = XCAFDocDocumentTool::layer_tool(&main).map_err(failure)?;
    let found = [
        shapes.get_shapes().map_err(failure)?.len(),
        shapes.get_free_shapes().map_err(failure)?.len(),
        colors
            .get()
            .ok_or("no colors")?
            .get_colors()
            .map_err(failure)?
            .len(),
        layers
            .get()
            .ok_or("no layers")?
            .get_layer_labels()
            .map_err(failure)?
            .len(),
        count_names(&main.root().map_err(failure)?)?,
    ];
    if found == row.3 {
        Ok(())
    } else {
        Err(format!("document {found:?}, expected {:?}", row.3))
    }
}

/// Runs a check over the rows, one transfer at a time: the actor of the format is process-wide.
fn on_every_file(check: impl Fn(&Row) -> Result<(), String>) {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    let _guard = ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner);
    assert!(PUBLIC_FILES.len() >= 50, "{} files", PUBLIC_FILES.len());
    let problems: Vec<String> = PUBLIC_FILES
        .iter()
        .filter_map(|row| {
            check(row)
                .err()
                .map(|problem| format!("{}: {problem}", row.0))
        })
        .collect();
    assert!(problems.is_empty(), "{problems:#?}");
}

#[test]
fn every_public_file_reads_and_comes_back_the_same() {
    let directory = std::env::temp_dir().join(format!("iges-public-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("a directory");
    on_every_file(|row| check_shapes(row, &directory));
}

#[test]
fn every_public_file_fills_a_document() {
    on_every_file(check_document);
}
