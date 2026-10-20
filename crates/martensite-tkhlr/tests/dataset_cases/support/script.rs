//! The script state a case body works on: its shape variables and its log.
//!
//! A command runs through [`Script::command`], which logs it the way the case harness
//! does in echo mode (the arguments, then the result) and turns a non-zero status or a
//! `Standard_Failure` into a script error, which the body propagates with `?`.
use super::log::ScriptLog;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;

/// A script error: an uncaught one ends the script with `Script exception: <message>`.
#[derive(Debug)]
pub struct ScriptError(pub String);

/// The result of a script step.
pub type ScriptStep<T = ()> = Result<T, ScriptError>;

/// What a command returns: its result text and whether it returned 0.
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

/// The shape variables and the log of one case; `Script::default()`.
#[derive(Default)]
pub struct Script {
    shapes: Vec<(String, Shape)>,
    pub log: ScriptLog,
}

impl Script {
    /// script `puts text`; `draw.puts("TEST COMPLETED")`.
    pub fn puts(&mut self, text: &str) {
        self.log.add(text);
    }

    /// One command `args`, logged with its result; `draw.command(&["box", "b"], body)?`.
    pub fn command(
        &mut self,
        args: &[&str],
        body: impl FnOnce(&mut Self) -> Result<Reply, Failure>,
    ) -> ScriptStep<String> {
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
            Err(ScriptError(reply.text))
        }
    }

    /// `Set(name, shape)`; `draw.set_shape("b", &shape)`.
    pub fn set_shape(&mut self, name: &str, shape: &Shape) {
        self.shapes.retain(|(variable, _)| variable != name);
        self.shapes.push((name.to_string(), shape.clone()));
    }

    /// `Get(name)`: the null shape for an unknown variable; `draw.shape("b")`.
    pub fn shape(&self, name: &str) -> Shape {
        self.shapes
            .iter()
            .find(|(variable, _)| variable == name)
            .map(|(_, shape)| shape.clone())
            .unwrap_or_default()
    }
}
