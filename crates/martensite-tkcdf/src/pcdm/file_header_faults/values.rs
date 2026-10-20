//! Forward unaffected steps to the real released memory-backed driver.
use super::*;
impl DataSections for FaultBoundaryDriver {
    fn begin_write_data_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_write_data_section()
    }
    fn write_persistent_object_header(
        &mut self,
        reference: i32,
        type_number: i32,
    ) -> Result<(), Failure> {
        self.inner
            .write_persistent_object_header(reference, type_number)
    }
    fn begin_write_persistent_object_data(&mut self) -> Result<(), Failure> {
        self.inner.begin_write_persistent_object_data()
    }
    fn begin_write_object_data(&mut self) -> Result<(), Failure> {
        self.inner.begin_write_object_data()
    }
    fn end_write_object_data(&mut self) -> Result<(), Failure> {
        self.inner.end_write_object_data()
    }
    fn end_write_persistent_object_data(&mut self) -> Result<(), Failure> {
        self.inner.end_write_persistent_object_data()
    }
    fn end_write_data_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_write_data_section()
    }
    fn begin_read_data_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.begin_read_data_section()
    }
    fn read_persistent_object_header(
        &mut self,
        reference: &mut i32,
        type_number: &mut i32,
    ) -> Result<(), Failure> {
        self.inner
            .read_persistent_object_header(reference, type_number)
    }
    fn begin_read_persistent_object_data(&mut self) -> Result<(), Failure> {
        self.inner.begin_read_persistent_object_data()
    }
    fn begin_read_object_data(&mut self) -> Result<(), Failure> {
        self.inner.begin_read_object_data()
    }
    fn end_read_object_data(&mut self) -> Result<(), Failure> {
        self.inner.end_read_object_data()
    }
    fn end_read_persistent_object_data(&mut self) -> Result<(), Failure> {
        self.inner.end_read_persistent_object_data()
    }
    fn end_read_data_section(&mut self) -> Result<StorageError, Failure> {
        self.inner.end_read_data_section()
    }
}
impl DriverPrimitives for FaultBoundaryDriver {
    fn put_reference(&mut self, value: i32) -> Result<(), Failure> {
        self.inner.put_reference(value)
    }
    fn put_character(&mut self, value: u8) -> Result<(), Failure> {
        self.inner.put_character(value)
    }
    fn put_ext_character(&mut self, value: u16) -> Result<(), Failure> {
        self.inner.put_ext_character(value)
    }
    fn put_integer(&mut self, value: i32) -> Result<(), Failure> {
        self.inner.put_integer(value)
    }
    fn put_boolean(&mut self, value: bool) -> Result<(), Failure> {
        self.inner.put_boolean(value)
    }
    fn put_real(&mut self, value: f64) -> Result<(), Failure> {
        self.inner.put_real(value)
    }
    fn put_short_real(&mut self, value: f32) -> Result<(), Failure> {
        self.inner.put_short_real(value)
    }
    fn get_reference(&mut self, value: &mut i32) -> Result<(), Failure> {
        self.inner.get_reference(value)
    }
    fn get_character(&mut self, value: &mut u8) -> Result<(), Failure> {
        self.inner.get_character(value)
    }
    fn get_ext_character(&mut self, value: &mut u16) -> Result<(), Failure> {
        self.inner.get_ext_character(value)
    }
    fn get_integer(&mut self, value: &mut i32) -> Result<(), Failure> {
        self.inner.get_integer(value)
    }
    fn get_boolean(&mut self, value: &mut bool) -> Result<(), Failure> {
        self.inner.get_boolean(value)
    }
    fn get_real(&mut self, value: &mut f64) -> Result<(), Failure> {
        self.inner.get_real(value)
    }
    fn get_short_real(&mut self, value: &mut f32) -> Result<(), Failure> {
        self.inner.get_short_real(value)
    }
}
