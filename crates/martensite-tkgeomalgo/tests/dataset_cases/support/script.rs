//! The script state a case body works on: its variables, its log and its viewer session.
//!
//! A command runs through [`Script::command`], which logs it the way the script interpreter
//! does (the arguments, the output written while it runs and its result) and turns a
//! non-zero status or a `Standard_Failure` into a script error, which the body propagates
//! with `?` unless the script catches it.
use super::log::ScriptLog;
use super::session::{CurveColor, Look, Marker, Session, ViewStep};
use super::value::Value;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::gp::{Point2, Point3};

/// A script error: an uncaught one ends the script with `ScriptStep Exception: <message>`.
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
    /// Status 0 with `text` as the result.
    pub fn ok(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: true,
        }
    }

    /// Status 1 with `text` as the result.
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: false,
        }
    }
}

/// The variables, log and viewer of one case.
pub struct Script<'s> {
    session: &'s mut dyn Session,
    variables: Vec<(String, Value)>,
    /// The variables whose drawable is displayed.
    visible: Vec<String>,
    pub log: ScriptLog,
}

impl<'s> Script<'s> {
    /// A fresh case session on `session`.
    pub fn new(session: &'s mut dyn Session) -> Self {
        Self {
            session,
            variables: Vec::new(),
            visible: Vec::new(),
            log: ScriptLog::default(),
        }
    }

    /// Marks `name` displayed; true when it was hidden.
    pub fn show(&mut self, name: &str) -> bool {
        if !self.isdraw(name) || self.visible.iter().any(|v| v == name) {
            return false;
        }
        self.visible.push(name.into());
        true
    }

    /// Marks `name` hidden; true when it was displayed.
    pub fn hide(&mut self, name: &str) -> bool {
        let before = self.visible.len();
        self.visible.retain(|v| v != name);
        before != self.visible.len()
    }

    /// Marks every variable hidden (`donly` before it displays).
    pub fn hide_all(&mut self) {
        self.visible.clear();
    }

    /// Marks the 3D variables hidden (`clear`, which is `Clear3D`).
    pub fn hide_3d(&mut self) {
        let variables = &self.variables;
        self.visible.retain(|name| {
            let value = variables
                .iter()
                .find(|(v, _)| v == name)
                .map(|(_, value)| value);
            value.is_some_and(|value| !value.is_3d())
        });
    }

    /// script `puts text`.
    pub fn puts(&mut self, text: &str) {
        self.log.add(text);
    }

    /// Runs one command `args` with `body` (see the module documentation).
    pub fn command(
        &mut self,
        args: &[&str],
        body: impl FnOnce(&mut Self) -> Result<Reply, Failure>,
    ) -> ScriptStep<String> {
        let echoes = self.log.echoes();
        if echoes {
            let line: String = args.iter().map(|arg| format!("{arg} ")).collect();
            self.log.capture(&format!("{line}\n"));
        }
        let reply = body(self)
            .unwrap_or_else(|failure| Reply::error(format!("An exception was caught {failure}")));
        if echoes && !reply.text.is_empty() {
            self.log.capture(&format!("{}\n", reply.text));
        }
        if reply.ok {
            Ok(reply.text)
        } else {
            Err(ScriptError(reply.text))
        }
    }

    /// Text a command writes to `std::cout` while it runs.
    pub fn cout(&mut self, text: &str) {
        self.log.capture(text);
    }

    /// `Script::Set(name, value)`: (re)defines and displays the variable.
    pub fn set(&mut self, name: &str, value: impl Into<Value>) {
        self.set_with(name, value.into(), Look::Default);
    }

    /// `Script::Set` of a curve the command draws in `color`.
    pub fn set_colored(&mut self, name: &str, value: impl Into<Value>, color: CurveColor) {
        self.set_with(name, value.into(), Look::Colored(color));
    }

    /// `restore` of a file naming a generic curve or surface drawable.
    pub fn set_generic(&mut self, name: &str, value: impl Into<Value>) {
        self.set_with(name, value.into(), Look::Generic);
    }

    fn set_with(&mut self, name: &str, value: Value, look: Look) {
        self.variables.retain(|(existing, _)| existing != name);
        self.hide(name);
        let displayed = value.is_displayed();
        if displayed {
            self.session.set_look(name, &value, look);
        }
        self.variables.push((name.into(), value));
        if displayed {
            self.show(name);
        }
    }

    /// The variable `name`, if defined.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables
            .iter()
            .find(|(existing, _)| existing == name)
            .map(|(_, value)| value)
    }

    /// `isdraw name`.
    pub fn isdraw(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// `whatis name`: empty for an undefined variable.
    pub fn whatis(&self, name: &str) -> String {
        self.get(name).map(Value::whatis).unwrap_or_default()
    }

    /// `directory pattern` for a `prefix*` pattern, in definition order.
    pub fn directory(&self, prefix: &str) -> Vec<String> {
        self.variables
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// `unset name`.
    pub fn unset(&mut self, name: &str) {
        if self.isdraw(name) {
            self.session.unset(name);
        }
        self.hide(name);
        self.variables.retain(|(existing, _)| existing != name);
    }

    /// `renamevar from to`.
    pub fn rename(&mut self, from: &str, to: &str) {
        if !self.isdraw(from) {
            return;
        }
        let was_visible = self.hide(from);
        self.unset(to);
        let renamed = self.variables.iter_mut().find(|(name, _)| name == from);
        renamed.expect("renamed variable").0 = to.into();
        if was_visible {
            self.visible.push(to.into());
        }
        self.session.rename(from, to);
    }

    /// A marker displayed without a variable.
    pub fn marker(&mut self, marker: Marker) {
        self.session.marker(marker);
    }

    /// One view command.
    pub fn view(&mut self, step: ViewStep<'_>) {
        self.session.step(step);
    }

    /// `GetCurve(name)`: a null handle unless `name` is a 3D curve.
    pub fn curve(&self, name: &str) -> Handle<dyn GeomCurve> {
        match self.get(name) {
            Some(Value::Curve(curve)) => curve.clone(),
            _ => Handle::default(),
        }
    }

    /// `GetCurve2d(name)`.
    pub fn curve2d(&self, name: &str) -> Handle<dyn Geom2dCurve> {
        match self.get(name) {
            Some(Value::Curve2d(curve)) => curve.clone(),
            _ => Handle::default(),
        }
    }

    /// `GetSurface(name)`.
    pub fn surface(&self, name: &str) -> Handle<dyn GeomSurface> {
        match self.get(name) {
            Some(Value::Surface(surface)) => surface.clone(),
            _ => Handle::default(),
        }
    }

    /// `Get(name)`: a null shape unless `name` is a shape.
    pub fn shape(&self, name: &str) -> Shape {
        match self.get(name) {
            Some(Value::Shape(shape)) => shape.clone(),
            _ => Shape::default(),
        }
    }

    /// `GetPoint(name)`.
    pub fn point(&self, name: &str) -> Option<Point3> {
        match self.get(name) {
            Some(Value::Point(point)) => Some(*point),
            _ => None,
        }
    }

    /// `GetPoint2d(name)`.
    pub fn point2d(&self, name: &str) -> Option<Point2> {
        match self.get(name) {
            Some(Value::Point2d(point)) => Some(*point),
            _ => None,
        }
    }

    /// `dval name`: the value of a numeric variable.
    pub fn number(&self, name: &str) -> f64 {
        match self.get(name) {
            Some(Value::Number(value)) => *value,
            _ => panic!("dval {name}: expected a numeric case variable"),
        }
    }
}
