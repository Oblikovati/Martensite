//! `tests/bugs/xde`: the scripts that write their file or show their document as read.
use crate::commands::mentions_unknown;
use crate::grading::BUGS;
use crate::host::ViewerHost;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;

/// The bodies of `bug22535_1` and `bug22535_2`: a shape through a written file and back.
fn written_and_read(
    host: &mut ViewerHost,
    input: &str,
    image: &'static str,
    surface: f64,
    counts: &[&str],
) {
    let file = host.session.path_of(Place::Image(step_file_of(image)));
    println!("restore {input} a ");
    crate::support::restore::restore(&mut host.session.raster, input, "a");
    println!("testwritestep {file} a ");
    host.session.testwritestep(&file, "a");
    println!("stepread {file} b * ");
    host.session.stepread(&file, "b");
    println!("renamevar b_1 result ");
    host.session.renamevar("b_1", "result");
    host.vinit();
    host.vsetdispmode_shaded();
    host.vdisplay("result");
    host.vfit();
    host.checkprops_surface("result", surface);
    host.checkshape("result");
    host.checknbshapes("result", counts);
    host.checkview_screenshot(image);
}

fn step_file_of(image: &str) -> &'static str {
    match image {
        "bug22535_1" => "bug22535_1.stp",
        "bug22535_2" => "bug22535_2.stp",
        other => panic!("{other}: expected a case of this file"),
    }
}

fn bug22535_1(host: &mut ViewerHost) {
    for line in ["===========", "22535", "==========="] {
        println!("{line}");
    }
    written_and_read(
        host,
        "OCC22535-sphere_cyl2_cut.brep",
        "bug22535_1",
        14217.2,
        &[
            "-vertex",
            "6",
            "-edge",
            "9",
            "-wire",
            "3",
            "-face",
            "2",
            "-shell",
            "1",
            "-solid",
            "1",
            "-compsolid",
            "0",
            "-compound",
            "0",
            "-shape",
            "22",
        ],
    );
}
viewer_case!(
    bugs_xde_bug22535_1,
    "tests/bugs/xde/bug22535_1",
    BUGS,
    Verdict::Ok,
    bug22535_1
);

fn bug22535_2(host: &mut ViewerHost) {
    for line in ["===========", "22535", "==========="] {
        println!("{line}");
    }
    written_and_read(
        host,
        "OCC22535-cut_comp.brep",
        "bug22535_2",
        50201.3,
        &[
            "-vertex",
            "15",
            "-edge",
            "24",
            "-wire",
            "9",
            "-face",
            "8",
            "-shell",
            "5",
            "-solid",
            "4",
            "-compsolid",
            "0",
            "-compound",
            "1",
            "-shape",
            "66",
        ],
    );
}
viewer_case!(
    bugs_xde_bug22535_2,
    "tests/bugs/xde/bug22535_2",
    BUGS,
    Verdict::Ok,
    bug22535_2
);

fn read_document(host: &mut ViewerHost, doc: &str, file: &'static str) {
    let path = host.session.path_of(Place::Data(file));
    println!("ReadStep {doc} {path} ");
    assert!(
        host.session.read_step(doc, &path).is_ok(),
        "ReadStep {doc} {path}"
    );
}

fn bug25381(host: &mut ViewerHost) {
    println!("Close D -silent ");
    read_document(host, "D", "bug25381_test_assembly_invisible.step");
    host.xshow("D");
    host.vaxo();
    host.vfit();
    host.vsetdispmode_shaded();
    host.vselprops_dynamic_highlight_own_mode();
    host.vmoveto(300, 200);
    if host.vreadpixel_name(250, 200) != "CYAN2" {
        println!("Error: object is NOT picked");
    }
    host.vmoveto(200, 250);
    if host.vreadpixel_name(200, 250) != "BLACK" {
        println!("Error: hidden part is displayed");
    }
    if host.vreadpixel_name(250, 200) != "RED4" {
        println!("Error: object is picked by hidden part");
    }
    host.vdump("bug25381");
}
viewer_case!(
    bugs_xde_bug25381,
    "tests/bugs/xde/bug25381",
    BUGS,
    Verdict::Ok,
    bug25381
);

fn bug6307(host: &mut ViewerHost) {
    println!("NewDocument D ");
    host.session.new_named_document_silently("D");
    println!("document (not handled by application)  D created");
    println!("0:1");
    println!("UndoLimit D 100 ");
    let document = host.session.documents["D"].clone();
    document.get().unwrap().set_undo_limit(100).unwrap();
    println!("100 0 0");
    println!("NewCommand D ");
    document.get().unwrap().new_command().unwrap();
    read_document(host, "D", "trj2_as1-tc-214.stp");
    println!("XGetOneShape result D ");
    assert!(
        host.session.get_one_shape("result", "D").is_ok(),
        "XGetOneShape result D"
    );
    host.xshow("D");
    host.vsetdispmode_shaded();
    host.vfit();
    host.checkprops_surface("result", 141051.0);
    host.checkshape("result");
    host.checknbshapes(
        "result",
        &[
            "-vertex",
            "84",
            "-edge",
            "126",
            "-wire",
            "76",
            "-face",
            "53",
            "-shell",
            "5",
            "-solid",
            "5",
            "-compsolid",
            "0",
            "-compound",
            "4",
            "-shape",
            "353",
        ],
    );
    host.checkview_screenshot("bug6307");
}
viewer_case!(
    bugs_xde_bug6307,
    "tests/bugs/xde/bug6307",
    BUGS,
    Verdict::Ok,
    bug6307
);

/// The wall of bricks the script builds, then its names.
fn build_wall(host: &mut ViewerHost) {
    println!("XNewDoc SA ");
    host.session.new_document("SA");
    host.box_at("b1", [0., 0., 0.], [10., 10., 10.]);
    host.box_at("b2", [0., 0., 0.], [20., 10., 10.]);
    host.ttranslate("b1", [10., 0., 0.]);
    host.ttranslate("b2", [20., 0., 0.]);
    host.compound(&["b1", "b2"], "bl");
    host.copy_shape("bl", "nbl");
    host.ttranslate("nbl", [30., 0., 0.]);
    host.compound(&["bl", "nbl"], "ryad");
    host.copy_shape("ryad", "hryad");
    host.ttranslate("hryad", [-10., 0., 10.]);
    host.copy_shape("ryad", "uryad");
    host.ttranslate("uryad", [0., 0., 20.]);
    host.compound(&["ryad", "hryad", "uryad"], "stena");
    host.xadd_shape_assembly("SA", "stena");
    for (entry, name) in [
        ("0:1:1:1", "Wall"),
        ("0:1:1:2", "Block-Array"),
        ("0:1:1:3", "Block"),
        ("0:1:1:4", "Half-Brick"),
        ("0:1:1:5", "Brick"),
        ("0:1:1:1:1", "base-array"),
        ("0:1:1:1:2", "next-array"),
        ("0:1:1:1:3", "top-array"),
        ("0:1:1:2:1", "left-block"),
        ("0:1:1:2:2", "right-block"),
        ("0:1:1:3:1", "half-brick"),
        ("0:1:1:3:2", "brick"),
    ] {
        host.set_name("SA", entry, name);
    }
}

/// A TODO of the script: also the history of the read shapes is unknown, so the script says
/// `Faulty` and the case is a known problem.
fn bug1747(host: &mut ViewerHost) {
    build_wall(host);
    let file = host.session.path_of(Place::Image("bug1747.stp"));
    println!("WriteStep SA {file} ");
    assert!(host.session.write_step("SA", &file).is_ok(), "WriteStep SA");
    host.dall(&["b1", "b2", "bl", "nbl", "ryad", "hryad", "uryad", "stena"]);
    println!("ReadStep D {file} ");
    assert!(host.session.read_step("D", &file).is_ok(), "ReadStep D");
    println!("INFO: get main shape from the XDE document D");
    host.xget_shape("a", "D", "0:1:1:1");
    println!("INFO: now try to get translation history for it");
    let whole = host.fromshape("a");
    println!("explode a ");
    let nb_parts = host.session.explode("a", "");
    assert!(
        nb_parts >= 2,
        "explode a: {nb_parts} parts; expected at least a_1 and a_2"
    );
    println!("INFO: now try to get translation history for subshape a_1");
    let first = host.fromshape("a_1");
    println!("INFO: now try to get translation history for subshape a_2");
    let second = host.fromshape("a_2");
    if mentions_unknown(&whole) || mentions_unknown(&first) || mentions_unknown(&second) {
        println!("Faulty 1747");
    } else {
        println!("OK 1747");
    }
    host.checkview_display("result", "bug1747");
}
viewer_case!(
    bugs_xde_bug1747,
    "tests/bugs/xde/bug1747",
    BUGS,
    Verdict::KnownProblem,
    bug1747
);
