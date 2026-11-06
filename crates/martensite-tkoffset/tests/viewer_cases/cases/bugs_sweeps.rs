//! The `bugs` scripts that demonstrate the sweeping algorithm on a cutter and a drill.
use super::sample_models::{cutter_model, drill_end_section, drill_model};
use crate::commands::{check_shape, incremental_mesh};
use crate::grading::{BUGS, run_alone};
use crate::scene::{SHADED, ViewScene};
use crate::support::checkprops::{Expected, checkprops};
use crate::support::properties::Prop;
use crate::support::rules::Verdict;
use crate::variables::Variables;

const BLACK: [f64; 3] = [0., 0., 0.];

/// `renamevar`, the area and shape checks and `checkview`, the script's last lines.
fn final_checks(scene: ViewScene, variables: &mut Variables, shown: &str, area: f64) {
    variables.set("result", variables.get(shown));
    checkprops(&*variables, "result", Prop::Area, Expected::Mass(area));
    check_shape(variables, "result");
    assert!(scene.dump(BLACK) > 0, "the view shows nothing");
    scene.close();
    println!("TEST COMPLETED");
}

#[test]
fn bug24758_1() {
    run_alone("cases::bugs_sweeps::bug24758_1", &BUGS, Verdict::Ok, || {
        let mut variables = Variables::default();
        cutter_model(&mut variables);
        let cutter = variables.get("cutter");
        incremental_mesh(&cutter, 0.01);
        let mut scene = ViewScene::open("bug24758_1");
        scene.display("cutter", &cutter);
        scene.set_display_mode("cutter", SHADED);
        let section = variables.get("profile_1");
        incremental_mesh(&section, 0.01);
        scene.display("profile_1", &section);
        scene.fit();
        final_checks(scene, &mut variables, "cutter", 1403.85);
    });
}

#[test]
fn bug24758_2() {
    run_alone("cases::bugs_sweeps::bug24758_2", &BUGS, Verdict::Ok, || {
        let mut variables = Variables::default();
        drill_model(&mut variables);
        println!("Displaying result...");
        let drill = variables.get("drill");
        incremental_mesh(&drill, 0.01);
        let mut scene = ViewScene::open("bug24758_2");
        scene.display("drill", &drill);
        scene.set_display_mode("drill", SHADED);
        scene.fit();
        let guide = variables.get("gg");
        incremental_mesh(&guide, 0.01);
        scene.display("gg", &guide);
        scene.display("sec_1", &drill_end_section(&variables));
        final_checks(scene, &mut variables, "drill", 3277.87);
    });
}
