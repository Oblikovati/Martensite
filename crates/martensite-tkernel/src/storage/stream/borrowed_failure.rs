//! Owned cleanup diagnostics keep operation outcomes and unread input distinct.
use std::{
    any::Any,
    fmt,
    io::{self, Cursor, SeekFrom},
};

/// The borrowed-input boundary that could not complete; `StreamBridgeStage::Rejoin`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamBridgeStage {
    /// The initial observation could not be represented without loss.
    Entry,
    /// The operation returned but its shared input could not be rejoined.
    Rejoin,
}

/// Actual caller observations after a bridge attempt; `observation.position_coherent`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamBridgeObservation {
    /// The state committed to the caller, including a real cleanup failure's bad bit.
    pub state: u8,
    /// The operation's input count, or unknown after unsupported device replacement.
    pub gcount: Option<usize>,
    /// Whether the caller's buffer position agrees with the logical input cursor.
    pub position_coherent: bool,
}

/// An owned panic cause; `payload.is::<MyPanic>()` inspects its original type.
/// It is Send but deliberately not Sync: caller panic payloads need not be Sync.
pub struct StreamPanicPayload(Box<dyn Any + Send>);

impl StreamPanicPayload {
    /// Retain a panic without changing its payload; `StreamPanicPayload::new(payload)`.
    pub(super) fn new(payload: Box<dyn Any + Send>) -> Self {
        let retained: Self = Self(payload);
        retained
    }
    /// Test the retained cause without formatting it; `payload.is::<MyPanic>()`.
    pub fn is<T: Any>(&self) -> bool {
        let payload = self.0.as_ref();
        payload.is::<T>()
    }
    /// Resume exactly the retained panic; `payload.resume()`.
    pub fn resume(self) -> ! {
        let payload = self.0;
        std::panic::resume_unwind(payload)
    }
}
impl fmt::Debug for StreamPanicPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let identity = self.0.as_ref().type_id();
        formatter
            .debug_tuple("StreamPanicPayload")
            .field(&identity)
            .finish()
    }
}

/// A typed reason why the input could not rejoin; `StreamRejoinCause::DeviceReplaced`.
/// Panic-bearing variants are Send, not Sync; they remain invocation-local diagnostics.
#[derive(Debug)]
pub enum StreamRejoinCause {
    /// An initial or final count could not fit the existing input model.
    CountOutOfRange {
        /// The actual unrepresentable count.
        actual: i128,
    },
    /// The callback replaced or removed the borrowed stream's device.
    DeviceReplaced,
    /// A device read panicked before reporting its actual extracted byte count.
    ReadInterrupted,
    /// A callback caught a panic during an unread-input rewind.
    SeekInterrupted {
        /// The actual relative seek interrupted by the panic.
        requested: SeekFrom,
    },
    /// The actual unread-input rewind was refused.
    Seek {
        /// The actual relative seek needed to return unread input.
        requested: SeekFrom,
        /// The device's original typed failure.
        error: io::Error,
    },
    /// Cleanup itself panicked; the original cause remains owned.
    Panicked(StreamPanicPayload),
}

/// Failed cleanup retains the returned outcome; `failure.outcome.as_ref()`.
/// This is Send when R is Send, not Sync because an injected device may panic.
/// It is not a published modelling result or a rollback of consumed input.
pub struct StreamRejoinFailure<R> {
    /// The boundary that failed.
    pub stage: StreamBridgeStage,
    /// The exact typed cleanup cause.
    pub cause: StreamRejoinCause,
    /// None means the operation never returned; Some preserves even an Err outcome.
    pub outcome: Option<R>,
    /// The caller-visible state/count and position-coherence qualification.
    pub observation: StreamBridgeObservation,
    pub(super) unread: Option<Cursor<Vec<u8>>>,
}
impl<R> StreamRejoinFailure<R> {
    /// Borrow retained unread input without copying it; `failure.unread_bytes()`.
    /// None means the original device was replaced. Retained bytes do not certify overall read progress.
    pub fn unread_bytes(&self) -> Option<&[u8]> {
        let unread = self.unread.as_ref()?;
        let offset = unread.position() as usize;
        Some(&unread.get_ref()[offset..])
    }
}
impl<R> fmt::Debug for StreamRejoinFailure<R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StreamRejoinFailure")
            .field("stage", &self.stage)
            .field("cause", &self.cause)
            .field("outcome_returned", &self.outcome.is_some())
            .field("observation", &self.observation)
            .field("unread", &self.unread_bytes())
            .finish()
    }
}
impl<R> fmt::Display for StreamRejoinFailure<R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "borrowed input {:?}: cause={:?}, observation={:?}; expected coherent state, count and unread input",
            self.stage, self.cause, self.observation
        )
    }
}
impl<R> std::error::Error for StreamRejoinFailure<R> {}

/// An operation panic accompanied by failed cleanup; `unwind.primary.is::<MyPanic>()`.
/// Both causes are retained; this Send, non-Sync value is an invocation-local panic payload.
#[derive(Debug)]
pub struct BorrowedInputUnwind {
    /// The operation's exact original panic payload.
    pub primary: StreamPanicPayload,
    /// The separate cleanup failure, with no fabricated operation result.
    pub cleanup: StreamRejoinFailure<()>,
}
