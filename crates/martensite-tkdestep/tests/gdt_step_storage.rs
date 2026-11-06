//! The cases of `storage`: GD&T, names in code pages, assemblies, sharing and other data of a
//! document through a round trip in AP242.
#[path = "support/memory_stream.rs"]
mod memory_stream;

use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Shape};
use martensite_tkde::de::{
    DeProvider, DeProviderReadStreamList, DeProviderReadStreamNode, DeProviderWriteStreamList,
    DeProviderWriteStreamNode, new_configuration_node,
};
use martensite_tkdestep::de_step::{
    DeStepConfigurationNode, DeStepParameters, DeStepProvider, DeStepRWModeTessellated,
    DeStepWriteModeAssembly, DeStepWriteModeStepSchema, StepControlStepModelType,
};
use martensite_tkdestep::stepcaf_control::{STEPCAFControlReader, STEPCAFControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::resource::{FormatType, ResourceUnicode};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis2, Direction3, Line3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkmesh::brep_mesh::{BRepMeshDiscretRoot, BRepMeshIncrementalMesh};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex,
    BRepBuilderAPITransform,
};
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumSingleModif, XCAFDimTolObjectsDimensionObject,
    XCAFDimTolObjectsDimensionType, XCAFDimTolObjectsGeomToleranceType,
    XCAFDimTolObjectsGeomToleranceTypeValue,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDatum, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocGeomTolerance,
    XCAFDocMaterialFields, XCAFDocShapeTool,
};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use memory_stream::{memory_istream, memory_ostream};
use std::sync::{Mutex, MutexGuard, PoisonError};

/// The write actor belongs to the controller, of which a process has one:, where each case
/// has the process alone, one case at a time.
fn alone() -> MutexGuard<'static, ()> {
    static ONE_CASE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_CASE_AT_A_TIME
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

fn name(value: &str) -> ExtendedString {
    ExtendedString::from_utf8(value)
}

fn new_document() -> Handle<TDocStdDocument> {
    let application = XCAFAppApplication::get_application();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&name("BinXCAF"), &mut document)
        .expect("document");
    document
}

fn main_of(document: &Handle<TDocStdDocument>) -> TdfLabel {
    document.get().expect("document").main().expect("main")
}

fn shape_tool(document: &Handle<TDocStdDocument>) -> Handle<XCAFDocShapeTool> {
    XCAFDocDocumentTool::shape_tool(&main_of(document)).expect("tool")
}

fn a_box(dx: f64, dy: f64, dz: f64) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)
        .expect("box")
        .solid()
        .expect("solid")
        .into_shape()
}

fn box_between(first: Point3, second: Point3) -> Shape {
    BRepPrimAPIMakeBox::from_points(first, second)
        .expect("box")
        .solid()
        .expect("solid")
        .into_shape()
}

fn add_shape(document: &Handle<TDocStdDocument>, shape: &Shape) -> TdfLabel {
    let label = shape_tool(document)
        .add_shape(shape, true, true)
        .expect("label");
    assert!(!label.is_null());
    label
}

fn set_name(label: &TdfLabel, value: &ExtendedString) {
    assert!(TDataStdName::set(label, value).expect("name").is_some());
}

fn name_of(label: &TdfLabel) -> Option<ExtendedString> {
    let mut attribute: Handle<TDataStdName> = Handle::default();
    label
        .find_attribute_as(&TDataStdName::get_id(), &mut attribute)
        .expect("find")
        .then(|| attribute.get().expect("name").get())
}

fn dimension_attribute(label: &TdfLabel) -> Handle<XCAFDocDimension> {
    let mut attribute: Handle<XCAFDocDimension> = Handle::default();
    assert!(
        label
            .find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)
            .expect("find")
    );
    attribute
}

/// `AddBoxAndDimension`: the label of a dimension of a new box.
fn add_box_and_dimension(document: &Handle<TDocStdDocument>) -> TdfLabel {
    let shape = add_shape(document, &a_box(10.0, 10.0, 10.0));
    let tool = XCAFDocDocumentTool::dim_tol_tool(&main_of(document)).expect("tool");
    let dimension = tool.add_dimension().expect("dimension");
    tool.set_dimension(&shape, &dimension).expect("set");
    dimension
}

/// `RestoreDimension`: the object of the first dimension of a document.
fn restore_dimension(
    document: &Handle<TDocStdDocument>,
) -> Handle<XCAFDimTolObjectsDimensionObject> {
    let tool = XCAFDocDocumentTool::dim_tol_tool(&main_of(document)).expect("tool");
    let labels = tool.get_dimension_labels().expect("labels");
    let label = labels.first().expect("a dimension");
    dimension_attribute(label)
        .get()
        .expect("attribute")
        .get_object()
        .expect("object")
}

fn write_stream(
    provider: &DeStepProvider,
    key: &str,
    document: &Handle<TDocStdDocument>,
) -> Option<Vec<u8>> {
    let (mut stream, written) = memory_ostream();
    let mut list = DeProviderWriteStreamList::default();
    list.append(DeProviderWriteStreamNode::new(
        &AsciiString::new(key),
        &mut stream,
    ));
    provider
        .write_document_streams(&mut list, document, &range())
        .expect("write")
        .then(|| written.contents())
}

fn read_stream(
    provider: &DeStepProvider,
    key: &str,
    content: &[u8],
    document: &Handle<TDocStdDocument>,
) -> bool {
    let mut stream = memory_istream(content);
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new(key),
        &mut stream,
    ));
    provider
        .read_document_streams(&mut list, document, &range())
        .expect("read")
}

/// A provider writing in AP242, after the settings.
fn ap242_provider(settings: impl FnOnce(&mut DeStepParameters)) -> DeStepProvider {
    let node = new_configuration_node(DeStepConfigurationNode::default());
    {
        let mut parameters = node.get().expect("node").internal_parameters.write();
        parameters.write_schema = DeStepWriteModeStepSchema::AP242_DIS;
        settings(&mut parameters);
    }
    DeStepProvider::with_node(&node.cast())
}

/// `RoundTrip`.
fn round_trip_with(
    document: &Handle<TDocStdDocument>,
    write_nonmanifold: bool,
) -> Handle<TDocStdDocument> {
    let provider = ap242_provider(|parameters| parameters.write_nonmanifold = write_nonmanifold);
    let content = write_stream(&provider, "gdt.step", document)
        .expect("DeStepProvider failed to write the GD&T stream");
    assert!(!content.is_empty());
    let restored = new_document();
    assert!(
        read_stream(&provider, "gdt.step", &content, &restored),
        "DeStepProvider failed to read the GD&T stream"
    );
    restored
}

fn round_trip(document: &Handle<TDocStdDocument>) -> Handle<TDocStdDocument> {
    round_trip_with(document, false)
}

/// `EXPECT_DOUBLE_EQ`: four units in the last place.
fn assert_double_eq(actual: f64, expected: f64) {
    let (a, b) = (actual.to_bits() as i64, expected.to_bits() as i64);
    assert!(
        actual == expected || (a - b).abs() <= 4,
        "{actual} is not {expected}"
    );
}

fn assert_point(actual: Point3, expected: Point3) {
    assert_double_eq(actual.x(), expected.x());
    assert_double_eq(actual.y(), expected.y());
    assert_double_eq(actual.z(), expected.z());
}

fn assert_direction(actual: Direction3, expected: Direction3) {
    assert_double_eq(actual.x(), expected.x());
    assert_double_eq(actual.y(), expected.y());
    assert_double_eq(actual.z(), expected.z());
}

#[test]
fn a7_dimension_descriptions() {
    let _alone = alone();
    let document = new_document();
    let attribute = dimension_attribute(&add_box_and_dimension(&document));
    let attribute = attribute.get().expect("attribute");
    let object = attribute.get_object().expect("object");
    let live = object.get().expect("object");
    live.set_type(XCAFDimTolObjectsDimensionType::SizeCurveLength);
    live.set_value(10.0);
    live.add_description(&text("description1"), &text(""));
    live.add_description(&text("description2"), &text("name2"));
    attribute.set_object(&object).expect("set");
    let restored = restore_dimension(&round_trip(&document));
    let restored = restored.get().expect("object");
    assert_eq!(
        restored.get_type(),
        XCAFDimTolObjectsDimensionType::SizeCurveLength
    );
    assert_double_eq(restored.get_value(), 10.0);
    assert_eq!(restored.nb_descriptions(), 2);
    let string_of =
        |value: Handle<HAsciiString>| value.get().expect("text").string().as_bytes().to_vec();
    assert_eq!(string_of(restored.get_description(0)), b"description1");
    assert_eq!(string_of(restored.get_description_name(0)), b"");
    assert_eq!(string_of(restored.get_description(1)), b"description2");
    assert_eq!(string_of(restored.get_description_name(1)), b"name2");
}

#[test]
fn a8_dimension_annotation_plane() {
    let _alone = alone();
    let document = new_document();
    let attribute = dimension_attribute(&add_box_and_dimension(&document));
    let attribute = attribute.get().expect("attribute");
    let object = attribute.get_object().expect("object");
    let live = object.get().expect("object");
    live.set_type(XCAFDimTolObjectsDimensionType::SizeCurveLength);
    live.set_value(10.0);
    let point = Point3::new(-5.0, 0.0, 7.0);
    let normal = Direction3::new(-1.0, 0.0, 0.0).expect("normal");
    let x_direction = Direction3::new(0.0, 1.0, 0.0).expect("x");
    live.set_plane(&Axis2::with_x_direction(point, normal, x_direction).expect("plane"));
    live.set_point_text_attach(&point);
    let edge = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(-5.0, 0.0, 0.0),
        Point3::new(-5.0, 0.0, 10.0),
    )
    .expect("edge");
    live.set_presentation(
        &edge.edge().expect("edge").into_shape(),
        &text("presentation"),
    );
    attribute.set_object(&object).expect("set");
    let restored = restore_dimension(&round_trip(&document));
    let restored = restored.get().expect("object");
    assert_double_eq(restored.get_value(), 10.0);
    assert!(restored.has_plane());
    assert_point(restored.get_plane().location(), point);
    assert_direction(restored.get_plane().direction(), normal);
    assert_direction(restored.get_plane().x_direction(), x_direction);
    assert!(restored.has_text_point());
    assert_point(restored.get_point_text_attach(), point);
}

#[test]
fn a9_dimension_connection_points() {
    let _alone = alone();
    let document = new_document();
    let attribute = dimension_attribute(&add_box_and_dimension(&document));
    let attribute = attribute.get().expect("attribute");
    let object = attribute.get_object().expect("object");
    let live = object.get().expect("object");
    live.set_type(XCAFDimTolObjectsDimensionType::LocationLinearDistance);
    live.set_value(10.0);
    let (first, second) = (Point3::new(1.0, 0.0, 0.0), Point3::new(9.0, 0.0, 0.0));
    live.set_point(&first);
    live.set_point2(&second);
    attribute.set_object(&object).expect("set");
    let restored = restore_dimension(&round_trip(&document));
    let restored = restored.get().expect("object");
    assert_double_eq(restored.get_value(), 10.0);
    assert!(restored.has_point());
    assert!(restored.has_point2());
    assert_point(restored.get_point(), first);
    assert_point(restored.get_point2(), second);
}

fn step_special_name() -> ExtendedString {
    name("a'''\\b\n\t\\c\\\\\\\\")
}

fn restored_first_shape_name(document: &Handle<TDocStdDocument>) -> Option<ExtendedString> {
    let labels = shape_tool(document).get_shapes().expect("shapes");
    name_of(labels.first()?)
}

#[test]
fn step_bug_32310_special_name() {
    let _alone = alone();
    let document = new_document();
    let label = add_shape(&document, &a_box(1.0, 1.0, 1.0));
    let expected = step_special_name();
    set_name(&label, &expected);
    assert_eq!(
        restored_first_shape_name(&round_trip(&document)),
        Some(expected)
    );
}

/// `MakeStepNameTemplate`: a written file with `@tmp_name@` as the name of a face.
fn make_step_name_template() -> Vec<u8> {
    let document = new_document();
    let the_box = a_box(1.0, 2.0, 3.0);
    let tool = shape_tool(&document);
    let label = tool.add_shape(&the_box, false, true).expect("label");
    assert!(!label.is_null());
    let faces = Explorer::new(&the_box, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    assert!(faces.more());
    let face_label = tool
        .add_sub_shape(&label, faces.current())
        .expect("sub-shape");
    assert!(!face_label.is_null());
    set_name(&face_label, &name("@tmp_name@"));
    let provider = ap242_provider(|parameters| {
        parameters.write_name = true;
        parameters.write_subshape_names = true;
    });
    write_stream(&provider, "codepage_template.step", &document).expect("template")
}

/// `EncodeStepName`: the bytes of a name in a format; a code page is encoded character by character,
/// with room for the character and its terminator.
fn encode_step_name(value: &ExtendedString, format: FormatType) -> Vec<u8> {
    if format != FormatType::Utf8 && format != FormatType::Gb {
        let mut result = Vec::new();
        for unit in value.as_utf16().iter().copied() {
            if unit <= 0x7F {
                result.push(unit as u8);
                continue;
            }
            let mut buffer = [0u8; 2];
            let single = ExtendedString::from_utf16(&[unit]);
            assert!(
                ResourceUnicode::convert_unicode_to_format(format, &single, &mut buffer)
                    .expect("encode"),
                "{format:?}: U+{unit:04X}"
            );
            result.push(buffer[0]);
        }
        return result;
    }
    let mut buffer = vec![0u8; 4 * value.as_utf16().len() + 1];
    assert!(
        ResourceUnicode::convert_unicode_to_format(format, value, &mut buffer).expect("encode")
    );
    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(buffer.len());
    buffer.truncate(end);
    buffer
}

/// `ReplaceStepName`: the template with the encoded name in place of `@tmp_name@`.
fn replace_step_name(template: &[u8], value: &ExtendedString, format: FormatType) -> Vec<u8> {
    let placeholder = b"@tmp_name@";
    let position = template
        .windows(placeholder.len())
        .position(|window| window == placeholder)
        .expect("the placeholder");
    let encoded = encode_step_name(value, format);
    assert!(!encoded.is_empty());
    let mut result = template[..position].to_vec();
    result.extend_from_slice(&encoded);
    result.extend_from_slice(&template[position + placeholder.len()..]);
    result
}

/// `FindFirstLabelName`.
fn first_label_name(label: &TdfLabel) -> Option<ExtendedString> {
    if let Some(found) = name_of(label) {
        return Some(found);
    }
    let mut children = TdfChildIterator::new(label, false).expect("iterator");
    while children.more() {
        if let Some(found) = first_label_name(&children.value()) {
            if !found.is_empty() {
                return Some(found);
            }
        }
        children.next();
    }
    None
}

/// `FindFirstSubShapeName`.
fn first_sub_shape_name(document: &Handle<TDocStdDocument>) -> ExtendedString {
    for shape in shape_tool(document).get_shapes().expect("shapes") {
        let mut subs = Vec::new();
        XCAFDocShapeTool::get_sub_shapes(&shape, &mut subs).expect("subs");
        for sub in &subs {
            if let Some(found) = first_label_name(sub).filter(|found| !found.is_empty()) {
                return found;
            }
        }
    }
    ExtendedString::default()
}

/// `ReadStepName`.
fn read_step_name(payload: &[u8], format: FormatType) -> ExtendedString {
    let node = new_configuration_node(DeStepConfigurationNode::default());
    {
        let mut parameters = node.get().expect("node").internal_parameters.write();
        parameters.read_code_page = format as i32;
        parameters.read_name = true;
        parameters.read_subshape_names = true;
    }
    let provider = DeStepProvider::with_node(&node.cast());
    let document = new_document();
    if !read_stream(&provider, "codepage_test.step", payload, &document) {
        return ExtendedString::default();
    }
    first_sub_shape_name(&document)
}

/// `ExpectStepCodePageName`.
fn assert_code_page_name(template: &[u8], expected: &str, format: FormatType) {
    let expected = name(expected);
    let utf8 = replace_step_name(template, &expected, FormatType::Utf8);
    let target = replace_step_name(template, &expected, format);
    assert!(
        read_step_name(&utf8, FormatType::Utf8) == expected,
        "{format:?}: UTF-8"
    );
    assert!(
        read_step_name(&target, format) == expected,
        "{format:?}: its code page"
    );
    assert!(
        read_step_name(&target, FormatType::Utf8) != expected,
        "{format:?}: read as UTF-8"
    );
}

#[test]
fn step_bug_28454_1_iso8859_names() {
    let _alone = alone();
    let template = make_step_name_template();
    for (format, expected) in [
        (FormatType::Iso8859_1, "Test Prob\u{00ED}h\u{00E1}"),
        (FormatType::Iso8859_2, "Test Prob\u{00ED}h\u{00E1}"),
        (FormatType::Iso8859_3, "Test \u{00D6}l\u{00E7}ek"),
        (FormatType::Iso8859_4, "Test m\u{0113}\u{0123}iniet"),
        (
            FormatType::Iso8859_5,
            "Test \u{0422}\u{0435}\u{0441}\u{0442}",
        ),
        (
            FormatType::Iso8859_6,
            "Test \u{0627}\u{062e}\u{062a}\u{0628}\u{0627}\u{0631}",
        ),
        (
            FormatType::Iso8859_7,
            "Test \u{03B4}\u{03C0}\u{03A8}\u{03AE}",
        ),
        (
            FormatType::Iso8859_8,
            "Test \u{00B1}\u{05E4}\u{05DE}\u{05DC}\u{05E9}",
        ),
        (
            FormatType::Iso8859_9,
            "Test \u{011F}\u{015F}\u{011E}\u{015E}\u{00C6}",
        ),
    ] {
        assert_code_page_name(&template, expected, format);
    }
}

#[test]
fn step_bug_30694_gb2312_name() {
    let _alone = alone();
    assert_code_page_name(
        &make_step_name_template(),
        "\u{56FD}\u{6807}",
        FormatType::Gb,
    );
}

#[test]
fn step_bug_31670_1_cp125x_names() {
    let _alone = alone();
    let template = make_step_name_template();
    for (format, expected) in [
        (FormatType::Cp1250, "Test Prob\u{00ED}h\u{00E1}"),
        (FormatType::Cp1251, "Test \u{0422}\u{0435}\u{0441}\u{0442}"),
        (FormatType::Cp1252, "Test Prob\u{00ED}h\u{00E1}"),
        (
            FormatType::Cp1253,
            "Test \u{0394}\u{03BF}\u{03BA}\u{03B9}\u{03BC}\u{03AE}",
        ),
        (FormatType::Cp1254, "Test \u{00D6}l\u{00E7}ek"),
        (
            FormatType::Cp1255,
            "Test \u{05DE}\u{05B4}\u{05D1}\u{05B0}\u{05D7}\u{05B8}\u{05DF}",
        ),
        (
            FormatType::Cp1256,
            "Test \u{0627}\u{062E}\u{062A}\u{0628}\u{0627}\u{0631}",
        ),
        (FormatType::Cp1257, "Test P\u{0101}rbaude"),
        (FormatType::Cp1258, "Test Th\u{00ED} nghi\u{00EA}\u{0323}m"),
    ] {
        assert_code_page_name(&template, expected, format);
    }
}

#[test]
fn step_bug_31923_cp850_name() {
    let _alone = alone();
    assert_code_page_name(
        &make_step_name_template(),
        "\u{00DC}berpr\u{00FC}fung de codifica\u{00E7}\u{00E3}o",
        FormatType::Cp850,
    );
}

#[test]
fn xde_bug_22728_write_stream() {
    let _alone = alone();
    let document = new_document();
    add_shape(&document, &a_box(10.0, 10.0, 10.0));
    let content =
        write_stream(&ap242_provider(|_| {}), "draw_step.stream", &document).expect("written");
    let content = String::from_utf8_lossy(&content);
    assert!(!content.is_empty());
    assert!(content.contains("ISO-10303-21;"));
    assert!(content.contains("MANIFOLD_SOLID_BREP"));
}

#[test]
fn step_bug_33815_nonmanifold_special_name() {
    let _alone = alone();
    let document = new_document();
    let label = add_shape(&document, &a_box(1.0, 1.0, 1.0));
    let expected = step_special_name();
    set_name(&label, &expected);
    assert_eq!(
        restored_first_shape_name(&round_trip_with(&document, true)),
        Some(expected)
    );
}

/// `ExpectStepEdgeExport`.
fn assert_edge_export(edge: &Shape) {
    let document = new_document();
    add_shape(&document, edge);
    let content =
        write_stream(&ap242_provider(|_| {}), "untrimmed_edge.step", &document).expect("written");
    assert!(!content.is_empty());
    assert!(String::from_utf8_lossy(&content).contains("LINE"));
}

fn line_through_origin(x: f64, y: f64) -> Line3 {
    Line3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(x, y, 0.0).expect("direction"),
    )
}

#[test]
fn step_bug_32817_1_untrimmed_line() {
    let _alone = alone();
    let edge = BRepBuilderAPIMakeEdge::from_lin(line_through_origin(0.0, 1.0)).expect("edge");
    assert_edge_export(&edge.edge().expect("edge").into_shape());
}

#[test]
fn step_bug_32817_2_huge_end_parameter() {
    let _alone = alone();
    let edge =
        BRepBuilderAPIMakeEdge::from_lin_params(line_through_origin(0.0, 1.0), 10.0, 2.0e100)
            .expect("edge");
    assert_edge_export(&edge.edge().expect("edge").into_shape());
}

#[test]
fn step_bug_32817_3_huge_start_parameter() {
    let _alone = alone();
    let edge =
        BRepBuilderAPIMakeEdge::from_lin_params(line_through_origin(1.0, 1.0), -2.0e100, 10.0)
            .expect("edge");
    assert_edge_export(&edge.edge().expect("edge").into_shape());
}

#[test]
fn xde_bug_25910_zero_density_material() {
    let _alone = alone();
    let document = new_document();
    let label = add_shape(&document, &a_box(10.0, 10.0, 10.0));
    let tool = XCAFDocDocumentTool::material_tool(&main_of(&document)).expect("tool");
    let fields = XCAFDocMaterialFields {
        name: text("Vacuum"),
        description: text(""),
        density: 0.0,
        dens_name: text("density measure"),
        dens_val_type: text("POSITIVE_RATIO_MEASURE"),
    };
    tool.set_material(&label, &fields).expect("material");
    let content =
        write_stream(&ap242_provider(|_| {}), "zero_density.step", &document).expect("written");
    assert!(!content.is_empty());
}

#[test]
fn step_bug_27313_pmi_export() {
    let _alone = alone();
    let document = new_document();
    let label = add_shape(&document, &a_box(10.0, 10.0, 10.0));
    let tool = XCAFDocDocumentTool::dim_tol_tool(&main_of(&document)).expect("tool");
    let tolerance_label = tool.add_geom_tolerance().expect("tolerance");
    tool.set_geom_tolerance(&label, &tolerance_label)
        .expect("set");
    let datum_label = tool.add_datum().expect("datum");
    tool.set_datum_shapes(std::slice::from_ref(&label), &datum_label)
        .expect("set");
    let mut datum: Handle<XCAFDocDatum> = Handle::default();
    assert!(
        datum_label
            .find_attribute_as(&XCAFDocDatum::get_id(), &mut datum)
            .expect("find")
    );
    let datum = datum.get().expect("datum");
    let datum_object = datum.get_object().expect("object");
    let live = datum_object.get().expect("object");
    live.set_name(&text("A"));
    live.set_position(1);
    live.add_modifier(XCAFDimTolObjectsDatumSingleModif::DistanceVariable);
    datum.set_object(&datum_object).expect("set");
    tool.set_datum_to_geom_tol(&datum_label, &tolerance_label)
        .expect("set");
    let mut tolerance: Handle<XCAFDocGeomTolerance> = Handle::default();
    assert!(
        tolerance_label
            .find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut tolerance)
            .expect("find")
    );
    let tolerance = tolerance.get().expect("tolerance");
    let tolerance_object = tolerance.get_object().expect("object");
    let live = tolerance_object.get().expect("object");
    live.set_value(0.5);
    live.set_type(XCAFDimTolObjectsGeomToleranceType::Angularity);
    live.set_type_of_value(XCAFDimTolObjectsGeomToleranceTypeValue::Diameter);
    tolerance.set_object(&tolerance_object).expect("set");
    let content =
        write_stream(&ap242_provider(|_| {}), "draw_step.stream", &document).expect("written");
    assert!(!content.is_empty());
}

/// `BRepBuilderAPITransform(shape, translation)`.
fn translated(shape: &Shape, x: f64, y: f64, z: f64) -> Shape {
    let mut moved = BRepBuilderAPITransform::with_shape(
        shape,
        Transform3::translation([x, y, z]),
        false,
        false,
    )
    .expect("transform");
    moved.shape().expect("shape").clone()
}

fn compound_of(shapes: &[&Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut compound: Shape = compound.into();
    for shape in shapes {
        builder.add(&mut compound, shape).expect("add");
    }
    compound
}

fn components_of(label: &TdfLabel) -> Vec<TdfLabel> {
    let mut components = Vec::new();
    assert!(XCAFDocShapeTool::get_components(label, &mut components, false).expect("components"));
    components
}

#[test]
fn modalg_bug_33165_assembly_instance_names() {
    let _alone = alone();
    let document = new_document();
    let first = a_box(1.0, 1.0, 1.0);
    let second = translated(&first, 1.0, 1.0, 1.0);
    let assembly = add_shape(&document, &compound_of(&[&first, &second]));
    let components = components_of(&assembly);
    assert_eq!(components.len(), 2);
    set_name(&components[0], &name("name1"));
    set_name(&components[1], &name("name2"));
    let restored = round_trip(&document);
    let assemblies = shape_tool(&restored).get_free_shapes().expect("free");
    assert_eq!(assemblies.len(), 1);
    let components = components_of(&assemblies[0]);
    assert_eq!(components.len(), 2);
    assert_eq!(name_of(&components[0]), Some(name("name1")));
    assert_eq!(name_of(&components[1]), Some(name("name2")));
}

/// `MakeLocatedBoxAssembly`.
fn located_box_assembly(count: i32) -> Shape {
    let the_box = a_box(1.0, 2.0, 3.0);
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut compound: Shape = compound.into();
    for index in 0..count {
        for sub_index in 0..count {
            let transform =
                Transform3::translation([5.0 * f64::from(index), 0.5 * f64::from(sub_index), 0.0]);
            let mut located = the_box.clone();
            located
                .set_location(&Location::new(transform), true)
                .expect("location");
            builder.add(&mut compound, &located).expect("add");
        }
    }
    compound
}

#[test]
fn xde_bug_7141_large_assembly_export() {
    let _alone = alone();
    let document = new_document();
    let assembly = add_shape(&document, &located_box_assembly(20));
    assert_eq!(components_of(&assembly).len(), 400);
    let provider =
        ap242_provider(|parameters| parameters.write_assembly = DeStepWriteModeAssembly::ON);
    let content = write_stream(&provider, "bug7141.step", &document).expect("written");
    let content = String::from_utf8_lossy(&content);
    assert!(!content.is_empty());
    assert!(content.contains("ISO-10303-21;"));
    assert!(content.contains("NEXT_ASSEMBLY_USAGE_OCCURRENCE"));
    assert!(content.contains("MANIFOLD_SOLID_BREP"));
}

fn nb_users(label: &TdfLabel) -> i32 {
    XCAFDocShapeTool::get_users(label, &mut Vec::new(), false).expect("users")
}

#[test]
fn step_bug_30189_2_located_root_structure() {
    let _alone = alone();
    let document = new_document();
    let the_box = a_box(1.0, 1.0, 1.0);
    let second = box_between(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0));
    let third = box_between(Point3::new(4.0, 0.0, 0.0), Point3::new(5.0, 1.0, 1.0));
    let compound = compound_of(&[&the_box, &second]);
    let located = translated(&compound, 0.0, 2.0, 0.0);
    let root = compound_of(&[&third, &compound]);
    add_shape(&document, &root);
    add_shape(&document, &located);
    let shapes = shape_tool(&document).get_shapes().expect("shapes");
    assert_eq!(shapes.len(), 6);
    assert_eq!(nb_users(&shapes[2]), 2);
    let restored = round_trip(&document);
    let shapes = shape_tool(&restored).get_shapes().expect("shapes");
    assert_eq!(shapes.len(), 6);
    assert_eq!(nb_users(&shapes[2]), 2);
    assert_eq!(
        XCAFDocShapeTool::nb_components(&shapes[5], false).expect("components"),
        1
    );
}

/// `FindLabelByName`.
fn has_label_named(label: &TdfLabel, expected: &ExtendedString) -> bool {
    if name_of(label).as_ref() == Some(expected) {
        return true;
    }
    let mut children = TdfChildIterator::new(label, false).expect("iterator");
    while children.more() {
        if has_label_named(&children.value(), expected) {
            return true;
        }
        children.next();
    }
    false
}

#[test]
fn xde_bug_1669_nested_assembly_names() {
    let _alone = alone();
    let document = new_document();
    let box1 = translated(&a_box(10.0, 10.0, 10.0), 10.0, 0.0, 0.0);
    let box2 = translated(&a_box(20.0, 10.0, 10.0), 20.0, 0.0, 0.0);
    let block = compound_of(&[&box1, &box2]);
    let next_block = translated(&block, 30.0, 0.0, 0.0);
    let row = compound_of(&[&block, &next_block]);
    let half_brick = translated(&row, -10.0, 0.0, 10.0);
    let brick = translated(&row, 0.0, 0.0, 20.0);
    let wall = compound_of(&[&row, &half_brick, &brick]);
    add_shape(&document, &wall);
    let shapes = shape_tool(&document).get_shapes().expect("shapes");
    assert_eq!(shapes.len(), 5);
    for (shape, value) in shapes
        .iter()
        .zip(["Wall", "Block-Array", "Block", "Half-Brick", "Brick"])
    {
        set_name(shape, &name(value));
    }
    for (shape, names) in [
        (&shapes[0], &["base-array", "next-array", "top-array"][..]),
        (&shapes[1], &["left-block", "right-block"][..]),
        (&shapes[2], &["half-brick", "brick"][..]),
    ] {
        let components = components_of(shape);
        assert_eq!(components.len(), names.len());
        for (component, value) in components.iter().zip(names) {
            set_name(component, &name(value));
        }
    }
    let restored = main_of(&round_trip(&document));
    for value in [
        "Wall",
        "Block-Array",
        "Block",
        "Half-Brick",
        "Brick",
        "base-array",
        "next-array",
        "top-array",
        "left-block",
        "right-block",
        "half-brick",
        "brick",
    ] {
        assert!(has_label_named(&restored, &name(value)), "{value}");
    }
}

/// `ShapeDiagonal`.
fn shape_diagonal(shape: &Shape) -> f64 {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true).expect("bounds");
    bounds
        .corner_min()
        .expect("min")
        .distance(&bounds.corner_max().expect("max"))
}

#[test]
fn step_bug_ocp1949_2_tessellated_scaling() {
    let _alone = alone();
    let the_box = a_box(1.0, 1.0, 1.0);
    let mesh = BRepMeshIncrementalMesh::with_shape(&the_box, 1.0, false, 0.5, false).expect("mesh");
    assert!(mesh.is_done());
    let document = new_document();
    XCAFDocDocumentTool::set_length_unit(&document, 1.0).expect("unit");
    add_shape(&document, &the_box);
    let provider =
        ap242_provider(|parameters| parameters.write_tessellated = DeStepRWModeTessellated::ON);
    let content = write_stream(&provider, "ocp1949_2.step", &document).expect("written");
    assert!(!content.is_empty());
    assert!(String::from_utf8_lossy(&content).contains("TESSELLATED_SHAPE_REPRESENTATION"));
    let restored = new_document();
    assert!(read_stream(
        &provider,
        "ocp1949_2.step",
        &content,
        &restored
    ));
    let restored_shape = shape_tool(&restored).get_one_shape().expect("shape");
    assert!(!restored_shape.is_null());
    assert!((shape_diagonal(&the_box) * 1000.0 - shape_diagonal(&restored_shape)).abs() <= 0.1);
}

#[test]
fn step_bug_30189_3_located_root_sharing() {
    let _alone = alone();
    let document = new_document();
    let base = a_box(1.0, 1.0, 1.0);
    let base_label = add_shape(&document, &base);
    let reference1 = add_shape(&document, &translated(&base, 2.0, 0.0, 0.0));
    let reference2 = add_shape(&document, &translated(&base, 0.0, 2.0, 0.0));
    let reference3 = add_shape(&document, &translated(&base, 0.0, 0.0, 2.0));
    set_name(&base_label, &name("box"));
    set_name(&reference1, &name("ref1_box"));
    set_name(&reference2, &name("ref2_box"));
    set_name(&reference3, &name("ref3_box"));
    assert_eq!(shape_tool(&document).get_shapes().expect("shapes").len(), 4);
    assert_eq!(nb_users(&base_label), 3);
    let restored = round_trip(&document);
    let shapes = shape_tool(&restored).get_shapes().expect("shapes");
    assert_eq!(shapes.len(), 4);
    let named = |value: &str| {
        shapes
            .iter()
            .find(|label| name_of(label) == Some(name(value)))
            .cloned()
    };
    let restored_base = named("box").expect("box");
    assert!(named("ref1_box").is_some());
    assert!(named("ref2_box").is_some());
    assert!(named("ref3_box").is_some());
    assert_eq!(nb_users(&restored_base), 3);
}

/// `FindFirstVertex`.
fn first_vertex(label: &TdfLabel) -> Option<Shape> {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape).expect("shape");
    if !shape.is_null() {
        let vertices =
            Explorer::new(&shape, ShapeEnum::Vertex, ShapeEnum::Shape).expect("explorer");
        if vertices.more() {
            return Some(vertices.current().clone());
        }
    }
    let mut children = TdfChildIterator::new(label, false).expect("iterator");
    while children.more() {
        if let Some(found) = first_vertex(&children.value()) {
            return Some(found);
        }
        children.next();
    }
    None
}

#[test]
fn xde_bug_13175_single_vertex() {
    let _alone = alone();
    let document = new_document();
    let tool = shape_tool(&document);
    let label = tool.new_shape().expect("label");
    let mut maker = BRepBuilderAPIMakeVertex::new(Point3::new(10.0, 10.0, 10.0)).expect("vertex");
    tool.set_shape(&label, &maker.shape().expect("shape").clone())
        .expect("shape");
    // `RoundTripWithSTEPCAF`
    let mut parameters = DeStepParameters::default();
    parameters.write_schema = DeStepWriteModeStepSchema::AP242_DIS;
    let mut writer = STEPCAFControlWriter::new().expect("writer");
    assert!(
        writer
            .transfer_document_with(
                &document,
                &parameters,
                StepControlStepModelType::AS_IS,
                None,
                &range()
            )
            .expect("transfer")
    );
    let mut content = Vec::new();
    assert_eq!(
        writer.write_stream(&mut content).expect("write"),
        IFSelectReturnStatus::RetDone
    );
    assert!(!content.is_empty());
    let mut reader = STEPCAFControlReader::new().expect("reader");
    assert_eq!(
        reader
            .read_stream("stepcaf_vertex.step", &mut content.as_slice())
            .expect("read"),
        IFSelectReturnStatus::RetDone
    );
    let restored = new_document();
    assert!(reader.transfer(&restored, &range()).expect("transfer"));
    let shapes = shape_tool(&restored).get_shapes().expect("shapes");
    assert_eq!(shapes.len(), 1);
    let vertex = first_vertex(&shapes[0]).expect("a vertex");
    assert_eq!(vertex.shape_type().expect("type"), ShapeEnum::Vertex);
    let point = BRepTool::pnt(&topods::vertex(&vertex).expect("vertex")).expect("point");
    assert_double_eq(point.x(), 10.0);
    assert_double_eq(point.y(), 10.0);
    assert_double_eq(point.z(), 10.0);
}

#[test]
fn xde_bug_31851_unicode_shape_name() {
    let _alone = alone();
    let document = new_document();
    let label = add_shape(&document, &a_box(10.0, 10.0, 10.0));
    let expected = name("\u{30DC}\u{30C3}\u{30AF}\u{30B9}");
    set_name(&label, &expected);
    assert_eq!(
        restored_first_shape_name(&round_trip(&document)),
        Some(expected)
    );
}

/// A transfer that gives no entity, `WriteStep D file f` on a box: the faceted translation raises inside the
/// transfer process and leaves an empty model. The session has no graph then, so the data modes write nothing,
/// the transfer still answers true and `Write` is `RetVoid` ("no file written").
fn writes_no_file_when_the_faceted_mode_gives_no_entity(clean_duplicates: bool) {
    let document = new_document();
    add_shape(&document, &a_box(1.0, 1.0, 1.0));
    let mut writer = STEPCAFControlWriter::new().expect("a writer");
    writer.set_clean_duplicates(clean_duplicates);
    let transferred = writer
        .transfer_document(
            &document,
            StepControlStepModelType::FACETED_BREP,
            None,
            &ProgressRange::default(),
        )
        .expect("the transfer answers");
    assert!(transferred);
    let output = std::env::temp_dir().join(format!(
        "martensite-no-file-{clean_duplicates}-{}.stp",
        std::process::id()
    ));
    let status = writer
        .write(output.to_str().expect("a path"))
        .expect("the write answers");
    assert_eq!(status, IFSelectReturnStatus::RetVoid);
    assert!(!output.exists());
}

#[test]
fn a_transfer_without_entities_writes_no_file() {
    writes_no_file_when_the_faceted_mode_gives_no_entity(false);
}

#[test]
fn a_transfer_without_entities_writes_no_file_when_cleaning_duplicates() {
    writes_no_file_when_the_faceted_mode_gives_no_entity(true);
}
