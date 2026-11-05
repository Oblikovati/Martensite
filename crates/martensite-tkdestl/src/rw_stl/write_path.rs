//! The path overloads of `RWStl::write_binary` and `RWStl::write_ascii`.
use super::RwStl;
use martensite_tkernel::{
    message::ProgressRange,
    osd::{CadPath, FileStream, IosOpenMode, OStream, open_file_stream_utf8},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkmath::poly::Triangulation;
use std::io::{self, Write};

/// A `Write` over a `std::ostream`: a write that fails the stream is an error;
/// `RwStl::write_ascii(&mesh, &mut OStreamWriter(&mut stream), &progress)?`.
pub struct OStreamWriter<'a>(pub &'a mut dyn OStream);
impl Write for OStreamWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.write(bytes);
        if self.0.fail() {
            return Err(io::Error::other("failed STL output stream"));
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        // Never flushes here; the owner of the stream decides.
        Ok(())
    }
}

/// `std::ofstream`, or `none` when the file cannot be opened; used by both write path families;
/// `open_output_file(&"part.stl".into(), IosOpenMode::BINARY)`.
pub(crate) fn open_output_file(path: &AsciiString, mode: IosOpenMode) -> Option<FileStream> {
    let name = String::from_utf8_lossy(path.as_bytes());
    let mut stream = FileStream::output();
    open_file_stream_utf8(&mut stream, &name, mode);
    stream.is_open().then_some(stream)
}

impl RwStl {
    /// False when the mesh is empty or the
    /// file cannot be opened; `RwStl::write_binary_path(&mesh, &CadPath::new("a.stl"), &range)?`.
    pub fn write_binary_path(
        mesh: &Handle<Triangulation>,
        path: &CadPath,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if !mesh.get().is_some_and(|mesh| mesh.nb_triangles() > 0) {
            return Ok(false);
        }
        let Some(mut stream) = open_output_file(&path.system_name(), IosOpenMode::BINARY) else {
            return Ok(false);
        };
        Self::write_binary(mesh, &mut OStreamWriter(&mut stream), progress)
    }
    /// False when the mesh is empty or the
    /// file cannot be opened; `RwStl::write_ascii_path(&mesh, &CadPath::new("a.stl"), &range)?`.
    pub fn write_ascii_path(
        mesh: &Handle<Triangulation>,
        path: &CadPath,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if !mesh.get().is_some_and(|mesh| mesh.nb_triangles() > 0) {
            return Ok(false);
        }
        let Some(mut stream) = open_output_file(&path.system_name(), IosOpenMode::NONE) else {
            return Ok(false);
        };
        Self::write_ascii(mesh, &mut OStreamWriter(&mut stream), progress)
    }
}
