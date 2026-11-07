//! The transfers of `DeIgesProvider`: shapes and documents, from and to files and streams.
use super::statics::StaticScope;
use super::{
    AsciiString, DEShapeFixParameters, DeIgesConfigurationNode, DeIgesProvider, DeWorkSession,
    Failure, Handle, IFSelectReturnStatus, IStream, IgesCafControlReader, IgesCafControlWriter,
    IgesControlReader, IgesControlWriter, LengthUnit, OStream, ProgressRange, Shape, StreamRead,
    StreamWrite, TDocStdDocument, WHERE, XCAFDocDocumentTool, XSControlReaderOps,
    XSControlWorkSession, fail, iges_live, path_text, personized_session, setup_units,
    transfer_shape_fix_parameters, unit_name,
};

/// Where a transfer reads from.
pub(super) enum ReadSource<'a> {
    File(&'a AsciiString),
    Stream(String, &'a mut dyn IStream),
}

impl ReadSource<'_> {
    /// How messages name the source.
    fn subject(&self) -> String {
        match self {
            Self::File(path) => format!("the file {}", path_text(path)),
            Self::Stream(key, _) => format!("stream {key}"),
        }
    }
    fn load(&mut self, reader: &mut IgesControlReader) -> Result<IFSelectReturnStatus, Failure> {
        match self {
            Self::File(path) => reader.read_file(&path_text(path)),
            Self::Stream(key, stream) => reader.read_stream(key, &mut StreamRead(&mut **stream)),
        }
    }
}

/// Where a transfer writes to.
pub(super) enum WriteTarget<'a> {
    File(&'a AsciiString),
    Stream(String, &'a mut dyn OStream),
}

impl WriteTarget<'_> {
    /// How messages name the target.
    fn subject(&self) -> String {
        match self {
            Self::File(path) => format!("the file {}", path_text(path)),
            Self::Stream(key, _) => format!("stream {key}"),
        }
    }
    /// Whether a stream can take the output; a file can until it is opened.
    fn is_writable(&self) -> bool {
        let Self::Stream(key, stream) = self else {
            return true;
        };
        if !stream.good() {
            let shown = if key.is_empty() {
                "<empty key>"
            } else {
                key.as_str()
            };
            fail(&format!(
                "Error: Output stream '{shown}' is not in good state for writing"
            ));
        }
        stream.good()
    }
    fn store(&mut self, writer: &mut IgesControlWriter) -> Result<bool, Failure> {
        match self {
            Self::File(path) => writer.write_file(&path_text(path), false),
            Self::Stream(_, stream) => writer.write_stream(&mut StreamWrite(&mut **stream), false),
        }
    }
    /// What is told when a shape is not transferred.
    fn shape_not_transferred(&self) -> String {
        match self {
            Self::File(_) => "DeIgesProvider: Shape not written".to_string(),
            Self::Stream(key, _) => {
                format!("Error: DeIgesProvider failed to transfer shape for stream {key}")
            }
        }
    }
    /// What is told when a shape is not written.
    fn shape_not_written(&self) -> String {
        match self {
            Self::File(path) => {
                format!("DeIgesProvider: Error on writing file {}", path_text(path))
            }
            Self::Stream(key, _) => {
                format!("Error: DeIgesProvider failed to write shape to stream {key}")
            }
        }
    }
}

/// The healing parameters of a node, for a reader or a writer.
fn fix_parameters(node: &DeIgesConfigurationNode) -> DEShapeFixParameters {
    transfer_shape_fix_parameters(&node.shape_fix_parameters())
}

/// A reader of shapes on the settings of a node.
fn shape_reader(
    ws: &Handle<XSControlWorkSession>,
    node: &DeIgesConfigurationNode,
) -> Result<IgesControlReader, Failure> {
    let mut reader = IgesControlReader::with_session(ws, true)?;
    reader.set_read_visible(node.internal_parameters.snapshot().read_only_visible);
    reader
        .xs_reader()
        .set_shape_fix_parameters_from(&fix_parameters(node), &Default::default());
    Ok(reader)
}

/// A writer of shapes on the settings of a node.
fn shape_writer(node: &DeIgesConfigurationNode) -> Result<IgesControlWriter, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let mut writer = IgesControlWriter::with_unit(
        unit_name(node).as_bytes(),
        parameters.write_b_rep_mode.as_raw(),
    )?;
    let model = writer.model();
    let live = iges_live(&model, WHERE)?;
    let mut section = live.global_section();
    setup_units(&mut section, node, None)?;
    live.set_global_section(&section);
    writer.set_shape_fix_parameters_from(&fix_parameters(node), &Default::default());
    Ok(writer)
}

/// A reader of documents on the settings of a node.
fn document_reader(
    ws: &Handle<XSControlWorkSession>,
    node: &DeIgesConfigurationNode,
) -> Result<IgesCafControlReader, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let mut reader = IgesCafControlReader::with_session(ws, true)?;
    reader
        .base_mut()
        .set_read_visible(parameters.read_only_visible);
    reader.set_color_mode(parameters.read_color);
    reader.set_name_mode(parameters.read_name);
    reader.set_layer_mode(parameters.read_layer);
    reader
        .base_mut()
        .xs_reader()
        .set_shape_fix_parameters_from(&fix_parameters(node), &Default::default());
    Ok(reader)
}

/// A writer of documents on the settings of a node, in the units of the document.
fn document_writer(
    ws: &Handle<XSControlWorkSession>,
    node: &DeIgesConfigurationNode,
    document: &Handle<TDocStdDocument>,
    subject: &str,
) -> Result<IgesCafControlWriter, Failure> {
    let parameters = node.internal_parameters.snapshot();
    let mut writer = IgesCafControlWriter::with_session(ws)?;
    let model = writer.base().model();
    let live = iges_live(&model, WHERE)?;
    let mut section = live.global_section();
    setup_units(&mut section, node, Some((document, subject)))?;
    live.set_global_section(&section);
    writer.set_color_mode(parameters.write_color);
    writer.set_name_mode(parameters.write_name);
    writer.set_layer_mode(parameters.write_layer);
    writer
        .base_mut()
        .set_shape_fix_parameters_from(&fix_parameters(node), &Default::default());
    Ok(writer)
}

impl DeIgesProvider {
    pub(super) fn perform_read_shape(
        &self,
        mut source: ReadSource<'_>,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let subject = source.subject();
        let Some(handle) = self.valid_node(&format!("reading {subject}"))? else {
            return Ok(false);
        };
        let node = iges_live(&handle, WHERE)?;
        let ws = personized_session(work_session)?;
        let _scope = StaticScope::new(node)?;
        let mut reader = shape_reader(&ws, node)?;
        if source.load(&mut reader)? != IFSelectReturnStatus::RetDone {
            let reason = match source {
                ReadSource::File(_) => "abandon, no model loaded",
                ReadSource::Stream(..) => "Could not read stream, no model loaded",
            };
            fail(&format!(
                "Error in the DeIgesProvider during reading {subject}\t: {reason}"
            ));
            return Ok(false);
        }
        if reader.transfer_roots(progress)? <= 0 {
            fail(&format!(
                "Error in the DeIgesProvider during reading {subject}\t: Cannot read any relevant data from the IGES source"
            ));
            return Ok(false);
        }
        *shape = reader.xs_reader().one_shape();
        Ok(true)
    }
    pub(super) fn perform_write_shape(
        &self,
        mut target: WriteTarget<'_>,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(handle) = self.valid_node(&format!("writing {}", target.subject()))? else {
            return Ok(false);
        };
        if !target.is_writable() {
            return Ok(false);
        }
        let node = iges_live(&handle, WHERE)?;
        let ws = personized_session(work_session)?;
        let _scope = StaticScope::new(node)?;
        let mut writer = shape_writer(node)?;
        if !writer.add_shape(shape, progress)? {
            fail(&target.shape_not_transferred());
            return Ok(false);
        }
        let session = iges_live(&ws, WHERE)?;
        session.set_model(&writer.model().cast(), false)?;
        session.set_map_writer(&writer.transfer_process())?;
        if !target.store(&mut writer)? {
            fail(&target.shape_not_written());
            return Ok(false);
        }
        Ok(true)
    }
    pub(super) fn perform_read_document(
        &self,
        mut source: ReadSource<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let subject = source.subject();
        let Some(handle) = self.valid_document_and_node(document, &format!("reading {subject}"))?
        else {
            return Ok(false);
        };
        let node = iges_live(&handle, WHERE)?;
        let ws = personized_session(work_session)?;
        let _scope = StaticScope::new(node)?;
        let unit = node.global_parameters().length_unit;
        XCAFDocDocumentTool::set_length_unit_in(document, unit, LengthUnit::Millimeter)?;
        let mut reader = document_reader(&ws, node)?;
        if source.load(reader.base_mut())? != IFSelectReturnStatus::RetDone {
            fail(&format!(
                "Error in the DeIgesProvider during reading {subject}\t: abandon, no model loaded"
            ));
            return Ok(false);
        }
        if !reader.transfer(document, progress)? {
            fail(&format!(
                "Error in the DeIgesProvider during reading {subject}\t: Cannot read any relevant data from the IGES source"
            ));
            return Ok(false);
        }
        Ok(true)
    }
    pub(super) fn perform_write_document(
        &self,
        mut target: WriteTarget<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let subject = target.subject();
        let Some(handle) = self.valid_document_and_node(document, &format!("writing {subject}"))?
        else {
            return Ok(false);
        };
        if !target.is_writable() {
            return Ok(false);
        }
        let node = iges_live(&handle, WHERE)?;
        let ws = personized_session(work_session)?;
        let _scope = StaticScope::new(node)?;
        let mut writer = document_writer(&ws, node, document, &subject)?;
        if !writer.transfer_document(document, progress)? {
            fail(&format!(
                "Error in the DeIgesProvider during writing {subject}\t: The document cannot be translated or gives no result"
            ));
            return Ok(false);
        }
        if !target.store(writer.base_mut())? {
            fail(&format!(
                "Error in the DeIgesProvider during writing {subject}\t: Write failed"
            ));
            return Ok(false);
        }
        Ok(true)
    }
}
