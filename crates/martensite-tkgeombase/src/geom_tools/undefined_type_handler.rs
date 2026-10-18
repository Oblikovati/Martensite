//! `UndefinedTypeHandler`: the extension point the GeomTools printers and
//! readers call for a geometry class, or a type code, they do not know.
use super::console::{ConsoleOutput, stdout_console};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient};
use martensite_tkernel::storage::IoStream;
use martensite_tkernel::{impl_transient, impl_transient_interface};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use std::sync::Arc;

const UNKNOWN_CURVE: &[u8] = b"****** UNKNOWN CURVE TYPE ******";
const UNKNOWN_CURVE2D: &[u8] = b"****** UNKNOWN CURVE2d TYPE ******";
const UNKNOWN_SURFACE: &[u8] = b"****** UNKNOWN SURFACE TYPE ******";

/// The virtual members of `UndefinedTypeHandler`. The provided methods are
/// the retained base behaviour: a print writes an "UNKNOWN ... TYPE" line (to the
/// stream for a dump, to the console for the compact form, which therefore stays
/// empty), and a read consumes nothing and returns a null handle. Override them to
/// store and restore other classes; `GeomTools::SetUndefinedTypeHandler` installs
/// the handler ([`super::set_undefined_type_handler`]).
///
/// Every member returns `Result` because an override may raise, and the readers
/// catch a failure as they catch a retained Failure.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkernel::storage::IoStream;
/// use martensite_tkgeombase::geom_tools::{UndefinedTypeHandler, UndefinedTypeHandlerBase};
/// let handler = UndefinedTypeHandlerBase::default();
/// let mut stream = IoStream::from_bytes(Vec::new());
/// assert!(handler.read_curve(42, &mut stream).unwrap().is_null());
/// ```
pub trait UndefinedTypeHandler: Transient {
    /// Port-only seam: where the retained `std::cout` writes of the provided members
    /// go; the process standard output unless overridden; `handler.console()`.
    fn console(&self) -> Arc<dyn ConsoleOutput> {
        stdout_console()
    }

    /// `PrintCurve(C, OS, compact)`; `handler.print_curve(&curve, &mut stream, false)?`.
    fn print_curve(
        &self,
        _curve: &Handle<dyn GeomCurve>,
        stream: &mut IoStream,
        compact: bool,
    ) -> Result<(), Failure> {
        report_unknown(&*self.console(), stream, compact, UNKNOWN_CURVE);
        Ok(())
    }

    /// `ReadCurve(ctype, IS, C)`: the curve read for type code `ctype` (the retained
    /// output handle, null when nothing is read); `handler.read_curve(12, &mut stream)?`.
    fn read_curve(
        &self,
        _ctype: i32,
        _stream: &mut IoStream,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        Ok(Handle::default())
    }

    /// `PrintCurve2d(C, OS, compact)`; `handler.print_curve2d(&curve, &mut stream, false)?`.
    fn print_curve2d(
        &self,
        _curve: &Handle<dyn Geom2dCurve>,
        stream: &mut IoStream,
        compact: bool,
    ) -> Result<(), Failure> {
        report_unknown(&*self.console(), stream, compact, UNKNOWN_CURVE2D);
        Ok(())
    }

    /// `ReadCurve2d(ctype, IS, C)`; `handler.read_curve2d(12, &mut stream)?`.
    fn read_curve2d(
        &self,
        _ctype: i32,
        _stream: &mut IoStream,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        Ok(Handle::default())
    }

    /// `PrintSurface(S, OS, compact)`; `handler.print_surface(&surface, &mut stream, false)?`.
    fn print_surface(
        &self,
        _surface: &Handle<dyn GeomSurface>,
        stream: &mut IoStream,
        compact: bool,
    ) -> Result<(), Failure> {
        report_unknown(&*self.console(), stream, compact, UNKNOWN_SURFACE);
        Ok(())
    }

    /// `ReadSurface(ctype, IS, S)`; `handler.read_surface(12, &mut stream)?`.
    fn read_surface(
        &self,
        _ctype: i32,
        _stream: &mut IoStream,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        Ok(Handle::default())
    }
}
impl_transient_interface!(
    dyn UndefinedTypeHandler,
    "GeomTools_UndefinedTypeHandler",
    StandardTransient
);

/// The retained base behaviour of the print members: the dump gets the line, while the
/// compact form writes it to `std::cout` and leaves the stream untouched.
fn report_unknown(console: &dyn ConsoleOutput, stream: &mut IoStream, compact: bool, line: &[u8]) {
    if compact {
        console.write(&[line, b"\n"].concat());
    } else {
        stream.insert_bytes(line).insert_bytes(b"\n");
    }
}

/// An instance of the retained class itself (`new UndefinedTypeHandler`), the
/// handler GeomTools starts with; its dynamic type is `UndefinedTypeHandler`.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkgeombase::geom_tools::UndefinedTypeHandlerBase;
/// let handler = Handle::new(UndefinedTypeHandlerBase::default());
/// assert!(handler.is_instance_name("UndefinedTypeHandler"));
/// ```
pub struct UndefinedTypeHandlerBase {
    parent: StandardTransient,
    console: Arc<dyn ConsoleOutput>,
}
impl_transient!(
    UndefinedTypeHandlerBase,
    "GeomTools_UndefinedTypeHandler",
    StandardTransient,
    parent,
    interfaces: [dyn UndefinedTypeHandler]
);

impl Default for UndefinedTypeHandlerBase {
    /// `UndefinedTypeHandler()`, writing its console lines to standard output.
    fn default() -> Self {
        Self::with_console(stdout_console())
    }
}

impl UndefinedTypeHandlerBase {
    /// The retained handler with its `std::cout` writes sent to `console`;
    /// `UndefinedTypeHandlerBase::with_console(Arc::new(StdoutConsole))`.
    pub fn with_console(console: Arc<dyn ConsoleOutput>) -> Self {
        Self {
            parent: StandardTransient::default(),
            console,
        }
    }
}

impl UndefinedTypeHandler for UndefinedTypeHandlerBase {
    fn console(&self) -> Arc<dyn ConsoleOutput> {
        Arc::clone(&self.console)
    }
}
