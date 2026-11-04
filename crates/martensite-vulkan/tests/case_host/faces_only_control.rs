//! The scripted faces-only control of an image: the view dumped again with an object replaced
//! by a compound of its faces, so the loose edges and vertices drawn with its wire aspect are
//! gone. An object erase cannot hide them, since they belong to the displayed shape. The
//! uncovered-surface tie rule (user decision 2026-10-28, the recording tool) grades
//! a thin-line tie pixel against this `<image>.uncovered.png`; the recorded control run does the
//! same with the object's faces exploded into one compound, displayed shaded
//! (the recording tool, `FacesOnly`).
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::SessionRun;
use crate::source_output::control_capture::control_dump;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::error::Error;

/// The session name of the faces-only compound while its control is dumped.
const FACES_ONLY_OBJECT: &str = "owned_faces_only";
/// The control's image suffix: the uncovered surface of the tie rule.
const SUFFIX: &str = "uncovered";

/// A compound of the distinct faces of `shape` in explored order, sharing their
/// triangulations;
/// `faces_compound(&c)?`.
pub(crate) fn faces_compound(shape: &Shape) -> Result<Shape, Box<dyn Error>> {
    let faces = all_distinct(shape, ShapeEnum::Face);
    if faces.is_empty() {
        return Err(format!("shape {shape:?} has no face; expected at least one face").into());
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for face in &faces {
        builder.add(&mut result, face)?;
    }
    Ok(result)
}

impl SessionRun {
    /// Dump `basename` with its tie sidecar, then `<basename>.uncovered.png` with `object`
    /// replaced by a compound of the faces of `shape` in `display_mode`, then show `object`
    /// again; `run.dump_with_faces_only_control("a.png", "c", &c, 1)?`.
    pub(crate) fn dump_with_faces_only_control(
        &mut self,
        basename: &'static str,
        object: &str,
        shape: &Shape,
        display_mode: i32,
    ) -> Result<(), Box<dyn Error>> {
        self.dump(basename)?;
        let faces = faces_compound(shape)?;
        let request = DisplayRequest {
            display_mode: Some(display_mode),
            ..DisplayRequest::default()
        };
        let session = self.session();
        session.named_objects(&[object])?;
        session.erase(&[object])?;
        session.display(FACES_ONLY_OBJECT, &faces, &request)?;
        control_dump(&*session, basename, SUFFIX)?;
        session.remove(&[FACES_ONLY_OBJECT])?;
        Ok(session.show_erased(&[object])?)
    }
}
