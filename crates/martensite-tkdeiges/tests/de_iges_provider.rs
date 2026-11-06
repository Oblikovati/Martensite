//! The provider of IGES: shapes and documents through streams, and records the file reader takes.
#[path = "support/memory_stream.rs"]
mod memory_stream;

use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeProvider, DeProviderReadStreamList, DeProviderReadStreamNode, DeProviderWriteStreamList,
    DeProviderWriteStreamNode,
};
use martensite_tkdeiges::de_iges::{DeIgesConfigurationNode, DeIgesProvider};
use martensite_tkdeiges::iges_data::IgesDataIgesModel;
use martensite_tkdeiges::iges_file::{iges_file_read, iges_file_read_stream_with};
use martensite_tkdeiges::iges_select::IgesSelectWorkLibrary;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::osd::{IStream, StreamAccess};
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocLayerTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use memory_stream::{memory_istream, memory_ostream};
use std::sync::{Mutex, MutexGuard, PoisonError};

/// The settings of the format are process-wide.
static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

fn locked() -> MutexGuard<'static, ()> {
    ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner)
}

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

fn new_provider() -> DeIgesProvider {
    DeIgesProvider::with_node(&Handle::new(DeIgesConfigurationNode::default()).cast())
}

fn new_document() -> Handle<TDocStdDocument> {
    let application = XCAFAppApplication::get_application();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("a document");
    document
}

fn new_box() -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0).expect("the sizes");
    maker.shape().expect("a box").clone()
}

fn count(shape: &Shape, kind: ShapeEnum) -> i32 {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    let mut total = 0;
    while explorer.more() {
        total += 1;
        explorer.next().expect("the next one");
    }
    total
}

/// Whether the text is made of records of 80 characters of all five sections.
fn is_valid_iges(content: &[u8]) -> bool {
    let mut found = [false; 5];
    for line in content.split(|byte| *byte == b'\n') {
        if line.is_empty() {
            continue;
        }
        let record = line.strip_suffix(b"\r").unwrap_or(line);
        if record.len() != 80 {
            return false;
        }
        match b"SGDPT".iter().position(|section| *section == record[72]) {
            Some(rank) => found[rank] = true,
            None => return false,
        }
    }
    found.iter().all(|section| *section)
}

fn write_shape(provider: &DeIgesProvider, shape: &Shape) -> Vec<u8> {
    let (mut stream, written) = memory_ostream();
    let mut list = DeProviderWriteStreamList::default();
    list.append(DeProviderWriteStreamNode::new(
        &AsciiString::new("shape.igs"),
        &mut stream,
    ));
    assert!(
        provider
            .write_shape_streams(&mut list, shape, &range())
            .expect("the writing")
    );
    written.contents()
}

fn read_shape(provider: &DeIgesProvider, content: &[u8]) -> Option<Shape> {
    let mut stream = memory_istream(content);
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new("shape.igs"),
        &mut stream,
    ));
    let mut shape = Shape::default();
    provider
        .read_shape_streams(&mut list, &mut shape, &range())
        .expect("the reading")
        .then_some(shape)
}

fn write_document(provider: &DeIgesProvider, document: &Handle<TDocStdDocument>) -> Vec<u8> {
    let (mut stream, written) = memory_ostream();
    let mut list = DeProviderWriteStreamList::default();
    list.append(DeProviderWriteStreamNode::new(
        &AsciiString::new("document.igs"),
        &mut stream,
    ));
    assert!(
        provider
            .write_document_streams(&mut list, document, &range())
            .expect("the writing")
    );
    written.contents()
}

fn read_document(provider: &DeIgesProvider, content: &[u8]) -> Handle<TDocStdDocument> {
    let document = new_document();
    let mut stream = memory_istream(content);
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new("document.igs"),
        &mut stream,
    ));
    assert!(
        provider
            .read_document_streams(&mut list, &document, &range())
            .expect("the reading")
    );
    document
}

fn read_model(content: &[u8], is_fnes: bool) -> Handle<IgesDataIgesModel> {
    let model = IgesDataIgesModel::new();
    let protocol = IgesSelectWorkLibrary::define_protocol();
    let mut bytes = content;
    let status = iges_file_read_stream_with(
        "memory.igs",
        &mut bytes,
        &model,
        &protocol,
        &Handle::default(),
        is_fnes,
    )
    .expect("the reading");
    assert_eq!(status, 0);
    model
}

fn main_label(document: &Handle<TDocStdDocument>) -> TdfLabel {
    document
        .get()
        .expect("a document")
        .main()
        .expect("its main label")
}

#[test]
fn format_vendor_and_node_are_known() {
    let provider = new_provider();
    assert_eq!(provider.get_format().as_bytes(), b"IGES");
    assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
    assert!(provider.get_node().is_some());
}

#[test]
fn a_shape_goes_through_streams() {
    let _guard = locked();
    let provider = new_provider();
    let content = write_shape(&provider, &new_box());
    assert!(is_valid_iges(&content));
    let shape = read_shape(&provider, &content).expect("a shape");
    assert!(!shape.is_null());
    assert_eq!(count(&shape, ShapeEnum::Face), 6);
}

#[test]
fn a_document_goes_through_streams() {
    let _guard = locked();
    let provider = new_provider();
    let document = new_document();
    let tool = XCAFDocDocumentTool::shape_tool(&main_label(&document)).expect("the tool");
    let live = tool.get().expect("a tool");
    let label = live.add_shape(&new_box(), false, true).expect("the label");
    assert!(!label.is_null());
    let content = write_document(&provider, &document);
    assert!(is_valid_iges(&content));
    let read = read_document(&provider, &content);
    let tool = XCAFDocDocumentTool::shape_tool(&main_label(&read)).expect("the tool");
    assert_eq!(
        tool.get()
            .expect("a tool")
            .get_shapes()
            .expect("the labels")
            .len(),
        1
    );
}

#[test]
fn colors_names_and_layers_come_back_in_a_document() {
    let _guard = locked();
    let provider = new_provider();
    let document = new_document();
    let main = main_label(&document);
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("the tool");
    let label = tool
        .get()
        .expect("a tool")
        .add_shape(&new_box(), false, true)
        .expect("the label");
    let red = ColorRgba::from_rgb(Color::named(NamedColor::RED), 1.0);
    let colors: Handle<XCAFDocColorTool> = XCAFDocDocumentTool::color_tool(&main).expect("colors");
    colors
        .get()
        .expect("a tool")
        .set_color(&label, &red, XCAFDocColorType::ColorGen)
        .expect("set");
    let _ = TDataStdName::set(&label, &ExtendedString::new("Bracket")).expect("the name");
    let layers: Handle<XCAFDocLayerTool> = XCAFDocDocumentTool::layer_tool(&main).expect("layers");
    let name = ExtendedString::new("7");
    layers
        .get()
        .expect("a tool")
        .set_layer(&label, &name, false)
        .expect("the layer");

    let content = write_document(&provider, &document);
    let read = read_document(&provider, &content);
    let main = main_label(&read);
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("the tool");
    let free = tool
        .get()
        .expect("a tool")
        .get_free_shapes()
        .expect("the labels");
    assert_eq!(free.len(), 1);
    let mut shape = Shape::default();
    assert!(XCAFDocShapeTool::get_shape(&free[0], &mut shape).expect("its shape"));
    assert_eq!(count(&shape, ShapeEnum::Face), 6);
    let colors = XCAFDocDocumentTool::color_tool(&main).expect("colors");
    let colors = colors.get().expect("a tool");
    let mut found_label = TdfLabel::default();
    assert!(
        colors
            .find_color(&red, &mut found_label)
            .expect("the search"),
        "the color did not come back"
    );
    let layers = XCAFDocDocumentTool::layer_tool(&main).expect("layers");
    let layers = layers.get().expect("a tool");
    let labels = layers.get_layer_labels().expect("their labels");
    let names: Vec<String> = labels
        .iter()
        .filter_map(|label| layers.get_layer(label).expect("a layer"))
        .map(|name| String::from_utf16_lossy(name.as_utf16()))
        .collect();
    assert_eq!(names, ["7"]);
    assert!(
        String::from_utf8_lossy(&content).contains("Bracket"),
        "the name is not in the file"
    );
    let mut attribute = Handle::<TDataStdName>::default();
    assert!(
        free[0]
            .find_attribute_as(&TDataStdName::get_id(), &mut attribute)
            .expect("find"),
        "the name did not come back"
    );
    let value = attribute.get().expect("a name").get();
    assert_eq!(String::from_utf16_lossy(value.as_utf16()).trim(), "Bracket");
}

#[test]
fn a_cr_separates_records() {
    let _guard = locked();
    let provider = new_provider();
    let content = write_shape(&provider, &new_box());
    let with_cr: Vec<u8> = content
        .iter()
        .filter(|byte| **byte != b'\r')
        .map(|byte| if *byte == b'\n' { b'\r' } else { *byte })
        .collect();
    assert!(read_shape(&provider, &with_cr).is_some_and(|shape| !shape.is_null()));
}

#[test]
fn a_cr_inside_a_fixed_record_keeps_the_start_record() {
    let _guard = locked();
    let provider = new_provider();
    let mut content = write_shape(&provider, &new_box());
    assert_eq!(content[72], b'S');
    let start = format!("{:<45}", "Legacy fixed-width Start record");
    content[..45].copy_from_slice(start.as_bytes());
    content[45] = b'\r';
    let model = read_model(&content, false);
    let live = model.get().expect("a model");
    assert_eq!(live.nb_start_lines(), 1);
    assert_eq!(live.nb_entities(), 49);
}

#[test]
fn an_empty_file_name_does_not_open() {
    let model = IgesDataIgesModel::new();
    let protocol = IgesSelectWorkLibrary::define_protocol();
    assert_eq!(
        iges_file_read("", &model, &protocol).expect("the status"),
        -1
    );
}

#[test]
fn records_without_delimiters_read() {
    let _guard = locked();
    let provider = new_provider();
    let content = write_shape(&provider, &new_box());
    let fixed: Vec<u8> = content
        .iter()
        .copied()
        .filter(|byte| *byte != b'\r' && *byte != b'\n')
        .collect();
    assert!(read_shape(&provider, &fixed).is_some_and(|shape| !shape.is_null()));
}

#[test]
fn a_shifted_exponent_record_keeps_its_value() {
    let _guard = locked();
    let provider = new_provider();
    let mut content = write_shape(&provider, &new_box());
    let value = ".123D-4";
    let start = format!("{value:<72}");
    content[..72].copy_from_slice(start.as_bytes());
    assert_eq!(content[72], b'S');
    content.remove(0);
    let model = read_model(&content, false);
    let live = model.get().expect("a model");
    assert_eq!(live.nb_start_lines(), 1);
    assert_eq!(live.start_line(1), value.as_bytes());
}

#[test]
fn a_plain_preamble_is_skipped() {
    let _guard = locked();
    let content = write_shape(&new_provider(), &new_box());
    let mut with_preamble = b"FNES compatibility preamble\n".to_vec();
    with_preamble.extend_from_slice(&content);
    let model = read_model(&with_preamble, false);
    assert_eq!(model.get().expect("a model").nb_entities(), 49);
}

#[test]
fn a_long_coded_preamble_is_one_physical_line() {
    let _guard = locked();
    let content = write_shape(&new_provider(), &new_box());
    let mut with_preamble = vec![b'X'; 96];
    with_preamble.push(b'\n');
    with_preamble.extend_from_slice(&content);
    let model = read_model(&with_preamble, true);
    assert_eq!(model.get().expect("a model").nb_entities(), 49);
}

#[test]
fn a_failed_stream_reads_nothing() {
    let provider = new_provider();
    let mut stream = memory_istream(b"");
    let _ = stream.read(&mut [0u8; 1]);
    assert!(stream.fail());
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new("failed.igs"),
        &mut stream,
    ));
    let mut shape = Shape::default();
    assert!(
        !provider
            .read_shape_streams(&mut list, &mut shape, &range())
            .expect("the status")
    );
    assert!(shape.is_null());
}
