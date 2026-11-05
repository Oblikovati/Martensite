//! The shape transfers of `DeStlProvider`.
use super::{DeStlConfigurationNode, DeStlInternalSection, DeStlProvider};
use crate::rw_stl::{OStreamWriter, RwStl};
use crate::stl_api::{StlApiReader, StlApiWriter};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Face, Shape},
};
use martensite_tkde::de::{
    DeProvider, DeProviderReadStreamList, DeProviderWriteStreamList, DeValidationUtils,
};
use martensite_tkernel::{
    message::{Message, ProgressRange},
    standard::{Failure, Handle, RuntimeTyped},
    tcollection::AsciiString,
};
use martensite_tkmath::poly::Triangulation;
use std::f64::consts::{FRAC_PI_2, PI};

const READ_SCALING_WARNING: &str =
    "Martensite Stl reader does not support model scaling according to custom length unit";
const WRITE_SCALING_WARNING: &str =
    "Martensite Stl writer does not support model scaling according to custom length unit";

fn send_warning(text: &str) {
    Message::send_warning_text(&AsciiString::new(text));
}

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

/// The `DeStlConfigurationNode` of the provider, after the validation of its type.
pub(super) fn validated_node(
    provider: &DeStlProvider,
    context: &str,
) -> Option<Handle<DeStlConfigurationNode>> {
    let node = provider.get_node();
    let expected = DeStlConfigurationNode::static_type();
    let context = AsciiString::new(context);
    DeValidationUtils::validate_configuration_node(&node, expected, &context, true)
        .then(|| node.cast::<DeStlConfigurationNode>())
}

/// The configured angle in radians, or `none` outside [0, 90] degrees.
#[allow(clippy::manual_range_contains)] // A NaN angle passes these two comparisons.
fn checked_merge_angle(parameters: &DeStlInternalSection) -> Option<f64> {
    let angle = parameters.read_merge_angle * PI / 180.0;
    (angle == FRAC_PI_2 || !(angle < 0.0 || angle > FRAC_PI_2)).then_some(angle)
}

fn face_on_mesh(mesh: &Handle<Triangulation>, shape: &mut Shape) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let mut face = Face::default();
    builder.make_face(&mut face);
    builder.update_face_triangulation(&face, mesh, true)?;
    *shape = face.into();
    Ok(())
}

fn text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

fn parameters_of(node: &Handle<DeStlConfigurationNode>) -> Option<DeStlInternalSection> {
    node.get().map(|node| node.internal_parameters.snapshot())
}

/// Reads a shape from a path. In triangulation mode an unreadable file still
/// succeeds with a face that has no mesh: only the stream overload checks the mesh.
pub(super) fn read_file(
    provider: &DeStlProvider,
    path: &AsciiString,
    shape: &mut Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    send_warning(READ_SCALING_WARNING);
    let name = text(path);
    let Some(node) = validated_node(provider, &format!("reading the file {name}")) else {
        return Ok(false);
    };
    let Some(parameters) = parameters_of(&node) else {
        return Ok(false);
    };
    let Some(angle) = checked_merge_angle(&parameters) else {
        send_fail(&format!(
            "Error in the DeStlProvider during reading the file {name}\t: The merge angle is out of the valid range"
        ));
        return Ok(false);
    };
    if parameters.read_brep {
        return read_file_faces(&name, path, shape);
    }
    face_on_mesh(&RwStl::read_file_with_angle(path, angle, progress)?, shape)?;
    Ok(true)
}

fn read_file_faces(name: &str, path: &AsciiString, shape: &mut Shape) -> Result<bool, Failure> {
    let is_read = StlApiReader.read_file(shape, path)?;
    if !is_read {
        send_fail(&format!(
            "Error in the DeStlProvider during reading the file {name}"
        ));
    }
    Ok(is_read)
}

/// Writes a shape to a path.
pub(super) fn write_file(
    provider: &DeStlProvider,
    path: &AsciiString,
    shape: &Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    send_warning(WRITE_SCALING_WARNING);
    let name = text(path);
    let context = format!("writing the file {name}");
    let Some(writer) = configured_writer(provider, &context, &context) else {
        return Ok(false);
    };
    let is_written = writer.write_file(shape, path, progress)?;
    if !is_written {
        send_fail(&format!(
            "Error in the DeStlProvider during writing the file {name}\t: Mesh writing has been failed"
        ));
    }
    Ok(is_written)
}

/// The writer the node configures, after the node and length unit checks.
fn configured_writer(
    provider: &DeStlProvider,
    node_context: &str,
    length_context: &str,
) -> Option<StlApiWriter> {
    let node = validated_node(provider, node_context)?;
    let node = node.get()?;
    warn_length_unit(node, length_context);
    let mut writer = StlApiWriter::new();
    *writer.ascii_mode() = node.internal_parameters.read().write_ascii;
    Some(writer)
}

pub(super) fn warn_length_unit(node: &DeStlConfigurationNode, context: &str) {
    let length_unit = node
        .configuration_node_base()
        .global_parameters()
        .length_unit;
    DeValidationUtils::warn_length_unit_not_supported(
        length_unit,
        &AsciiString::new(context),
        true,
    );
}

/// The first stream is read, the others ignored.
pub(super) fn read_streams(
    provider: &DeStlProvider,
    streams: &mut DeProviderReadStreamList<'_>,
    shape: &mut Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    if !check_stream_count(streams.extent(), "reading") {
        return Ok(false);
    }
    send_warning(READ_SCALING_WARNING);
    let Some(first) = streams.first_mut() else {
        return Ok(false);
    };
    let key = text(&first.path);
    let Some(node) = validated_node(provider, &format!("reading stream {key}")) else {
        return Ok(false);
    };
    let Some(parameters) = parameters_of(&node) else {
        return Ok(false);
    };
    let Some(angle) = checked_merge_angle(&parameters) else {
        send_fail(&format!(
            "Error in the DeStlProvider during reading stream {key}: The merge angle is out of the valid range"
        ));
        return Ok(false);
    };
    if parameters.read_brep {
        return read_stream_faces(&key, &mut *first.stream, shape);
    }
    let mesh = RwStl::read_stream(&mut *first.stream, angle, progress)?;
    if mesh.is_null() {
        send_fail(&format!(
            "Error in the DeStlProvider during reading stream {key}: Failed to create triangulation"
        ));
        return Ok(false);
    }
    face_on_mesh(&mesh, shape)?;
    Ok(true)
}

fn read_stream_faces(
    key: &str,
    stream: &mut dyn martensite_tkernel::osd::IStream,
    shape: &mut Shape,
) -> Result<bool, Failure> {
    let is_read = StlApiReader.read_stream(shape, stream)?;
    if !is_read {
        send_fail(&format!(
            "Error in the DeStlProvider during reading stream {key}"
        ));
    }
    Ok(is_read)
}

/// The stream map checks: an empty list fails, more than one stream warns.
fn check_stream_count(count: i32, action: &str) -> bool {
    if count == 0 {
        send_fail("Error: DeStlProvider stream map is empty");
        return false;
    }
    if count > 1 {
        send_warning(&format!(
            "Warning: DeStlProvider received {count} streams for {action}, using only the first one"
        ));
    }
    true
}

/// The first stream is written, the others ignored.
pub(super) fn write_streams(
    provider: &DeStlProvider,
    streams: &mut DeProviderWriteStreamList<'_>,
    shape: &Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    if !check_stream_count(streams.extent(), "writing") {
        return Ok(false);
    }
    send_warning(WRITE_SCALING_WARNING);
    let Some(first) = streams.first_mut() else {
        return Ok(false);
    };
    let key = text(&first.path);
    let context = format!("writing stream {key}");
    let Some(writer) = configured_writer(provider, &context, &context) else {
        return Ok(false);
    };
    let is_written =
        writer.write_stream(shape, &mut OStreamWriter(&mut *first.stream), progress)?;
    if !is_written {
        send_fail(&format!(
            "Error in the DeStlProvider during writing stream {key}: Mesh writing has been failed"
        ));
    }
    Ok(is_written)
}
