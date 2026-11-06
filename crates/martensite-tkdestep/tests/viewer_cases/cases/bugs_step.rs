//! `tests/bugs/step`.
use crate::grading::BUGS;
use crate::host::ViewerHost;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;

fn puts(lines: &[&str]) {
    for line in lines {
        println!("{line}");
    }
}

/// `stepread [locate_data_file file] name *`.
fn stepread(host: &mut ViewerHost, file: &'static str, name: &str) {
    let path = host.session.path_of(Place::Data(file));
    println!("stepread {path} {name} * ");
    host.session.stepread(&path, name);
}

fn bug24283_2(host: &mut ViewerHost) {
    puts(&["============", "24283", "============", ""]);
    stepread(host, "bug24283_spheres.stp", "a");
    host.session.tpcompound("result");
    host.vinit();
    host.vdisplay("result");
    host.vfit();
    host.vsetdispmode_shaded();
    host.checkview_screenshot("bug24283_2");
}
viewer_case!(
    bugs_step_bug24283_2,
    "tests/bugs/step/bug24283_2",
    BUGS,
    Verdict::Ok,
    bug24283_2
);

fn bug25167(host: &mut ViewerHost) {
    puts(&["========", "25167", "========", ""]);
    stepread(host, "bug25167.stp", "a");
    host.session.tpcompound("result");
    host.checkprops_surface("result", 20800.0);
    host.checkshape("result");
    host.checknbshapes(
        "result",
        &[
            "-vertex",
            "26",
            "-edge",
            "37",
            "-wire",
            "18",
            "-face",
            "18",
            "-shell",
            "3",
            "-solid",
            "3",
            "-compsolid",
            "0",
            "-compound",
            "5",
            "-shape",
            "110",
        ],
    );
    host.checkview_display("result", "bug25167");
}
viewer_case!(
    bugs_step_bug25167,
    "tests/bugs/step/bug25167",
    BUGS,
    Verdict::Ok,
    bug25167
);

fn bug25169(host: &mut ViewerHost) {
    puts(&["========", "25169", "========", ""]);
    stepread(host, "bug25169_box-openshell.stp", "a");
    host.session.tpcompound("result");
    host.checkprops_surface("result", 2400.0);
    host.checkshape("result");
    host.checknbshapes(
        "result",
        &[
            "-vertex",
            "8",
            "-edge",
            "12",
            "-wire",
            "6",
            "-face",
            "6",
            "-shell",
            "1",
            "-solid",
            "1",
            "-compsolid",
            "0",
            "-compound",
            "1",
            "-shape",
            "35",
        ],
    );
    host.checkview_display("result", "bug25169");
}
viewer_case!(
    bugs_step_bug25169,
    "tests/bugs/step/bug25169",
    BUGS,
    Verdict::Ok,
    bug25169
);

fn bug28414(host: &mut ViewerHost) {
    stepread(host, "bug28414_bug_toroidal_surface.stp", "a");
    println!("checkshape a_1 f ");
    if crate::support::checkshape::checkshape_text_quiet(&host.shape("a_1")).is_valid() {
        println!(" -- The Shape a_1 looks OK");
    } else {
        host.session.check_shape("a_1");
    }
    host.checkprops_surface("a_1", 3593.35);
    host.checkgravitycenter_surface("a_1", ["-30.", "0.", "137.993"], "0.001");
    host.vinit();
    host.vdisplay("a_1");
    host.vfit();
    host.vdump("bug28414");
}
viewer_case!(
    bugs_step_bug28414,
    "tests/bugs/step/bug28414",
    BUGS,
    Verdict::Ok,
    bug28414
);

fn bug32239(host: &mut ViewerHost) {
    stepread(host, "bug32239.STEP", "b");
    host.checkshape("b_1");
    let expected = [
        -62.992562093513783,
        -51.45371311501097,
        -33.223762093513777,
        43.257600511674156,
        33.211062093513789,
        37.744962093513792,
    ];
    let bounds = host.bounding("b_1");
    if bounds
        .iter()
        .zip(expected)
        .any(|(got, wanted)| (got - wanted).abs() > 1.0e-5)
    {
        println!("Error: not expected bounding box");
    }
    println!("renamevar b_1 result ");
    host.session.renamevar("b_1", "result");
    host.checkview_display("result", "bug32239");
}
viewer_case!(
    bugs_step_bug32239,
    "tests/bugs/step/bug32239",
    BUGS,
    Verdict::Ok,
    bug32239
);

fn bug32350(host: &mut ViewerHost) {
    puts(&[
        "===================================================",
        "0032350: Data Exchange - StepControlWriter.writeStream(std::ostream)",
        "===================================================",
        "",
    ]);
    println!("Close D -silent ");
    let source = host.session.path_of(Place::Data("as1-oc-214-mat.stp"));
    let (first, second) = (
        host.session.path_of(Place::Image("bug32350_1.stp")),
        host.session.path_of(Place::Image("bug32350_2.stp")),
    );
    host.read_step_stream("D", &source);
    println!("WriteStep D {first} ");
    assert!(host.session.write_step("D", &first).is_ok());
    host.write_step_stream("D", &second);
    println!("ReadStep D1 {first} ");
    assert!(host.session.read_step("D1", &first).is_ok());
    println!("ReadStep D2 {second} ");
    assert!(host.session.read_step("D2", &second).is_ok());
    let _ = std::fs::remove_file(&first);
    let _ = std::fs::remove_file(&second);
    host.vclear();
    host.vinit_named("View1");
    host.xdisplay_shaded("D", false);
    host.vfit();
    host.vdump("bug32350_src");
    host.vclear();
    host.xdisplay_shaded("D1", false);
    host.vdump("bug32350_out1");
    host.vclear();
    host.xdisplay_shaded("D2", false);
    host.vdump("bug32350_out2");
}
viewer_case!(
    bugs_step_bug32350,
    "tests/bugs/step/bug32350",
    BUGS,
    Verdict::Ok,
    bug32350
);

fn bug32596(host: &mut ViewerHost) {
    let file = host.session.path_of(Place::Data("bug32596_abc_0_13.step"));
    println!("testreadstep {file} res ");
    host.session.testreadstep(&file, "res");
    host.checkshape("res");
    host.checknbshapes(
        "res",
        &[
            "-face", "99", "-wire", "118", "-edge", "232", "-vertex", "144",
        ],
    );
    host.vinit_named("View1");
    host.vright();
    host.vdisplay_shaded("res", "-dispMode 1 res");
    host.vfit();
    println!("vreadpixel 200 200 -rgb -name ");
    let color = host
        .live_scene()
        .pixel(200, 200)
        .expect("a pixel in the view");
    let name = martensite_tkernel::quantity::Color::string_name(color.name());
    println!("{name}");
    if name != "BLACK" {
        println!("Error: Incorrect shape");
    }
}
viewer_case!(
    bugs_step_bug32596,
    "tests/bugs/step/bug32596",
    BUGS,
    Verdict::Ok,
    bug32596
);
