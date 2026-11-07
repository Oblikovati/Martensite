//! The provider: settings reaching the file, shapes through a document, and what it refuses.
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeProvider as _, DeWorkSession,
    new_configuration_node,
};
use martensite_tkdeply::de_ply::{DePlyConfigurationNode, DePlyProvider};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{BaseAllocator, Buffer};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use std::path::PathBuf;

pub(crate) fn file_in_temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("tkdeply-{}-provider-{name}", std::process::id()))
}

pub(crate) fn name_of(path: &std::path::Path) -> AsciiString {
    AsciiString::new(path.to_str().unwrap())
}

fn meshed_box() -> Shape {
    let (low, high) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
    let solid: Shape = BRepPrimAPIMakeBox::from_points(low, high)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    BRepMeshIncrementalMesh::with_shape(&solid, 0.1, false, 0.5, false).unwrap();
    solid
}

pub(crate) fn node_and_provider() -> (Handle<DePlyConfigurationNode>, DePlyProvider) {
    let node = new_configuration_node(DePlyConfigurationNode::new());
    let provider = DePlyProvider::with_node(&node.clone().cast::<dyn DeConfigurationNode>());
    (node, provider)
}

pub(crate) fn take_text(path: &std::path::Path) -> String {
    let text = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    text
}

#[test]
fn a_shape_is_written_with_the_settings_of_the_node() {
    let (node, provider) = node_and_provider();
    {
        let live = node.get().unwrap();
        let mut parameters = live.internal_parameters.write();
        parameters.write_author = AsciiString::new("Someone");
        parameters.write_comment = AsciiString::new("Text");
        parameters.write_normals = false;
        parameters.write_colors = false;
        parameters.write_tex_coords = true;
        parameters.write_part_id = false;
        parameters.write_face_id = true;
    }
    let path = file_in_temp("settings.ply");
    assert!(
        provider
            .write_shape(&name_of(&path), &meshed_box(), &ProgressRange::default())
            .unwrap()
    );
    let text = take_text(&path);
    assert!(
        text.contains("comment Author:comment Someone\ncomment Comments:comment Text\n"),
        "{text}"
    );
    assert!(
        !text.contains("nx") && !text.contains("red") && text.contains("property float s\n"),
        "{text}"
    );
    assert!(text.contains("property uint SurfaceID\n"), "{text}");
    assert!(text.lines().last().unwrap().ends_with(" 6"), "{text}");
}

#[test]
fn a_session_is_accepted_and_ignored() {
    let (_node, provider) = node_and_provider();
    let (shape_path, document_path) = (
        file_in_temp("session_shape.ply"),
        file_in_temp("session_doc.ply"),
    );
    let progress = ProgressRange::default();
    let mut session = DeWorkSession::default();
    assert!(
        provider
            .write_shape_with_session(
                &name_of(&shape_path),
                &meshed_box(),
                &mut session,
                &progress
            )
            .unwrap()
    );
    let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap();
    let tool = XCAFDocDocumentTool::shape_tool(&document.get().unwrap().main().unwrap()).unwrap();
    tool.get()
        .unwrap()
        .add_shape(&meshed_box(), true, true)
        .unwrap();
    assert!(
        provider
            .write_document_with_session(
                &name_of(&document_path),
                &document,
                &mut session,
                &progress
            )
            .unwrap()
    );
    assert_eq!(
        take_text(&shape_path),
        take_text(&document_path).replace("session_doc", "session_shape")
    );
}

#[test]
fn an_empty_document_succeeds_with_no_file_and_a_shape_without_mesh_fails() {
    let (_node, provider) = node_and_provider();
    let progress = ProgressRange::default();
    let path = file_in_temp("empty.ply");
    let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap();
    assert!(
        provider
            .write_document(&name_of(&path), &document, &progress)
            .unwrap()
    );
    assert!(!path.exists());
    assert!(
        provider
            .write_document(&name_of(&path), &Handle::default(), &progress)
            .is_err()
    );
    let unmeshed: Shape =
        BRepPrimAPIMakeBox::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0))
            .unwrap()
            .solid()
            .unwrap()
            .into_shape();
    assert!(
        !provider
            .write_shape(&name_of(&path), &unmeshed, &progress)
            .unwrap()
    );
}

#[test]
fn a_provider_without_this_node_refuses_and_reading_is_not_supported() {
    let provider = DePlyProvider::new();
    let progress = ProgressRange::default();
    let path = file_in_temp("refused.ply");
    assert!(
        !provider
            .write_shape(&name_of(&path), &meshed_box(), &progress)
            .unwrap()
    );
    assert!(!path.exists());
    let (_node, provider) = node_and_provider();
    let mut shape = Shape::default();
    assert!(
        !provider
            .read_shape(&name_of(&path), &mut shape, &progress)
            .unwrap()
    );
    assert_eq!(provider.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(provider.get_format(), AsciiString::new("PLY"));
}

#[test]
fn the_node_of_the_framework_builds_providers_on_itself() {
    let node = new_configuration_node(DePlyConfigurationNode::new());
    let dynamic: Handle<dyn DeConfigurationNode> = node.clone().cast();
    let live = dynamic.get().unwrap();
    assert!(live.is_export_supported() && !live.is_import_supported());
    assert_eq!(live.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(live.get_format(), AsciiString::new("PLY"));
    assert!(
        live.get_extensions()
            .iter()
            .any(|text| text == &AsciiString::new("ply"))
    );
    let provider = live.build_provider();
    assert_eq!(
        provider.get().unwrap().get_vendor(),
        AsciiString::new("Martensite")
    );
    let resource = DeConfigurationContext::new();
    assert!(
        resource
            .load_str(&"provider.PLY.Martensite.write.author : Loaded\n".into())
            .unwrap()
    );
    assert!(live.load(&Handle::new(resource)).unwrap());
    assert_eq!(
        node.get()
            .unwrap()
            .internal_parameters
            .snapshot()
            .write_author,
        AsciiString::new("Loaded")
    );
    let copied = live.copy();
    live.set_enabled(false);
    assert!(copied.get().unwrap().is_enabled());
    assert!(
        String::from_utf8_lossy(live.save().as_bytes())
            .contains("provider.PLY.Martensite.write.author :\t Loaded")
    );
    assert_eq!(
        DePlyConfigurationNode::static_type().name(),
        "DePlyConfigurationNode"
    );
    let mut buffer = Buffer::new(Some(BaseAllocator::common()), 8).unwrap();
    buffer.as_mut_slice().copy_from_slice(b"ply\nrest");
    assert!(live.check_content(&Handle::new(buffer)));
    assert!(!live.check_content(&Handle::default()));
}

#[test]
fn keys_of_another_vendor_do_not_reach_the_settings() {
    let node = DePlyConfigurationNode::new();
    let resource = DeConfigurationContext::new();
    assert!(
        resource
            .load_str(&"provider.PLY.Other.write.author : Old\n".into())
            .unwrap()
    );
    assert!(node.load(&resource));
    assert!(node.internal_parameters.snapshot().write_author.is_empty());
    assert!(
        String::from_utf8_lossy(node.save().as_bytes()).contains("Vendor: Martensite Format: PLY")
    );
}

#[test]
fn provider_and_node_are_shared_between_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DePlyProvider>();
    assert_send_sync::<DePlyConfigurationNode>();
}
