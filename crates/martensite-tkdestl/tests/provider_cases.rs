//! The `provider` suite, on the real STL node and provider.
mod support;

use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderReadStreamList, DeProviderReadStreamNode,
    DeProviderWriteStreamList, DeProviderWriteStreamNode, DeWrapper, new_configuration_node,
};
use martensite_tkdestl::de_stl::{DeStlConfigurationNode, DeStlProvider};
use martensite_tkernel::{
    message::ProgressRange, osd::IStream, standard::Handle, tcollection::AsciiString,
};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdDocument};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use support::memory_stream::{memory_istream, memory_ostream};
use support::shapes::{
    count_faces, count_triangles, mesh_of, new_xcaf_document, planar_face_with_mesh,
    triangulated_face,
};

fn progress() -> ProgressRange<'static> {
    ProgressRange::default()
}

/// The `set_up` of the fixture.
struct Fixture {
    node: Handle<DeStlConfigurationNode>,
    provider: DeStlProvider,
    triangular_face: Shape,
}
impl Fixture {
    fn new() -> Self {
        let node = new_configuration_node(DeStlConfigurationNode::new());
        let provider = DeStlProvider::with_node(&node.cast());
        Self {
            node,
            provider,
            triangular_face: triangulated_face(),
        }
    }
    fn node(&self) -> &DeStlConfigurationNode {
        self.node.get().unwrap()
    }
}

fn write_list<'a>(
    key: &str,
    stream: &'a mut dyn martensite_tkernel::osd::OStream,
) -> DeProviderWriteStreamList<'a> {
    let mut list = DeProviderWriteStreamList::default();
    list.append(DeProviderWriteStreamNode::new(
        &AsciiString::new(key),
        stream,
    ));
    list
}

fn read_list<'a>(key: &str, stream: &'a mut dyn IStream) -> DeProviderReadStreamList<'a> {
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new(key),
        stream,
    ));
    list
}

fn write_shape(provider: &DeStlProvider, key: &str, shape: &Shape) -> (bool, Vec<u8>) {
    let (mut stream, written) = memory_ostream();
    let mut list = write_list(key, &mut stream);
    let done = provider
        .write_shape_streams(&mut list, shape, &progress())
        .unwrap();
    drop(list);
    (done, written.contents())
}

fn read_shape(provider: &DeStlProvider, key: &str, content: &[u8]) -> (bool, Shape) {
    let mut stream = memory_istream(content);
    let mut list = read_list(key, &mut stream);
    let mut shape = Shape::default();
    let done = provider
        .read_shape_streams(&mut list, &mut shape, &progress())
        .unwrap();
    (done, shape)
}

fn wrapper_write(wrapper: &DeWrapper, key: &str, shape: &Shape) -> (bool, Vec<u8>) {
    let (mut stream, written) = memory_ostream();
    let mut list = write_list(key, &mut stream);
    let done = wrapper
        .write_shape_streams(&mut list, shape, &progress())
        .unwrap();
    drop(list);
    (done, written.contents())
}

fn wrapper_read(wrapper: &DeWrapper, key: &str, content: &[u8]) -> (bool, Shape) {
    let mut stream = memory_istream(content);
    let mut list = read_list(key, &mut stream);
    let mut shape = Shape::default();
    let done = wrapper
        .read_shape_streams(&mut list, &mut shape, &progress())
        .unwrap();
    (done, shape)
}

fn contains(content: &[u8], word: &[u8]) -> bool {
    content.windows(word.len()).any(|window| window == word)
}

fn bound_wrapper(node: &Handle<DeStlConfigurationNode>) -> DeWrapper {
    let wrapper = DeWrapper::new();
    assert!(wrapper.bind(&node.cast::<dyn DeConfigurationNode>()));
    wrapper
}

fn fresh_node() -> Handle<DeStlConfigurationNode> {
    new_configuration_node(DeStlConfigurationNode::new())
}

fn shape_tool(document: &Handle<TDocStdDocument>) -> Handle<XCAFDocShapeTool> {
    let main = document.get().unwrap().main().unwrap();
    XCAFDocDocumentTool::shape_tool(&main).unwrap()
}

fn all_labels(document: &Handle<TDocStdDocument>) -> Vec<TdfLabel> {
    shape_tool(document).get().unwrap().get_shapes().unwrap()
}

fn add_shape(document: &Handle<TDocStdDocument>, shape: &Shape) -> TdfLabel {
    shape_tool(document)
        .get()
        .unwrap()
        .add_shape(shape, false, true)
        .unwrap()
}

#[test]
fn basic_properties() {
    let fixture = Fixture::new();
    assert_eq!(fixture.provider.get_format().as_bytes(), b"STL");
    assert_eq!(fixture.provider.get_vendor().as_bytes(), b"Martensite");
    assert!(!fixture.provider.get_node().is_null());
}

#[test]
fn stream_shape_write_read() {
    let fixture = Fixture::new();
    let (done, content) = write_shape(&fixture.provider, "test.stl", &fixture.triangular_face);
    assert!(done);
    assert!(!content.is_empty());
    assert!(contains(&content, b"solid") || contains(&content, b"facet"));
    let (read, shape) = read_shape(&fixture.provider, "test.stl", &content);
    assert!(read);
    assert!(!shape.is_null());
    assert!(count_faces(&shape) > 0);
}

#[test]
fn stream_document_write_read() {
    let fixture = Fixture::new();
    let (_application, document) = new_xcaf_document();
    assert!(!add_shape(&document, &fixture.triangular_face).is_null());
    let (mut output, written) = memory_ostream();
    let mut list = write_list("document.stl", &mut output);
    assert!(
        fixture
            .provider
            .write_document_streams(&mut list, &document, &progress())
            .unwrap()
    );
    drop(list);
    let content = written.contents();
    assert!(!content.is_empty());

    let (_second_application, new_document) = new_xcaf_document();
    let mut input = memory_istream(&content);
    let mut list = read_list("document.stl", &mut input);
    assert!(
        fixture
            .provider
            .read_document_streams(&mut list, &new_document, &progress())
            .unwrap()
    );
    assert!(!all_labels(&new_document).is_empty());
}

#[test]
fn de_wrapper_integration() {
    let fixture = Fixture::new();
    let node = fresh_node();
    let wrapper = bound_wrapper(&node);
    let (done, content) = wrapper_write(&wrapper, "test.stl", &fixture.triangular_face);
    assert!(done);
    assert!(!content.is_empty());
    let (wrapper_result, wrapper_shape) = wrapper_read(&wrapper, "test.stl", &content);
    let direct_provider = DeStlProvider::with_node(&node.cast());
    let (direct_result, direct_shape) = read_shape(&direct_provider, "test.stl", &content);
    // The requirement: the wrapper works exactly as the direct provider.
    assert_eq!(wrapper_result, direct_result);
    assert_eq!(wrapper_shape.is_null(), direct_shape.is_null());
    if direct_result && !direct_shape.is_null() {
        assert!(count_faces(&direct_shape) > 0);
    }
}

#[test]
fn de_wrapper_stl_a1_shape_round_trip() {
    let fixture = Fixture::new();
    let (done, content) = write_shape(&fixture.provider, "source.stl", &fixture.triangular_face);
    assert!(done);
    let wrapper = bound_wrapper(&fresh_node());
    let (read, shape) = wrapper_read(&wrapper, "source.stl", &content);
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
    assert_eq!(count_triangles(&shape), 2);
}

#[test]
fn de_wrapper_stl_a2_document_write() {
    let fixture = Fixture::new();
    let (_application, document) = new_xcaf_document();
    assert!(!document.is_null());
    add_shape(&document, &fixture.triangular_face);
    let node = fresh_node();
    node.get().unwrap().internal_parameters.write().write_ascii = true;
    let wrapper = bound_wrapper(&node);
    let (mut output, written) = memory_ostream();
    let mut list = write_list("document.stl", &mut output);
    assert!(
        wrapper
            .write_document_streams(&mut list, &document, &progress())
            .unwrap()
    );
    drop(list);
    let content = written.contents();
    assert!(!content.is_empty());
    let provider = DeStlProvider::with_node(&fresh_node().cast());
    let (read, shape) = read_shape(&provider, "document.stl", &content);
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
}

#[test]
fn de_wrapper_stl_a3_document_read() {
    let fixture = Fixture::new();
    let node = fresh_node();
    node.get().unwrap().internal_parameters.write().write_ascii = true;
    let wrapper = bound_wrapper(&node);
    let (done, content) = wrapper_write(&wrapper, "shape.stl", &fixture.triangular_face);
    assert!(done);
    let (_application, document) = new_xcaf_document();
    let mut input = memory_istream(&content);
    let mut list = read_list("shape.stl", &mut input);
    assert!(
        wrapper
            .read_document_streams(&mut list, &document, &progress())
            .unwrap()
    );
    let labels = shape_tool(&document)
        .get()
        .unwrap()
        .get_free_shapes()
        .unwrap();
    assert_eq!(labels.len(), 1);
    let mut shape = Shape::default();
    assert!(XCAFDocShapeTool::get_shape(&labels[0], &mut shape).unwrap());
    assert!(count_faces(&shape) > 0);
}

#[test]
fn de_wrapper_stl_a4_shape_round_trip() {
    let fixture = Fixture::new();
    let (done, source) = write_shape(&fixture.provider, "source.stl", &fixture.triangular_face);
    assert!(done);
    let wrapper = bound_wrapper(&fresh_node());
    let (read, read_shape) = wrapper_read(&wrapper, "source.stl", &source);
    assert!(read);
    let (written, round_trip) = wrapper_write(&wrapper, "roundtrip.stl", &read_shape);
    assert!(written);
    let (read_again, shape) = wrapper_read(&wrapper, "roundtrip.stl", &round_trip);
    assert!(read_again);
    assert!(count_faces(&shape) > 0);
}

#[test]
fn error_handling() {
    let fixture = Fixture::new();
    let mut empty_write = DeProviderWriteStreamList::default();
    assert!(
        !fixture
            .provider
            .write_shape_streams(&mut empty_write, &fixture.triangular_face, &progress())
            .unwrap()
    );
    let mut empty_read = DeProviderReadStreamList::default();
    let mut shape = Shape::default();
    assert!(
        !fixture
            .provider
            .read_shape_streams(&mut empty_read, &mut shape, &progress())
            .unwrap()
    );

    // The test ignores this result: STL may produce empty content for a null shape.
    let (mut output, _written) = memory_ostream();
    let mut writes = write_list("null_test.stl", &mut output);
    let _ = fixture
        .provider
        .write_shape_streams(&mut writes, &Shape::default(), &progress());

    let (read, _) = read_shape(
        &fixture.provider,
        "invalid.stl",
        b"This is not valid STL content",
    );
    assert!(!read);

    let null_document: Handle<TDocStdDocument> = Handle::default();
    assert!(
        !fixture
            .provider
            .write_document_streams(&mut writes, &null_document, &progress())
            .unwrap()
    );
    assert!(
        !fixture
            .provider
            .read_document_streams(&mut empty_read, &null_document, &progress())
            .unwrap()
    );
}

#[test]
fn configuration_modes() {
    let fixture = Fixture::new();
    let parameters = &fixture.node().internal_parameters;
    parameters.write().write_ascii = true;
    assert!(parameters.read().write_ascii);
    parameters.write().write_ascii = false;
    assert!(!parameters.read().write_ascii);
    parameters.write().read_merge_angle = 45.0;
    assert_eq!(parameters.read().read_merge_angle, 45.0);
    assert_eq!(fixture.provider.get_format().as_bytes(), b"STL");
    assert_eq!(fixture.provider.get_vendor().as_bytes(), b"Martensite");
}

#[test]
fn ascii_vs_binary_modes() {
    let fixture = Fixture::new();
    fixture.node().internal_parameters.write().write_ascii = true;
    let (ascii_done, ascii) = write_shape(
        &fixture.provider,
        "ascii_test.stl",
        &fixture.triangular_face,
    );
    assert!(ascii_done);
    assert!(!ascii.is_empty());
    assert!(contains(&ascii, b"solid"));
    fixture.node().internal_parameters.write().write_ascii = false;
    let (binary_done, binary) = write_shape(
        &fixture.provider,
        "binary_test.stl",
        &fixture.triangular_face,
    );
    assert!(binary_done);
    assert!(!binary.is_empty());
    assert_ne!(ascii, binary);
}

#[test]
fn multiple_shapes_in_document() {
    let fixture = Fixture::new();
    let (_application, document) = new_xcaf_document();
    assert!(!add_shape(&document, &fixture.triangular_face).is_null());
    assert!(!add_shape(&document, &fixture.triangular_face).is_null());
    let (mut output, written) = memory_ostream();
    let mut list = write_list("multi_shapes.stl", &mut output);
    assert!(
        fixture
            .provider
            .write_document_streams(&mut list, &document, &progress())
            .unwrap()
    );
    drop(list);
    let content = written.contents();
    assert!(!content.is_empty());
    let (_second_application, new_document) = new_xcaf_document();
    let mut input = memory_istream(&content);
    let mut list = read_list("multi_shapes.stl", &mut input);
    assert!(
        fixture
            .provider
            .read_document_streams(&mut list, &new_document, &progress())
            .unwrap()
    );
    assert!(!all_labels(&new_document).is_empty());
}

#[test]
fn triangulated_face_handling() {
    let fixture = Fixture::new();
    let (done, content) = write_shape(
        &fixture.provider,
        "triangulated_face.stl",
        &fixture.triangular_face,
    );
    assert!(done);
    assert!(!content.is_empty());
    assert!(content.len() > 100);
    let (read, shape) = read_shape(&fixture.provider, "triangulated_face.stl", &content);
    assert!(read);
    assert!(!shape.is_null());
}

#[test]
fn de_wrapper_file_extensions() {
    let fixture = Fixture::new();
    let wrapper = bound_wrapper(&fresh_node());
    for key in ["test.stl", "test.STL", "mesh.stl"] {
        let (done, content) = wrapper_write(&wrapper, key, &fixture.triangular_face);
        assert!(done, "Failed to write with extension: {key}");
        assert!(!content.is_empty(), "Empty content for extension: {key}");
        let (read, shape) = wrapper_read(&wrapper, key, &content);
        assert!(read, "Failed to read with extension: {key}");
        assert!(!shape.is_null(), "Null shape read with extension: {key}");
    }
}

#[test]
fn stream_error_conditions() {
    let fixture = Fixture::new();
    let mut empty_write = DeProviderWriteStreamList::default();
    assert!(
        !fixture
            .provider
            .write_shape_streams(&mut empty_write, &fixture.triangular_face, &progress())
            .unwrap()
    );
    let mut empty_read = DeProviderReadStreamList::default();
    let mut shape = Shape::default();
    assert!(
        !fixture
            .provider
            .read_shape_streams(&mut empty_read, &mut shape, &progress())
            .unwrap()
    );
    let (corrupted, _) = read_shape(
        &fixture.provider,
        "corrupted.stl",
        b"This is not STL data at all",
    );
    assert!(!corrupted);
    let (partial, _) = read_shape(
        &fixture.provider,
        "partial.stl",
        b"solid test\n  facet normal 0 0 1\n",
    );
    assert!(!partial);
}

#[test]
fn configuration_parameter_validation() {
    let fixture = Fixture::new();
    let parameters = &fixture.node().internal_parameters;
    parameters.write().read_merge_angle = 30.0;
    assert_eq!(parameters.read().read_merge_angle, 30.0);
    parameters.write().read_merge_angle = 0.0;
    assert_eq!(parameters.read().read_merge_angle, 0.0);
    parameters.write().read_merge_angle = 90.0;
    assert_eq!(parameters.read().read_merge_angle, 90.0);
    parameters.write().read_brep = true;
    assert!(parameters.read().read_brep);
    parameters.write().read_brep = false;
    assert!(!parameters.read().read_brep);
}

#[test]
fn multiple_triangulated_faces() {
    let fixture = Fixture::new();
    let triangle = mesh_of(&[[0., 0., 0.], [5., 0., 0.], [2.5, 5., 0.]], &[[1, 2, 3]]);
    let triangle_face = planar_face_with_mesh(5., triangle);
    let (first_done, first) = write_shape(&fixture.provider, "face1.stl", &fixture.triangular_face);
    assert!(first_done);
    let (second_done, second) = write_shape(&fixture.provider, "face2.stl", &triangle_face);
    assert!(second_done);
    assert!(!first.is_empty());
    assert!(!second.is_empty());
    assert_ne!(first, second);
    let (first_read, first_shape) = read_shape(&fixture.provider, "face1.stl", &first);
    assert!(first_read);
    assert!(!first_shape.is_null());
    let (second_read, second_shape) = read_shape(&fixture.provider, "face2.stl", &second);
    assert!(second_read);
    assert!(!second_shape.is_null());
}
