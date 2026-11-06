//! `tests/bugs/xde`: `bug28055`, the top-down update of assembly compounds.
use crate::commands::same_shape;
use crate::grading::BUGS;
use crate::host::ViewerHost;
use crate::support::nbshapes::NbCount;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;

/// `1` the shapes stay the same, `0` they differ.
const SAME_MASK: [i32; 18] = [1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0];

fn read_and_show(host: &mut ViewerHost) {
    let path = host.session.path_of(Place::Data("trj3_as1-tc-214.stp"));
    println!("ReadStep d {path} ");
    assert!(host.session.read_step("d", &path).is_ok(), "ReadStep d");
    host.xshow("d");
    host.vfit();
    host.vsetdispmode_shaded();
}

fn one_shape(host: &mut ViewerHost, name: &str) {
    println!("XGetOneShape {name} d ");
    assert!(
        host.session.get_one_shape(name, "d").is_ok(),
        "XGetOneShape {name} d"
    );
}

fn bug28055_1(host: &mut ViewerHost) {
    read_and_show(host);
    one_shape(host, "model_before");
    println!("checknbshapes model_before -solid 18 -t ");
    host.checknbshapes_typed("model_before", "", &[NbCount::Solids(18)]);
    host.checkview_screenshot("bug28055_1_1");
    host.xremove_component("d", "0:1:1:1:1");
    host.xupdate_assemblies("d");
    host.xshow("d");
    host.checkview_screenshot("bug28055_1_2");
    one_shape(host, "model_after");
    println!("checknbshapes model_after -solid 11 -t ");
    host.checknbshapes_typed("model_after", "", &[NbCount::Solids(11)]);
}
viewer_case!(
    bugs_xde_bug28055_1,
    "tests/bugs/xde/bug28055_1",
    BUGS,
    Verdict::Ok,
    bug28055_1
);

fn explode_solids(host: &mut ViewerHost, name: &str) {
    println!("explode {name} So ");
    host.session.explode(name, "So");
}

fn bug28055_2(host: &mut ViewerHost) {
    read_and_show(host);
    one_shape(host, "model1");
    explode_solids(host, "model1");
    host.checkview_screenshot("bug28055_2_1");
    host.box_at("box_nut", [-2., -2., 0.], [22., 18., 8.]);
    host.xset_shape("d", "0:1:1:5", "box_nut");
    host.xupdate_assemblies("d");
    host.xshow("d");
    one_shape(host, "model2");
    explode_solids(host, "model2");
    for (at, wanted) in (1..).zip(SAME_MASK) {
        let (first, second) = (format!("model1_{at}"), format!("model2_{at}"));
        println!("IsSameShapes {first} {second} ");
        let same = i32::from(same_shape(&host.shape(&first), &host.shape(&second)));
        if same == 1 {
            println!("1");
        }
        println!("Comparison result for subshape {at}: {same}");
        if same != wanted {
            if wanted == 1 {
                println!("Error: shapes are expected to be identical");
            } else {
                println!("Error: shapes are expected to be different");
            }
        }
    }
    host.checkview_screenshot("bug28055_2_2");
}
viewer_case!(
    bugs_xde_bug28055_2,
    "tests/bugs/xde/bug28055_2",
    BUGS,
    Verdict::Ok,
    bug28055_2
);
