//! The scripts that hold a step out of scope: each runs its real checks up to that step, and says which.
use crate::support::{
    properties::checkgravitycenter_of_area, session::Session, tolerance_check::checkreal,
};
use crate::table::Place;
use crate::xs_session::XsSession;
use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
use martensite_tkernel::{
    standard::Handle,
    storage::format_general,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdf::{TdfLabel, TdfTool},
};
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

/// `persist/fsd/*`: the file is read and each `check_fsd` checks the shape; `fsdwrite` is storage.
fn fsd(session: &mut XsSession, file: &'static str) {
    let path = session.path_of(Place::Data(file));
    println!("stepread {path} s * ");
    session.stepread(&path, "s");
    println!("copy s_1 base ");
    session.copy_variable("s_1", "base");
    for _driver in ["bin", "cmp", "gen"] {
        println!("checkshape base ");
        session.check_shape("base");
    }
}

fn a_box(session: &mut XsSession, name: &str, sizes: [f64; 3]) {
    println!("box {name} {} {} {} ", sizes[0], sizes[1], sizes[2]);
    let shape = BRepPrimAPIMakeBox::from_sizes(sizes[0], sizes[1], sizes[2])
        .expect("a box")
        .shape()
        .expect("a box")
        .clone();
    session.raster.bind(name, &shape);
}

/// `bugs/step/bug51`: the STEP half; the last check is on a compound with the IGES result.
fn bug51(session: &mut XsSession) {
    println!(" write STEP and IGES file ");
    a_box(session, "b", [12.0, 34.0, 56.0]);
    let file = session.path_of(Place::Image("box.step"));
    let _ = std::fs::remove_file(&file);
    println!("stepwrite a b {file} ");
    session.stepwrite("a", "b", &file);
    println!(" Read STEP  file ");
    println!("stepread {file} a * ");
    session.stepread(&file, "a");
}

/// `bugs/xde/bug23561`: a file name past 200 characters; `brepiges` is out of scope.
fn bug23561(session: &mut XsSession) {
    println!("bug23561");
    a_box(session, "b", [10.0, 10.0, 10.0]);
    let folder = session.path_of(Place::Image(""));
    let last = std::path::Path::new(&folder)
        .file_name()
        .expect("a folder")
        .to_string_lossy()
        .into_owned();
    let mut path = folder.trim_end_matches(['/', '\\']).to_owned();
    while path.len() < 200 {
        path = format!("{path}/../{last}");
    }
    let file = format!("{path}/bug23561.stp");
    println!("stepwrite a b {file} ");
    session.stepwrite("a", "b", &file);
}

/// `bugs/demo/bug14673_2`: the STEP section with a Unicode file name; OCAF storage and IGES are out of scope.
fn bug14673_2(session: &mut XsSession) {
    println!("# STEP and IGES");
    a_box(session, "b", [10.0, 10.0, 10.0]);
    println!("NewDocument XDE BinOcaf ");
    session.new_document("XDE");
    println!("XAddShape XDE b ");
    let main = session.documents["XDE"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let shape = session.bound_shape("b");
    tool.get()
        .expect("a shape tool")
        .add_shape(&shape, true, true)
        .expect("a shape label");
    let file = session.path_of(Place::Image("bug14673_2_\u{65e5}\u{672c}\u{8a9e}.stp"));
    println!("WriteStep XDE {file} ");
    if session.write_step("XDE", &file).is_err() {
        crate::support::script_error::script_error("WriteStep XDE");
    }
    if std::path::Path::new(&file).exists() {
        println!("File {file} is created, OK");
    } else {
        println!("Error: Could not find file {file}");
    }
    println!("ReadStep Step {file} ");
    if session.read_step("Step", &file).is_err() {
        crate::support::script_error::script_error("ReadStep Step");
    }
    let _: Handle<martensite_tklcaf::tdoc_std::TDocStdDocument> = session.documents["Step"].clone();
}

fn name_label(session: &XsSession, entry: &str, name: &str) {
    println!("SetName Doc {entry} {name} ");
    let label = crate::script_values::label_of(session, "Doc", entry);
    TDataStdName::set(&label, &ExtendedString::from_utf8(name))
        .unwrap_or_else(|failure| panic!("SetName {entry}: {failure:?}"));
}

fn add_shape(session: &XsSession, name: &str) {
    println!("XAddShape Doc {name} ");
    let main = session.documents["Doc"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let label = tool
        .get()
        .expect("a shape tool")
        .add_shape(&session.bound_shape(name), true, true)
        .expect("a shape label");
    let mut entry = AsciiString::default();
    TdfTool::entry(&label, &mut entry);
    println!("{}", String::from_utf8_lossy(entry.as_bytes()));
}

fn set_location(session: &XsSession, words: &str, entry: &str, transform: Transform3) {
    println!("XSetLocation Doc {entry} {words} ");
    let main = session.documents["Doc"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let label = crate::script_values::label_of(session, "Doc", entry);
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
        let mut text = AsciiString::default();
        TdfTool::entry(&reference, &mut text);
        println!(
            "Reference to the shape at label {} was created and location was set",
            String::from_utf8_lossy(text.as_bytes())
        );
    }
}

fn translated(session: &mut XsSession, name: &str, by: [f64; 3]) {
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

/// `bugs/xde/bug27142`: the STEP branch of the location check. IGES, `.xbf` and the images are not run.
fn bug27142(session: &mut XsSession) {
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
    builder
        .add(&mut compound, &session.bound_shape("a"))
        .expect("a compound");
    builder
        .add(&mut compound, &session.bound_shape("b"))
        .expect("a compound");
    session.raster.bind("ab", &compound.into_shape());
    add_shape(session, "ab");
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
    add_shape(session, "c");
    name_label(session, "0:1:1:4", "ref_c");
    name_label(session, "0:1:1:5", "c");
    add_shape(session, "d");
    name_label(session, "0:1:1:6", "d");
    a_box(session, "e", [1.0, 1.0, 1.0]);
    add_shape(session, "e");
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
    let file = session.path_of(Place::Image("step.step"));
    println!("WriteStep Doc {file} ");
    if session.write_step("Doc", &file).is_err() {
        crate::support::script_error::script_error("WriteStep Doc");
    }
    println!("Close Doc ");
    session.close_document("Doc");
    println!("ReadStep Doc1 {file} ");
    if session.read_step("Doc1", &file).is_err() {
        crate::support::script_error::script_error("ReadStep Doc1");
    }
    println!("XGetShape d1 Doc1 0:1:1:8 ");
    session.get_shape_at("d1", "Doc1", "0:1:1:8");
    println!("Close Doc1 ");
    session.close_document("Doc1");
    let _ = std::fs::remove_file(&file);
    println!("Check area of the box from STEP file");
    let shape = session.bound_shape("d1");
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(&shape, &mut properties, 0.0015, false).expect("properties");
    checkreal(
        "area of d1",
        &format_general(properties.mass(), 6),
        "6",
        1e-7,
        0.001,
    );
    println!("Check gravity center of the box from STEP file");
    checkgravitycenter_of_area("d1", &shape, ["2.5", "2.5", "0.5"], "1e-7");
}

macro_rules! partly {
    ($name:ident, $source:literal, $body:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_partly(&format!("{module}::{}", stringify!($name)), $source, $body);
        }
    };
}

partly!(persist_fsd_a8, "tests/persist/fsd/A8", |s| fsd(
    s,
    "pr1db.stp"
));
partly!(persist_fsd_s1, "tests/persist/fsd/S1", |s| fsd(
    s,
    "bm2_ec_exhaust-A.stp"
));
partly!(persist_fsd_s2, "tests/persist/fsd/S2", |s| fsd(
    s,
    "bm2_pe_t4-B.stp"
));
partly!(persist_fsd_s3, "tests/persist/fsd/S3", |s| fsd(
    s,
    "bug21802_as1-oc-214.stp"
));
partly!(persist_fsd_s4, "tests/persist/fsd/S4", |s| fsd(
    s,
    "bug24595_281390.stp"
));
partly!(persist_fsd_s5, "tests/persist/fsd/S5", |s| fsd(
    s,
    "ec_soapbox-A.stp"
));
partly!(persist_fsd_s7, "tests/persist/fsd/S7", |s| fsd(
    s,
    "trj4_k1_geo-md-203.stp"
));
partly!(bugs_step_bug51, "tests/bugs/step/bug51", bug51);
partly!(bugs_xde_bug23561, "tests/bugs/xde/bug23561", bug23561);
partly!(
    bugs_demo_bug14673_2,
    "tests/bugs/demo/bug14673_2",
    bug14673_2
);
partly!(bugs_xde_bug27142, "tests/bugs/xde/bug27142", bug27142);
