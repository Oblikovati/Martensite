//! `CDF_Application::Read(theIStream, theDocument, theFilter, theRange)`: the format probe,
//! the target document, then the reader of that format on the rest of the stream.
use super::CdfApplication;
use crate::cdm::CdmDocument;
use crate::live;
use crate::pcdm::{
    PcdmInputReader, PcdmInputStream, PcdmReader, PcdmReaderFilter, PcdmReaderStatus,
    PcdmStreamFailure, ProbedFormat, probe_input_format,
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    storage::Data,
    tcollection::ExtendedString,
};

/// The caller's document and reading options.
pub(crate) struct StreamRead<'c, 'r, 'p> {
    pub(crate) document: &'c mut Handle<dyn CdmDocument>,
    pub(crate) filter: &'c Handle<PcdmReaderFilter>,
    pub(crate) range: &'r ProgressRange<'p>,
}

/// The whole read on explicit-state input.
pub(crate) fn read_input<A: CdfApplication + ?Sized>(
    application: &A,
    input: &mut PcdmInputStream<'_>,
    mut read: StreamRead<'_, '_, '_>,
) -> Result<(), PcdmStreamFailure> {
    let mut storage = Handle::default();
    let probed = probe_input_format(input, &mut storage)
        .map_err(|failure| classify_read_failure(application, failure))?;
    if probed.format.is_empty() {
        set_status(application, PcdmReaderStatus::FormatFailure);
        return Ok(());
    }
    let reader = application
        .reader_from_format(&probed.format)
        .map_err(|failure| propagated(failure, input))?;
    if !prepare_document(application, &probed.format, &mut read)
        .map_err(|failure| propagated(failure, input))?
    {
        return Ok(());
    }
    dispatch(application, input, &reader, probed, storage, read)
}

/// The document the reader fills: a new one, or in append mode the caller's document of
/// the same format. False when a status ends the read.
fn prepare_document<A: CdfApplication + ?Sized>(
    application: &A,
    format: &ExtendedString,
    read: &mut StreamRead<'_, '_, '_>,
) -> Result<bool, Failure> {
    let append = read
        .filter
        .get()
        .is_some_and(PcdmReaderFilter::is_append_mode);
    if !append {
        application.new_document(format, read.document)?;
        return Ok(true);
    }
    let Some(document) = read.document.get() else {
        set_status(application, PcdmReaderStatus::NoDocument);
        return Ok(false);
    };
    if document.storage_format() != *format {
        set_status(application, PcdmReaderStatus::FormatFailure);
        return Ok(false);
    }
    Ok(true)
}

/// The reader reads the rest of the stream: the staged bytes, then the live input.
fn dispatch<A: CdfApplication + ?Sized>(
    application: &A,
    input: &mut PcdmInputStream<'_>,
    reader: &Handle<dyn PcdmReader>,
    probed: ProbedFormat,
    storage: Handle<Data>,
    read: StreamRead<'_, '_, '_>,
) -> Result<(), PcdmStreamFailure> {
    let reader = live(reader, "CDF_Application::Read").map_err(|f| propagated(f, input))?;
    let this = application
        .application_base()
        .this()
        .map_err(|failure| propagated(failure, input))?;
    let mut source = PcdmInputReader::new(input, probed.ahead);
    let outcome = reader.read_stream(
        &mut source,
        &storage,
        read.document,
        &this,
        read.filter,
        read.range,
    );
    let (consumed, state) = (source.extracted(), source.state());
    let caught = match source.finish() {
        Ok(()) => outcome.err(),
        Err(thrown) => Some(caught_stream_exception(thrown)?),
    };
    finish_read(application, reader, caught, consumed, state)
}

/// The native catch: the reader's status, and a rethrow past `PCDM_RS_AlreadyRetrieved`.
fn finish_read<A: CdfApplication + ?Sized>(
    application: &A,
    reader: &dyn PcdmReader,
    caught: Option<Failure>,
    consumed: usize,
    state: u8,
) -> Result<(), PcdmStreamFailure> {
    set_status(application, reader.get_status());
    match caught {
        Some(failure) if reader.get_status() > PcdmReaderStatus::AlreadyRetrieved => Err(
            PcdmStreamFailure::rethrown_by_application_read(failure, consumed, state),
        ),
        _ => Ok(()),
    }
}

/// Only a kernel exception from the stream reaches the native `Standard_Failure` catch.
fn caught_stream_exception(thrown: PcdmStreamFailure) -> Result<Failure, PcdmStreamFailure> {
    match thrown.kernel_failure() {
        Some(failure) => Ok(failure.clone()),
        None => Err(thrown.in_application_read()),
    }
}

fn propagated(failure: Failure, input: &PcdmInputStream<'_>) -> PcdmStreamFailure {
    PcdmStreamFailure::propagated_by_application_read(failure, input.extracted(), input.state())
}

fn set_status<A: CdfApplication + ?Sized>(application: &A, status: PcdmReaderStatus) {
    application
        .cdf_application_base()
        .set_retrievable_status(status);
}

fn classify_read_failure<A: CdfApplication + ?Sized>(
    application: &A,
    failure: PcdmStreamFailure,
) -> PcdmStreamFailure {
    // CDF_Application::Read catches Standard_Failure, not arbitrary device exceptions.
    if failure.kernel_failure().is_some() {
        set_status(application, PcdmReaderStatus::FormatFailure);
    }
    failure.in_application_read()
}
