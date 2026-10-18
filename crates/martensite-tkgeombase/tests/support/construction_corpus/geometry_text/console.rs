//! Request-owned capture while the original process-wide extension hook is serialized.
use super::super::print::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeombase::geom_tools::{
    self, ConsoleOutput, UndefinedTypeHandler, UndefinedTypeHandlerBase,
};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default)]
struct RecordingConsole(Mutex<Vec<u8>>);
impl ConsoleOutput for RecordingConsole {
    fn write(&self, bytes: &[u8]) {
        self.0.lock().unwrap().extend_from_slice(bytes);
    }
}

/// Each isolated cohort executable coordinates every writer of the retained global hook.
pub struct Context<'a> {
    pub output: &'a Output,
    console: Arc<RecordingConsole>,
    original: Handle<dyn UndefinedTypeHandler>,
    _serial: MutexGuard<'static, ()>,
}
impl<'a> Context<'a> {
    pub fn new(output: &'a Output) -> Self {
        static SERIAL: Mutex<()> = Mutex::new(());
        let serial = SERIAL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let original = geom_tools::get_undefined_type_handler();
        let console = Arc::new(RecordingConsole::default());
        let handler = Handle::new(UndefinedTypeHandlerBase::with_console(console.clone()));
        geom_tools::set_undefined_type_handler(&handler.into_cast());
        Self {
            output,
            console,
            original,
            _serial: serial,
        }
    }

    pub fn console(&self) -> Arc<dyn ConsoleOutput> {
        self.console.clone()
    }
}
impl Drop for Context<'_> {
    fn drop(&mut self) {
        geom_tools::set_undefined_type_handler(&self.original);
    }
}

/// Captures only console bytes, leaving the token output independent of nested calls.
pub fn captured(ctx: &Context<'_>, body: impl FnOnce()) -> Vec<u8> {
    let saved = std::mem::take(&mut *ctx.console.0.lock().unwrap());
    body();
    std::mem::replace(&mut *ctx.console.0.lock().unwrap(), saved)
}

pub fn attempt_captured(ctx: &Context<'_>, body: impl FnOnce() -> Result<(), Failure>) {
    if let Err(error) = body() {
        ctx.console
            .write(format!("{} ", error.exception_type()).as_bytes());
    }
}
