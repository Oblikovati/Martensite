use std::io::{self, Read};

pub(super) struct FakeProbeInput {
    pub(super) bytes: Vec<u8>,
    pub(super) consumed: usize,
    pub(super) calls: usize,
    pub(super) fail_at: Option<usize>,
    pub(super) kernel_failure: bool,
    pub(super) kernel_payload: Option<martensite_tkernel::standard::Failure>,
}

impl FakeProbeInput {
    pub(super) fn unknown(length: usize) -> Self {
        Self {
            bytes: vec![b'x'; length],
            consumed: 0,
            calls: 0,
            fail_at: None,
            kernel_failure: false,
            kernel_payload: None,
        }
    }
}

impl Read for FakeProbeInput {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        self.calls += 1;
        assert_eq!(
            output.len(),
            1,
            "expected the native one-byte get operation"
        );
        if self.fail_at == Some(self.consumed) {
            return Err(self.error());
        }
        let Some(byte) = self.bytes.get(self.consumed) else {
            return Ok(0);
        };
        output[0] = *byte;
        self.consumed += 1;
        Ok(1)
    }
}

impl FakeProbeInput {
    fn error(&self) -> io::Error {
        if let Some(payload) = &self.kernel_payload {
            return io::Error::other(payload.clone());
        }
        if self.kernel_failure {
            return io::Error::other(martensite_tkernel::standard::Failure::with_message(
                "Standard_DomainError",
                "probe sentinel",
            ));
        }
        io::Error::new(
            io::ErrorKind::PermissionDenied,
            ProbeSentinel { marker: 734 },
        )
    }
}

#[derive(Debug)]
pub(super) struct ProbeSentinel {
    pub(super) marker: i32,
}
impl std::fmt::Display for ProbeSentinel {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(output, "probe sentinel {}", self.marker)
    }
}
impl std::error::Error for ProbeSentinel {}
