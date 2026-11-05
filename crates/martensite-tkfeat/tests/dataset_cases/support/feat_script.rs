//! One running script: the contract's session holds its shapes and view; this adds the variables
//! that are no shapes and the state the commands keep between lines. All names share one space, as
//! a script's variables do: a shape hides a curve or surface of the same name, and the reverse.
use super::{
    feat_definitions::Features,
    script_error::script_error,
    session::{Drawing, Session, ViewStep},
    two_step::TwoStepBoolean,
};
use martensite_tkbrep::{brep_tools::History, topods::Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use std::collections::HashMap;

/// The variables and command state of one case.
pub struct FeatScript<'s> {
    session: &'s mut dyn Session,
    surfaces: HashMap<String, Handle<dyn GeomSurface>>,
    curves: HashMap<String, Handle<dyn GeomCurve>>,
    /// The definitions `featprism` and its like leave for `featperform`.
    pub features: Features,
    /// The history the last history-filling command left for `savehistory`.
    pub history: Option<Handle<History>>,
    /// What `bop` leaves for `bopsection`.
    pub two_step: TwoStepBoolean,
}

impl<'s> FeatScript<'s> {
    /// A script over `session`; `FeatScript::new(&mut raster)`.
    pub fn new(session: &'s mut dyn Session) -> Self {
        Self {
            session,
            surfaces: HashMap::new(),
            curves: HashMap::new(),
            features: Features::default(),
            history: None,
            two_step: TwoStepBoolean::default(),
        }
    }

    /// Fill a variable with a surface; `script.set_surface("p", plane)`.
    pub fn set_surface(&mut self, name: &str, surface: Handle<dyn GeomSurface>) {
        self.curves.remove(name);
        self.session.bind_drawing(name, Drawing::Surface(&surface));
        self.surfaces.insert(name.to_owned(), surface);
    }

    /// Fill a variable with a curve; `script.set_curve("c", circle)`.
    pub fn set_curve(&mut self, name: &str, curve: Handle<dyn GeomCurve>) {
        self.surfaces.remove(name);
        self.session.bind_drawing(name, Drawing::Curve(&curve));
        self.curves.insert(name.to_owned(), curve);
    }

    /// The surface in a variable; `script.surface("p")`.
    pub fn surface(&self, name: &str) -> Option<Handle<dyn GeomSurface>> {
        self.surfaces.get(name).cloned()
    }

    /// The curve in a variable; `script.curve("c")`.
    pub fn curve(&self, name: &str) -> Option<Handle<dyn GeomCurve>> {
        self.curves.get(name).cloned()
    }

    /// The shape in a variable, a null shape when it holds none, as a command reads it.
    /// Example: `script.shape_or_null("funtil")`.
    pub fn shape_or_null(&self, name: &str) -> Shape {
        self.shape(name).cloned().unwrap_or_default()
    }
}

impl Session for FeatScript<'_> {
    fn bind(&mut self, name: &str, shape: &Shape) {
        self.surfaces.remove(name);
        self.curves.remove(name);
        self.session.bind(name, shape);
    }

    fn bind_drawing(&mut self, name: &str, drawing: Drawing<'_>) {
        self.session.bind_drawing(name, drawing);
    }

    fn shape(&self, name: &str) -> Option<&Shape> {
        if self.surfaces.contains_key(name) || self.curves.contains_key(name) {
            return None;
        }
        self.session.shape(name)
    }

    fn view(&mut self, step: ViewStep<'_>) {
        self.session.view(step);
    }
}

/// A command returning 1 with `message` as its result: the harness writes the result, then the
/// script stops on it. Example: `command_failed("invalid number of arguments")`.
pub fn command_failed(message: &str) -> ! {
    println!("{message}");
    script_error(message);
}

/// A command stopped by an exception: what the command wrapper makes of it.
/// Example: `exception_stop(&Failure::new("bad"))`.
pub fn exception_stop(failure: &Failure) -> ! {
    command_failed(&format!(
        "An exception was caught {}: {}",
        failure.exception_type(),
        String::from_utf8_lossy(failure.what())
    ));
}

/// The value of a command step, or the stop of its exception; `checked(prism.perform_thru_all())`.
pub fn checked<T>(step: Result<T, Failure>) -> T {
    step.unwrap_or_else(|failure| exception_stop(&failure))
}

/// `catch {command}`: whether the command stopped the script here; nothing else is caught.
/// Example: `if caught(|| featperform(s, ...)) { println!("Error in featperform"); }`.
pub fn caught(command: impl FnOnce()) -> bool {
    super::script_error::catch_script_error(command)
}
