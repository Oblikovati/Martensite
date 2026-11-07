//! `DeIgesProvider`: documents and shapes, from and to files and streams, by the settings of a node.
use super::shape_fix::transfer_shape_fix_parameters;
use super::{DeIgesConfigurationNode, DeIgesParameters};
use crate::iges_control::{IgesControlController, IgesControlReader, IgesControlWriter};
use crate::iges_data::{IgesDataBasicEditor, IgesDataGlobalSection, iges_data_init, iges_live};
use crate::igescaf_control::{IgesCafControlReader, IgesCafControlWriter};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderBase, DeProviderReadStreamList,
    DeProviderWriteStreamList, DeValidationUtils, DeWorkSession,
};
use martensite_tkernel::{
    impl_transient,
    message::{Message, ProgressRange},
    osd::{IStream, OStream},
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::AsciiString,
    units_methods::LengthUnit,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::InterfaceStatic;
use martensite_tkxsbase::xs_algo::DEShapeFixParameters;
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};
use std::fmt::Write;
use std::io::Read;
use std::sync::{Mutex, PoisonError};

mod statics;
mod transfers;
use transfers::{ReadSource, WriteTarget};

const WHERE: &str = "DeIgesProvider";

/// The settings are process-wide: one transfer at a time holds them.
static GLOBAL_IGES_LOCK: Mutex<()> = Mutex::new(());

fn fail(text: &str) {
    let _ = writeln!(Message::send_fail(), "{text}");
}

fn path_text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

/// A session of the IGES norm, a new one when none is given.
fn personized_session(
    session: &mut DeWorkSession,
) -> Result<Handle<XSControlWorkSession>, Failure> {
    let mut ws: Handle<XSControlWorkSession> = session.cast();
    if ws.is_null() {
        let _ = writeln!(
            Message::send_warning(),
            "Warning: DeIgesProvider : Null work session, use internal temporary session"
        );
        ws = XSControlWorkSession::new();
        *session = ws.cast();
    }
    let live = iges_live(&ws, WHERE)?;
    if live.norm_adaptor().is_null() {
        IgesControlController::init()?;
        live.select_norm("IGES")?;
    }
    Ok(ws)
}

/// A new session for a transfer that is not given one.
fn new_session() -> DeWorkSession {
    XSControlWorkSession::new().cast()
}

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
            Err(std::io::Error::other("the output stream is not good"))
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The main stream of a list, when the list is valid.
fn read_source<'s>(streams: &'s mut DeProviderReadStreamList<'_>) -> Option<ReadSource<'s>> {
    if !DeValidationUtils::validate_read_stream_list(streams, &"reading stream".into(), true) {
        return None;
    }
    let first = streams.first_mut()?;
    Some(ReadSource::Stream(
        path_text(&first.path),
        &mut *first.stream,
    ))
}

/// The main stream of a list, when the list is valid.
fn write_target<'s>(streams: &'s mut DeProviderWriteStreamList<'_>) -> Option<WriteTarget<'s>> {
    if !DeValidationUtils::validate_write_stream_list(streams, &"writing stream".into(), true) {
        return None;
    }
    let first = streams.first_mut()?;
    Some(WriteTarget::Stream(
        path_text(&first.path),
        &mut *first.stream,
    ))
}

/// The units of the header: from a document when it has them, else from the node.
fn setup_units(
    section: &mut IgesDataGlobalSection,
    node: &DeIgesConfigurationNode,
    document: Option<(&Handle<TDocStdDocument>, &str)>,
) -> Result<(), Failure> {
    let globals = node.global_parameters();
    let mut scale = globals.system_unit;
    if let Some((handle, subject)) = document {
        let mut in_document = 1.0;
        if XCAFDocDocumentTool::get_length_unit_in(
            handle,
            &mut in_document,
            LengthUnit::Millimeter,
        )? {
            scale = in_document;
        } else {
            let _ = writeln!(
                Message::send_warning(),
                "Warning in the DeIgesProvider during writing {subject}\t: The document has no information on Units. Using global parameter as initial Unit."
            );
        }
    }
    section.set_cascade_unit(scale);
    if IgesDataBasicEditor::get_flag_by_value(globals.length_unit) == 0 {
        section.set_scale(globals.length_unit);
    }
    Ok(())
}

/// The name of the unit of a node for a new writer.
fn unit_name(node: &DeIgesConfigurationNode) -> &'static str {
    let flag = IgesDataBasicEditor::get_flag_by_value(node.global_parameters().length_unit);
    if flag > 0 {
        IgesDataBasicEditor::unit_flag_name(flag)
    } else {
        "MM"
    }
}

/// The provider of IGES.
///
/// ```
/// use martensite_tkde::de::DeProvider;
/// use martensite_tkdeiges::de_iges::DeIgesProvider;
/// let provider = DeIgesProvider::default();
/// assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
/// assert_eq!(provider.get_format().as_bytes(), b"IGES");
/// ```
#[derive(Default)]
pub struct DeIgesProvider {
    transient: StandardTransient,
    base: DeProviderBase,
}
impl_transient!(DeIgesProvider, "DeIgesProvider", dyn DeProvider, transient, interfaces: [dyn DeProvider]);

impl DeIgesProvider {
    /// A provider on a node; `DeIgesProvider::with_node(&node)`.
    pub fn with_node(node: &Handle<dyn DeConfigurationNode>) -> Self {
        Self {
            base: DeProviderBase::with_node(node),
            ..Self::default()
        }
    }
    /// The node of this provider, when it is one of IGES.
    fn valid_node(
        &self,
        context: &str,
    ) -> Result<Option<Handle<DeIgesConfigurationNode>>, Failure> {
        let node = self.get_node();
        if !DeValidationUtils::validate_configuration_node(
            &node,
            DeIgesConfigurationNode::static_type(),
            &AsciiString::new(context),
            true,
        ) {
            return Ok(None);
        }
        Ok(Some(node.cast()))
    }
    /// The node of this provider, when it is one of IGES and the document is there.
    fn valid_document_and_node(
        &self,
        document: &Handle<TDocStdDocument>,
        context: &str,
    ) -> Result<Option<Handle<DeIgesConfigurationNode>>, Failure> {
        if !DeValidationUtils::validate_document(document, &AsciiString::new(context), true) {
            return Ok(None);
        }
        self.valid_node(context)
    }
}

impl DeProvider for DeIgesProvider {
    fn provider_base(&self) -> &DeProviderBase {
        &self.base
    }
    fn get_format(&self) -> AsciiString {
        AsciiString::new("IGES")
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
        self.perform_read_document(ReadSource::File(path), document, work_session, progress)
    }
    fn write_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_document(WriteTarget::File(path), document, work_session, progress)
    }
    fn read_document_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(source) = read_source(streams) else {
            return Ok(false);
        };
        self.perform_read_document(source, document, work_session, progress)
    }
    fn write_document_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(target) = write_target(streams) else {
            return Ok(false);
        };
        self.perform_write_document(target, document, work_session, progress)
    }
    fn read_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document_with_session(path, document, &mut new_session(), progress)
    }
    fn write_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document_with_session(path, document, &mut new_session(), progress)
    }
    fn read_document_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document_streams_with_session(streams, document, &mut new_session(), progress)
    }
    fn write_document_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document_streams_with_session(streams, document, &mut new_session(), progress)
    }
    fn read_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_read_shape(ReadSource::File(path), shape, work_session, progress)
    }
    fn write_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_write_shape(WriteTarget::File(path), shape, work_session, progress)
    }
    fn read_shape_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(source) = read_source(streams) else {
            return Ok(false);
        };
        self.perform_read_shape(source, shape, work_session, progress)
    }
    fn write_shape_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(target) = write_target(streams) else {
            return Ok(false);
        };
        self.perform_write_shape(target, shape, work_session, progress)
    }
    fn read_shape(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape_with_session(path, shape, &mut new_session(), progress)
    }
    fn write_shape(
        &self,
        path: &AsciiString,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape_with_session(path, shape, &mut new_session(), progress)
    }
    fn read_shape_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape_streams_with_session(streams, shape, &mut new_session(), progress)
    }
    fn write_shape_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape_streams_with_session(streams, shape, &mut new_session(), progress)
    }
}
