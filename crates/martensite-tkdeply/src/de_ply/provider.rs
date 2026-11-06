//! `DePlyProvider`: the file transfers of the provider interface; PLY is written, never read.
use super::DePlyConfigurationNode;
use crate::rw_ply::RwPlyCafWriter;
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderBase, DeValidationUtils, DeWorkSession,
};
use martensite_tkernel::{
    impl_transient,
    message::{Message, ProgressRange},
    ncollection::IndexedDataMap,
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystemConverter;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};

/// Writes PLY files with the settings of its node; a work session is accepted and ignored.
///
/// Positions and normals are converted from `system.cs` to `file.cs`, and positions from the unit
/// of the document to the global length unit. A document without a unit is taken to be in the
/// global system unit, with a warning. The default settings therefore write a Z-up model as Y-up.
///
/// ```
/// use martensite_tkde::de::{DeConfigurationNode, DeProvider, new_configuration_node};
/// use martensite_tkdeply::de_ply::{DePlyConfigurationNode, DePlyProvider};
/// let node = new_configuration_node(DePlyConfigurationNode::new());
/// let provider = DePlyProvider::with_node(&node.cast::<dyn DeConfigurationNode>());
/// assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
/// assert_eq!(provider.get_format().as_bytes(), b"PLY");
/// ```
#[derive(Default)]
pub struct DePlyProvider {
    transient: StandardTransient,
    base: DeProviderBase,
}
impl_transient!(DePlyProvider, "DePlyProvider", dyn DeProvider, transient, interfaces: [dyn DeProvider]);

fn path_text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

fn file_information(node: &DePlyConfigurationNode) -> IndexedDataMap<AsciiString, AsciiString> {
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

fn configured_writer(
    node: &DePlyConfigurationNode,
    path: &AsciiString,
    converter: &RwMeshCoordinateSystemConverter,
) -> RwPlyCafWriter {
    let parameters = node.internal_parameters.snapshot();
    let mut writer = RwPlyCafWriter::new(path);
    writer.set_coordinate_system_converter(converter);
    writer.set_normals(parameters.write_normals);
    writer.set_colors(parameters.write_colors);
    writer.set_tex_coords(parameters.write_tex_coords);
    writer.set_part_id(parameters.write_part_id);
    writer.set_face_id(parameters.write_face_id);
    writer
}

fn free_shape_count(document: &Handle<TDocStdDocument>) -> Result<usize, Failure> {
    let live = document.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "DePlyProvider: expected a document",
            "",
        )
    })?;
    let tool = XCAFDocDocumentTool::shape_tool(&live.main()?)?;
    Ok(tool
        .get()
        .map_or(Ok(Vec::new()), XCAFDocShapeTool::get_free_shapes)?
        .len())
}

/// The conversion from the document's system and unit to the file's, as the settings state them.
fn writing_converter(
    node: &DePlyConfigurationNode,
    document: &Handle<TDocStdDocument>,
    path: &AsciiString,
) -> Result<RwMeshCoordinateSystemConverter, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let global = node.configuration_node_base().global_parameters();
    let mut converter = RwMeshCoordinateSystemConverter::new();
    let mut unit = 1.0;
    if XCAFDocDocumentTool::get_length_unit(document, &mut unit)? {
        converter.set_input_length_unit(unit);
    } else {
        converter.set_input_length_unit(global.system_unit / 1000.0);
        Message::send_warning_text(&AsciiString::new(&format!(
            "Warning in the DePlyProvider during writing the file {}\t: The document has no information on Units. Using global parameter as initial Unit.",
            path_text(path)
        )));
    }
    converter.set_input_coordinate_system_kind(parameters.system_cs);
    converter.set_output_length_unit(global.length_unit / 1000.0);
    converter.set_output_coordinate_system_kind(parameters.file_cs);
    Ok(converter)
}

impl DePlyProvider {
    /// A provider with global configuration; `DePlyProvider::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A provider on a node, keeping that node rather than a copy; `DePlyProvider::with_node(&node)`.
    pub fn with_node(node: &Handle<dyn DeConfigurationNode>) -> Self {
        Self {
            base: DeProviderBase::with_node(node),
            ..Self::default()
        }
    }
    /// The `DePlyConfigurationNode` of the provider, after the validation of its type.
    fn validated_node(&self, path: &AsciiString) -> Option<Handle<DePlyConfigurationNode>> {
        let context = format!("writing the file {}", path_text(path));
        let node = self.get_node();
        let is_valid = DeValidationUtils::validate_configuration_node(
            &node,
            DePlyConfigurationNode::static_type(),
            &AsciiString::new(&context),
            true,
        );
        is_valid.then(|| node.cast::<DePlyConfigurationNode>())
    }
}
impl DeProvider for DePlyProvider {
    fn provider_base(&self) -> &DeProviderBase {
        &self.base
    }
    fn get_format(&self) -> AsciiString {
        AsciiString::new("PLY")
    }
    fn get_vendor(&self) -> AsciiString {
        AsciiString::new("Martensite")
    }
    fn write_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document(path, document, progress)
    }
    fn write_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(node) = self.validated_node(path) else {
            return Ok(false);
        };
        let Some(node) = node.get() else {
            return Ok(false);
        };
        if free_shape_count(document)? == 0 {
            return Ok(true);
        }
        let converter = writing_converter(node, document, path)?;
        if !configured_writer(node, path, &converter).perform(
            document,
            &file_information(node),
            progress,
        )? {
            Message::send_fail_text(&AsciiString::new(&format!(
                "Error in the DePlyProvider during writing the file {}\t: Cannot perform the document",
                path_text(path)
            )));
            return Ok(false);
        }
        Ok(true)
    }
    fn write_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &Shape,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape(path, shape, progress)
    }
    fn write_shape(
        &self,
        path: &AsciiString,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF"))?;
        let live = document.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "DePlyProvider: expected a new document",
                "",
            )
        })?;
        let tool = XCAFDocDocumentTool::shape_tool(&live.main()?)?;
        if let Some(tool) = tool.get() {
            tool.add_shape(shape, true, true)?;
        }
        self.write_document(path, &document, progress)
    }
}
