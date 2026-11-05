//! The samples of the cutter and the drill, on the models their `bugs` scripts share.
use super::sample_models::{cutter_model, drill_end_section, drill_model};
use crate::commands::incremental_mesh;
use crate::grading::{DEMO, run_alone};
use crate::scene::{SHADED, ViewScene};
use crate::support::rules::Verdict;
use crate::variables::Variables;

const BLACK: [f64; 3] = [0., 0., 0.];

#[test]
fn cutter() {
    run_alone("cases::samples_cutting::cutter", &DEMO, Verdict::Ok, || {
        let mut variables = Variables::default();
        cutter_model(&mut variables);
        let cutter = variables.get("cutter");
        incremental_mesh(&cutter, 0.01);
        let mut scene = ViewScene::open("cutter");
        scene.display("cutter", &cutter);
        scene.set_display_mode("cutter", SHADED);
        scene.set_sample_count(8);
        let section = variables.get("profile_1");
        incremental_mesh(&section, 0.01);
        scene.display("profile_1", &section);
        scene.fit();
        assert!(scene.dump(BLACK) > 0, "the view shows nothing");
        scene.close();
        println!("TEST COMPLETED");
    });
}

#[test]
fn drill() {
    run_alone("cases::samples_cutting::drill", &DEMO, Verdict::Ok, || {
        let mut variables = Variables::default();
        drill_model(&mut variables);
        println!("Displaying result...");
        let drill = variables.get("drill");
        incremental_mesh(&drill, 0.01);
        let mut scene = ViewScene::open("drill");
        scene.display("drill", &drill);
        scene.set_display_mode("drill", SHADED);
        scene.set_sample_count(8);
        scene.fit();
        let guide = variables.get("gg");
        incremental_mesh(&guide, 0.01);
        scene.display("gg", &guide);
        scene.display("sec_1", &drill_end_section(&variables));
        assert!(scene.dump(BLACK) > 0, "the view shows nothing");
        scene.close();
        println!("TEST COMPLETED");
    });
}
