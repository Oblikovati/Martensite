//! The transfers of `DeStepProvider` with a session: the methods of the framework call these.
use super::*;

impl DeStepProvider {
    pub(super) fn perform_read_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = format!("reading the file {}", path_text(path));
        let Some(node) = self.valid_document_and_node(document, &context)? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut reader = STEPCAFControlReader::new()?;
        configure_reader(&mut reader, node, &ws, document)?;
        let parameters = node.internal_parameters.snapshot();
        if reader.read_file_with(&path_text(path), &parameters)? != IFSelectReturnStatus::RetDone {
            fail(&format!(
                "Error in the DeStepProvider during reading the file {}\t: abandon",
                path_text(path)
            ));
            return Ok(false);
        }
        reader.set_shape_fix_parameters_from(
            &converted(&node.shape_fix_parameters()),
            &DataMap::default(),
        )?;
        if !reader.transfer(document, progress)? {
            fail(&format!(
                "Error in the DeStepProvider during reading the file {}\t: Cannot read any relevant data from the STEP file",
                path_text(path)
            ));
            return Ok(false);
        }
        Ok(true)
    }
    pub(super) fn perform_write_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = format!("writing the file {}", path_text(path));
        let Some(node) = self.valid_document_and_node(document, &context)? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut writer = STEPCAFControlWriter::new()?;
        configure_writer(
            &mut writer,
            node,
            &ws,
            document,
            node.global_parameters().system_unit,
        )?;
        let parameters = write_parameters(node, writer.writer())?;
        let mode = parameters.write_model_type;
        if !writer.transfer_document_with(document, &parameters, mode, None, progress)? {
            fail(&format!(
                "Error in the DeStepProvider during writing the file {}\t: The document cannot be translated or gives no result",
                path_text(path)
            ));
            return Ok(false);
        }
        Ok(match writer.write(&path_text(path))? {
            IFSelectReturnStatus::RetDone => true,
            IFSelectReturnStatus::RetVoid => {
                fail(&format!(
                    "Error in the DeStepProvider during writing the file {}\t: No file written",
                    path_text(path)
                ));
                false
            }
            _ => {
                fail(&format!(
                    "Error in the DeStepProvider during writing the file {}\t: Error on writing file",
                    path_text(path)
                ));
                false
            }
        })
    }
    pub(super) fn perform_read_document_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = AsciiString::new("reading stream");
        if !DeValidationUtils::validate_read_stream_list(streams, &context, true) {
            return Ok(false);
        }
        let first = first_stream(streams.first_mut())?;
        let key = path_text(&first.path);
        let full = format!("reading stream {key}");
        let Some(node) = self.valid_document_and_node(document, &full)? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut reader = STEPCAFControlReader::with_session(&ws, false)?;
        configure_reader(&mut reader, node, &ws, document)?;
        let parameters = node.internal_parameters.snapshot();
        if reader.read_stream_with(&key, &parameters, &mut StreamRead(&mut *first.stream))?
            != IFSelectReturnStatus::RetDone
        {
            return Ok(false);
        }
        reader.set_shape_fix_parameters_from(
            &converted(&node.shape_fix_parameters()),
            &DataMap::default(),
        )?;
        reader.transfer(document, progress)
    }
    pub(super) fn perform_write_document_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = AsciiString::new("writing stream");
        if !DeValidationUtils::validate_write_stream_list(streams, &context, true) {
            return Ok(false);
        }
        let first = first_stream(streams.first_mut())?;
        let key = path_text(&first.path);
        let full = format!("writing stream {key}");
        let Some(node) = self.valid_document_and_node(document, &full)? else {
            return Ok(false);
        };
        // checkStreamWritability
        if !first.stream.good() {
            let shown = if key.is_empty() {
                "<empty key>"
            } else {
                key.as_str()
            };
            fail(&format!(
                "Error: Output stream '{shown}' is not in good state for writing"
            ));
            return Ok(false);
        }
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut writer = STEPCAFControlWriter::with_session(&ws, false)?;
        configure_writer(
            &mut writer,
            node,
            &ws,
            document,
            node.global_parameters().length_unit,
        )?;
        let parameters = write_parameters(node, writer.writer())?;
        let mode = parameters.write_model_type;
        if !writer.transfer_document_with(document, &parameters, mode, None, progress)? {
            fail(&format!(
                "Error in the DeStepProvider during writing stream {key}\t: The document cannot be translated or gives no result"
            ));
            return Ok(false);
        }
        // the status converts to bool: anything but `RetVoid`
        Ok(writer.write_stream(&mut StreamWrite(&mut *first.stream))?
            != IFSelectReturnStatus::RetVoid)
    }
    pub(super) fn perform_read_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = format!("reading the file {}", path_text(path));
        let Some(node) = self.valid_node(&context)? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut reader = StepControlReader::new()?;
        reader.xs_reader_mut().set_ws(&ws, true)?;
        let parameters = node.internal_parameters.snapshot();
        if reader.read_file_with(&path_text(path), &parameters)? != IFSelectReturnStatus::RetDone {
            fail(&format!(
                "Error in the DeStepProvider during reading the file {}\t: abandon, no model loaded",
                path_text(path)
            ));
            return Ok(false);
        }
        read_roots(&mut reader, node, shape, progress, &path_text(path))
    }
    pub(super) fn perform_write_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = format!("writing the file {}", path_text(path));
        let Some(node) = self.valid_node(&context)? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut writer = StepControlWriter::new()?;
        writer.set_ws(&ws, true)?;
        if !transfer_shape(&writer, node, shape, progress)? {
            fail(&format!(
                "Error in the DeStepProvider during reading the file {}\t: abandon, no model loaded",
                path_text(path)
            ));
            return Ok(false);
        }
        if writer.write(&path_text(path))? != IFSelectReturnStatus::RetDone {
            fail("DeStepProvider: Error on writing file");
            return Ok(false);
        }
        Ok(true)
    }
    pub(super) fn perform_read_shape_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = AsciiString::new("reading stream");
        if !DeValidationUtils::validate_read_stream_list(streams, &context, true) {
            return Ok(false);
        }
        let first = first_stream(streams.first_mut())?;
        let key = path_text(&first.path);
        let Some(node) = self.valid_node(&format!("reading stream {key}"))? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut reader = StepControlReader::new()?;
        reader.xs_reader_mut().set_ws(&ws, true)?;
        let parameters = node.internal_parameters.snapshot();
        if reader.read_stream_with(&key, &parameters, &mut StreamRead(&mut *first.stream))?
            != IFSelectReturnStatus::RetDone
        {
            fail(&format!(
                "Error in the DeStepProvider during reading stream {key}\t: abandon, no model loaded"
            ));
            return Ok(false);
        }
        read_roots(&mut reader, node, shape, progress, &key)
    }
    pub(super) fn perform_write_shape_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let context = AsciiString::new("writing stream");
        if !DeValidationUtils::validate_write_stream_list(streams, &context, true) {
            return Ok(false);
        }
        let first = first_stream(streams.first_mut())?;
        let key = path_text(&first.path);
        let Some(node) = self.valid_node(&format!("writing stream {key}"))? else {
            return Ok(false);
        };
        let node = crate::step_entity::step_live(&node, WHERE)?;
        let ws = personized_session(work_session)?;
        let mut writer = StepControlWriter::new()?;
        writer.set_ws(&ws, true)?;
        if !transfer_shape(&writer, node, shape, progress)? {
            fail(&format!(
                "Error in the DeStepProvider during writing stream {key}\t: abandon, no model loaded"
            ));
            return Ok(false);
        }
        // the status converts to bool: anything but `RetVoid`
        Ok(writer.write_stream(&mut StreamWrite(&mut *first.stream))?
            != IFSelectReturnStatus::RetVoid)
    }
}
