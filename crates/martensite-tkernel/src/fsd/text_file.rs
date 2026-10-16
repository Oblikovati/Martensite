//! The text and compatible text container drivers over an
//! `std::fstream` (modelled by [`IoStream`]) in the "C" locale with 17 significant digits.
//!
//! The two dialects share every member except the virtual line readers and the object
//! framing that the compatible dialect overrides; [`TextFile`] implements the shared members once
//! and a [`TextDialect`] supplies the overrides, so the shared members still reach
//! the compatible dialect's line readers, as overridable methods must.
use super::{DriverFileSystem, FileOpenMode};
use crate::standard::Failure;
use crate::storage::{BaseDriver, DriverBase, IoStream, OpenMode, Position, StorageError};
use crate::tcollection::AsciiString;
use std::rc::Rc;
mod dialect;
mod hooks;
mod lines;
mod objects;
mod scalars;
mod sections;
mod tables;
pub use dialect::{CmpDialect, FsdDialect, TextDialect};

/// The text driver with the `FSDFILE` magic number.
pub type File = TextFile<FsdDialect>;

/// The compatible text driver with the `CMPFILE` magic number.
pub type CmpFile = TextFile<CmpDialect>;

/// A text storage driver of dialect `D`.
///
/// ```
/// use martensite_tkernel::fsd::{File, MemoryFileSystem};
/// use martensite_tkernel::storage::{BaseDriver, DriverPrimitives, OpenMode};
/// use martensite_tkernel::tcollection::AsciiString;
/// use std::rc::Rc;
/// let files = MemoryFileSystem::default();
/// let name = AsciiString::new("doc.fsd");
/// let mut driver = File::new(Rc::new(files.clone()));
/// driver.open(&name, OpenMode::Write).unwrap();
/// driver.put_real(0.1).unwrap();
/// driver.close().unwrap();
/// assert_eq!(files.contents(&name).unwrap(), b"0.10000000000000001 ");
/// ```
pub struct TextFile<D: TextDialect> {
    base: DriverBase,
    file_system: Rc<dyn DriverFileSystem>,
    stream: IoStream,
    // Keeps legacy `new()` usable for dialect types without imposing `Default`.
    dialect: Option<D>,
}

impl<D: TextDialect> TextFile<D> {
    /// Creates a stateless closed FSD stream; `File::new(file_system)`.
    /// Use `with_dialect` when the dialect needs owned state.
    pub fn new(file_system: Rc<dyn DriverFileSystem>) -> Self {
        Self::construct(file_system, None)
    }

    /// Creates a text driver with owned per-instance dialect state.
    /// Example: `TextFile::with_dialect(fs, MyDialect::default())`.
    pub fn with_dialect(file_system: Rc<dyn DriverFileSystem>, dialect: D) -> Self {
        Self::construct(file_system, Some(dialect))
    }

    fn construct(file_system: Rc<dyn DriverFileSystem>, dialect: Option<D>) -> Self {
        Self {
            base: DriverBase::default(),
            file_system,
            stream: IoStream::default(),
            dialect,
        }
    }

    /// Reads installed dialect state; `driver.dialect().is_some()`.
    pub fn dialect(&self) -> Option<&D> {
        self.dialect.as_ref()
    }

    /// Mutably accesses installed dialect state; `driver.dialect_mut().is_some()`.
    pub fn dialect_mut(&mut self) -> Option<&mut D> {
        self.dialect.as_mut()
    }

    /// `"FSDFILE"` or `"CMPFILE"`; `File::magic_number()`.
    pub fn magic_number() -> &'static str {
        D::MAGIC_NUMBER
    }

    /// Opens `name` for reading and compares its first seven characters
    /// with the magic number; `File::is_good_file_type(file_system, &name)`.
    pub fn is_good_file_type(
        file_system: Rc<dyn DriverFileSystem>,
        name: &AsciiString,
    ) -> Result<StorageError, Failure> {
        let mut file = Self::new(file_system);
        let status = file.open(name, OpenMode::Read)?;
        if status != StorageError::Ok {
            return Ok(status);
        }
        let magic = file.fsd_read_char(D::MAGIC_NUMBER.len());
        file.close()?;
        Ok(super::binary_file::magic_status(&magic, D::MAGIC_NUMBER))
    }

    /// Closes an open driver, as the destructor does; `driver.destroy()`.
    pub fn destroy(&mut self) {
        if self.base.open_mode() != OpenMode::None {
            let _ = self.close();
        }
    }

    /// The underlying stream; `driver.stream()`.
    pub fn stream(&mut self) -> &mut IoStream {
        &mut self.stream
    }

    /// Writes `text` to the stream, stopping at the first NUL like a C string.
    pub(crate) fn write_c_string(&mut self, text: &[u8]) {
        let length = text
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(text.len());
        self.stream.insert_bytes(&text[..length]);
    }

    /// Raises `Storage_StreamWriteError` when the stream is bad.
    pub(crate) fn check_written(&self, member: &str) -> Result<(), Failure> {
        if self.stream.bad() {
            return Err(super::binary_file::write_error(member));
        }
        Ok(())
    }
}

impl<D: TextDialect> Drop for TextFile<D> {
    fn drop(&mut self) {
        self.destroy();
    }
}

/// The `std::ios::openmode` of each storage mode; `None` opens nothing.
fn stream_mode(mode: OpenMode) -> Option<FileOpenMode> {
    match mode {
        OpenMode::None => None,
        OpenMode::Read => Some(FileOpenMode::READ),
        OpenMode::Write => Some(FileOpenMode::WRITE),
        OpenMode::ReadWrite => Some(FileOpenMode::UPDATE),
    }
}

impl<D: TextDialect> BaseDriver for TextFile<D> {
    fn driver_base(&self) -> &DriverBase {
        &self.base
    }

    fn driver_base_mut(&mut self) -> &mut DriverBase {
        &mut self.base
    }

    /// `in`, `out` or `in|out` with precision 17. `OpenMode::None` opens nothing
    /// yet succeeds on a stream that has not failed.
    fn open(&mut self, name: &AsciiString, mode: OpenMode) -> Result<StorageError, Failure> {
        self.base.set_name(name);
        if self.base.open_mode() != OpenMode::None {
            return Ok(StorageError::AlreadyOpen);
        }
        if let Some(file_mode) = stream_mode(mode) {
            match self.file_system.open(name, file_mode) {
                Ok(device) => self.stream.open(device, file_mode.read, file_mode.write),
                Err(_) => self.stream.fail_open(),
            }
        }
        if self.stream.fail() {
            return Ok(StorageError::OpenError);
        }
        self.stream.set_precision(17);
        self.base.set_open_mode(mode);
        Ok(StorageError::Ok)
    }

    fn is_end(&mut self) -> Result<bool, Failure> {
        Ok(self.stream.eof())
    }

    /// A read-only file reports `tellp` and a write-only file `tellg`, swapped on
    /// purpose; a read-write file reports the larger.
    fn tell(&mut self) -> Result<Position, Failure> {
        Ok(match self.base.open_mode() {
            OpenMode::Read => self.stream.tellp(),
            OpenMode::Write => self.stream.tellg(),
            OpenMode::ReadWrite => {
                let read = self.stream.tellp();
                let write = self.stream.tellg();
                read.max(write)
            }
            OpenMode::None => -1,
        })
    }

    /// The rest of the line.
    fn skip_object(&mut self) -> Result<(), Failure> {
        self.flush_end_of_line();
        Ok(())
    }

    fn close(&mut self) -> Result<StorageError, Failure> {
        if self.base.open_mode() == OpenMode::None {
            return Ok(StorageError::NotOpen);
        }
        self.stream.close();
        self.base.set_open_mode(OpenMode::None);
        Ok(StorageError::Ok)
    }
}
