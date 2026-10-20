//! `FSD_BinaryFile::ReadCompleteInfo` over borrowed input, as `PCDM_ReadWriter::FileFormat`
//! calls it on a stream whose magic number was already extracted.
//!
//! The released driver reads an owned `IoStream`, which cannot borrow the caller's reader.
//! The bytes are therefore staged: a prefix is fetched from the reader and the driver runs
//! over it until a run ends without reaching the prefix end (or the reader is exhausted).
//! Driver runs are deterministic, so that run equals a read of the live stream: same data,
//! same logical position, same state. The prefix grows towards the header's information
//! and data ends, so an ordinary container is fetched exactly; bytes fetched past the
//! logical position are returned for the caller to read first.
use super::{PcdmInputStream, PcdmStreamFailure};
use martensite_tkernel::{
    fsd::{BinaryFile, FileHeader, NativeDriverFileSystem},
    standard::{Failure, Handle},
    storage::{Data, HeaderSections, IoStream},
};
use std::{io, rc::Rc, sync::Arc};

const MAGIC: &[u8; 7] = b"BINFILE";
const MIN_STAGE: usize = 64;

/// The borrowed bytes fetched so far, starting with the extracted magic number.
struct StagedPrefix {
    bytes: Vec<u8>,
    exhausted: bool,
    device_error: Option<io::Error>,
}

/// One driver run over a staged prefix.
struct StagedRun {
    result: Result<(), Failure>,
    state: u8,
    position: usize,
    reached_end: bool,
}

/// Read the complete information of a binary container into `storage`.
/// Returns the bytes fetched past the logical position, which precede the live input.
pub(super) fn read_binary_information(
    input: &mut PcdmInputStream<'_>,
    storage: &mut Handle<Data>,
) -> Result<Vec<u8>, PcdmStreamFailure> {
    let mut prefix = StagedPrefix {
        bytes: MAGIC.to_vec(),
        exhausted: false,
        device_error: None,
    };
    while run_over(&prefix.bytes, &mut Handle::default()).reached_end && !prefix.exhausted {
        let wanted = next_length(&prefix.bytes);
        prefix.fetch(input, wanted);
    }
    let run = run_over(&prefix.bytes, storage);
    let device_error = prefix.device_error.take().filter(|_| run.reached_end);
    input.settle_staged(run.position, run.state, device_error)?;
    run.result
        .map_err(|failure| kernel_failure(failure, run.position, input.state()))?;
    Ok(prefix.bytes.split_off(run.position))
}

impl StagedPrefix {
    fn fetch(&mut self, input: &mut PcdmInputStream<'_>, wanted: usize) {
        while self.bytes.len() < wanted {
            match input.fetch_ahead() {
                Ok(Some(byte)) => self.bytes.push(byte),
                Ok(None) => return self.exhaust(None),
                Err(cause) => return self.exhaust(Some(cause)),
            }
        }
    }

    fn exhaust(&mut self, device_error: Option<io::Error>) {
        self.exhausted = true;
        self.device_error = device_error;
    }
}

/// The next prefix length: doubled, but not past a known information or data end.
fn next_length(bytes: &[u8]) -> usize {
    let doubled = (bytes.len() * 2).max(MIN_STAGE);
    match header_end(bytes) {
        Some(end) if end > bytes.len() => doubled.min(end),
        _ => doubled,
    }
}

/// `max(einfo, edata)` of a staged header, where an ordinary complete read stops.
fn header_end(bytes: &[u8]) -> Option<usize> {
    let mut stream = stream_after_magic(bytes);
    let mut header = FileHeader::default();
    BinaryFile::read_header_stream(&mut stream, &mut header).ok()?;
    usize::try_from(header.end_info.max(header.end_data)).ok()
}

fn stream_after_magic(bytes: &[u8]) -> IoStream {
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    stream.read(&mut [0; MAGIC.len()]);
    stream
}

fn run_over(bytes: &[u8], storage: &mut Handle<Data>) -> StagedRun {
    let mut stream = stream_after_magic(bytes);
    let mut driver = BinaryFile::new(Rc::new(NativeDriverFileSystem));
    let result = driver.read_complete_info(&mut stream, storage);
    let reached_end = stream.eof();
    let state = stream.rdstate();
    let position = if stream.good() {
        usize::try_from(stream.tellg()).unwrap_or(bytes.len())
    } else {
        bytes.len()
    };
    StagedRun {
        result,
        state,
        position,
        reached_end,
    }
}

fn kernel_failure(failure: Failure, consumed: usize, state: u8) -> PcdmStreamFailure {
    let failure = super::stream_format::native_stream_failure(failure);
    PcdmStreamFailure::new(Arc::new(io::Error::other(failure)), consumed, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(end_info: i32, end_data: i32) -> Vec<u8> {
        let mut bytes = MAGIC.to_vec();
        let mut fields = [0; 13];
        (fields[2], fields[12]) = (end_info, end_data);
        fields
            .iter()
            .for_each(|field| bytes.extend(field.to_ne_bytes()));
        bytes
    }

    #[test]
    fn stage_grows_to_the_header_end_but_never_past_it() {
        assert_eq!(next_length(MAGIC), MIN_STAGE);
        assert_eq!(next_length(&header(70, 90)), 90);
        assert_eq!(next_length(&header(500, 90)), 118);
    }

    #[test]
    fn stage_doubles_without_a_usable_header_end() {
        assert_eq!(next_length(&header(-1, -1)), 118);
        assert_eq!(next_length(&header(10, 20)), 118);
    }
}
