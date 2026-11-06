//! Runs a generated `StepCase`: the case's commands, in order. A command that returned 1 stops the
//! script, unless the script wrapped it in `catch`.
use crate::support::{
    checkprops::{CheckTolerances, Expected, checkprops_within},
    checkview::{CheckView, checkview, screenshot},
    properties::Prop,
    script_error::script_error,
    session::{Session, ViewStep},
    tolerance_check::checkmaxtol_ref,
};
use crate::table::{Place, Step, StepCase};
use crate::xs_session::XsSession;

/// The commands in order. Example: `run_steps(&mut session, &case)`.
pub fn run_steps(session: &mut XsSession, case: &StepCase) {
    for step in case.steps {
        apply(session, step);
    }
}

fn apply(session: &mut XsSession, step: &Step) {
    match step {
        Step::Puts(text) => println!("{text}"),
        Step::Param(key, value) => {
            session.pilot_command(&format!("param {key} {value}"));
        }
        Step::NewModel => {
            session.pilot_command("newmodel");
        }
        Step::ReadStep(doc, place) => read_step(session, doc, *place),
        Step::WriteStep(doc, place) => write_step(session, doc, *place),
        Step::StepRead(place, name) => {
            let file = session.path_of(*place);
            println!("stepread {file} {name} * ");
            session.stepread(&file, name);
        }
        Step::StepWrite(mode, shape, place) => {
            let file = session.path_of(*place);
            println!("stepwrite {mode} {shape} {file} ");
            session.stepwrite(mode, shape, &file);
        }
        Step::TestReadStep(place, name) => {
            let file = session.path_of(*place);
            println!("testreadstep {file} {name} ");
            session.testreadstep(&file, name);
        }
        Step::TestWriteStep(place, name) => {
            let file = session.path_of(*place);
            println!("testwritestep {file} {name} ");
            session.testwritestep(&file, name);
        }
        Step::TestReadStream(place, name) => {
            let file = session.path_of(*place);
            println!("testreadstep {file} {name} -stream ");
            session.testreadstream(&file, name);
        }
        Step::TestWriteStream(place, name) => {
            let file = session.path_of(*place);
            println!("testwritestep {file} {name} -stream ");
            session.testwritestream(&file, name);
        }
        Step::ReadFile(doc, place) => {
            let file = session.path_of(*place);
            stop_on_failure(session.read_file(doc, &file, None), "ReadFile", doc);
        }
        Step::WriteFile(doc, place) => {
            let file = session.path_of(*place);
            stop_on_failure(session.write_file(doc, &file, None), "WriteFile", doc);
        }
        Step::ReadShapeFile(shape, place) => {
            let file = session.path_of(*place);
            stop_on_failure(
                session.read_shape_file(shape, &file, None),
                "readfile",
                shape,
            );
        }
        Step::WriteShapeFile(shape, place) => {
            let file = session.path_of(*place);
            stop_on_failure(
                session.write_shape_file(shape, &file, None),
                "writefile",
                shape,
            );
        }
        Step::Restore(place, name) => restore(session, *place, name),
        Step::FileDelete(place) => {
            let _ = std::fs::remove_file(session.path_of(*place));
        }
        Step::TpCompound(name) => session.tpcompound(name),
        Step::RenameVar(from, to) => session.renamevar(from, to),
        Step::Copy(from, to) => session.copy_variable(from, to),
        Step::Explode(name, kind) => {
            session.explode(name, kind);
        }
        Step::XGetOneShape(shape, doc) => {
            println!("XGetOneShape {shape} {doc} ");
            if session.get_one_shape(shape, doc).is_err() {
                script_error(&format!("XGetOneShape {shape} {doc}"));
            }
        }
        Step::Close(doc) => {
            println!("Close {doc} ");
            session.close_document(doc);
        }
        Step::Command(name, words) => command(session, name, words),
        Step::CheckNbShapesLike(name, other) => session.check_counts_like(name, other),
        Step::CheckView(name) => checkview(&mut session.raster, &CheckView::of(name)),
        Step::Screenshot(image) => screenshot(&mut session.raster, image),
        Step::Draw2d(name, words) => draw_2d(session, name, words),
        Step::Catch(inner) => {
            // `catch` holds the failure; what the command wrote stays in the stream.
            let held =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply(session, inner)));
            if let Err(payload) = held
                && !crate::support::script_error::is_script_error(&*payload)
            {
                std::panic::resume_unwind(payload);
            }
        }
        other => panic!(
            "{}: this host does not run the step {other:?}",
            session.source
        ),
    }
}

fn stop_on_failure(step: Result<(), crate::documents::CommandFailed>, command: &str, name: &str) {
    if step.is_err() {
        script_error(&format!("{command} {name}"));
    }
}

fn read_step(session: &mut XsSession, doc: &str, place: Place) {
    let file = session.path_of(place);
    println!("ReadStep {doc} {file} ");
    if session.read_step(doc, &file).is_err() {
        script_error(&format!("ReadStep {doc} {file}"));
    }
}

fn write_step(session: &mut XsSession, doc: &str, place: Place) {
    let file = session.path_of(place);
    println!("WriteStep {doc} {file} ");
    if session.write_step(doc, &file).is_err() {
        script_error(&format!("WriteStep {doc} {file}"));
    }
}

fn restore(session: &mut XsSession, place: Place, name: &str) {
    let Place::Data(file) = place else {
        panic!("restore: {place:?}; expected a data file");
    };
    crate::support::restore::restore(&mut session.raster, file, name);
}

fn command(session: &mut XsSession, name: &str, words: &[&str]) {
    match name {
        "checkshape" => session.check_shape(words[0]),
        "checknbshapes" => session.check_counts(words[0], &words[1..]),
        "checkprops" => checkprops(session, words),
        "checkmaxtol" => checkmaxtol_ref(&session.raster, words[0], words[2]),
        "checktrinfo" => crate::meshing::checktrinfo(&session.raster, words),
        "incmesh" => crate::meshing::incmesh(&session.raster, words),
        other => panic!("{}: command {other} is not hosted", session.source),
    }
}

/// `checkprops name -s value`, the only form the scripts use here.
fn checkprops(session: &mut XsSession, words: &[&str]) {
    let kind = match words[1] {
        "-s" => Prop::Area,
        "-v" => Prop::Volume,
        "-l" => Prop::Length,
        other => panic!("checkprops: {other}; expected -s -v -l"),
    };
    let value: f64 = words[2]
        .parse()
        .unwrap_or_else(|_| panic!("checkprops: {}; expected a real", words[2]));
    checkprops_within(
        &session.raster,
        words[0],
        kind,
        Expected::Mass(value),
        CheckTolerances::default(),
    );
}

fn draw_2d(session: &mut XsSession, name: &str, words: &[&str]) {
    let raster = &mut session.raster;
    match name {
        "smallview" => raster.view(ViewStep::SmallView),
        "clear" => raster.view(ViewStep::Clear),
        "donly" => raster.view(ViewStep::Donly(words)),
        "don" => raster.view(ViewStep::Disp(words)),
        "fit" => raster.view(ViewStep::Fit),
        "isos" | "triangles" | "dall" => {}
        other => panic!("view command {other} is not hosted"),
    }
}
