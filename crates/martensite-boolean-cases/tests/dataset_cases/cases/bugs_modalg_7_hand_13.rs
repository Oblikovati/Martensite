//! `bugs_modalg_7` bodies written by hand, last file: a loop, the timing exception and a removal.
use crate::support::bugs_commands::options_bop::{SectionFlags, bsection_with};
use crate::support::{
    bugs_commands::{
        beyond::child_count,
        inspect::{bopargcheck, checkmaxtol_min},
        options_bop::ScriptBop,
        progress::{restore_tracked, tracked_lines},
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{add, bound_shape, compound, explode, orientation, prism, trotate},
    construction::{copy, wire},
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{circle_along, mkedge, mkface_bounded, plane},
    topology_commands::mkplane,
};
use martensite_tkbo::{bop_algo::Operation, brep_algo_api::BRepAlgoAPIDefeaturing};
use martensite_tkbrep::{brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, osd::Timer, tcollection::AsciiString};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

fn four_counts(s: &dyn Session, name: &str, [wires, faces, shells, solids]: [usize; 4]) {
    let counts = [
        NbCount::Wires(wires),
        NbCount::Faces(faces),
        NbCount::Shells(shells),
        NbCount::Solids(solids),
    ];
    checknbshapes(s, name, Counting::Located, &counts);
}

fn both_masses(s: &dyn Session, name: &str, area: f64, volume: f64) {
    checkprops(s, name, Prop::Area, Expected::Mass(area));
    checkprops(s, name, Prop::Volume, Expected::Mass(volume));
}

fn circle_wire(s: &mut dyn Session, name: &str, at: [f64; 3], normal: [f64; 3], radius: f64) {
    let curve = circle_along(at, normal, radius);
    mkedge(s, name, &curve);
    wire(s, name, &[name]);
}

pub fn bug24905(s: &mut dyn Session) {
    println!("========");
    println!("0024905: Boolean cut produced invalid result");
    println!("========");
    println!();
    circle_wire(s, "c1", [0., 0., 0.], [0., -1., 0.], 75.);
    circle_wire(s, "c2", [0., 0., 0.], [0., -1., 0.], 65.);
    orientation(s, "c2", Orientation::Reversed);
    mkplane(s, "f1", "c1", true);
    add(s, "c2", "f1");
    prism(s, "p1", "f1", [0., 1000., 0.]);
    trotate(s, "p1", [0., 0., 0.], [0., 0., 1.], 180.);
    circle_wire(s, "c3", [50., 500., -2000.], [0., 0., 1.], 50.);
    mkplane(s, "f2", "c3", true);
    prism(s, "p2", "f2", [0., 0., 4000.]);
    trotate(s, "p2", [0., 500., 0.], [1., 0., 0.], -77.);
    trotate(s, "p2", [0., 0., 0.], [0., 0., 1.], 180.);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["p1"]);
    bop.add_tools(s, &["p2"]);
    bop.bfillds();
    let kinds = [
        Operation::Common,
        Operation::Fuse,
        Operation::Cut,
        Operation::Cut21,
        Operation::Section,
    ];
    for (index, kind) in kinds.into_iter().enumerate() {
        bop.bbop(s, &format!("r_{index}"), kind);
    }
    for index in 0..=4 {
        let name = format!("r_{index}");
        checkshape_named(s, &name);
        if bopargcheck(s, (&name, None), None, "").contains("Faulties") {
            println!("Error: bopargcheck has found some faulties in {name}");
        }
    }
    four_counts(s, "r_0", [6, 5, 1, 1]);
    both_masses(s, "r_0", 196255., 857818.);
    four_counts(s, "r_1", [13, 9, 1, 1]);
    both_masses(s, "r_1", 1.96453e+06, 3.49563e+07);
    four_counts(s, "r_2", [11, 7, 1, 1]);
    both_masses(s, "r_2", 742474., 3.54041e+06);
    four_counts(s, "r_3", [8, 7, 2, 2]);
    both_masses(s, "r_3", 1.41831e+06, 3.05581e+07);
    checknbshapes(
        s,
        "r_4",
        Counting::Located,
        &[NbCount::Vertices(8), NbCount::Edges(8)],
    );
    checkprops(s, "r_4", Prop::Length, Expected::Mass(3965.07));
    checksection_expecting(s, "r_4", 0);
    copy(s, "r_2", "result");
    checkmaxtol_min(s, "result", 1.0e-4, "1.0e-4");
    checkview(s, &CheckView::of("result"));
}

/// One `bsection` between `chrono` lines: what the script reads is its elapsed time.
fn timed_section(s: &mut dyn Session, result: &str, first: &str, second: &str, label: &str) -> f64 {
    let mut timer = Timer::new(false);
    timer.start();
    let flags = SectionFlags {
        pcurve_on1: true,
        pcurve_on2: true,
        approximation: true,
    };
    bsection_with(s, result, (first, second), flags);
    timer.stop();
    println!("COUNTER CPU section {label}: {:.2}", timer.elapsed_time());
    timer.elapsed_time()
}

fn both_sections(s: &mut dyn Session) -> [f64; 2] {
    restore(s, "BUC60912_sec_slow.brep", "c");
    explode(s, "c", None);
    renamevar(s, "c_1", "sh");
    renamevar(s, "c_2", "pr");
    let surface = plane([0., 0., 0.], [1., 0., 0.]);
    mkface_bounded(s, "f", &surface, (-11., 11., -11., 11.));
    println!("Info: perform section with plane");
    let first = timed_section(s, "r1", "f", "pr", "r1");
    println!("Info: perform section with planar BSpline surface");
    let second = timed_section(s, "r2", "sh", "pr", "r2");
    [first, second]
}

fn brep_text(shape: &Shape, name: &str) -> Vec<u8> {
    let path = std::env::temp_dir().join(format!("bug83-{}-{name}.brep", std::process::id()));
    let target = AsciiString::from_bytes(path.as_os_str().as_encoded_bytes());
    assert!(brep_tools::write_file(shape, &target, &ProgressRange::default()).unwrap());
    let text = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    text
}

/// The script without its 0.1 s limits, waived by
/// The maintainer recorded a timing exception for them; `bug83_elapsed_limits` keeps them.
pub fn bug83(s: &mut dyn Session) {
    println!("=====");
    println!("OCC83");
    println!("=====");
    println!();
    both_sections(s);
    // Beyond the script: the results equal the native run's, as an earlier test checked.
    let native: [(&str, &[u8]); 2] = [
        ("r1", include_bytes!("../data/bug83/r1.brep")),
        ("r2", include_bytes!("../data/bug83/r2.brep")),
    ];
    for (name, bytes) in native {
        assert!(
            brep_text(&bound_shape(s, name), name) == bytes,
            "bsection {name}: expected the native BRep bytes"
        );
    }
}

/// The script's own limits; left out of the default run, see `bug83`.
pub fn bug83_elapsed_limits(s: &mut dyn Session) {
    println!("=====");
    println!("OCC83");
    println!("=====");
    println!();
    let [first, second] = both_sections(s);
    if first > 0.1 {
        println!("Error: Section of simple BSpline surfaces_1 is performed too slow");
    }
    if second > 0.1 {
        println!("Error: Section of simple BSpline surfaces_2 is performed too slow");
    }
}

pub fn bug30788(s: &mut dyn Session) {
    println!("============================================");
    println!(
        "0030788: Modeling Algorithms - BRepAlgoAPIDefeaturing does not propagate progress indicator to underlying algorithms"
    );
    println!("============================================");
    println!();
    println!(
        "Progress Indicator defaults: tcl mode is ON, console mode is OFF, graphical mode is OFF"
    );
    restore_tracked(s, "bug29481_ex1.brep", "s");
    explode(s, "s", Some(ShapeEnum::Face));
    compound(s, &["s_6", "s_18"], "step");
    let mut removal = BRepAlgoAPIDefeaturing::new();
    removal.set_shape(&bound_shape(s, "s"));
    removal.add_face_to_remove(&bound_shape(s, "step"));
    removal.set_run_parallel(false);
    let (_, lines) = tracked_lines(|range| removal.build_with_progress(range));
    s.bind("res", removal.shape());
    // Beyond the script: an earlier test required progress from the work inside the removal.
    child_count(s, "step", 2);
    for inner in ["Preparing features", "Simplifyingthe result"] {
        assert!(
            lines.iter().any(|line| line.contains(inner)),
            "removefeatures: expected progress of {inner}"
        );
    }
}
