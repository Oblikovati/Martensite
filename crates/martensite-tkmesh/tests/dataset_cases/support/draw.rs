//! Fixture state tracks shape variables and their command log.
//!
//! [`Draw::command`] records arguments and results. Unsuccessful status or kernel
//! failure propagates through the fixture error type.
use super::log::DrawLog;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;

/// A fixture step failure propagates after its diagnostic has been recorded.
#[derive(Debug)]
pub struct TclError(pub String);

/// The result of a fixture step.
pub type Tcl<T = ()> = Result<T, TclError>;

/// A command result contains its text and successful-status flag.
pub struct Reply {
    pub text: String,
    pub ok: bool,
}

impl Reply {
    /// Status 0 with `text` as the result; `Reply::ok("")`.
    pub fn ok(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: true,
        }
    }

    /// Status 1 with `text` as the result; `Reply::error("")`.
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: false,
        }
    }
}

/// The shape variables and the log of one case; `Draw::default()`.
#[derive(Default)]
pub struct Draw {
    shapes: Vec<(String, Shape)>,
    pub log: DrawLog,
}

impl Draw {
    /// Tcl `puts text`; `draw.puts("TEST COMPLETED")`.
    pub fn puts(&mut self, text: &str) {
        self.log.add(text);
    }

    /// One fixture command `args`, logged with its result; `draw.command(&["box", "b"], body)?`.
    pub fn command(
        &mut self,
        args: &[&str],
        body: impl FnOnce(&mut Self) -> Result<Reply, Failure>,
    ) -> Tcl<String> {
        let line: String = args.iter().map(|arg| format!("{arg} ")).collect();
        self.log.capture(&format!("{line}\n"));
        let reply = body(self)
            .unwrap_or_else(|failure| Reply::error(format!("An exception was caught {failure}")));
        if !reply.text.is_empty() {
            self.log.capture(&format!("{}\n", reply.text));
        }
        if reply.ok {
            Ok(reply.text)
        } else {
            Err(TclError(reply.text))
        }
    }

    /// `DBRep::Set(name, shape)`; `draw.set_shape("b", &shape)`.
    pub fn set_shape(&mut self, name: &str, shape: &Shape) {
        self.shapes.retain(|(variable, _)| variable != name);
        self.shapes.push((name.to_string(), shape.clone()));
    }

    /// `DBRep::Get(name)`: the null shape for an unknown variable; `draw.shape("b")`.
    pub fn shape(&self, name: &str) -> Shape {
        self.shapes
            .iter()
            .find(|(variable, _)| variable == name)
            .map(|(_, shape)| shape.clone())
            .unwrap_or_default()
    }
}
