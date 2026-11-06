//! The scripts whose `if`, `regexp` and `[XGetAll...]` forms decide on values only a run has: one
//! function each, on the same commands the tables use. A check here is the case's own, not a summary.
use crate::reports::g17;
use crate::support::session::{Drawing, Session, ViewStep};
use crate::support::{
    checkprops::{CheckTolerances, Expected, checkprops_within},
    checkview::{CheckView, checkview, screenshot},
    maxtolerance::maxtolerance,
    properties::{Prop, props},
    script_error::script_error,
    tolerance_check::checkreal,
    tricheck::tricheck,
};
use crate::table::Place;
use crate::xs_session::XsSession;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkernel::{quantity::Color, standard::Handle, tcollection::ExtendedString};
use martensite_tkg2d::geom2d::Geom2dTrimmedCurve;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tklcaf::tdf::TdfAttributeState;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorTool, XCAFDocDocumentTool, XCAFDocEditor};

/// `XGetAllColors doc`: the colour names, each followed by a blank.
pub fn all_colors(session: &XsSession, doc: &str) -> String {
    println!("XGetAllColors {doc} ");
    let main = session.documents[doc]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
    let mut text = String::new();
    for label in tool.get_colors().expect("colors") {
        let color = XCAFDocColorTool::get_color(&label)
            .expect("a color")
            .expect("a colour label");
        text += Color::string_name(color.name());
        text.push(' ');
    }
    println!("{text}");
    text
}

/// `XGetAllLayers doc`: each layer's name in quotes.
pub fn all_layers(session: &XsSession, doc: &str) -> String {
    println!("XGetAllLayers {doc} ");
    let main = session.documents[doc]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::layer_tool(&main).expect("a layer tool");
    let layer_tool = tool.get().expect("a layer tool");
    let mut text = String::new();
    for label in layer_tool.get_layer_labels().expect("layers") {
        let name = layer_tool
            .get_layer(&label)
            .expect("a layer")
            .unwrap_or_default();
        let name = martensite_tkernel::tcollection::AsciiString::from(&name);
        text += &format!("\"{}\" ", String::from_utf8_lossy(name.as_bytes()));
    }
    println!("{text}");
    text
}

fn read_step_or_stop(session: &mut XsSession, doc: &str, name: &'static str) {
    let file = session.path_of(Place::Data(name));
    println!("ReadStep {doc} {file} ");
    if session.read_step(doc, &file).is_err() {
        script_error(&format!("ReadStep {doc} {file}"));
    }
}

fn stepread_data(session: &mut XsSession, name: &'static str, prefix: &str) -> String {
    let file = session.path_of(Place::Data(name));
    println!("stepread {file} {prefix} * ");
    session.stepread(&file, prefix)
}

/// `bugs/step/bug222`.
pub fn bug222(session: &mut XsSession) {
    stepread_data(session, "trj3_s1-ac-214.stp", "a");
    session.tpcompound("result");
    let report = maxtolerance(&session.raster, "result");
    for _ in 0..3 {
        print!("{report}");
    }
    let largest = |kind: &str| -> f64 {
        report
            .lines()
            .find(|line| line.trim_start().starts_with(kind))
            .and_then(|line| line.rsplit("Max").next())
            .and_then(|tail| tail.trim().parse().ok())
            .unwrap_or_else(|| panic!("maxtolerance: no {kind} line in {report:?}"))
    };
    if largest("Face") > 1.0 || largest("Edge") > 1.0 || largest("Vertex") > 1.0 {
        println!("Error : Tolerance of shape is more then 1.0");
    }
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/step/bug23379`.
pub fn bug23379(session: &mut XsSession) {
    crate::support::restore::restore(&mut session.raster, "bug23379_point.brep", "point");
    let file = session.path_of(Place::Image("bug23379_point.step"));
    println!("stepwrite 0 point {file} ");
    session.stepwrite("0", "point", &file);
    println!("stepread {file} a * ");
    session.stepread(&file, "a");
    let count = session.explode("a_1", "");
    println!("L={count}");
    if count == 1 {
        println!("OK : Number of points is good");
    } else {
        println!("Error : Number of points is bad");
    }
    let _ = std::fs::remove_file(&file);
}

/// `bugs/step/bug2368`: `lsearch` looks for a word `Error` in what `stepread` returns.
pub fn bug2368(session: &mut XsSession) {
    let returned = stepread_data(session, "OCC2368.stp", "a");
    if returned.split_whitespace().any(|word| word == "Error") {
        println!("2368: Error during reading attached IGES file");
    } else {
        println!("2368: OK");
    }
}

/// `bugs/step/bug28345`.
pub fn bug28345(session: &mut XsSession) {
    read_step_or_stop(session, "D_1", "bug28345_30338.stp");
    if !all_layers(session, "D_1").contains("COMPONENTS") {
        println!("Error : wrong layer name");
    }
    println!("Close D_1 ");
    session.close_document("D_1");
}

/// `bugs/step/bug33261`.
pub fn bug33261(session: &mut XsSession) {
    read_step_or_stop(session, "D", "bug33261.stp");
    println!("XGetOneShape S1 D ");
    if session.get_one_shape("S1", "D").is_err() {
        script_error("XGetOneShape S1 D");
    }
    session.check_counts("S1", &["-face", "2025", "-solid", "24", "-compound", "100"]);
    println!("Close D -silent ");
    session.close_document("D");
}

/// `bugs/step/bug3883_N`: one bolt or bracket file, colours and layers read.
pub fn bug3883(session: &mut XsSession, file: &'static str) {
    read_step_or_stop(session, "D", file);
    let colors = all_colors(session, "D");
    let layers = all_layers(session, "D");
    if colors.is_empty() {
        println!("ERROR: 3883 is reproduced. Colors have not been read.");
    }
    if layers.is_empty() {
        println!("ERROR: 3883 is reproduced. Layers have not been read.");
    }
}

/// `bugs/xde/bug25176`.
pub fn bug25176(session: &mut XsSession) {
    stepread_data(session, "bug25176_occ25176.stp", "step");
    let log = session.pilot_command("data c");
    if log.contains("F:APPLICATION_PROTOCOL_DEFINITION: Parameter n0.4") {
        println!("OK: correct report on wrong type entity");
    } else {
        println!("Error: wrong report on wrong type entity");
    }
}

/// `bugs/xde/bug23010`.
pub fn bug23010(session: &mut XsSession) {
    read_step_or_stop(session, "D", "occ23010.step");
    session.pilot_command("data c");
    session.pilot_command("tpstat c");
}

/// `bugs/step/bug46`: the lengths of three edges survive a write and a read.
pub fn bug46(session: &mut XsSession) {
    crate::support::restore::restore(&mut session.raster, "OCC46.brep", "sh");
    session.explode("sh", "e");
    let before = three_edge_lengths(session, "sh");
    println!("{before}");
    let file = session.path_of(Place::Image("sss.step"));
    let _ = std::fs::remove_file(&file);
    println!("stepwrite a sh {file} ");
    session.stepwrite("a", "sh", &file);
    println!("stepread {file} ss * ");
    session.stepread(&file, "ss");
    session.tpcompound("result");
    session.explode("result", "e");
    let after = three_edge_lengths(session, "result");
    println!("{after}");
    if (before - after).abs() > 0.0001 {
        println!("Error : large difference between shapes");
    }
    checkview(&mut session.raster, &CheckView::of("result"));
}

fn three_edge_lengths(session: &mut XsSession, prefix: &str) -> f64 {
    [4, 7, 9]
        .iter()
        .map(|number| {
            let name = format!("{prefix}_{number}");
            let length = props(Prop::Length, &session.bound_shape(&name), None);
            println!("lprops {name} ");
            println!(
                "Mass : {:>15}",
                martensite_tkernel::storage::format_general(length, 6)
            );
            length
        })
        .sum()
}

/// `bugs/step/bug630`.
pub fn bug630(session: &mut XsSession) {
    crate::support::restore::restore(&mut session.raster, "OCC630.brep", "a1");
    session.check_shape("a1");
    let file = session.path_of(Place::Image("solid.stp"));
    let _ = std::fs::remove_file(&file);
    println!("stepwrite a a1 {file} ");
    session.stepwrite("a", "a1", &file);
    println!("stepread {file} a2 * ");
    session.stepread(&file, "a2");
    session.tpcompound("result");
    session.check_shape("result");
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/moddata_1/bug22459`: `lsearch` looks for the word `exception` in `tpstat c`.
pub fn bug22459(session: &mut XsSession) {
    stepread_data(session, "OCC22459-TEST2.stp", "result");
    let message = session.pilot_command("tpstat c");
    if message.split_whitespace().any(|word| word == "Exception") {
        println!("Faulty 22459");
    } else {
        println!("OK 22459");
    }
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/xde/bug23009`.
pub fn bug23009(session: &mut XsSession) {
    let file = session.path_of(Place::Data("OCC22459-TEST2.stp"));
    println!("stepfileunits {file} ");
    let units = crate::script_commands::stepfileunits(&file);
    let holds = |unit: &str| units.split_whitespace().any(|word| word == unit);
    if holds("centimetre") && holds("DEGREES") && holds("steradian") {
        println!("OK 23009");
    } else {
        println!("Faulty 23009");
    }
}

/// `bugs/xde/bug23597`.
pub fn bug23597(session: &mut XsSession) {
    let file = session.path_of(Place::Image("bug23597_f-ellipse.stp"));
    if std::path::Path::new(&file).exists() {
        println!("Error : There is old {file} file");
    }
    session.pilot_command("xnorm STEP");
    crate::support::restore::restore(&mut session.raster, "bug23597_f-ellipse.brep", "f");
    session.pilot_command("param write.step.unit FT");
    session.pilot_command("newmodel");
    println!("stepwrite a f {file} ");
    session.stepwrite("a", "f", &file);
    if !std::path::Path::new(&file).exists() {
        println!("Error : There is not {file} file");
    }
    let _ = std::fs::remove_file(&file);
}

/// `bugs/mesh/bug27453`.
pub fn bug27453(session: &mut XsSession) {
    stepread_data(session, "bug27453_165-174_piattello.stp", "a");
    session.explode_echoed("a_1", "F");
    session.renamevar("a_1_13", "result");
    session.check_shape("result");
    crate::meshing::incmesh(&session.raster, &["result", "0.01", "-a", "0.5"]);
    if !tricheck("result", &session.bound_shape("result")).is_empty() {
        println!("Error: Wrongly meshed surface");
    }
}

/// `bugs/mesh/bug29715`: its `chrono` only prints.
pub fn bug29715(session: &mut XsSession) {
    stepread_data(session, "bug29715_slow.stp", "a");
    session.renamevar("a_1", "a");
    crate::meshing::incmesh(&session.raster, &["a", "0.0027"]);
    tricheck("a", &session.bound_shape("a"));
}

/// `bugs/step/bug24595`.
pub fn bug24595(session: &mut XsSession) {
    stepread_data(session, "bug24595_281390.stp", "a");
    session.explode("a_1", "f");
    session.check_counts("a_1_3", &["-wire", "1"]);
    let bounds = session.bounding("a_1_3", " -dump -save Xmin Ymin Zmin Xmax Ymax Zmax", true);
    let expected = [
        "-7.1677412321949925",
        "-8.0000000999999994",
        "-150.00000009999999",
        "7.1677412321949898",
        "8.0000000999999994",
        "-141.99999990000001",
    ];
    for (index, name) in ["Xmin", "Ymin", "Zmin", "Xmax", "Ymax", "Zmax"]
        .iter()
        .enumerate()
    {
        let value = g17(bounds[index]);
        println!("dval {name} ");
        println!("{value}");
        checkreal(name, &value, expected[index], 0.1, 0.0);
    }
}

/// The names `bugs/step/bug28454_2` expects on `0:1:1:N`, with what the script says of each.
const DIRECTIVE_NAMES: [(&str, &str); 15] = [
    (
        "AaBbCc\u{c4}\u{e4}\u{d6}\u{f6}\u{dc}\u{fc}\u{df}*,.-;:_",
        "\\X2\\ directives",
    ),
    ("\u{3c0}", "\\X2\\ directive"),
    (
        "Expression: \u{3b1}\u{3b2}\\X2\\03B3*\u{3c0}",
        "\\X2\\ directive",
    ),
    ("\u{1f7ca}", "\\X4\\ directive"),
    (
        "Star, arrow, faces: \u{1f7ca}\u{1f80a}\u{1f637}\u{1f926} end",
        "\\X4\\ directive",
    ),
    ("\u{c4}rger", "\\S\\ directive"),
    ("h\u{f4}tel", "\\S\\ directive"),
    ("\u{40a}\u{435}\u{442}", "\\PE\\ and \\S\\ directives"),
    ("see \u{a7} 4.1", "\\X\\ directive"),
    ("line one\nline two", "\\X\\ directives"),
    (
        "Expression: \u{b1}3\u{3c0}*\u{3c0}+12",
        "\\X2\\ and \\X\\ directives",
    ),
    (
        "Expression: \u{3b1}*\u{3b2}\u{f7}\u{3b3}",
        "\\X2\\,\\PC\\ and \\S\\ directives",
    ),
    ("T\u{415}C\u{422}", "\\S\\ and \\PE\\ directives"),
    (
        "A\u{c4}aBbCc\u{c4}'h\u{f4}\u{e4}\u{1f7ca}\n\u{1f7ca}*,.-;:_\u{d6}\u{415}",
        "\\X\\,\\X2\\,\\X4\\ and \\S\\ directive",
    ),
    ("\u{3b1}\u{3b2}\u{3b3}", "\\X2\\ directive"),
];

/// `bugs/step/bug28454_2`.
pub fn bug28454_2(session: &mut XsSession) {
    session.close_silently("D");
    read_step_or_stop(session, "D", "bug28454_directives.stp");
    for (index, (expected, directive)) in DIRECTIVE_NAMES.iter().enumerate() {
        if session.get_name("D", &format!("0:1:1:{}", index + 1)) != *expected {
            println!("Error:  name has been lost on conversion {directive}");
        }
    }
}

/// `bugs/step/bug31670`.
pub fn bug31670(session: &mut XsSession) {
    session.pilot_command("param read.step.codepage CP1251");
    read_step_or_stop(session, "D", "bug31670_russian.stp");
    let assembly = "\u{421}\u{431}\u{43e}\u{440}\u{43a}\u{430}1";
    let part = "\u{414}\u{435}\u{442}";
    let mut is_ok = session.get_name("D", "0:1:1:1") == assembly;
    for number in 1..9 {
        if session.get_name("D", &format!("0:1:1:{}", number + 1)) != format!("{part}{number}") {
            is_ok = false;
        }
    }
    if !is_ok {
        println!("Error: unable to read CP1251 STEP");
    }
    println!("Close D ");
    session.close_document("D");
}

/// `bugs/step/bug33487`: a file with every `FILL_AREA_STYLE_COLOUR` removed reads without a crash.
pub fn bug33487(session: &mut XsSession) {
    let source = session.path_of(Place::Data("trj6_as1-hc-214.stp"));
    let text =
        std::fs::read(&source).unwrap_or_else(|cause| script_error(&format!("{source}: {cause}")));
    let mut broken = Vec::new();
    for part in text.split(|byte| *byte == b';') {
        if !part
            .windows(22)
            .any(|window| window == b"FILL_AREA_STYLE_COLOUR")
        {
            broken.extend_from_slice(part);
            broken.push(b';');
        }
    }
    broken.push(b'\n');
    let file = session.path_of(Place::Image("bug33487.stp"));
    std::fs::write(&file, broken).unwrap_or_else(|cause| script_error(&format!("{file}: {cause}")));
    println!("REQUIRED All: ERR StepReaderData : Unresolved Reference : Fails Count : 39");
    if session.read_file("aDoc", &file, None).is_err() {
        script_error(&format!("ReadFile aDoc {file}"));
    }
    let _ = std::fs::remove_file(&file);
}

/// `bugs/step/bug33569`.
pub fn bug33569(session: &mut XsSession) {
    read_step_or_stop(session, "D", "bug33569.stp");
    let dump = session.dump_nb_dgts("D");
    let attached = dump
        .lines()
        .find(|line| line.contains("NbOfAttachedDatum"))
        .and_then(|line| line.rsplit(':').next())
        .and_then(|word| word.trim().parse::<i64>().ok());
    if attached != Some(26) {
        println!("Error: Number of Datums incorrect");
    }
}

/// `bugs/step/bug33638`.
pub fn bug33638(session: &mut XsSession) {
    read_step_or_stop(session, "D", "bug33638.stp");
    if session.find_color("D", 0.0, 0.0, 1.0).is_empty() {
        println!("Error: Color not found");
    }
    if session.find_color("D", 1.0, 0.0, 0.0).is_empty() {
        println!("Error: Color not found");
    }
    println!("Close D ");
    session.close_document("D");
}

/// `bugs/xde/bug2845`: the count after the last `:` of `listtypes`.
pub fn bug2845(session: &mut XsSession) {
    stepread_data(session, "ec_exhaust-A.stp", "a");
    let listed = session.pilot_command("listtypes xst-transferrable-roots");
    let count = listed
        .rsplit(':')
        .next()
        .unwrap_or("")
        .trim()
        .split(' ')
        .next()
        .unwrap_or("");
    if count != "4" {
        println!("ERROR: 2845 is reproduced");
    }
}

/// `bugs/xde/bug33100`: a rescale by 1000 scales the box's diagonal, triangulations included.
pub fn bug33100(session: &mut XsSession) {
    session.close_silently("d");
    read_step_or_stop(session, "d", "bug33100_window.step");
    must_get_one_shape(session, "a", "d");
    let reference = diagonal(session);
    println!("XRescaleGeometry d 1000 ");
    let main = session.documents["d"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let shapes = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let shapes = shapes.get().expect("a shape tool").label();
    let rescaled = XCAFDocEditor::rescale_geometry(&shapes, 1000.0, false)
        .unwrap_or_else(|failure| panic!("XRescaleGeometry: {failure:?}"));
    assert!(rescaled, "XRescaleGeometry d 1000: expected a rescale");
    must_get_one_shape(session, "a", "d");
    let current = diagonal(session);
    checkreal(
        "bounding box diagonal",
        &g17(current),
        &g17(reference * 1000.0),
        0.0,
        0.001,
    );
}

fn must_get_one_shape(session: &mut XsSession, shape: &str, doc: &str) {
    println!("XGetOneShape {shape} {doc} ");
    if session.get_one_shape(shape, doc).is_err() {
        script_error(&format!("XGetOneShape {shape} {doc}"));
    }
}

fn diagonal(session: &XsSession) -> f64 {
    let bounds = session.bounding("a", "", false);
    let (dx, dy, dz) = (
        bounds[3] - bounds[0],
        bounds[4] - bounds[1],
        bounds[5] - bounds[2],
    );
    let value = (dx * dx + dy * dy + dz * dz).sqrt();
    println!("{}", g17(value));
    value
}

/// `de_wrapper/step/A5`.
pub fn de_wrapper_a5(session: &mut XsSession) {
    let file = session.path_of(Place::Data("bm2_pe_t4-A.stp"));
    session.close_document("D2");
    session.close_document("D3");
    session.pilot_command("param xstep.cascade.unit MM");
    println!("stepread {file} res0 * ");
    session.stepread(&file, "res0");
    println!("OK : Reading is correct");
    session.pilot_command("param xstep.cascade.unit M");
    println!("stepread {file} res1 * ");
    session.stepread(&file, "res1");
    println!("OK : Reading is correct");
    report_read(session.read_file("D2", &file, None));
    must_get_one_shape(session, "S2", "D2");
    if session.get_length_unit("D2") != "mm" {
        println!("Error: incrorrect document's length unit");
    }
    println!("XNewDoc D3 ");
    session.new_document("D3");
    report_read(session.read_file("D3", &file, Some("global.general.length.unit : 1000 ")));
    if session.get_length_unit("D3") != "m" {
        println!("Error: incrorrect document's length unit");
    }
    must_get_one_shape(session, "S3", "D3");
    report_read(session.read_shape_file("S4", &file, None));
    report_read(session.read_shape_file("S5", &file, Some("global.general.length.unit : 1000 ")));
    let areas = [
        ("res0_1", 2.52382e+06),
        ("res1_1", 2.52382),
        ("S2", 2.52382e+06),
        ("S3", 2.52382),
        ("S4", 2.52382e+06),
        ("S5", 2.52382),
    ];
    for (name, area) in areas {
        checkprops_eps(session, name, area);
    }
    for doc in ["D2", "D3"] {
        println!("Close {doc} ");
        session.close_document(doc);
    }
}

/// `checkprops name -s value -eps 1e-2`.
fn checkprops_eps(session: &XsSession, name: &str, area: f64) {
    println!("checkprops {name} -s {area} -eps 1e-2 ");
    checkprops_within(
        &session.raster,
        name,
        Prop::Area,
        Expected::Mass(area),
        CheckTolerances {
            epsilon: 1e-2,
            ..CheckTolerances::default()
        },
    );
}

fn report_read(step: Result<(), crate::documents::CommandFailed>) {
    println!(
        "{}",
        if step.is_ok() {
            "OK : Reading is correct"
        } else {
            "Error : Problem with reading file"
        }
    );
}

fn report_write(step: Result<(), crate::documents::CommandFailed>) {
    println!(
        "{}",
        if step.is_ok() {
            "OK : Writing is correct"
        } else {
            "Error : Problem with writing file"
        }
    );
}

/// `de_wrapper/step/A6`.
pub fn de_wrapper_a6(session: &mut XsSession) {
    let file = session.path_of(Place::Data("bm2_pe_t4-A.stp"));
    let written = session.path_of(Place::Image("A6.stp"));
    for doc in ["D0", "D1", "D6", "D7"] {
        session.close_document(doc);
    }
    session.pilot_command("param xstep.cascade.unit MM");
    println!("ReadStep D0 {file} ");
    report_read(session.read_step("D0", &file));
    must_get_one_shape(session, "S0", "D0");
    session.pilot_command("param xstep.cascade.unit M");
    println!("ReadStep D1 {file} ");
    report_read(session.read_step("D1", &file));
    must_get_one_shape(session, "S1", "D1");
    session.pilot_command("param xstep.cascade.unit MM");
    report_write(session.write_file("D0", &written, Some("global.general.length.unit : 25.4 ")));
    report_read(session.read_shape_file("S2", &written, None));
    report_write(session.write_shape_file("S0", &written, Some("global.general.system.unit : 1 ")));
    report_read(session.read_shape_file("S3", &written, None));
    report_write(session.write_file("D1", &written, Some("global.general.length.unit : 1000 ")));
    report_read(session.read_shape_file("S4", &written, Some("global.general.length.unit : 1 ")));
    report_write(session.write_shape_file("S1", &written, None));
    report_read(session.read_shape_file("S5", &written, None));
    report_write(session.write_file("D0", &written, Some("global.general.length.unit : 1 ")));
    report_read(session.read_file("D6", &written, Some("global.general.length.unit : 1 ")));
    must_get_one_shape(session, "S6", "D6");
    report_write(session.write_shape_file(
        "S1",
        &written,
        Some("global.general.length.unit : 25.4 "),
    ));
    report_read(session.read_file("D7", &written, Some("global.general.length.unit : 0.0254")));
    must_get_one_shape(session, "S7", "D7");
    let areas = [
        ("S0", 2.52381e+06),
        ("S1", 2.52381),
        ("S2", 2.52381e+06),
        ("S3", 2.52381e+06),
        ("S4", 2.52381e+06),
        ("S5", 2.52381),
        ("S6", 2.52381e+06),
        ("S7", 3911.92),
    ];
    for (name, area) in areas {
        checkprops_eps(session, name, area);
    }
    let _ = std::fs::remove_file(&written);
    for doc in ["D0", "D1", "D6", "D7"] {
        println!("Close {doc} ");
        session.close_document(doc);
    }
}

/// `bugs/heal/bug33398`.
pub fn bug33398(session: &mut XsSession) {
    let file = session.path_of(Place::Data("bug33398.step"));
    println!("testreadstep {file} s ");
    session.testreadstep(&file, "s");
    session.unifysamedom("result", "s");
    session.check_counts(
        "result",
        &[
            "-vertex", "506", "-edge", "908", "-wire", "394", "-face", "382", "-shell", "4",
            "-solid", "4",
        ],
    );
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/modalg_5/bug24746`.
pub fn bug24746(session: &mut XsSession) {
    read_step_or_stop(session, "doc", "bug24746_ENI.stp");
    println!("XGetShape c doc 0:1:1:1 ");
    session.get_shape_at("c", "doc", "0:1:1:1");
    session.explode_echoed("c", "f");
    session.explode_echoed("c_2", "e");
    let mut filler = session.bop("c_1", "c_2");
    session.bopcommon(&mut filler, "res1");
    let mut filler = session.bop("c_1", "c_2_4");
    session.bopcommon(&mut filler, "result");
    session.check_counts(
        "result",
        &[
            "-vertex",
            "2",
            "-edge",
            "1",
            "-wire",
            "0",
            "-face",
            "0",
            "-shell",
            "0",
            "-solid",
            "0",
            "-compsolid",
            "0",
            "-compound",
            "1",
            "-shape",
            "4",
        ],
    );
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/modalg_6/bug26308`: a B-spline surface evaluated far outside its domain does not crash.
pub fn bug26308(session: &mut XsSession) {
    read_step_or_stop(session, "D_First", "OCC26308-Face.stp");
    must_get_one_shape(session, "rr", "D_First");
    session.explode_echoed("rr", "f");
    println!("mksurface ss rr_1 ");
    let face = topods::face(&session.bound_shape("rr_1")).expect("a face");
    let surface = BRepTool::surface(&face).expect("a surface");
    println!("svalue ss -1.427997381773311e+018 4.512451574816904e+016 xx yy zz ");
    let point = surface
        .get()
        .expect("a surface")
        .value(-1.427997381773311e+018, 4.512451574816904e+016)
        .expect("a value");
    let checks = [
        ("xx", point.x(), -5.42659125962715e+207, "X"),
        ("yy", point.y(), 2.90803974424011e+209, "Y"),
        ("zz", point.z(), -3.1731154470359e+207, "Z"),
    ];
    for (name, value, expected, axis) in checks {
        println!("dump {name} ");
        println!("*********** Dump of {name} *************");
        let printed = martensite_tkernel::storage::format_general(value, 15);
        println!("{printed}");
        if printed.parse::<f64>().expect("a real") != expected {
            println!("ERROR: 26308 is reproduced. {axis} value is incorrect.");
        }
    }
}

/// `bugs/modalg_7/bug22821`: twelve fillets, each checked as the script writes it, only the area differs.
pub fn bug22821(session: &mut XsSession) {
    stepread_data(session, "bug22821_test.step", "t");
    session.renamevar("t_1", "m");
    session.check_shape("m");
    session.explode_echoed("m", "E");
    let areas = [
        0.220328, 0.220328, 0.220328, 0.220328, 0.218602, 0.220328, 0.218602, 0.220328, 0.218602,
        0.218602, 0.220328, 0.220328,
    ];
    for (index, area) in areas.into_iter().enumerate() {
        let number = index + 1;
        let (result, edge) = (format!("res{number}"), format!("m_{number}"));
        if session.fillet(&result, "m", 0.01, &edge) {
            session.check_shape(&result);
            println!("checkprops {result} -s {area}");
            checkprops_within(
                &session.raster,
                &result,
                Prop::Area,
                Expected::Mass(area),
                CheckTolerances::default(),
            );
            checkview(&mut session.raster, &CheckView::of(&result));
        } else {
            println!("Error: Crash with BRepFilletAPIMakeFillet");
        }
    }
}

/// `bugs/modalg_7/bug28366`: the heading declares the `TODO` for both errors; `bsection`'s check reads `Log_1`, as the script does.
pub fn bug28366(session: &mut XsSession) {
    stepread_data(session, "bug28366_fuselage.stp", "a");
    session.renamevar("a_1", "a");
    session.explode_echoed("a", "f");
    let faces = ["a_1", "a_2", "a_3", "a_4", "a_5", "a_6", "a_7", "a_8"];
    session.sewing("shell", 1.0e-6, "1.0e-6", &faces);
    session.mkvolume("fuselage", &["shell"]);
    println!("plane pln ");
    println!("mkface face pln ");
    let plane = martensite_tkmath::gp::Plane3::default();
    let face = BRepBuilderAPIMakeFace::from_plane(plane)
        .expect("a face")
        .shape()
        .expect("a face")
        .clone();
    session.raster.bind("face", &face);
    let first_log = session.bsmt("bcommon", Operation::Common, "common", "face", "fuselage");
    if first_log.contains("Warning") {
        println!("Error: BOP Common fails with solid and planar face");
    }
    if !session
        .bopcheck("common")
        .contains("This shape seems to be OK")
    {
        println!("Error: BOP Common fails with solid and planar face");
    }
    checkview(&mut session.raster, &CheckView::of("common"));
    session.bsection("section", "face", "fuselage");
    if first_log.contains("Warning") {
        println!("Error: BOP Section fails with solid and planar face");
    }
    if !session
        .bopcheck("section")
        .contains("This shape seems to be OK")
    {
        println!("Error: BOP Section fails with solid and planar face");
    }
}

/// `bugs/step/bug30378`.
pub fn bug30378(session: &mut XsSession) {
    println!("Prepare simple STEP model");
    println!("box b 10 10 10 ");
    let shape = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .expect("a box")
        .shape()
        .expect("a box")
        .clone();
    session.raster.bind("b", &shape);
    println!("stepwrite a b ");
    session.stepwrite_to_model("a", "b");
    println!("Write entity #3");
    let file = session.path_of(Place::Image("bug30378.stp"));
    let result = session.pilot_command(&format!("writeent {file} 3"));
    let written = result.lines().any(|line| {
        line.split_whitespace()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|pair| pair[0].ends_with("Write") && pair[1] == "Done")
    });
    if !written {
        println!("Error: expected output is not found");
    }
}

/// `bugs/step/bug31000`.
pub fn bug31000(session: &mut XsSession) {
    session.close_silently("D");
    read_step_or_stop(session, "D", "bug31000_InSensitive.stp");
    if !session
        .pilot_command("data c")
        .contains("361 Entities, of which 4 Root")
    {
        println!("Error : wrong count of transferred roots");
    }
    let statistics = session.statistics_with_structure("D");
    if !statistics.contains("Number of labels with color link = 1") {
        println!("Error : wrong count of color link");
    }
    if !statistics.contains("0:1:1:1 SOLID  has attributes: Color(RED 1)") {
        println!("Error : wrong shape 0:1:1:1");
    }
    must_get_one_shape(session, "res", "D");
    session.check_counts("res", &["-face", "6", "-shell", "1", "-solid", "1"]);
    println!("checkshape res f ");
    println!("{}", session.check_short("res"));
}

/// `bugs/step/bug32556`.
pub fn bug32556(session: &mut XsSession) {
    println!("ptorus a1 5 3 ");
    let torus = BRepPrimAPIMakeTorus::new(5.0, 3.0)
        .expect("a torus")
        .shape()
        .expect("a torus")
        .clone();
    session.raster.bind("a1", &torus);
    println!("plane pp 5 0 -5 ");
    println!("pcylinder a2 pp 2 10 ");
    let axes = Axis2::new(
        Point3::new(5.0, 0.0, -5.0),
        Direction3::new(0.0, 0.0, 1.0).expect("a direction"),
    )
    .expect("axes");
    let mut cylinder = BRepPrimAPIMakeCylinder::with_axes(axes, 2.0, 10.0);
    let cylinder = cylinder.shape().expect("a cylinder").clone();
    session.raster.bind("a2", &cylinder);
    session.bsmt("bfuse", Operation::Fuse, "a", "a1", "a2");
    let file = session.path_of(Place::Image("bug32556.stp"));
    println!("testwritestep {file} a ");
    session.testwritestep(&file, "a");
    println!("testreadstep {file} res ");
    session.testreadstep(&file, "res");
    let _ = std::fs::remove_file(&file);
    println!("checkshape res f ");
    println!("{}", session.check_short("res"));
    session.check_counts("res", &["-edge", "12", "-vertex", "7"]);
    println!("checkprops res -s 649.9");
    checkprops_within(
        &session.raster,
        "res",
        Prop::Area,
        Expected::Mass(649.9),
        CheckTolerances::default(),
    );
    session.raster.view(ViewStep::Clear);
    session.raster.view(ViewStep::SmallView);
    session.raster.view(ViewStep::Donly(&["res"]));
    session.raster.view(ViewStep::Fit);
    screenshot(&mut session.raster, "bug32556");
}

/// `bugs/step/bug32922`: with healing off in the resource file, the torus's face keeps its edge order.
pub fn bug32922(session: &mut XsSession) {
    session.read_healing_off = true;
    println!("ptorus tor 20 5 ");
    let torus = BRepPrimAPIMakeTorus::new(20.0, 5.0)
        .expect("a torus")
        .shape()
        .expect("a torus")
        .clone();
    session.raster.bind("tor", &torus);
    let file = session.path_of(Place::Image("torus.stp"));
    println!("testwritestep {file} tor ");
    session.testwritestep(&file, "tor");
    println!("stepread {file} a * ");
    session.stepread(&file, "a");
    session.explode_echoed("a_1", "f");
    session.renamevar("a_1_1", "result");
    session.check_shape("result");
    println!("pcurve result ");
    println!("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL");
    let mut names = Vec::new();
    let face = topods::face(&session.bound_shape("result").oriented(Orientation::Forward))
        .expect("a face");
    let mut edges =
        Explorer::new(face.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape).expect("an explorer");
    let mut number = 1;
    while edges.more() {
        let edge = topods::edge(edges.current()).expect("an edge");
        let curve = BRepTool::curve_on_face(&edge, &face).expect("a pcurve");
        if curve.pcurve.is_null() {
            println!("Error: Edge {number} does not have pcurve");
        } else {
            let trimmed = Handle::new(
                Geom2dTrimmedCurve::new(&curve.pcurve, curve.first, curve.last, true, true)
                    .expect("a trimmed pcurve"),
            );
            let name = format!("result_{number}");
            session
                .raster
                .bind_drawing(&name, Drawing::PlaneCurve(&trimmed.cast()));
            names.push(name);
        }
        edges.next().expect("an explorer step");
        number += 1;
    }
    println!("view 1 -2D- 728 20 400 400 ");
    session.raster.view(ViewStep::PlaneView);
    let shown: Vec<&str> = names.iter().map(String::as_str).collect();
    session.raster.view(ViewStep::Disp(&shown));
    println!("2dfit ");
    session.raster.view(ViewStep::Fit);
    screenshot(&mut session.raster, "bug32922");
    let _ = std::fs::remove_file(&file);
}

/// Statistics line by line, with the order-free comparison of the colour and layer lists.
fn statistics_differences(current: &str, reference: &str) -> (usize, String) {
    let (current, reference): (Vec<&str>, Vec<&str>) = (
        current.split('\n').collect(),
        reference.split('\n').collect(),
    );
    let (mut differences, mut text) = (0, String::new());
    let mut next_is_a_list = false;
    for (at, current_line) in current.iter().enumerate() {
        let reference_line = reference.get(at).copied().unwrap_or("");
        let is_ok = if reference_line.contains("Number of colors")
            || reference_line.contains("Number of layers")
        {
            next_is_a_list = true;
            true
        } else if next_is_a_list {
            next_is_a_list = false;
            let (ours, theirs): (Vec<&str>, Vec<&str>) = (
                current_line.split(' ').collect(),
                reference_line.split(' ').collect(),
            );
            ours.len() == theirs.len() && theirs.iter().all(|word| ours.contains(word))
        } else {
            reference_line == *current_line
        };
        if !is_ok {
            differences += 1;
            text += &format!(
                " Reference data - {reference_line}\n Current data   - {current_line}\n----------------------------------------------\n"
            );
        }
    }
    (differences, text)
}

/// `bugs/xde/bug28104_1`.
pub fn bug28104_1(session: &mut XsSession) {
    session.close_silently("D0");
    session.close_silently("D1");
    read_step_or_stop(session, "D0", "as1-oc-214-mat.stp");
    println!("XNewDoc D1 ");
    session.new_document("D1");
    println!("XExtract D1 D0 0:1:1:1 ");
    let source = crate::script_values::label_of(session, "D0", "0:1:1:1");
    let main = session.documents["D1"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let destination = XCAFDocDocumentTool::shape_tool(&main)
        .expect("a shape tool")
        .get()
        .expect("a shape tool")
        .label();
    assert!(
        XCAFDocEditor::extract(&[source], &destination, false).expect("an extract"),
        "XExtract: expected a result"
    );
    println!("XStat D0 ");
    let first = crate::reports::statistics_with_text(&session.documents["D0"]).1;
    print!("{first}");
    println!("XStat D1 ");
    let second = crate::reports::statistics_with_text(&session.documents["D1"]).1;
    print!("{second}");
    assert!(!first.is_empty(), "XStat D0: expected statistics");
    let (differences, report) = statistics_differences(&first, &second);
    if differences > 0 {
        println!("Error : {differences} differences with reference data found :\n{report}");
    } else {
        println!("Comparison of current result with reference data - OK\n");
    }
    must_get_one_shape(session, "res0", "D0");
    must_get_one_shape(session, "res1", "D1");
    if session.check_props("D0") != session.check_props("D1") {
        println!("Error: Incorrect extraction of shape properties");
    }
    println!("checkprops res0 -equal res1");
    crate::support::checkprops::checkprops_equal(&session.raster, "res0", "res1");
    session.close_silently("D0");
    session.close_silently("D1");
}

/// `bugs/xde/bug28748`: `undo` gives the layer its child back.
pub fn bug28748(session: &mut XsSession) {
    read_step_or_stop(session, "d", "bug21802_as1-oc-214.stp");
    let document = session.documents["d"].clone();
    let document = document.get().expect("a document");
    println!("UndoLimit d 1 ");
    document.set_undo_limit(1).expect("an undo limit");
    println!(
        "{} {} {}",
        document.get_undo_limit(),
        document.get_available_undos(),
        document.get_available_redos()
    );
    println!("OpenCommand d ");
    document.open_command().expect("a command");
    println!("XUnSetLayer d 0:1:1:3 256 ");
    let label = crate::script_values::label_of(session, "d", "0:1:1:3");
    let tool = XCAFDocDocumentTool::layer_tool(&document.main().expect("a main label"))
        .expect("a layer tool");
    tool.get()
        .expect("a layer tool")
        .unset_one_layer(&label, &ExtendedString::from_utf8("256"))
        .expect("a layer removed");
    println!("CommitCommand d ");
    document.commit_command().expect("a commit");
    println!("Undo d ");
    document.undo().expect("an undo");
    if session.layer_refs("d", "0:1:3:1").contains("0:1:1:3") {
        println!("OK: child restored on Undo");
    } else {
        println!("Error: child not restored on Undo");
    }
}

/// `gdt/presentation/C3`.
pub fn gdt_c3(session: &mut XsSession) {
    crate::presentations::run(
        session,
        "C3",
        &crate::presentations::Presentation {
            file: "bug33661_nist_ftc_08_asme1_ap242-e1-tg.stp",
            first: [
                -62.99891835601813,
                78.203476181663817,
                20.269103705327481,
                47784.542040997127,
            ],
            second: [
                -261.12759307366923,
                196.9986700086933,
                -13.86055926862322,
                17568.06377585962,
            ],
        },
    );
}

macro_rules! one_off_case {
    ($name:ident, $source:literal, $body:expr) => {
        one_off_case!($name, $source, $body; Ok);
    };
    ($name:ident, $source:literal, $body:expr; $recorded:ident) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_one_off(&format!("{module}::{}", stringify!($name)), $source, crate::table::RecordedVerdict::$recorded, $body);
        }
    };
}

one_off_case!(bugs_step_bug222, "tests/bugs/step/bug222", bug222);
one_off_case!(bugs_step_bug23379, "tests/bugs/step/bug23379", bug23379);
one_off_case!(bugs_step_bug2368, "tests/bugs/step/bug2368", bug2368);
one_off_case!(bugs_step_bug28345, "tests/bugs/step/bug28345", bug28345);
one_off_case!(bugs_step_bug33261, "tests/bugs/step/bug33261", bug33261);
one_off_case!(bugs_step_bug3883_1, "tests/bugs/step/bug3883_1", |s| {
    bug3883(s, "bug3883_xr2-ct-214_bolt.stp")
});
one_off_case!(bugs_step_bug3883_2, "tests/bugs/step/bug3883_2", |s| {
    bug3883(s, "bug3883_xr2-ct-214_l-bracket.stp")
});
one_off_case!(bugs_step_bug3883_3, "tests/bugs/step/bug3883_3", |s| {
    bug3883(s, "bug3883_xr2-ct-214_nut.stp")
});
one_off_case!(bugs_step_bug3883_4, "tests/bugs/step/bug3883_4", |s| {
    bug3883(s, "bug3883_xr2-ct-214_plate.stp")
});
one_off_case!(bugs_step_bug3883_5, "tests/bugs/step/bug3883_5", |s| {
    bug3883(s, "bug3883_xr2-ct-214_rod.stp")
});
one_off_case!(bugs_xde_bug25176, "tests/bugs/xde/bug25176", bug25176);
one_off_case!(bugs_xde_bug23010, "tests/bugs/xde/bug23010", bug23010);
one_off_case!(bugs_step_bug46, "tests/bugs/step/bug46", bug46);
one_off_case!(bugs_step_bug630, "tests/bugs/step/bug630", bug630);
one_off_case!(
    bugs_moddata_1_bug22459,
    "tests/bugs/moddata_1/bug22459",
    bug22459
);
one_off_case!(bugs_xde_bug23009, "tests/bugs/xde/bug23009", bug23009);
one_off_case!(bugs_xde_bug23597, "tests/bugs/xde/bug23597", bug23597);
one_off_case!(bugs_mesh_bug27453, "tests/bugs/mesh/bug27453", bug27453);
one_off_case!(bugs_mesh_bug29715, "tests/bugs/mesh/bug29715", bug29715);
one_off_case!(bugs_step_bug24595, "tests/bugs/step/bug24595", bug24595);
one_off_case!(
    bugs_step_bug28454_2,
    "tests/bugs/step/bug28454_2",
    bug28454_2
);
one_off_case!(bugs_step_bug31670, "tests/bugs/step/bug31670", bug31670);
one_off_case!(bugs_step_bug33487, "tests/bugs/step/bug33487", bug33487);
one_off_case!(bugs_step_bug33569, "tests/bugs/step/bug33569", bug33569);
one_off_case!(bugs_step_bug33638, "tests/bugs/step/bug33638", bug33638);
one_off_case!(bugs_xde_bug2845, "tests/bugs/xde/bug2845", bug2845);
one_off_case!(bugs_xde_bug33100, "tests/bugs/xde/bug33100", bug33100);
one_off_case!(
    de_wrapper_step_a5,
    "tests/de_wrapper/step/A5",
    de_wrapper_a5
);
one_off_case!(
    de_wrapper_step_a6,
    "tests/de_wrapper/step/A6",
    de_wrapper_a6
);
one_off_case!(bugs_heal_bug33398, "tests/bugs/heal/bug33398", bug33398);
one_off_case!(
    bugs_modalg_5_bug24746,
    "tests/bugs/modalg_5/bug24746",
    bug24746
);
one_off_case!(
    bugs_modalg_6_bug26308,
    "tests/bugs/modalg_6/bug26308",
    bug26308
);
one_off_case!(
    bugs_modalg_7_bug22821,
    "tests/bugs/modalg_7/bug22821",
    bug22821
);
one_off_case!(bugs_modalg_7_bug28366, "tests/bugs/modalg_7/bug28366", bug28366; Known);
one_off_case!(bugs_step_bug30378, "tests/bugs/step/bug30378", bug30378);
one_off_case!(bugs_step_bug31000, "tests/bugs/step/bug31000", bug31000);
one_off_case!(bugs_step_bug32556, "tests/bugs/step/bug32556", bug32556);
one_off_case!(bugs_step_bug32922, "tests/bugs/step/bug32922", bug32922);
one_off_case!(bugs_xde_bug28104_1, "tests/bugs/xde/bug28104_1", bug28104_1);
one_off_case!(bugs_xde_bug28748, "tests/bugs/xde/bug28748", bug28748);
one_off_case!(gdt_presentation_c3, "tests/gdt/presentation/C3", gdt_c3; Known);
