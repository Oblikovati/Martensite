//! A scoped input borrow rejoins state, counts and buffered bytes before returning.
use super::borrowed_failure::{
    BorrowedInputUnwind, StreamBridgeObservation, StreamBridgeStage, StreamPanicPayload,
    StreamRejoinCause, StreamRejoinFailure,
};
use super::{IoStream, borrowed_session::BorrowedSession, state};
use crate::osd::{IStream, IosOpenMode, StreamIo};
use std::{
    any::Any,
    io::{Cursor, SeekFrom},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::Arc,
};

struct InputBorrow<'a> {
    caller: &'a mut StreamIo,
    stream: IoStream,
    session: Arc<BorrowedSession>,
    initial_state: u8,
    initial_count: usize,
}

/// Run an existing decoder on the caller's shared input; `with_borrowed_input_stream(source, |stream| decode(stream))?`.
/// The caller grants exclusive operation access. No buffer lock spans the callback.
/// Reopening/replacing the provided device is rejected, not rolled back. An ordinary
/// callback Err remains its exact R value; cleanup failure instead owns R and unread input.
/// A callback panic resumes unchanged after successful cleanup; failed cleanup resumes
/// a [`BorrowedInputUnwind`] containing both original causes. No panic becomes success.
pub fn with_borrowed_input_stream<R>(
    source: &mut dyn IStream,
    operation: impl FnOnce(&mut IoStream) -> R,
) -> Result<R, StreamRejoinFailure<R>> {
    let mut scope = InputBorrow::new(source)?;
    let outcome = catch_unwind(AssertUnwindSafe(|| operation(&mut scope.stream)));
    let cleanup = catch_cleanup(&mut scope);
    match outcome {
        Ok(value) => finish_return(scope, value, cleanup),
        Err(payload) => finish_unwind(scope, payload, cleanup),
    }
}

impl<'a> InputBorrow<'a> {
    fn new<R>(source: &'a mut dyn IStream) -> Result<Self, StreamRejoinFailure<R>> {
        let caller = source.stream_io_mut();
        let (buffer, bits, count) = caller.storage_snapshot();
        let signed = i64::try_from(count).map_err(|_| entry_failure(bits, count))?;
        let mut stream = IoStream::with_shared_buffer(buffer, IosOpenMode::IN, true, false);
        let session = Arc::new(BorrowedSession::default());
        stream.state = bits;
        stream.gcount = signed;
        stream.borrowed_session = Some(Arc::clone(&session));
        Ok(Self {
            caller,
            stream,
            session,
            initial_state: bits,
            initial_count: count,
        })
    }
    fn same_device(&self) -> bool {
        self.stream
            .borrowed_session
            .as_ref()
            .is_some_and(|token| Arc::ptr_eq(token, &self.session))
    }
    fn rejoin(&mut self) -> Result<(), StreamRejoinCause> {
        if let Some(cause) = self.session.take_fault() {
            return Err(cause);
        }
        if !self.same_device() {
            return Err(StreamRejoinCause::DeviceReplaced);
        }
        let count = usize::try_from(self.stream.gcount).map_err(|_| {
            StreamRejoinCause::CountOutOfRange {
                actual: i128::from(self.stream.gcount),
            }
        })?;
        self.return_lookahead()?;
        self.caller
            .commit_storage_observation(self.stream.state, count);
        Ok(())
    }
    fn return_lookahead(&mut self) -> Result<(), StreamRejoinCause> {
        let unread = self.stream.pending.len() - self.stream.cursor;
        if unread > 0 {
            self.rewind(unread)?;
        }
        self.stream.pending.clear();
        self.stream.cursor = 0;
        Ok(())
    }
    fn rewind(&mut self, unread: usize) -> Result<(), StreamRejoinCause> {
        let signed = i64::try_from(unread).map_err(|_| StreamRejoinCause::CountOutOfRange {
            actual: unread as i128,
        })?;
        let requested = SeekFrom::Current(-signed);
        let Some(device) = self.stream.device.as_mut() else {
            return Err(StreamRejoinCause::DeviceReplaced);
        };
        device
            .seek(requested)
            .map(|_| ())
            .map_err(|error| StreamRejoinCause::Seek { requested, error })
    }
    fn failure<R>(
        &mut self,
        outcome: Option<R>,
        cause: StreamRejoinCause,
    ) -> StreamRejoinFailure<R> {
        let (trusted, observation) = self.failed_observation();
        self.caller.commit_storage_observation(
            observation.state,
            observation.gcount.unwrap_or(self.initial_count),
        );
        let unread = trusted.then(|| take_pending(&mut self.stream));
        StreamRejoinFailure {
            stage: StreamBridgeStage::Rejoin,
            cause,
            outcome,
            observation,
            unread,
        }
    }
    fn failed_observation(&self) -> (bool, StreamBridgeObservation) {
        let trusted = self.same_device();
        let count = trusted
            .then(|| usize::try_from(self.stream.gcount).ok())
            .flatten();
        let original = if trusted {
            self.stream.state
        } else {
            self.initial_state
        };
        let state = original | state::BAD;
        (
            trusted,
            StreamBridgeObservation {
                state,
                gcount: count,
                position_coherent: false,
            },
        )
    }
}

fn take_pending(stream: &mut IoStream) -> Cursor<Vec<u8>> {
    let mut unread = Cursor::new(std::mem::take(&mut stream.pending));
    unread.set_position(stream.cursor as u64);
    stream.cursor = 0;
    unread
}

fn entry_failure<R>(bits: u8, count: usize) -> StreamRejoinFailure<R> {
    let observation = StreamBridgeObservation {
        state: bits,
        gcount: Some(count),
        position_coherent: true,
    };
    StreamRejoinFailure {
        stage: StreamBridgeStage::Entry,
        cause: StreamRejoinCause::CountOutOfRange {
            actual: count as i128,
        },
        outcome: None,
        observation,
        unread: Some(Cursor::new(Vec::new())),
    }
}

fn catch_cleanup(scope: &mut InputBorrow<'_>) -> Result<(), StreamRejoinCause> {
    match catch_unwind(AssertUnwindSafe(|| scope.rejoin())) {
        Ok(result) => result,
        Err(payload) => Err(StreamRejoinCause::Panicked(StreamPanicPayload::new(
            payload,
        ))),
    }
}

fn finish_return<R>(
    mut scope: InputBorrow<'_>,
    outcome: R,
    cleanup: Result<(), StreamRejoinCause>,
) -> Result<R, StreamRejoinFailure<R>> {
    match cleanup {
        Ok(()) => Ok(outcome),
        Err(cause) => Err(scope.failure(Some(outcome), cause)),
    }
}

fn finish_unwind(
    mut scope: InputBorrow<'_>,
    payload: Box<dyn Any + Send>,
    cleanup: Result<(), StreamRejoinCause>,
) -> ! {
    let cause = match cleanup {
        Ok(()) => {
            drop(scope);
            resume_unwind(payload)
        }
        Err(cause) => cause,
    };
    let cleanup = scope.failure(None, cause);
    let retained = BorrowedInputUnwind {
        primary: StreamPanicPayload::new(payload),
        cleanup,
    };
    drop(scope);
    resume_unwind(Box::new(retained))
}
