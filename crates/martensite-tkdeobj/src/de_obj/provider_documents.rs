//! The document transfers of `DeObjProvider`.
use super::provider_shapes::{send_fail, send_warning, validated_node};
use super::{DeObjConfigurationNode, DeObjProvider};
use crate::rw_obj::{RwObjCafReader, RwObjCafWriter};
use martensite_tkde::de::DeValidationUtils;
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::IndexedDataMap,
    standard::{Failure, Handle},
    tcollection::AsciiString,
    units_methods::LengthUnit,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkrwmesh::rw_mesh::{RwMeshCafReader as _, RwMeshCoordinateSystemConverter};
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

fn path_text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

fn configured_reader(
    node: &DeObjConfigurationNode,
    document: &Handle<TDocStdDocument>,
) -> Result<RwObjCafReader, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let length_unit = node
        .configuration_node_base()
        .global_parameters()
        .length_unit;
    let mut reader = RwObjCafReader::new();
    reader.set_single_precision(parameters.read_single_precision);
    reader.set_system_length_unit(length_unit / 1000.0);
    reader.set_system_coordinate_system_kind(parameters.system_cs);
    reader.set_file_length_unit(parameters.file_length_unit);
    reader.set_file_coordinate_system_kind(parameters.file_cs);
    reader.set_document(document)?;
    reader.set_root_prefix(&parameters.read_root_prefix);
    reader.set_memory_limit_mib(parameters.read_memory_limit_mib);
    Ok(reader)
}

/// Reads a file into a document and gives the document the configured length unit.
pub(super) fn read_file(
    provider: &DeObjProvider,
    path: &AsciiString,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let context = format!("reading the file {}", path_text(path));
    if !DeValidationUtils::validate_document(document, &AsciiString::new(&context), true) {
        return Ok(false);
    }
    let Some(node) = validated_node(provider, &context) else {
        return Ok(false);
    };
    let Some(node) = node.get() else {
        return Ok(false);
    };
    let mut reader = configured_reader(node, document)?;
    if !reader.perform(path, progress)? {
        send_fail(&format!(
            "Error in the DeObjConfigurationNode during reading the file {}",
            path_text(path)
        ));
        return Ok(false);
    }
    let length_unit = node
        .configuration_node_base()
        .global_parameters()
        .length_unit;
    XCAFDocDocumentTool::set_length_unit_in(document, length_unit, LengthUnit::Millimeter)?;
    Ok(true)
}

fn file_information(node: &DeObjConfigurationNode) -> IndexedDataMap<AsciiString, AsciiString> {
    let parameters = node.internal_parameters.snapshot();
    let mut information = IndexedDataMap::default();
    if !parameters.write_author.is_empty() {
        information.add(AsciiString::new("Author"), parameters.write_author);
    }
    if !parameters.write_comment.is_empty() {
        information.add(AsciiString::new("Comments"), parameters.write_comment);
    }
    information
}

fn writing_converter(
    node: &DeObjConfigurationNode,
    document: &Handle<TDocStdDocument>,
    path: &AsciiString,
) -> Result<RwMeshCoordinateSystemConverter, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let global = node.configuration_node_base().global_parameters();
    let mut converter = RwMeshCoordinateSystemConverter::new();
    let mut scale = 1.0;
    if XCAFDocDocumentTool::get_length_unit_in(document, &mut scale, LengthUnit::Millimeter)? {
        converter.set_input_length_unit(scale / 1000.0);
    } else {
        converter.set_input_length_unit(global.system_unit / 1000.0);
        send_warning(&format!(
            "Warning in the DeObjProvider during writing the file {}\t: The document has no information on Units. Using global parameter as initial Unit.",
            path_text(path)
        ));
    }
    converter.set_input_coordinate_system_kind(parameters.system_cs);
    converter.set_output_length_unit(global.length_unit / 1000.0);
    converter.set_output_coordinate_system_kind(parameters.file_cs);
    Ok(converter)
}

/// Writes a document to a file.
pub(super) fn write_file(
    provider: &DeObjProvider,
    path: &AsciiString,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let context = format!("writing the file {}", path_text(path));
    let Some(node) = validated_node(provider, &context) else {
        return Ok(false);
    };
    let Some(node) = node.get() else {
        return Ok(false);
    };
    let converter = writing_converter(node, document, path)?;
    let mut writer = RwObjCafWriter::new(path);
    writer.set_coordinate_system_converter(&converter);
    if !writer.perform(document, &file_information(node), progress)? {
        send_fail(&format!(
            "Error in the DeObjConfigurationNode during writing the file {}",
            path_text(path)
        ));
        return Ok(false);
    }
    Ok(true)
}
