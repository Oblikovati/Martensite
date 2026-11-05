//! The 3D viewer commands of the cases, on a real view. `vinit` opens one per child process, as a
//! command-host session has; `finish` retires it with its validation report before the group's `end`.
use crate::case_view::viewer_session::DisplayRequest;
use crate::script::{FilletScript, command_stopped};
use crate::session_run::SessionRun;
use crate::steps::V3d;
use martensite_tkernel::quantity::{Color, ColorRgba};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use std::cell::RefCell;
use std::error::Error;

use crate::case_view::viewer_session::FaceBoundary;

/// The view of this process, and whether a dump has been taken from it.
struct Open {
    run: SessionRun,
    dumped: bool,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

fn stopped(cause: &dyn Error) -> Failure {
    command_stopped(&cause.to_string())
}

/// The open view, opened by this call when the script has not run `vinit` yet.
fn with_open<R>(body: impl FnOnce(&mut Open) -> Result<R, Failure>) -> Result<R, Failure> {
    OPEN.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            let run = SessionRun::new().map_err(|cause| stopped(cause.as_ref()))?;
            *slot = Some(Open { run, dumped: false });
        }
        body(slot.as_mut().expect("the view was just opened"))
    })
}

fn dump(open: &mut Open, image: &'static str) -> Result<(), Failure> {
    open.dumped = true;
    open.run
        .dump(image)
        .map_err(|cause| stopped(cause.as_ref()))
}

/// `vdisplay [-dispMode mode] name`, as `topology-command` hands it the shape in the variable.
fn display(
    open: &mut Open,
    script: &FilletScript<'_>,
    name: &str,
    mode: Option<i32>,
) -> Result<(), Failure> {
    let request = DisplayRequest {
        display_mode: mode,
        ..DisplayRequest::default()
    };
    open.run
        .session()
        .display(name, &script.required(name)?, &request)
}

/// `checkview -display name -3d -path image`: a name holding nothing shows nothing.
fn result_view(
    open: &mut Open,
    script: &FilletScript<'_>,
    name: &str,
    image: &'static str,
) -> Result<(), Failure> {
    open.run.session().clear()?;
    if !script.shape(name).is_null() {
        display(open, script, name, None)?;
    }
    open.run.session().set_display_mode(1)?;
    open.run.session().fit()?;
    dump(open, image)
}

fn background(open: &mut Open, hex: &str) -> Result<(), Failure> {
    let color = ColorRgba::from_hex(hex, true)
        .ok_or_else(|| command_stopped(&format!("vbackground: bad color {hex}")))?;
    open.run.session().set_background_color(*color.rgb())
}

fn white_boundaries(open: &mut Open, name: &str) -> Result<(), Failure> {
    let style = FaceBoundary {
        color: Some(Color::named(
            martensite_tkernel::quantity::NamedColor::WHITE,
        )),
        ..FaceBoundary::default()
    };
    open.run.session().draw_face_boundaries(name, &style)?;
    open.run.session().set_iso_on_triangulation(name, true)
}

fn boundaries_up_to_c2(open: &mut Open, name: &str) -> Result<(), Failure> {
    open.run
        .session()
        .draw_face_boundaries(name, &FaceBoundary::default())?;
    open.run
        .session()
        .set_face_boundary_continuity(name, Continuity::C2)
}

/// What `BUC60811` displays, in its order; the aluminum ones with that material.
fn show_buc60811(open: &mut Open, script: &FilletScript<'_>) -> Result<(), Failure> {
    for (name, aluminum) in [
        ("F1", true),
        ("FP", true),
        ("F2", false),
        ("FP1", true),
        ("Wire", false),
    ] {
        display(open, script, name, None)?;
        if aluminum {
            open.run
                .session()
                .set_material(name, Graphic3dNameOfMaterial::Aluminum)?;
        }
    }
    Ok(())
}

/// `trinfo name`.
fn mesh_report(script: &FilletScript<'_>, name: &str) -> Result<(), Failure> {
    crate::mesh_information::print_information(&script.required(name)?)
}

/// Run one viewer command; `apply(&mut script, &V3d::Fit)`.
pub fn apply(script: &mut FilletScript<'_>, view: &V3d) -> Result<(), Failure> {
    with_open(|open| match view {
        V3d::Init => Ok(()),
        V3d::DisplayMode(mode) => open.run.session().set_display_mode(*mode),
        V3d::Display(name, mode) => display(open, script, name, *mode),
        V3d::Fit => open.run.session().fit(),
        V3d::Dump(image) => dump(open, image),
        V3d::ResultView(name, image) => result_view(open, script, name, image),
        V3d::Background(hex) => background(open, hex),
        V3d::WhiteBoundaries(name) => white_boundaries(open, name),
        V3d::BoundariesUpToC2(name) => boundaries_up_to_c2(open, name),
        V3d::ShowBuc60811 => show_buc60811(open, script),
        V3d::MeshReport(name) => mesh_report(script, name),
    })
}

/// Retire the view, if a script opened one; its validation errors go to the graded stream.
pub fn finish() {
    let open = OPEN.with(|cell| cell.borrow_mut().take());
    let Some(open) = open else {
        return;
    };
    let outcome = if open.dumped {
        open.run.retire()
    } else {
        open.run.retire_without_image()
    };
    if let Err(cause) = outcome {
        println!("Error: the 3D view did not close cleanly: {cause}");
    }
}
