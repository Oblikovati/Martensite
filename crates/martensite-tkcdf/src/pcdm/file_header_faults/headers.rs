//! Forward unaffected steps to the real released memory-backed driver.
use super::*;
impl HeaderSections for FaultBoundaryDriver {
    fn begin_write_info_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_info_section()
    }
    fn write_info(
        &mut self,
        object_count: i32,
        storage_version: &AsciiString,
        date: &AsciiString,
        schema_name: &AsciiString,
        schema_version: &AsciiString,
        application_name: &ExtendedString,
        application_version: &AsciiString,
        data_type: &ExtendedString,
        user_info: &Sequence<AsciiString>,
    ) -> Result<(), Failure> {
        self.inner.write_info(
            object_count,
            storage_version,
            date,
            schema_name,
            schema_version,
            application_name,
            application_version,
            data_type,
            user_info,
        )
    }
    fn end_write_info_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_info_section()
    }
    fn begin_read_info_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_info_section()
    }
    fn read_info(
        &mut self,
        object_count: &mut i32,
        storage_version: &mut AsciiString,
        date: &mut AsciiString,
        schema_name: &mut AsciiString,
        schema_version: &mut AsciiString,
        application_name: &mut ExtendedString,
        application_version: &mut AsciiString,
        data_type: &mut ExtendedString,
        user_info: &mut Sequence<AsciiString>,
    ) -> Result<(), Failure> {
        self.inner.read_info(
            object_count,
            storage_version,
            date,
            schema_name,
            schema_version,
            application_name,
            application_version,
            data_type,
            user_info,
        )
    }
    fn read_complete_info(
        &mut self,
        stream: &mut IoStream,
        data: &mut Handle<Data>,
    ) -> Result<(), Failure> {
        self.inner.read_complete_info(stream, data)
    }
    fn end_read_info_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_info_section()
    }
    fn begin_write_comment_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_comment_section()
    }
    fn write_comment(&mut self, comments: &Sequence<ExtendedString>) -> Result<(), Failure> {
        self.inner.write_comment(comments)
    }
    fn end_write_comment_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_comment_section()
    }
    fn begin_read_comment_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_comment_section()
    }
    fn read_comment(&mut self, comments: &mut Sequence<ExtendedString>) -> Result<(), Failure> {
        self.inner.read_comment(comments)
    }
    fn end_read_comment_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_comment_section()
    }
}
