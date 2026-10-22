//! The named scenario variables of a script that builds its inputs command by command: each
//! command reads its arguments by name and sets its result, which the session displays
//! as the session store does. `vars.set("line", line([0., 0., 0., 0., 0., 1.]))`.
use super::restore::Restored;
use super::session::Session;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use std::collections::BTreeMap;

/// The variables of one case and its session.
pub struct Vars<'a> {
    session: &'a mut dyn Session,
    values: BTreeMap<String, Restored>,
}

impl<'a> Vars<'a> {
    /// No variable yet; `Vars::new(s)`.
    pub fn new(session: &'a mut dyn Session) -> Self {
        Self {
            session,
            values: BTreeMap::new(),
        }
    }

    /// The session, for the view steps of the script.
    pub fn session(&mut self) -> &mut dyn Session {
        &mut *self.session
    }

    /// Defines or replaces a displayed variable.
    pub fn set(&mut self, name: &str, value: impl Into<Restored>) {
        let value = value.into();
        self.session.set(name, &value);
        self.values.insert(name.to_string(), value);
    }

    /// `isdraw name`.
    pub fn is_drawn(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    fn get(&self, name: &str) -> Restored {
        self.values
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("{name} is not a variable"))
    }

    /// `DrawTrSurf::GetCurve(name)`.
    pub fn curve(&self, name: &str) -> Handle<dyn GeomCurve> {
        self.get(name).curve()
    }

    /// `DrawTrSurf::GetSurface(name)`.
    pub fn surface(&self, name: &str) -> Handle<dyn GeomSurface> {
        self.get(name).surface()
    }

    /// `DBRep::Get(name)`.
    pub fn shape(&self, name: &str) -> Shape {
        self.get(name).shape()
    }
}
