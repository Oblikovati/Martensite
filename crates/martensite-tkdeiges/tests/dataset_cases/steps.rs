//! Runs a generated script: its commands, in order. A command that returned 1 stops the script,
//! unless the script wrapped it in `catch`.
use crate::support::{
    checkprops::{CheckTolerances, Expected, checkprops_equal, checkprops_within},
    checkview::{CheckView, checkview, screenshot},
    nbshapes::{
        Counting, NbCount, checknbshapes, checknbshapes_like, checknbshapes_named, nbshapes,
    },
    properties::Prop,
    script_error::{is_script_error, script_error},
    session::{Session, ViewStep},
    tolerance_check::checkmaxtol,
};
use crate::table::Step;
use crate::xs_session::XsSession;

/// The commands in order. Example: `run_steps(&mut session, case.steps)`.
pub fn run_steps(session: &mut XsSession, steps: &[Step]) {
    for step in steps {
        match step {
            Step::Run(name, words) => run_command(session, name, words),
            Step::Catch(name, words) => {
                catching(session, name, words);
            }
        }
    }
}

/// `catch {command}`: the failure stays held, and what the command wrote stays in the stream.
pub fn catching(session: &mut XsSession, name: &str, words: &[&str]) -> bool {
    let held = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_command(session, name, words);
    }));
    match held {
        Ok(()) => true,
        Err(payload) if is_script_error(&*payload) => false,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// One command of a script, by the name the script gives it.
/// Example: `run_command(&mut session, "checkshape", &["result"])`.
pub fn run_command(session: &mut XsSession, name: &str, words: &[&str]) {
    match name {
        "puts" => println!("{}", words[0]),
        "param" => {
            session.pilot_command(&format!("param {} {}", words[0], words[1]));
        }
        "newmodel" | "tpclear" => {
            session.pilot_command(name);
        }
        "file_delete" => {
            let _ = std::fs::remove_file(session.path_word(words[0]));
        }
        _ => run_exchange_command(session, name, words),
    }
}

fn run_exchange_command(session: &mut XsSession, name: &str, words: &[&str]) {
    match name {
        "igesbrep" | "igesread" => {
            let file = session.path_word(words[0]);
            session.igesbrep(name, &file, words[1], words[2]);
        }
        "testreadiges" => {
            let file = session.path_word(words[0]);
            session.testreadiges(&file, words[1]);
        }
        "brepiges" => {
            let file = session.path_word(words[1]);
            session.brepiges(words[0], &file);
        }
        "testwriteiges" => {
            let file = session.path_word(words[0]);
            session.testwriteiges(&file, words[1]);
        }
        "tpcompound" => session.tpcompound(words[0]),
        "ReadIges" => {
            let file = session.path_word(words[1]);
            if session.read_iges(words[0], &file).is_err() {
                script_error(&format!("ReadIges {}", words[0]));
            }
        }
        "WriteIges" => {
            let file = session.path_word(words[1]);
            session.write_iges(words[0], &file);
        }
        "ReadFile" => {
            let file = session.path_word(words[1]);
            stop_on_failure(
                session.read_file(words[0], &file, None),
                "ReadFile",
                words[0],
            );
        }
        "WriteFile" => {
            let file = session.path_word(words[1]);
            stop_on_failure(
                session.write_file(words[0], &file, None),
                "WriteFile",
                words[0],
            );
        }
        "readfile" => {
            let file = session.path_word(words[1]);
            stop_on_failure(
                session.read_shape_file(words[0], &file, None),
                "readfile",
                words[0],
            );
        }
        "writefile" => {
            let file = session.path_word(words[1]);
            stop_on_failure(
                session.write_shape_file(words[0], &file, None),
                "writefile",
                words[0],
            );
        }
        "XGetOneShape" => session.get_one_shape(words[0], words[1]),
        "XNewDoc" => session.new_document(words[0]),
        "Close" => session.close_document(words[0]),
        _ => run_check_command(session, name, words),
    }
}

fn stop_on_failure(step: Result<(), crate::documents::CommandFailed>, command: &str, name: &str) {
    if step.is_err() {
        script_error(&format!("{command} {name}"));
    }
}

fn run_check_command(session: &mut XsSession, name: &str, words: &[&str]) {
    match name {
        "checkshape" => checkshape(session, words),
        "checknbshapes" => checknbshapes_command(session, words),
        "checknbshapes_like" => {
            println!("checknbshapes {} -ref [nbshapes {}]", words[0], words[1]);
            checknbshapes_like(&session.raster, words[0], words[1]);
        }
        "nbshapes" => {
            println!("nbshapes {} ", words[0]);
            nbshapes(&session.raster, words[0], Counting::Located);
        }
        "checkprops" => checkprops(session, words),
        "checkmaxtol" => {
            println!("checkmaxtol {}", words.join(" "));
            let minimum = (words.get(1) == Some(&"-min_tol")).then(|| number(words[2]));
            checkmaxtol(&session.raster, words[0], None, minimum);
        }
        "checktrinfo" => crate::meshing::checktrinfo(&session.raster, words),
        "incmesh" => crate::meshing::incmesh(&session.raster, words),
        "checkview" => checkview(
            &mut session.raster,
            &CheckView::of(words[0]).image_named(words[1]),
        ),
        "screenshot" => screenshot(&mut session.raster, words[0]),
        "tolerance" => {
            println!("tolerance {} ", words[0]);
            print!(
                "{}",
                crate::support::tolerance_check::tolerance_report(&session.raster, words[0])
            );
        }
        _ => crate::shape_commands::run_shape_command(session, name, words),
    }
}

fn number(word: &str) -> f64 {
    word.parse()
        .unwrap_or_else(|_| panic!("{word}: expected a real"))
}

/// `checkshape name` or `checkshape name r`.
fn checkshape(session: &XsSession, words: &[&str]) {
    println!("checkshape {} ", words.join(" "));
    match words {
        [name] => crate::support::checkshape::checkshape_named(&session.raster, name),
        [name, "r"] => {
            let line = crate::support::structural_dump::structural_check(
                &session.bound_shape(name),
                name,
                "r",
            )
            .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"));
            println!("{line}");
        }
        other => panic!("checkshape: {other:?}; expected a name and an optional r"),
    }
}

/// `checknbshapes name -kind count ... [-t] [-m message]`.
fn checknbshapes_command(session: &XsSession, words: &[&str]) {
    println!("checknbshapes {} ", words.join(" "));
    let mut counts = Vec::new();
    let mut typed = false;
    let mut message = None;
    let mut at = 1;
    while at < words.len() {
        match words[at] {
            "-t" => typed = true,
            "-m" => {
                at += 1;
                message = Some(words[at]);
            }
            kind => {
                at += 1;
                counts.push(count_of(kind, words[at]));
            }
        }
        at += 1;
    }
    let counting = if typed {
        Counting::Typed
    } else {
        Counting::Located
    };
    match message {
        Some(text) => checknbshapes_named(&session.raster, words[0], counting, &counts, text),
        None => checknbshapes(&session.raster, words[0], counting, &counts),
    }
}

fn count_of(word: &str, value: &str) -> NbCount {
    let count: usize = value
        .parse()
        .unwrap_or_else(|_| panic!("checknbshapes: {value}; expected a count"));
    match word {
        "-vertex" => NbCount::Vertices(count),
        "-edge" => NbCount::Edges(count),
        "-wire" => NbCount::Wires(count),
        "-face" => NbCount::Faces(count),
        "-shell" => NbCount::Shells(count),
        "-solid" => NbCount::Solids(count),
        "-compsolid" => NbCount::CompSolids(count),
        "-compound" => NbCount::Compounds(count),
        "-shape" => NbCount::Shapes(count),
        other => panic!("checknbshapes: {other}; expected a kind"),
    }
}

/// `checkprops name -s|-l|-v value [-eps e]`, or `-equal other` alone.
fn checkprops(session: &XsSession, words: &[&str]) {
    if let [name, "-equal", other] = words {
        checkprops_equal(&session.raster, name, other);
        return;
    }
    let kind = match words[1] {
        "-s" => Prop::Area,
        "-v" => Prop::Volume,
        "-l" => Prop::Length,
        other => panic!("checkprops: {other}; expected -s -v -l"),
    };
    let mut tolerances = CheckTolerances::default();
    if let [_, _, _, "-eps", epsilon] = words {
        tolerances.epsilon = number(epsilon);
    }
    checkprops_within(
        &session.raster,
        words[0],
        kind,
        Expected::Word(number(words[2]), leak(words[2])),
        tolerances,
    );
}

fn leak(word: &str) -> &'static str {
    Box::leak(word.to_owned().into_boxed_str())
}

/// A view command of a script on the memory raster.
/// Example: `view_command(&mut session, "fit", &[])`.
pub fn view_command(session: &mut XsSession, name: &str, words: &[&str]) {
    let raster = &mut session.raster;
    match name {
        "smallview" => raster.view(ViewStep::SmallView),
        "clear" => raster.view(ViewStep::Clear),
        "donly" => raster.view(ViewStep::Donly(words)),
        "display" | "don" => raster.view(ViewStep::Disp(words)),
        "fit" => raster.view(ViewStep::Fit),
        "xwd" => {
            let stem = std::path::Path::new(words[0])
                .file_stem()
                .expect("an image name");
            raster.view(ViewStep::Xwd(&stem.to_string_lossy()));
        }
        "isos" | "triangles" | "dall" => {}
        other => panic!("view command {other} is not hosted"),
    }
}
