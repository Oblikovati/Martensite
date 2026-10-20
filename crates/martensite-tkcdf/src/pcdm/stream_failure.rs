//! Typed input failure with its original device error, independent of header decoding.
use martensite_tkernel::standard::Failure;
use std::{fmt, io, sync::Arc};

/// A thrown stream error, retaining the original typed cause and extraction position.
/// Example: `assert_eq!(failure.operation(), "PCDM_ReadWriter::FileFormat(stream)");`.
#[derive(Clone, Debug)]
pub struct PcdmStreamFailure {
    cause: Arc<io::Error>,
    consumed: usize,
    state: u8,
    operation: &'static str,
    kernel_failure: Option<Arc<Failure>>,
}

impl PcdmStreamFailure {
    pub(crate) fn new(cause: Arc<io::Error>, consumed: usize, state: u8) -> Self {
        let kernel_failure = cause
            .get_ref()
            .and_then(|inner| inner.downcast_ref::<Failure>())
            .cloned()
            .map(Arc::new);
        Self {
            cause,
            consumed,
            state,
            operation: "PCDM_ReadWriter::FileFormat(stream)",
            kernel_failure,
        }
    }

    /// Identify the operation that encountered the device failure.
    /// Example: `assert!(failure.operation().contains("FileFormat"));`.
    pub fn operation(&self) -> &'static str {
        self.operation
    }

    /// Total bytes extracted before the failing one-byte request, including declaration bytes.
    /// Example: `assert!(failure.consumed() < 7);`.
    pub fn consumed(&self) -> usize {
        self.consumed
    }

    /// State bits at the instant the exception leaves the stream operation.
    /// Example: `assert_ne!(failure.state(), 0);`.
    pub fn state(&self) -> u8 {
        self.state
    }

    /// Borrow the actual device error, including its custom payload.
    /// Example: `assert_eq!(failure.cause().kind(), std::io::ErrorKind::PermissionDenied);`.
    pub fn cause(&self) -> &io::Error {
        &self.cause
    }

    /// Distinguish a kernel exception from an unrelated device exception.
    /// Example: `assert!(failure.kernel_failure().is_none());`.
    pub fn kernel_failure(&self) -> Option<&Failure> {
        self.kernel_failure.as_deref()
    }

    /// A kernel exception that leaves `CDF_Application::Read(stream)` unchanged.
    pub(crate) fn propagated_by_application_read(
        failure: Failure,
        consumed: usize,
        state: u8,
    ) -> Self {
        let mut thrown = Self::new(Arc::new(io::Error::other(failure)), consumed, state);
        thrown.operation = "CDF_Application::Read(stream)";
        thrown
    }

    /// A kernel exception `CDF_Application::Read(stream)` catches and rethrows printed.
    pub(crate) fn rethrown_by_application_read(
        failure: Failure,
        consumed: usize,
        state: u8,
    ) -> Self {
        Self::new(Arc::new(io::Error::other(failure)), consumed, state).in_application_read()
    }

    pub(crate) fn in_driver_probe(mut self) -> Self {
        self.operation = "PCDM::FileDriverType(stream)";
        self
    }

    pub(crate) fn in_file_format(mut self) -> Self {
        self.operation = "PCDM_ReadWriter::FileFormat(stream)";
        self
    }

    pub(crate) fn in_application_read(mut self) -> Self {
        self.operation = "CDF_Application::Read(stream)";
        self.kernel_failure = self.kernel_failure.map(|failure| {
            // CDF's ostream insertion preserves message/stack bytes, unlike Display's UTF-8 view.
            let mut message = Vec::new();
            failure
                .print_to(&mut message)
                .expect("printing Failure into Vec<u8> cannot fail");
            message.push(b'\n');
            Arc::new(Failure::from_message_bytes(&message))
        });
        self
    }
}

impl fmt::Display for PcdmStreamFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let phase = if self.consumed <= 7 {
            "of 7 probe bytes"
        } else {
            "input bytes after the 7-byte probe"
        };
        write!(
            output,
            "{}: extracted {} {phase}, state {}: {}",
            self.operation(),
            self.consumed,
            self.state,
            self.cause
        )
    }
}

impl std::error::Error for PcdmStreamFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause())
    }
}
