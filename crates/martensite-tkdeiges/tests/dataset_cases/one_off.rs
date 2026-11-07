//! The scripts whose `if`, `regexp` and `[...]` forms decide on values only a run has: one function
//! each, on the same commands the tables use. A check here is the case's own, not a summary.
use crate::shape_commands::{dump_shape, explode};
use crate::steps::{catching, run_command};
use crate::support::{
    checkprops::CHECKPROPS_EPSILON,
    checkview::{CheckView, checkview},
    nbshapes::{Counting, nbshapes},
    properties::{Prop, props},
    session::Session as _,
};
use crate::table::RecordedVerdict;
use crate::xs_session::XsSession;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dOffsetCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::{Axis1, Axis3, Axis22d, Direction2, Direction3, Point2, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use std::f64::consts::PI;

macro_rules! one_off_case {
    ($(#[$marker:meta])* $name:ident, $source:literal, $verdict:ident, $script:path) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            crate::runner::run_one_off(
                concat!("one_off::", stringify!($name)),
                $source,
                RecordedVerdict::$verdict,
                $script,
            );
        }
    };
}

/// `checkview -display name -2d -path image`.
fn view_2d(session: &mut XsSession, name: &str, image: &str) {
    checkview(&mut session.raster, &CheckView::of(name).image_named(image));
}

/// The number the pattern `<label> ... (number)` finds on a line of `text`, as `regexp` reads it.
fn point_after(text: &str, label: &str) -> [String; 3] {
    let line = text
        .lines()
        .find(|line| line.starts_with(label))
        .unwrap_or_else(|| panic!("{label}: expected a line in {text}"));
    let inner = line
        .split_once('(')
        .and_then(|(_, rest)| rest.split_once(')'))
        .unwrap_or_else(|| panic!("{line}: expected a point"))
        .0;
    let parts: Vec<String> = inner.split(',').map(str::to_owned).collect();
    [parts[0].clone(), parts[1].clone(), parts[2].clone()]
}

/// `bugs/demo/bug14673_3`: a message resource in UTF-8, loaded instead of the standard one.
fn bug14673_3(session: &mut XsSession) {
    let message = "Б-сплайн поверхность не является гладкой";
    let file = session.image_file("XSTEP.ru");
    std::fs::write(
        &file,
        format!(".XSTEP_1\nMessage used to check file load\n.IGES_1250\n{message}\n"),
    )
    .expect("a resource file");
    martensite_tkernel::message::MsgFile::global()
        .load_file(Some(&file.to_string_lossy()))
        .expect("the resource file");
    println!("# Load IGES file which is known to generate the message, and check it");
    println!("REQUIRED 14673 ALL: {message}");
    run_command(session, "igesbrep", &["@data/hammer.iges", "a", "*"]);
    session.pilot_command("tpstat c");
}

/// `bugs/fclasses/bug22125`: a file name with characters out of ASCII.
fn bug22125(session: &mut XsSession) {
    let name = "bug22125_有用.igs";
    println!("Test saving file with non-ascii file name: {name}");
    let word = format!("@image/{name}");
    run_command(session, "file_delete", &[&word]);
    run_command(session, "box", &["b", "10", "10", "10"]);
    run_command(session, "brepiges", &["b", &word]);
    run_command(session, "igesbrep", &[&word, "result", "*"]);
    if !session.image_file(name).exists() {
        println!("Error: file is not found with expected name \"{name}\"");
    }
}

/// `bugs/fclasses/bug25367_igs`.
fn bug25367(session: &mut XsSession) {
    let word = "@image/written_25367_マヹヱ.igs";
    run_command(session, "box", &["b", "1", "1", "1"]);
    run_command(session, "param", &["write.iges.brep.mode", "1"]);
    run_command(session, "brepiges", &["b", word]);
    run_command(session, "igesbrep", &[word, "result", "*"]);
    run_command(session, "checkprops", &["result", "-s", "6"]);
    run_command(session, "checkshape", &["result"]);
    let counts =
        "-vertex 8 -edge 12 -wire 6 -face 6 -shell 1 -solid 1 -compsolid 0 -compound 0 -shape 34";
    let mut words = vec!["result"];
    words.extend(counts.split(' '));
    run_command(session, "checknbshapes", &words);
    run_command(session, "file_delete", &[word]);
    view_2d(session, "result", "bug25367_igs");
}

/// `bugs/iges/bug22820`: a plane stays a plane.
fn bug22820(session: &mut XsSession) {
    run_command(session, "box", &["b", "1", "1", "1"]);
    println!("dump b ");
    if dump_shape(session, "b").contains("BSplineSurface") {
        println!("Error in dump b");
    }
    run_command(session, "brepiges", &["b", "@image/22820.igs"]);
    run_command(session, "igesbrep", &["@image/22820.igs", "bb", "*"]);
    let types = session.pilot_command("listtypes");
    println!("types_info={types}");
    if types.contains("BSplineSurface") {
        println!("Error in listtypes");
    }
    run_command(session, "igesbrep", &["@image/22820.igs", "bb", "*"]);
    println!("dump bb ");
    if dump_shape(session, "bb").contains("BSplineSurface") {
        println!("Error in dump bb");
    }
}

fn out_of_range(value: f64) -> bool {
    const TOLERANCE: f64 = 1.0e-7;
    (value - TOLERANCE).abs() < 0.0 || value > 1.0
}

/// `bugs/iges/bug23018`: a tabulated cylinder's line, read from the session's model.
fn bug23018(session: &mut XsSession) {
    run_command(session, "restore", &["@data/bug23018_f-ext.brep", "f"]);
    run_command(session, "brepiges", &["f", "@image/bug23018_f-ext.igs"]);
    let info = session.pilot_command("entity 6");
    let start = point_after(&info, "Starting Point");
    let end = point_after(&info, "End Point");
    println!("x1={} y1={} z1={}", start[0], start[1], start[2]);
    println!("x2={} y2={} z2={}", end[0], end[1], end[2]);
    for point in [&start, &end] {
        let x: f64 = point[0].parse().expect("a real");
        let y: f64 = point[1].parse().expect("a real");
        if out_of_range(x) || out_of_range(y) {
            println!("Error : Bad Starting Point");
        }
    }
}

/// `bugs/iges/bug26138`: `checkprops a -s -equal b` measures length and area, printed mass against printed mass.
fn bug26138(session: &mut XsSession) {
    run_command(session, "igesbrep", &["@data/OCC26138_Torus.igs", "a", "*"]);
    run_command(session, "brepiges", &["a", "@image/bug26138.igs"]);
    run_command(session, "igesbrep", &["@image/bug26138.igs", "b", "*"]);
    run_command(session, "file_delete", &["@image/bug26138.igs"]);
    println!("checkprops a -s -equal b");
    for kind in [Prop::Length, Prop::Area] {
        let first = props(kind, &session.bound_shape("a"), Some(CHECKPROPS_EPSILON));
        let second = props(kind, &session.bound_shape("b"), Some(CHECKPROPS_EPSILON));
        if first != second {
            println!("Error: Shape b is not equal to shape a");
        }
    }
}

/// The wire of two edges of `bug29435` and `bug29435_1`.
fn wire_of_two_edges(session: &mut XsSession) {
    run_command(session, "vertex", &["v1", "-25", "0", "0"]);
    run_command(session, "vertex", &["v2", "-25", "-7.5", "0"]);
    run_command(session, "edge", &["e1", "v1", "v2"]);
    run_command(session, "point", &["p2", "-25", "-7.5", "0"]);
    run_command(session, "point", &["p3", "0", "-15", "0"]);
    run_command(session, "point", &["p4", "25", "-7.5", "0"]);
    catching(session, "gcarc", &["arc", "cir", "p2", "p3", "p4"]);
    run_command(session, "mkedge", &["e2", "arc"]);
    run_command(session, "wire", &["w1", "e1", "e2"]);
    session.new_document("D");
    session.add_shape("D", "w1", true, " 1");
}

fn colors_come_back(session: &mut XsSession, image: &str) {
    let word = format!("@image/{image}.igs");
    run_command(session, "WriteIges", &["D", &word]);
    run_command(session, "ReadIges", &["D1", &word]);
    if session.all_colors("D") != session.all_colors("D1") {
        println!("Error: incorrect color transfer");
    }
    run_command(session, "file_delete", &[&word]);
}

/// `bugs/iges/bug29435`.
fn bug29435(session: &mut XsSession) {
    catching(session, "Close", &["D"]);
    catching(session, "Close", &["D1"]);
    wire_of_two_edges(session);
    session.set_color("D", "0:1:1:1", &["128", "128", "0"]);
    colors_come_back(session, "bug29435");
}

/// `bugs/iges/bug29435_1`.
fn bug29435_1(session: &mut XsSession) {
    catching(session, "Close", &["D"]);
    catching(session, "Close", &["D1"]);
    wire_of_two_edges(session);
    session.expand_label("D", "0:1:1:1");
    session.set_color("D", "0:1:1:2", &["0", "128", "0"]);
    session.set_color("D", "0:1:1:3", &["128", "0", "0"]);
    colors_come_back(session, "bug29435_1");
}

/// `bugs/xde/bug12905`.
fn bug12905(session: &mut XsSession) {
    run_command(session, "vertex", &["v1", "0", "0", "0"]);
    run_command(session, "vertex", &["v2", "0", "0", "10"]);
    run_command(session, "edge", &["e1", "v1", "v2"]);
    let word = "@image/written_12905.igs";
    run_command(session, "file_delete", &[word]);
    let is_ok = catching(session, "brepiges", &["e1", word])
        && catching(session, "igesbrep", &[word, "result", "*"]);
    if !is_ok {
        println!("Faulty case");
        return;
    }
    let counts =
        "-vertex 2 -edge 1 -wire 0 -face 0 -shell 0 -solid 0 -compsolid 0 -compound 0 -shape 3";
    let mut words = vec!["result"];
    words.extend(counts.split(' '));
    run_command(session, "checknbshapes", &words);
    println!("OK case");
}

/// `bugs/xde/bug22470`: word 31 of the answer of `nbshapes` is the count of shapes.
fn bug22470(session: &mut XsSession) {
    run_command(session, "igesbrep", &["@data/occ22470.igs", "a", "*"]);
    println!("nbshapes a ");
    let counts = nbshapes(&session.raster, "a", Counting::Located);
    if counts[8] == 0 {
        println!("ERROR: the defect is reproduced");
    }
}

/// `test_offset`: a surface of revolution on a trimmed circle (`f1`), and on its offset (`f2`).
fn test_offset(session: &mut XsSession) {
    let z = Direction3::new(0.0, 0.0, 1.0).expect("z");
    let axis = Axis1::new(Point3::new(0.0, 0.0, 0.0), z);
    let frame = Axis22d::new(
        Point2::new(0.0, 0.0),
        Direction2::new(0.0, 1.0).expect("y"),
        false,
    )
    .expect("a frame");
    let yoz = Axis3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).expect("x"),
        Direction3::new(0.0, 1.0, 0.0).expect("y"),
    )
    .expect("a plane");
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(yoz)).cast();
    let circle: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dCircle::new(frame, 1.0).expect("a circle")).cast();
    let trimmed: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dTrimmedCurve::new(&circle, 0.0, PI / 2.0, true, true).expect("a trim"))
            .cast();
    let offset: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dOffsetCurve::new(&trimmed, -0.5, false).expect("an offset")).cast();
    println!("test_offset ");
    println!("<<<< Preparing sample surface of revolution based on trimmed curve >>>>");
    println!("-----------------------------------------------------------------------");
    revolved(session, &trimmed, &plane, &axis, "f1");
    println!("<<<< Preparing sample surface of revolution based on offset curve  >>>>");
    println!("-----------------------------------------------------------------------");
    revolved(session, &offset, &plane, &axis, "f2");
}

fn revolved(
    session: &mut XsSession,
    curve: &Handle<dyn Geom2dCurve>,
    plane: &Handle<dyn GeomSurface>,
    axis: &Axis1,
    name: &str,
) {
    let mut edge_maker = BRepBuilderAPIMakeEdge::from_pcurve(curve, plane).expect("an edge");
    let edge = edge_maker.shape().expect("an edge").clone();
    let mut maker = BRepPrimAPIMakeRevol::new(&edge, *axis, 2.0 * PI, false).expect("a revolution");
    let face = maker.shape().expect("a revolution").clone();
    session.raster.bind(name, &face);
    println!("Result: {name}");
}

fn offset_based_revolution(session: &mut XsSession, shape: &str, image: &str) {
    test_offset(session);
    let word = format!("@image/{shape}.igs");
    run_command(session, "brepiges", &[shape, &word]);
    run_command(session, "igesread", &[&word, "result", "*"]);
    view_2d(session, "result", image);
}

fn bug23771_1(session: &mut XsSession) {
    offset_based_revolution(session, "f1", "bug23771_1");
}

fn bug23771_2(session: &mut XsSession) {
    offset_based_revolution(session, "f2", "bug23771_2");
}

/// `bugs/xde/bug27069`: the parser does not read comments; both of its errors are the `TODO`'s.
fn bug27069(session: &mut XsSession) {
    run_command(
        session,
        "igesbrep",
        &["@data/bug27069_points-with-comments.igs", "a", "*"],
    );
    let log = session.pilot_command("data c");
    for pattern in [
        "Unknown message invoked with the keyword",
        "Directory Entry : parameter 1 (entity type) is incorrect",
    ] {
        if log.contains(pattern) {
            println!("Error: IGES parser does not recognize comments in Parameters section");
        } else {
            println!("OK: IGES parser recognize comments in Parameters section");
        }
    }
}

/// `bugs/xde/bug27447`: a long name of a label reaches the written file.
fn bug27447(session: &mut XsSession) {
    run_command(session, "pcylinder", &["pin", "1", "10"]);
    run_command(session, "pcylinder", &["nut", "2", "2"]);
    run_command(session, "compound", &["nut", "nut", "nuts"]);
    explode(session, &["nuts"]);
    run_command(session, "ttranslate", &["nuts_1", "0", "0", "7"]);
    run_command(session, "ttranslate", &["nuts_2", "0", "0", "1"]);
    run_command(session, "compound", &["pin", "nuts_1", "nuts_2", "link"]);
    session.new_document_with_format("D", "XCAF");
    session.add_shape("D", "link", true, "");
    let pin = session.find_shape("D", "pin");
    session.set_name("D", &pin, "Pin");
    let nut = session.find_shape("D", "nut");
    session.set_name("D", &nut, "Nut");
    let link = session.find_shape("D", "link");
    session.set_name("D", &link, "Very long link name");
    for (instance, name) in [
        (1, "Pin instance"),
        (2, "Nut instance 1"),
        (3, "Nut instance 2"),
    ] {
        let link = session.find_shape("D", "link");
        session.set_name("D", &format!("{link}:{instance}"), name);
    }
    let word = "@image/bug27447.igs";
    run_command(session, "WriteIges", &["D", word]);
    let written = std::fs::read_to_string(session.path_word(word)).expect("the written file");
    run_command(session, "file_delete", &[word]);
    if !written
        .lines()
        .any(|line| line.trim().contains("Very long link name"))
    {
        println!("ERROR: the defect is reproduced. Long labels in IGES files are not supported.");
    }
}

fn written_in_faces_and_read(session: &mut XsSession, input: &str, image: &str) {
    run_command(session, "restore", &[input, "result"]);
    let word = "@image/written_816.stp";
    if !catching(session, "brepiges", &["result", word]) {
        println!("Faulty case : brepiges not complete");
    }
    if !catching(session, "igesbrep", &[word, "a", "*"]) {
        println!("Faulty case : igesbrep not complete");
    }
    view_2d(session, "result", image);
}

fn bug816_1(session: &mut XsSession) {
    written_in_faces_and_read(session, "@data/OCC816-apple.rle", "bug816_1");
}

fn bug816_2(session: &mut XsSession) {
    written_in_faces_and_read(session, "@data/OCC816-lemon.rle", "bug816_2");
}

one_off_case!(
    bugs_demo_bug14673_3,
    "tests/bugs/demo/bug14673_3",
    Ok,
    bug14673_3
);
one_off_case!(
    bugs_fclasses_bug22125,
    "tests/bugs/fclasses/bug22125",
    Ok,
    bug22125
);
one_off_case!(
    bugs_fclasses_bug25367_igs,
    "tests/bugs/fclasses/bug25367_igs",
    Ok,
    bug25367
);
one_off_case!(bugs_iges_bug22820, "tests/bugs/iges/bug22820", Ok, bug22820);
one_off_case!(bugs_iges_bug23018, "tests/bugs/iges/bug23018", Ok, bug23018);
one_off_case!(bugs_iges_bug26138, "tests/bugs/iges/bug26138", Ok, bug26138);
one_off_case!(bugs_iges_bug29435, "tests/bugs/iges/bug29435", Ok, bug29435);
one_off_case!(
    bugs_iges_bug29435_1,
    "tests/bugs/iges/bug29435_1",
    Ok,
    bug29435_1
);
one_off_case!(bugs_xde_bug12905, "tests/bugs/xde/bug12905", Ok, bug12905);
one_off_case!(bugs_xde_bug22470, "tests/bugs/xde/bug22470", Ok, bug22470);
one_off_case!(
    bugs_xde_bug23771_1,
    "tests/bugs/xde/bug23771_1",
    Ok,
    bug23771_1
);
one_off_case!(
    bugs_xde_bug23771_2,
    "tests/bugs/xde/bug23771_2",
    Ok,
    bug23771_2
);
one_off_case!(
    bugs_xde_bug27069,
    "tests/bugs/xde/bug27069",
    Known,
    bug27069
);
one_off_case!(bugs_xde_bug27447, "tests/bugs/xde/bug27447", Ok, bug27447);
one_off_case!(bugs_xde_bug816_1, "tests/bugs/xde/bug816_1", Ok, bug816_1);
one_off_case!(bugs_xde_bug816_2, "tests/bugs/xde/bug816_2", Ok, bug816_2);
one_off_case!(
    de_wrapper_iges_a5,
    "tests/de_wrapper/iges/A5",
    Ok,
    crate::wrapper_cases::wrapper_a5
);
one_off_case!(
    de_wrapper_iges_a6,
    "tests/de_wrapper/iges/A6",
    Ok,
    crate::wrapper_cases::wrapper_a6
);
