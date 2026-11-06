//! Reads files of the test dataset into a document through `STEPCAFControlReader`. The expected
//! counts are recorded.
use martensite_tkdestep::stepcaf_control::{STEPCAFControlController, STEPCAFControlReader};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocArea, XCAFDocCentroid, XCAFDocDocumentTool, XCAFDocVolume,
};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Counts {
    shapes: usize,
    free: usize,
    colors: usize,
    layers: usize,
    dims: usize,
    tols: usize,
    datums: usize,
    mats: usize,
    views: usize,
    names: usize,
    areas: usize,
    vols: usize,
    centroids: usize,
}

fn dataset(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step")
        .join(name);
    path.to_string_lossy().into_owned()
}

/// The labels under `label`, at all levels, with the attribute.
fn count_attributes(label: &TdfLabel, id: &martensite_tkernel::standard::Guid) -> usize {
    let mut count = 0;
    let mut children = TdfChildIterator::new(label, true).expect("iterator");
    while children.more() {
        if children.value().is_attribute(id).expect("attribute") {
            count += 1;
        }
        children.next();
    }
    count
}

fn read_counts(name: &str) -> Counts {
    STEPCAFControlController::init().expect("controller");
    let application = XCAFAppApplication::get_application();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("document");
    let mut reader = STEPCAFControlReader::new().expect("reader");
    assert!(
        reader
            .perform(&dataset(name), &document, &ProgressRange::default())
            .expect("perform"),
        "{name}"
    );
    let main = document.get().expect("document").main().expect("main");
    let shape_tool = XCAFDocDocumentTool::shape_tool(&main).expect("tool");
    let color_tool = XCAFDocDocumentTool::color_tool(&main).expect("tool");
    let layer_tool = XCAFDocDocumentTool::layer_tool(&main).expect("tool");
    let dim_tol_tool = XCAFDocDocumentTool::dim_tol_tool(&main).expect("tool");
    let material_tool = XCAFDocDocumentTool::material_tool(&main).expect("tool");
    let view_tool = XCAFDocDocumentTool::view_tool(&main).expect("tool");
    Counts {
        shapes: shape_tool.get_shapes().expect("shapes").len(),
        free: shape_tool.get_free_shapes().expect("free").len(),
        colors: color_tool.get_colors().expect("colors").len(),
        layers: layer_tool.get_layer_labels().expect("layers").len(),
        dims: dim_tol_tool.get_dimension_labels().expect("dims").len(),
        tols: dim_tol_tool
            .get_geom_tolerance_labels()
            .expect("tols")
            .len(),
        datums: dim_tol_tool.get_datum_labels().expect("datums").len(),
        mats: material_tool.get_material_labels().expect("mats").len(),
        views: view_tool.get_view_labels().expect("views").len(),
        names: count_attributes(&main.root().expect("root"), &TDataStdName::get_id()),
        areas: count_attributes(&main, &XCAFDocArea::get_id()),
        vols: count_attributes(&main, &XCAFDocVolume::get_id()),
        centroids: count_attributes(&main, &XCAFDocCentroid::get_id()),
    }
}

/// Reads a file and compares its counts with `expected`, in the order of `counts`.
fn check(name: &str, expected: [usize; 13]) {
    let [
        shapes,
        free,
        colors,
        layers,
        dims,
        tols,
        datums,
        mats,
        views,
        names,
        areas,
        vols,
        centroids,
    ] = expected;
    let wanted = Counts {
        shapes,
        free,
        colors,
        layers,
        dims,
        tols,
        datums,
        mats,
        views,
        names,
        areas,
        vols,
        centroids,
    };
    assert_eq!(read_counts(name), wanted, "{name}");
}

#[test]
fn a_layer_on_a_simple_shape() {
    check(
        "bug27333_simple.stp",
        [1, 1, 0, 1, 0, 0, 0, 0, 0, 10, 0, 0, 0],
    );
}

#[test]
fn colours_and_layers_on_one_shape() {
    check("brep_wv1.stp", [1, 1, 1, 1, 0, 0, 0, 0, 0, 11, 0, 0, 0]);
}

#[test]
fn an_assembly_with_colours_materials_and_validation_properties() {
    check(
        "as1-oc-214-mat.stp",
        [9, 1, 5, 1, 0, 0, 0, 3, 0, 39, 9, 9, 9],
    );
}

#[test]
fn an_assembly_with_colours_and_validation_properties() {
    check(
        "bug21802_as1-oc-214.stp",
        [9, 1, 5, 1, 0, 0, 0, 0, 0, 36, 9, 9, 9],
    );
}

#[test]
fn datums_of_the_first_version() {
    check(
        "trj12_gd1-id-214.stp",
        [1, 1, 2, 0, 0, 0, 3, 0, 0, 16, 1, 1, 1],
    );
}

#[test]
fn layers_on_a_part() {
    check("bm2_pe_t4-A.stp", [1, 1, 0, 3, 0, 0, 0, 0, 0, 12, 0, 0, 0]);
}

#[test]
fn a_small_assembly_with_colours() {
    check(
        "trj7_c2-ec-214.stp",
        [3, 1, 2, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0],
    );
}

#[test]
fn colours_on_a_tessellated_assembly() {
    check("bug33638.stp", [2, 1, 2, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0]);
}

#[test]
fn a_reference_to_an_external_document_that_is_not_there() {
    check("xr1-db-214_1.stp", [1, 1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0]);
}
