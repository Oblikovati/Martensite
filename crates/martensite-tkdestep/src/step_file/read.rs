//! `the file read`: reads a STEP file into a model.
use super::{StepFileReadData, parser};
use crate::step_data::{
    StepDataFileRecognizer, StepDataProtocolOps, StepDataStepModel, StepDataStepReaderData,
    StepDataStepReaderTool,
};
use martensite_tkernel::{
    message::Message,
    standard::{Failure, Handle},
};
use martensite_tkxsbase::interface::{
    InterfaceFileReaderData, InterfaceFileReaderDataParams, InterfaceFileReaderToolLoad,
    InterfaceFileReaderToolOps, InterfaceInterfaceModel, InterfaceModelEntities,
};
use std::fmt::Write as _;
use std::io::Read;
use std::sync::Mutex;

/// `GetGlobalReadMutex`: the lock around the conversion of a parsed file.
static GLOBAL_READ_MUTEX: Mutex<()> = Mutex::new(());

/// `the parser interrupt(message, isFail = true)`: the error as a fail, or a trace.
pub fn step_file_interrupt(message: Option<&str>, is_fail: bool) {
    let Some(message) = message else {
        return;
    };
    let text = format!("**** ERR StepFile : {message}    ****\n");
    if is_fail {
        let _ = Message::send_fail().write_str(&text);
    } else {
        let _ = Message::send_trace().write_str(&text);
    }
}

fn load_text(name: &str, stream: Option<&mut dyn Read>) -> Option<Vec<u8>> {
    let mut text = Vec::new();
    match stream {
        Some(stream) => stream.read_to_end(&mut text).ok()?,
        None => {
            text = std::fs::read(name).ok()?;
            text.len()
        }
    };
    Some(text)
}

/// `the file read(theName, theIStream, theModel, theProtocol)`: reads the file `name`, or
/// `stream` when given. Returns 0 on success, -1 if the input fails, 1 on a parsing error.
/// `Err` is what the loading raises, an exception out of this function.
///
/// ```no_run
/// use martensite_tkdestep::{step_data::{StepData, StepDataStepModel}, step_file::step_file_read};
/// let model = StepDataStepModel::new();
/// let status = step_file_read("part.stp", None, &model, &StepData::protocol()).unwrap();
/// assert_eq!(status, 0);
/// ```
pub fn step_file_read(
    name: &str,
    stream: Option<&mut dyn Read>,
    model: &Handle<StepDataStepModel>,
    protocol: &Handle<dyn StepDataProtocolOps>,
) -> Result<i32, Failure> {
    let Some(text) = load_text(name, stream) else {
        return Ok(-1);
    };
    let _ = write!(
        Message::send_trace(),
        "      ...    Step File Reading : '{name}'"
    );
    let mut file_data = StepFileReadData::new();
    match parser::parse(&text, &mut file_data) {
        Ok(0) => {}
        Ok(_) => {
            step_file_interrupt(file_data.last_error(), true);
            return Ok(1);
        }
        Err(failure) => {
            let _ = write!(
                Message::send_fail(),
                " ...  Exception Raised while reading Step File : '{name}':\n{failure}    ..."
            );
            return Ok(1);
        }
    }
    let _ = writeln!(
        Message::send_trace(),
        "      ...    STEP File   Read    ..."
    );
    let _lock = GLOBAL_READ_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let reader = fill_reader_data(&mut file_data, model)?;
    let live = reader.get().expect("the new reader data");
    let fails = live.global_check().nb_fails();
    if fails > 0 {
        let _ = write!(
            Message::send_info(),
            "**** ERR StepFile : Incorrect Syntax : Fails Count : {fails} ****"
        );
    }
    file_data.clear_recorder(1);
    analyse(live, &reader, model, protocol, fails)?;
    let count = model.get().map_or(0, |m| m.nb_entities());
    let _ = write!(
        Message::send_trace(),
        "  STEP Loading done : {count} Entities"
    );
    Ok(0)
}

/// The records of the parser, in a `StepDataStepReaderData`.
fn fill_reader_data(
    file_data: &mut StepFileReadData,
    model: &Handle<StepDataStepModel>,
) -> Result<Handle<StepDataStepReaderData>, Failure> {
    let (nbhead, nbrec, nbpar) = file_data.get_file_nb_r();
    let code_page = model.get().map(StepDataStepModel::source_code_page);
    let reader = StepDataStepReaderData::new(
        nbhead,
        nbrec,
        nbpar,
        code_page.unwrap_or(martensite_tkernel::resource::FormatType::Utf8),
    );
    for num in 1..=nbrec {
        if let Some((ident, record_type, nbarg)) = file_data.get_record_description() {
            reader.set_record(num, &ident, &record_type, nbarg);
            while let Some((kind, value)) = file_data.get_arg_description() {
                reader.add_step_param(num, &value, kind, 0)?;
            }
        }
        reader.init_params(num)?;
        file_data.next_record();
    }
    file_data.error_handle(&reader.global_check());
    Ok(Handle::new(reader))
}

/// The analysis by the reader tool.
fn analyse(
    live: &StepDataStepReaderData,
    reader: &Handle<StepDataStepReaderData>,
    model: &Handle<StepDataStepModel>,
    protocol: &Handle<dyn StepDataProtocolOps>,
    fails: i32,
) -> Result<(), Failure> {
    let _ = writeln!(
        Message::send_trace(),
        "      ... Step File loaded  ...\n   {} records (entities,sub-lists,scopes), {} parameters",
        live.nb_records(),
        live.nb_params(0)?
    );
    let tool = StepDataStepReaderTool::new(reader, protocol);
    tool.set_error_handle(true);
    let no_recognizer = Handle::<dyn StepDataFileRecognizer>::default();
    tool.prepare_header(&no_recognizer)?;
    tool.prepare_with(&no_recognizer, true)?;
    let _ = writeln!(Message::send_trace(), "      ... Parameters prepared ...");
    let any_model: Handle<dyn InterfaceInterfaceModel> = model.cast();
    tool.load_model(&any_model)?;
    if let Some(live_model) = model.get()
        && live_model.protocol().is_null()
    {
        live_model.set_protocol(&protocol.cast());
    }
    let unresolved = live.global_check().nb_fails() - fails;
    if unresolved > 0 {
        let _ = write!(
            Message::send_info(),
            "*** ERR StepReaderData : Unresolved Reference : Fails Count : {unresolved} ***"
        );
    }
    tool.clear();
    let _ = writeln!(Message::send_trace(), "      ...   Objects analysed  ...");
    Ok(())
}
