//! Forward unaffected steps to the real released memory-backed driver.
use super::*;
impl TableSections for FaultBoundaryDriver {
    fn begin_write_type_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_type_section()
    }
    fn set_type_section_size(&mut self, size: i32) -> Result<(), Failure> {
        self.inner.set_type_section_size(size)
    }
    fn write_type_informations(&mut self, number: i32, name: &AsciiString) -> Result<(), Failure> {
        self.inner.write_type_informations(number, name)
    }
    fn end_write_type_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_type_section()
    }
    fn begin_read_type_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_type_section()
    }
    fn type_section_size(&mut self) -> Result<i32, Failure> {
        self.inner.type_section_size()
    }
    fn read_type_informations(
        &mut self,
        number: &mut i32,
        name: &mut AsciiString,
    ) -> Result<(), Failure> {
        self.inner.read_type_informations(number, name)
    }
    fn end_read_type_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_type_section()
    }
    fn begin_write_root_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_root_section()
    }
    fn set_root_section_size(&mut self, size: i32) -> Result<(), Failure> {
        self.inner.set_root_section_size(size)
    }
    fn write_root(
        &mut self,
        name: &AsciiString,
        reference: i32,
        type_name: &AsciiString,
    ) -> Result<(), Failure> {
        self.inner.write_root(name, reference, type_name)
    }
    fn end_write_root_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_root_section()
    }
    fn begin_read_root_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_root_section()
    }
    fn root_section_size(&mut self) -> Result<i32, Failure> {
        self.inner.root_section_size()
    }
    fn read_root(
        &mut self,
        name: &mut AsciiString,
        reference: &mut i32,
        type_name: &mut AsciiString,
    ) -> Result<(), Failure> {
        self.inner.read_root(name, reference, type_name)
    }
    fn end_read_root_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_root_section()
    }
    fn begin_write_ref_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_ref_section()
    }
    fn set_ref_section_size(&mut self, size: i32) -> Result<(), Failure> {
        self.inner.set_ref_section_size(size)
    }
    fn write_reference_type(&mut self, reference: i32, type_number: i32) -> Result<(), Failure> {
        self.inner.write_reference_type(reference, type_number)
    }
    fn end_write_ref_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_ref_section()
    }
    fn begin_read_ref_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_ref_section()
    }
    fn ref_section_size(&mut self) -> Result<i32, Failure> {
        self.inner.ref_section_size()
    }
    fn read_reference_type(
        &mut self,
        reference: &mut i32,
        type_number: &mut i32,
    ) -> Result<(), Failure> {
        self.inner.read_reference_type(reference, type_number)
    }
    fn end_read_ref_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_ref_section()
    }
}
