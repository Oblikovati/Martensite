//! `DeStepProvider`: documents and shapes, from and to files and streams, by the settings of a node.
use super::DeStepConfigurationNode;
use crate::step_control::shape_fix_parameters::converted;
use crate::step_control::{StepControlReader, StepControlWriter};
use crate::stepcaf_control::{
    STEPCAFControlController, STEPCAFControlReader, STEPCAFControlWriter,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderBase, DeProviderReadStreamList,
    DeProviderWriteStreamList, DeValidationUtils, DeWorkSession,
};
use martensite_tkernel::{
    impl_transient,
    message::{Message, ProgressRange},
    ncollection::DataMap,
    osd::{IStream, OStream},
    standard::{Failure, Handle, RuntimeTyped, StandardTransient, exceptions},
    tcollection::AsciiString,
    units_methods::{LengthUnit, UnitsMethods},
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};
use std::fmt::Write;
use std::io::Read;

mod transfers;

const WHERE: &str = "DeStepProvider";

/// The input stream of a list, as the readers take it.
struct StreamRead<'a>(&'a mut dyn IStream);

impl Read for StreamRead<'_> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        Ok(self.0.read(out))
    }
}

/// The output stream of a list, as the writers take it.
struct StreamWrite<'a>(&'a mut dyn OStream);

impl std::io::Write for StreamWrite<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.write(bytes);
        if self.0.good() {
            Ok(bytes.len())
        } else {
            Err(std::io::Error::other(
                "DeStepProvider: the output stream is not good",
            ))
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The first stream of a list that validation found not empty.
fn first_stream<T>(first: Option<T>) -> Result<T, Failure> {
    first.ok_or_else(|| {
        exceptions::null_object("DeStepProvider: no stream in the list; expected one")
    })
}

/// The provider of STEP.
///
/// ```
/// use martensite_tkde::de::DeProvider;
/// use martensite_tkdestep::de_step::DeStepProvider;
/// let provider = DeStepProvider::default();
/// assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
/// ```
#[derive(Default)]
pub struct DeStepProvider {
    transient: StandardTransient,
    base: DeProviderBase,
}
impl_transient!(DeStepProvider, "DeStepProvider", dyn DeProvider, transient, interfaces: [dyn DeProvider]);

fn fail(text: &str) {
    let _ = writeln!(Message::send_fail(), "{text}");
}

fn path_text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

/// `personizeWS`: a session of the STEP norm with the actors of the document.
fn personized_session(
    session: &mut DeWorkSession,
) -> Result<Handle<XSControlWorkSession>, Failure> {
    let mut ws: Handle<XSControlWorkSession> = session.cast();
    if ws.is_null() {
        ws = XSControlWorkSession::new();
        *session = ws.cast();
    }
    let live = crate::step_entity::step_live(&ws, WHERE)?;
    if live
        .norm_adaptor()
        .cast::<STEPCAFControlController>()
        .is_null()
    {
        STEPCAFControlController::init()?;
        live.select_norm("STEP")?;
    }
    Ok(ws)
}

/// The unit of the target length, and the length unit a model writes with.
fn target_unit(length_unit: f64) -> LengthUnit {
    UnitsMethods::get_length_unit_by_factor_value(length_unit, LengthUnit::Millimeter)
}

impl DeStepProvider {
    /// `DeStepProvider(theNode)`.
    pub fn with_node(node: &Handle<dyn DeConfigurationNode>) -> Self {
        Self {
            base: DeProviderBase::with_node(node),
            ..Self::default()
        }
    }
    /// `validateNode`: the node of this provider, when it is one of STEP.
    fn valid_node(
        &self,
        context: &str,
    ) -> Result<Option<Handle<DeStepConfigurationNode>>, Failure> {
        let node = self.get_node();
        if !DeValidationUtils::validate_configuration_node(
            &node,
            DeStepConfigurationNode::static_type(),
            &AsciiString::new(context),
            true,
        ) {
            return Ok(None);
        }
        Ok(Some(node.cast()))
    }
    fn valid_document_and_node(
        &self,
        document: &Handle<TDocStdDocument>,
        context: &str,
    ) -> Result<Option<Handle<DeStepConfigurationNode>>, Failure> {
        if !DeValidationUtils::validate_document(document, &AsciiString::new(context), true) {
            return Ok(None);
        }
        self.valid_node(context)
    }
}

/// `configureSTEPCAFReader`.
fn configure_reader(
    reader: &mut STEPCAFControlReader,
    node: &DeStepConfigurationNode,
    ws: &Handle<XSControlWorkSession>,
    document: &Handle<TDocStdDocument>,
) -> Result<(), Failure> {
    reader.init(ws, true)?;
    let parameters = node.internal_parameters.snapshot();
    reader.set_color_mode(parameters.read_color);
    reader.set_name_mode(parameters.read_name);
    reader.set_layer_mode(parameters.read_layer);
    reader.set_props_mode(parameters.read_props);
    reader.set_meta_mode(parameters.read_metadata);
    reader.set_product_meta_mode(parameters.read_product_metadata);
    XCAFDocDocumentTool::set_length_unit_in(
        document,
        node.global_parameters().length_unit,
        LengthUnit::Millimeter,
    )
}

/// `configureSTEPCAFWriter`.
fn configure_writer(
    writer: &mut STEPCAFControlWriter,
    node: &DeStepConfigurationNode,
    ws: &Handle<XSControlWorkSession>,
    document: &Handle<TDocStdDocument>,
    length_unit: f64,
) -> Result<(), Failure> {
    writer.init(ws, true)?;
    let parameters = node.internal_parameters.snapshot();
    writer.set_color_mode(parameters.write_color);
    writer.set_name_mode(parameters.write_name);
    writer.set_layer_mode(parameters.write_layer);
    writer.set_props_mode(parameters.write_props);
    writer.set_material_mode(parameters.write_material);
    writer.set_visual_material_mode(parameters.write_vis_material);
    writer.set_clean_duplicates(parameters.clean_duplicates);
    writer.set_shape_fix_parameters_from(
        &converted(&node.shape_fix_parameters()),
        Default::default(),
    )?;
    let model = writer.writer().model(false)?;
    let live = crate::step_entity::step_live(&model, WHERE)?;
    let mut scale = 1.0;
    if XCAFDocDocumentTool::get_length_unit_in(document, &mut scale, LengthUnit::Millimeter)? {
        live.set_local_length_unit(scale);
    } else {
        live.set_local_length_unit(length_unit);
        let _ = writeln!(
            Message::send_warning(),
            "Warning in the DeStepProvider during writing\t: The document has no information on Units. Using global parameter as initial Unit."
        );
    }
    Ok(())
}

/// The parameters and the model of a transfer to the target unit.
fn write_parameters(
    node: &DeStepConfigurationNode,
    writer: &StepControlWriter,
) -> Result<super::DeStepParameters, Failure> {
    let model = writer.model(false)?;
    let mut parameters = node.internal_parameters.snapshot();
    let length_unit = node.global_parameters().length_unit;
    parameters.write_unit = target_unit(length_unit) as i32;
    crate::step_entity::step_live(&model, WHERE)?.set_write_length_unit(length_unit);
    Ok(parameters)
}

impl DeProvider for DeStepProvider {
    fn provider_base(&self) -> &DeProviderBase {
        &self.base
    }
    fn get_format(&self) -> AsciiString {
        AsciiString::new("STEP")
    }
    fn get_vendor(&self) -> AsciiString {
        AsciiString::new("Martensite")
    }
    fn read_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_read_document_with_session(path, document, work_session, progress)
    }
    fn write_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_document_with_session(path, document, work_session, progress)
    }
    fn read_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document_with_session(path, document, &mut DeWorkSession::default(), progress)
    }
    fn write_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document_with_session(path, document, &mut DeWorkSession::default(), progress)
    }
    fn read_document_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_read_document_streams_with_session(streams, document, work_session, progress)
    }
    fn write_document_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_document_streams_with_session(streams, document, work_session, progress)
    }
    fn read_document_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document_streams_with_session(
            streams,
            document,
            &mut DeWorkSession::default(),
            progress,
        )
    }
    fn write_document_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document_streams_with_session(
            streams,
            document,
            &mut DeWorkSession::default(),
            progress,
        )
    }
    fn read_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_read_shape_with_session(path, shape, work_session, progress)
    }
    fn write_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_shape_with_session(path, shape, work_session, progress)
    }
    fn read_shape(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape_with_session(path, shape, &mut DeWorkSession::default(), progress)
    }
    fn write_shape(
        &self,
        path: &AsciiString,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape_with_session(path, shape, &mut DeWorkSession::default(), progress)
    }
    fn read_shape_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_read_shape_streams_with_session(streams, shape, work_session, progress)
    }
    fn write_shape_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_shape_streams_with_session(streams, shape, work_session, progress)
    }
    fn read_shape_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape_streams_with_session(
            streams,
            shape,
            &mut DeWorkSession::default(),
            progress,
        )
    }
    fn write_shape_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape_streams_with_session(
            streams,
            shape,
            &mut DeWorkSession::default(),
            progress,
        )
    }
}

/// The roots of a file read, as one shape.
fn read_roots(
    reader: &mut StepControlReader,
    node: &DeStepConfigurationNode,
    shape: &mut Shape,
    progress: &ProgressRange<'_>,
    source: &str,
) -> Result<bool, Failure> {
    reader.xs_reader_mut().set_shape_fix_parameters_from(
        &converted(&node.shape_fix_parameters()),
        &DataMap::default(),
    );
    let model = reader.step_model()?;
    crate::step_entity::step_live(&model, WHERE)?
        .set_local_length_unit(node.global_parameters().length_unit);
    if reader.transfer_roots(progress)? <= 0 {
        fail(&format!(
            "Error in the DeStepProvider during reading {source}\t:Cannot read any relevant data from the STEP file"
        ));
        return Ok(false);
    }
    *shape = reader.xs_reader().one_shape();
    Ok(true)
}

/// A shape into the model of a writer, in the target unit.
fn transfer_shape(
    writer: &StepControlWriter,
    node: &DeStepConfigurationNode,
    shape: &Shape,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let model = writer.model(false)?;
    let live = crate::step_entity::step_live(&model, WHERE)?;
    let global = node.global_parameters();
    live.set_local_length_unit(global.system_unit);
    let unit = target_unit(global.length_unit);
    let mut parameters = node.internal_parameters.snapshot();
    parameters.write_unit = unit as i32;
    if unit == LengthUnit::Undefined {
        live.set_write_length_unit(1.0);
        let _ = writeln!(
            Message::send_warning(),
            "Custom units are not supported by STEP format, but LengthUnit global parameter doesn't fit any predefined unit. Units will be scaled to Millimeters"
        );
    } else {
        live.set_write_length_unit(global.length_unit);
    }
    writer.set_shape_fix_parameters_from(
        &converted(&node.shape_fix_parameters()),
        Default::default(),
    )?;
    let mode = parameters.write_model_type;
    let status = writer.transfer_with(shape, mode, &parameters, true, progress)?;
    if status != IFSelectReturnStatus::RetDone {
        return Ok(false);
    }
    if parameters.clean_duplicates {
        writer.clean_duplicate_entities()?;
    }
    Ok(true)
}
