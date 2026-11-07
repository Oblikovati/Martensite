//! The scripts of the toolkit that end on a shaded view, one function each, in the case's order of commands.
#[path = "../dataset_cases/cases/headings.rs"]
pub mod headings;

use crate::grading::{BUGS, V3D, VSELECT, run_case};
use crate::host::ViewerHost;
use crate::steps::{catching, run_command};
use crate::support::rules::Verdict;
use crate::support::session::Session as _;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};

macro_rules! viewer_case {
    ($name:ident, $source:literal, $group:expr, $script:path) => {
        #[test]
        fn $name() {
            run_case(
                concat!("cases::", stringify!($name)),
                $source,
                &$group,
                Verdict::Ok,
                $script,
            );
        }
    };
}

/// A command of the shared tables on the host's session.
fn run(host: &mut ViewerHost, name: &str, words: &[&str]) {
    run_command(&mut host.session, name, words);
}

fn bug25518(host: &mut ViewerHost) {
    run(host, "psphere", &["s", "10"]);
    host.session.new_document("D");
    host.session.add_shape("D", "s", true, "");
    host.session.set_color("D", "s", &["0", "0", "1"]);
    run(host, "param", &["write.iges.brep.mode", "1"]);
    let word = "@image/bug25518.igs";
    run(host, "WriteIges", &["D", word]);
    run(host, "ReadIges", &["T", word]);
    run(host, "file_delete", &[word]);
    if host.session.all_colors("T").trim() != "BLUE" {
        println!("ERROR: the defect is reproduced. Color is lost.");
    }
    host.xshow("T");
    host.vfit();
    host.vsetdispmode_shaded();
    host.checkview_screenshot("bug25518");
}

fn bug26174(host: &mut ViewerHost) {
    catching(&mut host.session, "Close", &["D"]);
    catching(&mut host.session, "Close", &["D1"]);
    run(host, "box", &["b", "0", "0", "0", "10", "10", "10"]);
    run(host, "compound", &["b", "c"]);
    host.session.new_document("D");
    host.session.add_shape("D", "c", true, " 1");
    host.session.set_color("D", "0:1:1:1:1", &["1", "0", "0"]);
    run(host, "WriteIges", &["D", "@image/bug26174_orig.igs"]);
    host.session.get_one_shape("res", "D");
    println!("dump res ");
    let dump = crate::shape_commands::dump_shape(&host.session, "res");
    if !dump.contains("Dump of 24 Curve2ds") {
        println!("Error: incorrect Curve transfer");
    }
    run(host, "WriteIges", &["D", "@image/bug26174_comp.igs"]);
    run(host, "ReadIges", &["D1", "@image/bug26174_comp.igs"]);
    host.session.get_one_shape("res1", "D1");
    host.vinit_named("view1");
    host.vclear();
    host.xdisplay_label("D1", "0:1:1:1");
    host.vfit();
    if host.vreadpixel_name_only(204, 204).trim() != "RED 1" {
        println!("Error: expected color of shape from Document1 is RED.");
    }
    run(host, "checkprops", &["res", "-equal", "res1"]);
    if host.session.all_colors("D") != host.session.all_colors("D1") {
        println!("Error: incorrect color transfer");
    }
    run(
        host,
        "checknbshapes",
        &[
            "res", "-vertex", "8", "-edge", "12", "-wire", "6", "-face", "6",
        ],
    );
    host.checkview_screenshot("bug26174");
}

fn bug33327(host: &mut ViewerHost) {
    host.session.close_silently("D");
    run(host, "ReadIges", &["D", "@data/bug33327.igs"]);
    host.vclear();
    host.vinit_named("View1");
    host.xdisplay_shaded("D", false);
    host.vfit();
    host.vdump("bug33327_src");
    run(host, "Close", &["D"]);
}

fn read_and_shade(host: &mut ViewerHost, unit: &str) {
    run(host, "param", &["xstep.cascade.unit", unit]);
    run(host, "newmodel", &[]);
    run(host, "igesbrep", &["@data/bug25837_S.igs", "result", "*"]);
    host.vinit();
    host.vdisplay("result");
    host.vfit();
    host.vsetdispmode_shaded();
    host.checkcolor(200, 200, [0.0, 0.0, 0.0]);
}

fn bug25837_1(host: &mut ViewerHost) {
    read_and_shade(host, "MM");
    host.checkview_screenshot("bug25837_1");
}

fn bug25837_2(host: &mut ViewerHost) {
    read_and_shade(host, "M");
    let (of_triangles, of_faces) = host.triarea("result", 1.0e-4, "1.0e-4");
    let relative = of_triangles / of_faces.abs();
    println!("checkreal Relative area {relative} 1.0 0.01 0.0 ");
    crate::support::tolerance_check::checkreal(
        "Relative area",
        &crate::view_commands::script_real(relative),
        "1.0",
        0.01,
        0.0,
    );
    host.checkview_screenshot("bug25837_2");
}

fn bug24012(host: &mut ViewerHost) {
    run(host, "igesread", &["@data/bug24012_face.igs", "face", "*"]);
    run(host, "igesread", &["@data/bug24012_line.igs", "edge", "*"]);
    host.nproject("r", "edge", "face");
    run(host, "checknbshapes", &["r", "-edge", "1"]);
    run(host, "checkprops", &["r", "-l", "5.96"]);
    host.vinit();
    host.vsetdispmode_all(0);
    host.vdisplay("face");
    host.vdisplay("edge");
    host.vdisplay("r");
    host.vsetcolor("r", "yellow", Color::named(NamedColor::YELLOW));
    host.vfit();
    host.vviewparams(
        "-scale 71.101493567712652 -proj -0.14605970947882216 -0.18639384905183365 0.97155745805516014 -up -0.587582742029223 0.80643668322534767 0.066380699137021923 -at 6.30475074082204 6.748073489527 8.5106037329062",
        71.10149356771265,
        [-0.14605970947882216, -0.18639384905183365, 0.9715574580551601],
        [-0.587582742029223, 0.8064366832253477, 0.06638069913702192],
        [6.30475074082204, 6.748073489527, 8.5106037329062],
    );
    host.checkview_screenshot("bug24012");
}

fn buc60661(host: &mut ViewerHost) {
    run(host, "restore", &["@data/buc60661a.brep", "buc60661"]);
    run(host, "checkshape", &["buc60661"]);
    let word = "@image/buc60661.igs";
    run(host, "file_delete", &[word]);
    run(host, "brepiges", &["buc60661", word]);
    host.dall(&["buc60661"]);
    if catching(&mut host.session, "igesbrep", &[word, "a", "*"]) {
        println!("Reading of iges in BUC60661 is OK");
    } else {
        println!("Faulty BUC60661: here is reading problem of iges file");
    }
    host.session.tpcompound("result");
    run(host, "checkshape", &["result"]);
    host.vinit_named("View1");
    run(host, "tclean", &["result"]);
    host.vdisplay("result");
    host.vsetdispmode_of("result", 1);
    crate::meshing::checktrinfo(&host.session.raster, &["result", "-tri", "-nod"]);
    host.vdump("buc60661");
}

fn g2(host: &mut ViewerHost) {
    run(host, "testreadiges", &["@data/bug29356.igs", "a"]);
    host.vinit_again("View1");
    host.vdisplay_in_mode("a", 0);
    host.visos("a", 12, 6);
    host.vfit();
    host.vdump("G2");
}

/// `G3` and `G4`: the isolines of a prism on a spline whose poles are at 1.4e+6.
fn prism_on_a_far_spline(
    host: &mut ViewerHost,
    word: &str,
    isos: (i32, i32),
    zoomed: bool,
    image: &str,
) {
    run(host, "testreadiges", &["@data/bug29356.igs", "a"]);
    run(host, "mksurface", &["s", "a"]);
    println!("viso c s 0 ");
    let surface = host.session.surfaces["s"].clone();
    let curve: Handle<dyn GeomCurve> = surface.get().unwrap().v_iso(0.0).unwrap();
    host.session.curves.insert("c".to_owned(), curve.clone());
    let spline: Handle<GeomBSplineCurve> = curve.cast();
    host.dump_bspline_curve("c", &spline);
    run(host, "mkedge", &["e", "c"]);
    println!("view 1 AXON 465 20 800 800*20.4/29.1 ");
    host.session
        .raster
        .view(crate::support::session::ViewStep::SmallView);
    host.session
        .raster
        .view(crate::support::session::ViewStep::Donly(&["e"]));
    host.prism("r", "e", [0.0, 1.0, 0.0], word);
    println!("fit ");
    host.session
        .raster
        .view(crate::support::session::ViewStep::Fit);
    host.xbounds("r");
    host.vinit_second();
    host.vdisplay("r");
    host.visos("r", isos.0, isos.1);
    host.vfit();
    if zoomed {
        host.vzoom(3000.0);
        host.vdump(&format!("{image}_zoom"));
    }
    host.vdump(image);
}

fn g3(host: &mut ViewerHost) {
    prism_on_a_far_spline(host, "Inf", (2, 3), true, "G3");
}

fn g4(host: &mut ViewerHost) {
    prism_on_a_far_spline(host, "Fin", (2, 2), false, "G4");
}

viewer_case!(
    bugs_iges_bug25518,
    "tests/bugs/iges/bug25518",
    BUGS,
    bug25518
);
viewer_case!(
    bugs_iges_bug26174,
    "tests/bugs/iges/bug26174",
    BUGS,
    bug26174
);
viewer_case!(
    bugs_iges_bug33327,
    "tests/bugs/iges/bug33327",
    BUGS,
    bug33327
);
viewer_case!(
    bugs_mesh_bug25837_1,
    "tests/bugs/mesh/bug25837_1",
    BUGS,
    bug25837_1
);
viewer_case!(
    bugs_mesh_bug25837_2,
    "tests/bugs/mesh/bug25837_2",
    BUGS,
    bug25837_2
);
viewer_case!(
    bugs_modalg_5_bug24012,
    "tests/bugs/modalg_5/bug24012",
    BUGS,
    bug24012
);
viewer_case!(v3d_bugs_buc60661, "tests/v3d/bugs/buc60661", V3D, buc60661);
viewer_case!(vselect_face_g2, "tests/vselect/face/G2", VSELECT, g2);
viewer_case!(vselect_face_g3, "tests/vselect/face/G3", VSELECT, g3);
viewer_case!(vselect_face_g4, "tests/vselect/face/G4", VSELECT, g4);
