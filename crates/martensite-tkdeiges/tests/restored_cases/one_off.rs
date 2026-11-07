//! The `bugs` scripts with commands of their own. `bugs/xde/bug27142` and `bugs/demo/bug14673_2` hold
//! a step of a stored format: here each runs its IGES step, which is what this crate owes.
use crate::documents::label_of;
use crate::iges_commands::data_path;
use crate::iges_session::{IgesSession, image_folder, keep_written_file};
use crate::support::{
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    properties::checkgravitycenter_of_area,
    script_error::script_error,
    session::Session,
    tolerance_check::checkreal,
};
use crate::table::RecordedVerdict;
use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
use martensite_tkernel::{storage::format_general, tcollection::ExtendedString};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{tdata_std::TDataStdName, tdf::TdfLabel};
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

/// One `#[test]` per script, named as libtest names it, which its child process runs.
macro_rules! one_off_case {
    ($name:ident, $source:literal, $recorded:expr, $script:expr) => {
        #[test]
        fn $name() {
            crate::runner::run_one_off(
                concat!("one_off::", stringify!($name)),
                $source,
                $recorded,
                $script,
            );
        }
    };
}

fn a_box(session: &mut IgesSession, name: &str, sizes: [f64; 3]) {
    println!("box {name} {} {} {} ", sizes[0], sizes[1], sizes[2]);
    let shape = BRepPrimAPIMakeBox::from_sizes(sizes[0], sizes[1], sizes[2])
        .expect("a box")
        .shape()
        .expect("a box")
        .clone();
    session.raster.bind(name, &shape);
}

fn image(session: &IgesSession, name: &str) -> String {
    image_folder(&session.source)
        .join(name)
        .to_string_lossy()
        .into_owned()
}

/// `bugs/xde/bug21802`: the names of a STEP assembly reach the IGES file.
fn bug21802(session: &mut IgesSession) {
    println!("==========");
    println!("Not all names are transferred from STEP to IGES via XDE");
    println!("==========");
    println!();
    let input = data_path("bug21802_as1-oc-214.stp");
    println!("ReadStep d {input} ");
    if session.read_step("d", &input).is_err() {
        script_error("ReadStep d");
    }
    let file = image(session, "bug21802.igs");
    println!("WriteIges d {file} ");
    if session.write_iges("d", &file).is_err() {
        script_error("WriteIges d");
    }
    keep_written_file(&file);
    let text =
        String::from_utf8_lossy(&std::fs::read(&file).expect("the written file")).into_owned();
    let _ = std::fs::remove_file(&file);
    if !text.lines().any(|line| line.trim().contains("l-bracke")) {
        println!("ERROR: the defect is reproduced");
    }
}

/// `bugs/step/bug51`: a box to STEP and IGES and back. `igesbrep` binds `b`, so `compound` of `a_1 b_1`
/// finds no `b_1` and holds the STEP shape alone.
fn bug51(session: &mut IgesSession) {
    println!("================");
    println!("Regression case");
    println!("================");
    println!();
    println!(" write STEP and IGES file ");
    a_box(session, "b", [12.0, 34.0, 56.0]);
    let step = image(session, "box.step");
    let iges = image(session, "box.igs");
    let _ = std::fs::remove_file(&step);
    let _ = std::fs::remove_file(&iges);
    session.stepwrite("b", &step);
    session.brepiges("b", &iges);
    keep_written_file(&iges);
    session.raster.unbind("b");
    println!(" Read STEP  file ");
    session.stepread(&step, "a");
    println!(" Read IGES file ");
    session.igesbrep(&iges, "b");
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder
        .add(&mut compound, &session.bound_shape("a_1"))
        .expect("a compound");
    session.raster.bind("result", &compound.into_shape());
    println!("checkshape result ");
    checkshape_named(&session.raster, "result");
    checkview(&mut session.raster, &CheckView::of("result"));
}

/// `bugs/xde/bug23561`: both writers under a path of at least 200 characters.
fn bug23561(session: &mut IgesSession) {
    println!("bug23561");
    a_box(session, "b", [10.0, 10.0, 10.0]);
    let folder = image(session, "");
    let last = std::path::Path::new(&folder)
        .file_name()
        .expect("a folder")
        .to_string_lossy()
        .into_owned();
    let mut path = folder.trim_end_matches(['/', '\\']).to_owned();
    while path.len() < 200 {
        path = format!("{path}/../{last}");
    }
    session.brepiges("b", &format!("{path}/bug23561.igs"));
    session.stepwrite("b", &format!("{path}/bug23561.stp"));
}

/// `bugs/demo/bug14673_2`: the IGES section with a Unicode file name. The OCAF documents are a stored
/// format and the STEP section is the STEP crate's test.
fn bug14673_2(session: &mut IgesSession) {
    println!("# STEP and IGES");
    a_box(session, "b", [10.0, 10.0, 10.0]);
    println!("NewDocument XDE BinOcaf ");
    session.new_document("XDE");
    session.add_shape("XDE", "b");
    let file = image(session, "bug14673_2_\u{65e5}\u{672c}\u{8a9e}.igs");
    println!("WriteIges XDE {file} ");
    if session.write_iges("XDE", &file).is_err() {
        script_error("WriteIges XDE");
    }
    if std::path::Path::new(&file).exists() {
        println!("File {file} is created, OK");
    } else {
        println!("Error: Could not find file {file}");
    }
    println!("ReadIges Iges {file} ");
    if session.read_iges("Iges", &file).is_err() {
        script_error("ReadIges Iges");
    }
}

fn name_label(session: &IgesSession, entry: &str, name: &str) {
    println!("SetName Doc {entry} {name} ");
    let label = label_of(session, "Doc", entry);
    TDataStdName::set(&label, &ExtendedString::from_utf8(name))
        .unwrap_or_else(|failure| panic!("SetName {entry}: {failure:?}"));
}

fn set_location(session: &IgesSession, words: &str, entry: &str, transform: Transform3) {
    println!("XSetLocation Doc {entry} {words} ");
    let main = session.documents["Doc"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let label = label_of(session, "Doc", entry);
    let mut reference = TdfLabel::default();
    let set = tool
        .get()
        .expect("a shape tool")
        .set_location(&label, &Location::new(transform), &mut reference)
        .expect("a location");
    if !set {
        println!(
            "Error: an attempt to set the location to a shape to which there is a reference, or to not a shape at all"
        );
    } else if reference == label {
        println!("New location was set");
    } else {
        println!(
            "Reference to the shape at label {} was created and location was set",
            crate::documents::entry_of(&reference)
        );
    }
}

fn translated(session: &mut IgesSession, name: &str, by: [f64; 3]) {
    println!("ttranslate {name} {} {} {} ", by[0], by[1], by[2]);
    let moved = session
        .bound_shape(name)
        .moved(
            &Location::new(Transform3::translation_vector(Vector3::new(
                by[0], by[1], by[2],
            ))),
            true,
        )
        .expect("a moved shape");
    session.raster.bind(name, &moved);
}

/// `bugs/xde/bug27142`: the IGES branch of the location check. `.xbf` and the images are not run, and
/// the STEP branch is the STEP crate's test. The script's two `TODO`s are for this branch.
fn bug27142(session: &mut IgesSession) {
    println!("==========");
    println!("Data Exchange - add possibility to set location in the shape tool interface");
    println!("==========");
    println!();
    println!("TODO known ALL: Error: area of d2");
    println!("TODO known ALL: Error: center of gravity");
    session.close_silently("Doc");
    println!("XNewDoc Doc ");
    session.new_document("Doc");
    a_box(session, "a", [1.0, 1.0, 1.0]);
    a_box(session, "b", [1.0, 1.0, 1.0]);
    translated(session, "b", [1.0, 1.0, 0.0]);
    println!("compound a b ab ");
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for name in ["a", "b"] {
        builder
            .add(&mut compound, &session.bound_shape(name))
            .expect("a compound");
    }
    session.raster.bind("ab", &compound.into_shape());
    session.add_shape("Doc", "ab");
    for (entry, name) in [
        ("0:1:1:1", "ab"),
        ("0:1:1:1:1", "ref_a"),
        ("0:1:1:1:2", "ref_b"),
        ("0:1:1:2", "a"),
        ("0:1:1:3", "b"),
    ] {
        name_label(session, entry, name);
    }
    a_box(session, "c", [1.0, 1.0, 1.0]);
    translated(session, "c", [1.0, 0.0, 0.0]);
    a_box(session, "d", [0.5, 0.5, 0.5]);
    session.add_shape("Doc", "c");
    name_label(session, "0:1:1:4", "ref_c");
    name_label(session, "0:1:1:5", "c");
    session.add_shape("Doc", "d");
    name_label(session, "0:1:1:6", "d");
    a_box(session, "e", [1.0, 1.0, 1.0]);
    session.add_shape("Doc", "e");
    name_label(session, "0:1:1:7", "e");
    let move_by = |x: f64, y: f64, z: f64| Transform3::translation_vector(Vector3::new(x, y, z));
    let axis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).expect("a direction"),
    );
    let rotation = Transform3::rotation(axis, 90.0_f64.to_radians()).expect("a rotation");
    // `pre_multiply`: each option applies after the ones before it.
    set_location(
        session,
        "-rotate 0 0 0 0 0 1 90 -move 4 2 0",
        "0:1:1:1",
        move_by(4.0, 2.0, 0.0).multiplied(rotation),
    );
    name_label(session, "0:1:1:8", "ref_ab");
    set_location(
        session,
        "-move 1 0 0 -move 1 3 0",
        "0:1:1:4",
        move_by(1.0, 3.0, 0.0).multiplied(move_by(1.0, 0.0, 0.0)),
    );
    let scale = Transform3::scaling(Point3::new(2.0, 2.0, 0.0), 2.0).expect("a scale");
    set_location(
        session,
        "-move 2 2 0 -scale 2 2 0 2",
        "0:1:1:6",
        scale.multiplied(move_by(2.0, 2.0, 0.0)),
    );
    name_label(session, "0:1:1:9", "ref_d");
    let file = image(session, "iges.iges");
    println!("WriteIges Doc {file} ");
    if session.write_iges("Doc", &file).is_err() {
        script_error("WriteIges Doc");
    }
    println!("Close Doc ");
    session.close_document("Doc");
    println!("ReadIges Doc2 {file} ");
    if session.read_iges("Doc2", &file).is_err() {
        script_error("ReadIges Doc2");
    }
    println!("XGetShape d2 Doc2 0:1:1:4 ");
    session.get_shape_at("d2", "Doc2", "0:1:1:4");
    println!("Close Doc2 ");
    session.close_document("Doc2");
    let _ = std::fs::remove_file(&file);
    println!("Check area of the box from IGES file");
    let shape = session.bound_shape("d2");
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(&shape, &mut properties, 0.0015, false).expect("properties");
    checkreal(
        "area of d2",
        &format_general(properties.mass(), 6),
        "6",
        1e-7,
        0.001,
    );
    println!("Check gravity center of the box from IGES file");
    checkgravitycenter_of_area("d2", &shape, ["2.5", "2.5", "0.5"], "1e-7");
}

one_off_case!(
    bugs_xde_bug21802,
    "tests/bugs/xde/bug21802",
    RecordedVerdict::Ok,
    bug21802
);
one_off_case!(
    bugs_step_bug51,
    "tests/bugs/step/bug51",
    RecordedVerdict::Ok,
    bug51
);
one_off_case!(
    bugs_xde_bug23561,
    "tests/bugs/xde/bug23561",
    RecordedVerdict::Ok,
    bug23561
);
one_off_case!(
    bugs_demo_bug14673_2,
    "tests/bugs/demo/bug14673_2",
    RecordedVerdict::Ok,
    bug14673_2
);
one_off_case!(
    bugs_xde_bug27142,
    "tests/bugs/xde/bug27142",
    RecordedVerdict::Known,
    bug27142
);
