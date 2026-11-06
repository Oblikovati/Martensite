//! Stream insertion semantics used by `TopOpeBRepBuildShapeSet::DumpName`.
//!
//! `std::ostream` carries formatting state, whereas [`std::io::Write`] only carries
//! bytes. This small interface keeps the original sequence of formatted insertions
//! observable by callers that need a non-default integer base, field width, locale,
//! or other stream state.
use martensite_tkernel::tcollection::AsciiString;
use std::io::{self, Write};

/// The formatted insertion operations used by `TopOpeBRepBuildShapeSet::DumpName`.
///
/// Implement this for a stream that owns formatting state. The shape-set method calls
/// the operations in the same order as the native chained `operator<<` expression.
///
/// ```
/// use martensite_tkbool::top_ope_brep_build::ShapeSetDumpStream;
/// use martensite_tkernel::tcollection::AsciiString;
/// use std::io;
///
/// struct Bytes(Vec<u8>);
/// impl ShapeSetDumpStream for Bytes {
///     fn insert_ascii(&mut self, text: &AsciiString) -> io::Result<()> {
///         self.0.extend_from_slice(text.as_bytes());
///         Ok(())
///     }
///     fn insert_literal(&mut self, text: &[u8]) -> io::Result<()> {
///         self.0.extend_from_slice(text);
///         Ok(())
///     }
///     fn insert_i32(&mut self, value: i32) -> io::Result<()> {
///         self.0.extend_from_slice(value.to_string().as_bytes());
///         Ok(())
///     }
/// }
/// ```
pub trait ShapeSetDumpStream {
    /// Inserts an `AsciiString` with this stream's string formatting state.
    fn insert_ascii(&mut self, text: &AsciiString) -> io::Result<()>;

    /// Inserts fixed punctuation without changing the stream's numeric state.
    fn insert_literal(&mut self, text: &[u8]) -> io::Result<()>;

    /// Inserts an integer with this stream's configured integer formatting state.
    fn insert_i32(&mut self, value: i32) -> io::Result<()>;
}

/// Adapts a plain Rust byte writer to default `std::ostream` insertion behavior.
pub(crate) struct DefaultShapeSetDumpStream<'a> {
    output: &'a mut dyn Write,
}

impl<'a> DefaultShapeSetDumpStream<'a> {
    /// Creates the default decimal insertion adapter; `DefaultShapeSetDumpStream::new(&mut out)`.
    pub(crate) fn new(output: &'a mut dyn Write) -> Self {
        Self { output }
    }
}

impl ShapeSetDumpStream for DefaultShapeSetDumpStream<'_> {
    fn insert_ascii(&mut self, text: &AsciiString) -> io::Result<()> {
        text.print(self.output)
    }

    fn insert_literal(&mut self, text: &[u8]) -> io::Result<()> {
        self.output.write_all(text)
    }

    fn insert_i32(&mut self, value: i32) -> io::Result<()> {
        write!(self.output, "{value}")
    }
}
