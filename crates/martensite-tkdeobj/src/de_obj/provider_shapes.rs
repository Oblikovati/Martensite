//! The shape transfers of `DeObjProvider`.
use super::{DeObjConfigurationNode, DeObjProvider};
use crate::rw_obj::{RwObjReader as _, RwObjTriangulationReader};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{DeProvider, DeValidationUtils};
use martensite_tkernel::{
    message::{Message, ProgressRange},
    standard::{Failure, Handle, RuntimeTyped},
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystemConverter;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

pub(super) fn send_warning(text: &str) {
    Message::send_warning_text(&AsciiString::new(text));
}

pub(super) fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

/// The `DeObjConfigurationNode` of the provider, after the validation of its type.
pub(super) fn validated_node(
    provider: &DeObjProvider,
    context: &str,
) -> Option<Handle<DeObjConfigurationNode>> {
    let node = provider.get_node();
    let is_valid = DeValidationUtils::validate_configuration_node(
        &node,
        DeObjConfigurationNode::static_type(),
        &AsciiString::new(context),
        true,
    );
    is_valid.then(|| node.cast::<DeObjConfigurationNode>())
}

fn reading_converter(node: &DeObjConfigurationNode) -> RwMeshCoordinateSystemConverter {
    let parameters = node.internal_parameters.snapshot();
    let length_unit = node
        .configuration_node_base()
        .global_parameters()
        .length_unit;
    let mut converter = RwMeshCoordinateSystemConverter::new();
    converter.set_output_length_unit(length_unit / 1000.0);
    converter.set_output_coordinate_system_kind(parameters.system_cs);
    converter.set_input_length_unit(parameters.file_length_unit);
    converter.set_input_coordinate_system_kind(parameters.file_cs);
    converter
}

/// The configured limit in MiB as the reader's limit in bytes.
fn memory_limit_bytes(limit_mib: i32) -> usize {
    match limit_mib {
        -1 => usize::MAX,
        mib => (mib as usize).saturating_mul(1024 * 1024),
    }
}

/// Reads a file into a shape: one face, or the shapes of the file with `read.create.shapes`.
pub(super) fn read_file(
    provider: &DeObjProvider,
    path: &AsciiString,
    shape: &mut Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let name = String::from_utf8_lossy(path.as_bytes()).into_owned();
    let context = format!("reading the file {name}");
    let Some(node) = validated_node(provider, &context) else {
        return Ok(false);
    };
    let Some(node) = node.get() else {
        return Ok(false);
    };
    let parameters = node.internal_parameters.snapshot();
    let mut reader = RwObjTriangulationReader::new();
    reader.set_transformation(&reading_converter(node));
    reader.set_single_precision(parameters.read_single_precision);
    reader.set_create_shapes(parameters.read_create_shapes);
    reader.set_memory_limit(memory_limit_bytes(parameters.read_memory_limit_mib));
    if !reader.read(path, progress)? {
        send_fail(&format!(
            "Error in the DeObjConfigurationNode during reading the file {name}"
        ));
        return Ok(false);
    }
    *shape = reader.result_shape()?;
    Ok(true)
}

/// Writes a shape through a new document.
pub(super) fn write_file(
    provider: &DeObjProvider,
    path: &AsciiString,
    shape: &Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF"))?;
    let live = document.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "DeObjProvider: expected a new document",
            "",
        )
    })?;
    let tool = XCAFDocDocumentTool::shape_tool(&live.main()?)?;
    if let Some(tool) = tool.get() {
        tool.add_shape(shape, true, true)?;
    }
    provider.write_document(path, &document, progress)
}
