//! The document transfers of `DeStlProvider`.
use super::DeStlProvider;
use super::provider_shapes::{validated_node, warn_length_unit};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkde::de::{
    DeProvider, DeProviderReadStreamList, DeProviderWriteStreamList, DeValidationUtils,
};
use martensite_tkernel::{
    message::{Message, ProgressRange},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdDocument};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};

fn is_valid_document(document: &Handle<TDocStdDocument>, context: &str) -> bool {
    DeValidationUtils::validate_document(document, &AsciiString::new(context), true)
}

fn main_label(document: &Handle<TDocStdDocument>) -> Result<TdfLabel, Failure> {
    let document = document.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "DeStlProvider: the document was validated; expected a non-null handle",
            "",
        )
    })?;
    document.main()
}

fn add_to_document(document: &Handle<TDocStdDocument>, shape: &Shape) -> Result<(), Failure> {
    let tool = XCAFDocDocumentTool::shape_tool(&main_label(document)?)?;
    let tool = tool.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "DeStlProvider: the document shape tool returned null; expected a tool",
            "",
        )
    })?;
    tool.add_shape(shape, false, true)?;
    Ok(())
}

/// The free shapes of a document: the shape itself when there is one, else a compound.
fn free_shape_labels(document: &Handle<TDocStdDocument>) -> Result<Vec<TdfLabel>, Failure> {
    let tool = XCAFDocDocumentTool::shape_tool(&main_label(document)?)?;
    match tool.get() {
        Some(tool) => tool.get_free_shapes(),
        None => Ok(Vec::new()),
    }
}

fn label_shape(label: &TdfLabel) -> Result<Shape, Failure> {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape)?;
    Ok(shape)
}

fn combined_shape(labels: &[TdfLabel]) -> Result<Shape, Failure> {
    if let [only] = labels {
        return label_shape(only);
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for label in labels {
        builder.add(&mut compound, &label_shape(label)?)?;
    }
    Ok(compound.into())
}

/// Reads a document from a path.
pub(super) fn read_file(
    provider: &DeStlProvider,
    path: &AsciiString,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let name = String::from_utf8_lossy(path.as_bytes());
    if !is_valid_document(document, &format!("reading the file {name}")) {
        return Ok(false);
    }
    let mut shape = Shape::default();
    if !provider.read_shape(path, &mut shape, progress)? {
        return Ok(false);
    }
    add_to_document(document, &shape)?;
    Ok(true)
}

/// Writes a document to a path.
pub(super) fn write_file(
    provider: &DeStlProvider,
    path: &AsciiString,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let name = String::from_utf8_lossy(path.as_bytes()).into_owned();
    let context = format!("writing the file {name}");
    if !is_valid_document(document, &context) {
        return Ok(false);
    }
    let labels = free_shape_labels(document)?;
    if labels.is_empty() {
        send_no_shapes(&format!("the file {name}"));
        return Ok(false);
    }
    if !check_writable_node(provider, &context, &context) {
        return Ok(false);
    }
    provider.write_shape(path, &combined_shape(&labels)?, progress)
}

fn send_no_shapes(target: &str) {
    Message::send_fail_text(&AsciiString::new(&format!(
        "Error in the DeStlProvider during writing {target}: Document contain no shapes"
    )));
}

fn check_writable_node(provider: &DeStlProvider, node_context: &str, length_context: &str) -> bool {
    let Some(node) = validated_node(provider, node_context) else {
        return false;
    };
    if let Some(node) = node.get() {
        warn_length_unit(node, length_context);
    }
    true
}

/// Reads a document from a stream.
pub(super) fn read_streams(
    provider: &DeStlProvider,
    streams: &mut DeProviderReadStreamList<'_>,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let context = AsciiString::new("reading stream");
    if !DeValidationUtils::validate_read_stream_list(streams, &context, true) {
        return Ok(false);
    }
    let key = first_key(streams.first().map(|first| &first.path));
    if !is_valid_document(document, &format!("reading stream {key}")) {
        return Ok(false);
    }
    let mut shape = Shape::default();
    if !provider.read_shape_streams(streams, &mut shape, progress)? {
        return Ok(false);
    }
    add_to_document(document, &shape)?;
    Ok(true)
}

fn first_key(path: Option<&AsciiString>) -> String {
    path.map(|path| String::from_utf8_lossy(path.as_bytes()).into_owned())
        .unwrap_or_default()
}

/// Writes a document to a stream.
pub(super) fn write_streams(
    provider: &DeStlProvider,
    streams: &mut DeProviderWriteStreamList<'_>,
    document: &Handle<TDocStdDocument>,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let context = AsciiString::new("writing stream");
    if !DeValidationUtils::validate_write_stream_list(streams, &context, true) {
        return Ok(false);
    }
    let key = first_key(streams.first().map(|first| &first.path));
    let full_context = format!("writing stream {key}");
    if !is_valid_document(document, &full_context) {
        return Ok(false);
    }
    let labels = free_shape_labels(document)?;
    if labels.is_empty() {
        send_no_shapes(&format!("stream {key}"));
        return Ok(false);
    }
    if !check_writable_node(provider, &full_context, &full_context) {
        return Ok(false);
    }
    provider.write_shape_streams(streams, &combined_shape(&labels)?, progress)
}
