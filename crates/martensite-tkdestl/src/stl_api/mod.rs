//! Shape-level STL transfers: `StlApiReader` and `StlApiWriter`.
mod reader;
mod writer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{standard::Failure, tcollection::AsciiString};
pub use reader::StlApiReader;
pub use writer::StlApiWriter;

/// `StlAPI`. Its deprecated `Read` is excluded (ADR0002); use [`StlApiReader`] or `RwStl`.
pub struct StlApi;
impl StlApi {
    /// `Write`: writes a shape's triangulations, as text when `ascii_mode`, else binary;
    /// `StlApi::write(&shape, &"part.stl".into(), true)?`.
    pub fn write(shape: &Shape, file: &AsciiString, ascii_mode: bool) -> Result<bool, Failure> {
        let mut writer = StlApiWriter::new();
        *writer.ascii_mode() = ascii_mode;
        writer.write_file(shape, file, &Default::default())
    }
}
