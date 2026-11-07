//! The reading of a file into a model.
use super::parameters::{IgesFileParamKind, IgesFileParamState};
use super::records::{
    IgesFileInput, IgesFileRead, IgesFileRecord, RECORD_SIZE, directory_line, read_section_record,
};
use super::structure::{IgesFileParam, IgesFileStructure};
use crate::iges_data::{
    IgesDataDirValues, IgesDataFileRecognizer, IgesDataIgesModel, IgesDataIgesReaderData,
    IgesDataIgesReaderTool, IgesDataProtocolOps, iges_atoi, iges_live, iges_msg,
};
use martensite_tkernel::{
    message::Msg,
    standard::{Failure, Handle},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataParams,
    InterfaceFileReaderToolLoad, InterfaceFileReaderToolOps, InterfaceInterfaceModel,
    InterfaceModelEntities, InterfaceModelReports,
};
use std::fs::File;
use std::io::Read;

const SECTION_NAMES: &[u8; 7] = b" SGDPT ";

/// The diagnostics of one reading.
struct Diagnostics {
    check: Handle<InterfaceCheck>,
}

impl Diagnostics {
    fn fail(&self, msg: &Msg) {
        if let Some(check) = self.check.get() {
            check.send_fail(msg);
        }
    }
    fn warning(&self, msg: &Msg) {
        if let Some(check) = self.check.get() {
            check.send_warning(msg);
        }
    }
    fn info(&self, msg: &Msg) {
        if let Some(check) = self.check.get() {
            check.send_msg(msg);
        }
    }
    fn wrong_record(&self, key: &str, number: i32, section: i32) -> Result<(), Failure> {
        let mut msg = iges_msg(key);
        msg.arg_integer(number)?;
        let name = SECTION_NAMES[section as usize];
        msg.arg_text(&[name])?;
        self.fail(&msg);
        Ok(())
    }
}

/// The record counts per section and what was parsed; `None` when the data cannot be read.
fn parse_records(
    stream: &mut dyn Read,
    is_fnes: bool,
    diagnostics: &Diagnostics,
) -> Result<Option<([i32; 6], IgesFileStructure)>, Failure> {
    let mut input = IgesFileInput::new(stream);
    let mut data = IgesFileStructure::new();
    let mut record: IgesFileRecord = [0; RECORD_SIZE];
    let mut counts = [0; 6];
    let mut parser = RecordParser::default();
    let (mut section_number, mut record_number, mut previous) = (0, 0, 0);
    loop {
        record_number += 1;
        let read = read_section_record(&mut input, &mut section_number, &mut record, is_fnes);
        let section = match read {
            IgesFileRead::Error => return Ok(None),
            IgesFileRead::End => break,
            IgesFileRead::BadTrailer => 0,
            IgesFileRead::Section(section) => section,
        };
        if section <= 0 || section < previous {
            diagnostics.wrong_record("XSTEP_18", record_number, previous)?;
            if previous == 0 {
                return Ok(None);
            }
            counts[previous as usize] += 1;
            continue;
        }
        counts[section as usize] += 1;
        previous = section;
        if section_number != counts[section as usize] {
            diagnostics.wrong_record("XSTEP_19", record_number, previous)?;
        }
        parser.record(
            &mut data,
            section,
            section_number,
            counts[section as usize],
            &mut record,
        );
    }
    if counts[5] == 0 {
        diagnostics.warning(&iges_msg("XSTEP_20"));
    }
    Ok(Some((counts, data)))
}

/// The state of the parsing of the records of the sections.
struct RecordParser {
    directory_status: i32,
    params: IgesFileParamState,
    separator: u8,
    delimiter: u8,
}

impl Default for RecordParser {
    fn default() -> Self {
        Self {
            directory_status: 0,
            params: IgesFileParamState::default(),
            separator: b',',
            delimiter: b';',
        }
    }
}

impl RecordParser {
    fn record(
        &mut self,
        data: &mut IgesFileStructure,
        section: i32,
        number: i32,
        count: i32,
        record: &mut IgesFileRecord,
    ) {
        match section {
            1 => {
                record[72] = 0;
                data.new_param(IgesFileParamKind::Void, &record[..73]);
            }
            2 => {
                data.set_global();
                if count == 1 {
                    self.find_delimiters(record);
                }
                self.parameters(data, record, 72);
            }
            3 => directory_line(data, &mut self.directory_status, number, record.as_slice()),
            4 => {
                let directory = iges_atoi(&record[65..]);
                record[64] = 0;
                data.cur_part(directory);
                self.parameters(data, record, 64);
            }
            _ => {}
        }
    }
    /// The delimiters of the file, from the start of its global section.
    fn find_delimiters(&mut self, record: &IgesFileRecord) {
        let mut start = 0;
        if record[0] != b',' {
            self.separator = record[2];
            start = 3;
        }
        if record[start + 1] != self.separator {
            self.delimiter = record[start + 3];
        }
    }
    fn parameters(&mut self, data: &mut IgesFileStructure, record: &IgesFileRecord, length: i32) {
        loop {
            self.params.param(
                data,
                record.as_slice(),
                self.separator,
                self.delimiter,
                length,
            );
            if self.params.status != 2 {
                return;
            }
        }
    }
}

/// The start line of a parameter of the start section: without what ends it below a blank.
fn start_line(parameter: &IgesFileParam) -> &[u8] {
    let text = parameter.text();
    let end = text
        .iter()
        .rposition(|byte| (*byte as i8) > 32)
        .map_or(0, |at| at + 1);
    &text[..end]
}

fn read_header(reader: &IgesDataIgesReaderData, data: &IgesFileStructure) -> Result<(), Failure> {
    for (rank, parameter) in data.start.iter().enumerate() {
        let line = start_line(parameter);
        if !line.is_empty() || rank > 0 {
            reader.add_start_line(line);
        }
    }
    for parameter in &data.header {
        reader.add_global(parameter.kind.param_type(), parameter.text())?;
    }
    reader.set_global_section()
}

/// Where the loading of the entities stands, for its diagnostics.
#[derive(Default)]
struct Recovery {
    entity: i32,
    parameter: i32,
}

fn read_content(
    reader: &IgesDataIgesReaderData,
    data: &IgesFileStructure,
    recovery: &mut Recovery,
) -> Result<(), Failure> {
    for part in &data.parts {
        recovery.parameter = 0;
        recovery.entity = (part.numpart + 1) / 2;
        let values = IgesDataDirValues {
            values: part.values,
            res1: part.res1,
            res2: part.res2,
            label: part.nom,
            subscript: part.num,
        };
        reader.set_dir_part(recovery.entity, &values)?;
        for parameter in &part.list {
            recovery.parameter += 1;
            add_param(reader, recovery.entity, parameter)?;
        }
        reader.init_params(recovery.entity)?;
    }
    Ok(())
}

fn add_param(
    reader: &IgesDataIgesReaderData,
    entity: i32,
    parameter: &IgesFileParam,
) -> Result<(), Failure> {
    let text = parameter.text();
    let is_integer = matches!(
        parameter.kind,
        IgesFileParamKind::Integer | IgesFileParamKind::SignedInteger
    );
    let nument = if is_integer {
        let magnitude = iges_atoi(text).wrapping_abs();
        if magnitude & 1 != 0 {
            (magnitude + 1) / 2
        } else {
            0
        }
    } else {
        0
    };
    reader.add_param(entity, text, parameter.kind.param_type(), nument)
}

fn load_header_and_content(
    reader: &IgesDataIgesReaderData,
    data: &IgesFileStructure,
    diagnostics: &Diagnostics,
) -> Result<(), Failure> {
    if read_header(reader, data).is_err() {
        diagnostics.warning(&iges_msg("XSTEP_11"));
    }
    if data.parts.is_empty() {
        return Ok(());
    }
    let mut recovery = Recovery::default();
    if read_content(reader, data, &mut recovery).is_ok() {
        return Ok(());
    }
    let mut msg;
    if recovery.parameter == 0 {
        msg = iges_msg("XSTEP_13");
        msg.arg_integer(recovery.entity)?;
    } else {
        msg = iges_msg("XSTEP_14");
        msg.arg_integer(recovery.entity)?;
        msg.arg_integer(recovery.parameter)?;
    }
    diagnostics.warning(&msg);
    Ok(())
}

fn load_model(
    reader: &Handle<IgesDataIgesReaderData>,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
    recognizer: &Handle<dyn IgesDataFileRecognizer>,
    diagnostics: &Diagnostics,
) -> Result<(), Failure> {
    let tool = IgesDataIgesReaderTool::new(reader, protocol);
    tool.prepare(recognizer)?;
    tool.set_error_handle(true);
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    tool.load_model(&general)?;
    let live = iges_live(model, "IGES file: read")?;
    if live.protocol().is_null() {
        live.set_protocol(&protocol.cast());
    }
    let check = iges_live(&diagnostics.check, "IGES file: read")?;
    if check.nb_warnings() + check.nb_fails() > 0 {
        let old_global = live.global_check(true);
        if let Some(old) = old_global.get() {
            check.get_messages(old);
        }
        live.set_global_check(&diagnostics.check);
    }
    check.trace(0, 1);
    Ok(())
}

/// Reads IGES data from a stream into a model, the enciphered form when `is_fnes`, using a
/// recognizer when not null. The result is 0, or negative when the data cannot be read.
///
/// # Errors
/// A failure of the loading of the model.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataIgesModel, IgesDataProtocol};
/// use martensite_tkdeiges::iges_file::iges_file_read_stream_with;
/// use martensite_tkernel::standard::Handle;
/// let model = IgesDataIgesModel::new();
/// let protocol = Handle::new(IgesDataProtocol::default());
/// let mut empty: &[u8] = b"";
/// let status = iges_file_read_stream_with("empty", &mut empty, &model, &protocol.cast(), &Handle::default(), false);
/// assert_eq!(status.unwrap(), 0);
/// ```
pub fn iges_file_read_stream_with(
    _name: &str,
    stream: &mut dyn Read,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
    recognizer: &Handle<dyn IgesDataFileRecognizer>,
    is_fnes: bool,
) -> Result<i32, Failure> {
    let diagnostics = Diagnostics {
        check: Handle::new(InterfaceCheck::new()),
    };
    let Some((counts, data)) = parse_records(stream, is_fnes, &diagnostics)? else {
        return Ok(-1);
    };
    let reader = Handle::new(IgesDataIgesReaderData::new(
        (counts[3] + 1) / 2,
        data.nb_params,
    ));
    let live = iges_live(&reader, "IGES file: read")?;
    load_header_and_content(live, &data, &diagnostics)?;
    let mut msg = iges_msg("XSTEP_15");
    msg.arg_integer(live.nb_records())?;
    diagnostics.info(&msg);
    drop(data);
    load_model(&reader, model, protocol, recognizer, &diagnostics)?;
    Ok(0)
}

/// [`iges_file_read_stream_with`] without recognizer, for plain data.
///
/// # Errors
/// A failure of the loading of the model.
pub fn iges_file_read_stream(
    name: &str,
    stream: &mut dyn Read,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
) -> Result<i32, Failure> {
    iges_file_read_stream_with(name, stream, model, protocol, &Handle::default(), false)
}

/// Reads a file into a model, the enciphered form when `is_fnes`, using a recognizer when not null.
/// The result is 0, or -1 when the file cannot be opened or read.
///
/// # Errors
/// A failure of the loading of the model.
pub fn iges_file_read_with(
    file_name: &str,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
    recognizer: &Handle<dyn IgesDataFileRecognizer>,
    is_fnes: bool,
) -> Result<i32, Failure> {
    let Ok(mut file) = File::open(file_name) else {
        return Ok(-1);
    };
    iges_file_read_stream_with(file_name, &mut file, model, protocol, recognizer, is_fnes)
}

/// Reads a plain file into a model. The result is 0, or -1 when it cannot be opened or read.
///
/// # Errors
/// A failure of the loading of the model.
pub fn iges_file_read(
    file_name: &str,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
) -> Result<i32, Failure> {
    iges_file_read_with(file_name, model, protocol, &Handle::default(), false)
}

/// Reads an enciphered file into a model. The result is 0, or -1 when it cannot be opened or read.
///
/// # Errors
/// A failure of the loading of the model.
pub fn iges_file_read_fnes(
    file_name: &str,
    model: &Handle<IgesDataIgesModel>,
    protocol: &Handle<dyn IgesDataProtocolOps>,
) -> Result<i32, Failure> {
    iges_file_read_with(file_name, model, protocol, &Handle::default(), true)
}

#[cfg(test)]
#[path = "read_tests.rs"]
mod read_tests;
