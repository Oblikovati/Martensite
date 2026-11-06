//! The two scripts of the wrapper grid whose `catch` forms decide what each command says: the units of a
//! read and of a write, through the session and through the wrapper.
use crate::steps::{catching, run_command};
use crate::support::{
    checkprops::{CheckTolerances, Expected, checkprops_within},
    properties::Prop,
};
use crate::xs_session::XsSession;

const SPRING: &str = "@data/bug22869_Spring.igs";

fn said_reading(done: bool) {
    println!(
        "{}",
        if done {
            "OK : Reading is correct"
        } else {
            "Error : Problem with reading file"
        }
    );
}

fn said_writing(done: bool) {
    println!(
        "{}",
        if done {
            "OK : Writing is correct"
        } else {
            "Error : Problem with writing file"
        }
    );
}

/// `checkprops name -s value -eps 1e-2`.
fn check_area(session: &XsSession, name: &str, area: f64, word: &'static str) {
    checkprops_within(
        &session.raster,
        name,
        Prop::Area,
        Expected::Word(area, word),
        CheckTolerances {
            epsilon: 1.0e-2,
            ..CheckTolerances::default()
        },
    );
}

/// `de_wrapper/iges/A5`: the units of a read, through the session and through the wrapper.
pub fn wrapper_a5(session: &mut XsSession) {
    catching(session, "Close", &["D2"]);
    catching(session, "Close", &["D3"]);
    run_command(session, "param", &["xstep.cascade.unit", "MM"]);
    said_reading(catching(session, "igesread", &[SPRING, "res0", "*"]));
    run_command(session, "param", &["xstep.cascade.unit", "M"]);
    said_reading(catching(session, "igesread", &[SPRING, "res1", "*"]));
    let file = session.path_word(SPRING);
    said_reading(session.read_file("D2", &file, None).is_ok());
    session.get_one_shape("S2", "D2");
    if session.get_length_unit("D2") != "mm" {
        println!("Error: incrorrect document's length unit");
    }
    session.new_document("D3");
    said_reading(
        session
            .read_file("D3", &file, Some("global.general.length.unit : 1000 "))
            .is_ok(),
    );
    if session.get_length_unit("D3") != "m" {
        println!("Error: incrorrect document's length unit");
    }
    session.get_one_shape("S3", "D3");
    said_reading(
        session
            .read_shape_file("S4", &file, Some("global.general.length.unit : 1 "))
            .is_ok(),
    );
    said_reading(
        session
            .read_shape_file("S5", &file, Some("global.general.length.unit : 1000 "))
            .is_ok(),
    );
    for (name, area, word) in [
        ("res0", 11995.4, "11995.4"),
        ("res1", 0.0119954, "0.0119954"),
        ("S2", 11995.4, "11995.4"),
        ("S3", 0.0119954, "0.0119954"),
        ("S4", 11995.4, "11995.4"),
        ("S5", 0.0119954, "0.0119954"),
    ] {
        check_area(session, name, area, word);
    }
    run_command(session, "Close", &["D2"]);
    run_command(session, "Close", &["D3"]);
}

/// `de_wrapper/iges/A6`: the units of a write, through the wrapper, read back.
pub fn wrapper_a6(session: &mut XsSession) {
    for name in ["D0", "D1", "D6", "D7"] {
        catching(session, "Close", &[name]);
    }
    let path = session.image_file("A6.igs").to_string_lossy().into_owned();
    run_command(session, "param", &["xstep.cascade.unit", "MM"]);
    said_reading(catching(session, "ReadIges", &["D0", SPRING]));
    session.get_one_shape("S0", "D0");
    run_command(session, "param", &["xstep.cascade.unit", "M"]);
    said_reading(catching(session, "ReadIges", &["D1", SPRING]));
    session.get_one_shape("S1", "D1");
    said_writing(
        session
            .write_file("D0", &path, Some("global.general.length.unit : 25.4 "))
            .is_ok(),
    );
    said_reading(session.read_shape_file("S2", &path, None).is_ok());
    said_writing(session.write_shape_file("S0", &path, None).is_ok());
    said_reading(session.read_shape_file("S3", &path, None).is_ok());
    said_writing(
        session
            .write_file("D1", &path, Some("global.general.length.unit : 1000 "))
            .is_ok(),
    );
    said_reading(
        session
            .read_shape_file("S4", &path, Some("global.general.length.unit : 1 "))
            .is_ok(),
    );
    said_writing(session.write_shape_file("S1", &path, None).is_ok());
    said_reading(session.read_shape_file("S5", &path, None).is_ok());
    said_writing(
        session
            .write_file("D0", &path, Some("global.general.length.unit : 1 "))
            .is_ok(),
    );
    said_reading(
        session
            .read_file("D6", &path, Some("global.general.length.unit : 1 "))
            .is_ok(),
    );
    session.get_one_shape("S6", "D6");
    said_writing(
        session
            .write_shape_file("S1", &path, Some("global.general.length.unit : 25.4 "))
            .is_ok(),
    );
    said_reading(
        session
            .read_file("D7", &path, Some("global.general.length.unit : 0.0254 "))
            .is_ok(),
    );
    session.get_one_shape("S7", "D7");
    for (name, area, word) in [
        ("S0", 11995.4, "11995.4"),
        ("S1", 0.0119954, "0.0119954"),
        ("S2", 11979.2, "11979.2"),
        ("S3", 11979.2, "11979.2"),
        ("S4", 11979.2, "11979.2"),
        ("S5", 0.0119792, "0.0119792"),
        ("S6", 11979.2, "11979.2"),
        ("S7", 18.5678, "18.5678"),
    ] {
        check_area(session, name, area, word);
    }
    let _ = std::fs::remove_file(&path);
    for name in ["D0", "D1", "D6", "D7"] {
        run_command(session, "Close", &[name]);
    }
}
