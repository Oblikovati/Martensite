//! Explicit provider failures pin the private catch boundary, not a public filename fault seam.
use super::file_header::query_catching_read_failure;
use super::file_header_test_support::{FROZEN_FILES, FakeHeaderFiles};
use martensite_tkernel::{
    fsd::BinaryFile,
    ncollection::Sequence,
    standard::{Failure, Handle},
    storage::{
        BaseDriver, Data, DataSections, DriverBase, DriverPrimitives, HeaderSections, IoStream,
        OpenMode, Position, StorageError, TableSections,
    },
    tcollection::{AsciiString, ExtendedString},
};
#[path = "file_header_faults/headers.rs"]
mod headers;
#[path = "file_header_faults/tables.rs"]
mod tables;
#[path = "file_header_faults/values.rs"]
mod values;

struct FaultBoundaryDriver {
    inner: BinaryFile,
    trace: Vec<&'static str>,
    open_failure: bool,
    close_failure: bool,
}
fn provider_failure() -> Failure {
    Failure::with_type_bytes("Standard_DomainError", b"boundary\xffcause", b"owned-stack")
}
impl FaultBoundaryDriver {
    fn new(open_failure: bool, close_failure: bool) -> Self {
        Self {
            inner: BinaryFile::new(FakeHeaderFiles::new(FROZEN_FILES[2].1[0], None)),
            trace: Vec::new(),
            open_failure,
            close_failure,
        }
    }
}
impl BaseDriver for FaultBoundaryDriver {
    fn driver_base(&self) -> &DriverBase {
        self.inner.driver_base()
    }
    fn driver_base_mut(&mut self) -> &mut DriverBase {
        self.inner.driver_base_mut()
    }
    fn open(&mut self, name: &AsciiString, mode: OpenMode) -> Result<StorageError, Failure> {
        self.trace.push("open");
        if self.open_failure {
            return Err(provider_failure());
        }
        self.inner.open(name, mode)
    }
    fn is_end(&mut self) -> Result<bool, Failure> {
        self.inner.is_end()
    }
    fn tell(&mut self) -> Result<Position, Failure> {
        self.trace.push("query");
        self.inner.tell()
    }
    fn skip_object(&mut self) -> Result<(), Failure> {
        self.inner.skip_object()
    }
    fn close(&mut self) -> Result<StorageError, Failure> {
        self.trace.push("close");
        if self.close_failure {
            return Err(provider_failure());
        }
        self.inner.close()
    }
}
fn publish_query(driver: &mut dyn BaseDriver, output: &mut i32) -> Result<(), Failure> {
    driver.tell()?;
    *output = 9;
    Ok(())
}
fn fail_after_publication(driver: &mut dyn BaseDriver, output: &mut i32) -> Result<(), Failure> {
    publish_query(driver, output)?;
    Err(provider_failure())
}

#[test]
fn query_open_failure_is_caught_without_querying_or_closing() {
    let mut driver = FaultBoundaryDriver::new(true, false);
    let output = query_catching_read_failure(
        &mut driver,
        &ExtendedString::new("owned.fsd"),
        7,
        publish_query,
    );
    assert_eq!(output.unwrap(), 7);
    assert_eq!(driver.trace, ["open"]);
    assert_eq!(driver.open_mode(), OpenMode::None);
}
#[test]
fn query_failure_preserves_the_completed_output_then_closes_once() {
    let mut driver = FaultBoundaryDriver::new(false, false);
    let output = query_catching_read_failure(
        &mut driver,
        &ExtendedString::new("owned.fsd"),
        7,
        fail_after_publication,
    );
    assert_eq!(output.unwrap(), 9);
    assert_eq!(driver.trace, ["open", "query", "close"]);
    assert_eq!(driver.open_mode(), OpenMode::None);
}
#[test]
fn close_failure_propagates_after_successful_or_failed_query() {
    for read in [publish_query, fail_after_publication] {
        let mut driver = FaultBoundaryDriver::new(false, true);
        let output =
            query_catching_read_failure(&mut driver, &ExtendedString::new("owned.fsd"), 7, read);
        assert_eq!(output.unwrap_err(), provider_failure());
        assert_eq!(driver.trace, ["open", "query", "close"]);
        assert_eq!(driver.open_mode(), OpenMode::Read);
    }
}

#[test]
fn query_success_closes_before_returning_the_completed_output() {
    let mut driver = FaultBoundaryDriver::new(false, false);
    let output = query_catching_read_failure(
        &mut driver,
        &ExtendedString::new("owned.fsd"),
        7,
        publish_query,
    );
    assert_eq!(output.unwrap(), 9);
    assert_eq!(driver.trace, ["open", "query", "close"]);
    assert_eq!(driver.open_mode(), OpenMode::None);
}
