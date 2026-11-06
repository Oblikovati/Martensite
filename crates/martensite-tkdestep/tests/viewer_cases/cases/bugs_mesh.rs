//! `tests/bugs/mesh`.
use crate::grading::BUGS;
use crate::host::ViewerHost;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;

/// `stepread [locate_data_file file] a *`, `renamevar a_1 result`.
fn read_as_result(host: &mut ViewerHost, file: &'static str) {
    let path = host.session.path_of(Place::Data(file));
    println!("stepread {path} a * ");
    host.session.stepread(&path, "a");
    println!("renamevar a_1 result ");
    host.session.renamevar("a_1", "result");
}

fn bug25045(host: &mut ViewerHost) {
    read_as_result(host, "bug25045_ick_0433.stp");
    host.vinit();
    host.vsetdispmode_shaded();
    host.vbottom();
    host.vdisplay("result");
    host.vfit();
    host.checkview_screenshot("bug25045");
    println!("explode result f ");
    let nb_faces = host.session.explode("result", "f");
    let log = host.tricheck("result");
    if nb_faces != 13 {
        println!(
            "Error: Shape restored incorrectly - number of faces is different than expected value ({nb_faces} != 13)"
        );
    }
    if !log.split_whitespace().next().is_none() {
        println!("Error: Some faces has failed to mesh");
    }
}
viewer_case!(
    bugs_mesh_bug25045,
    "tests/bugs/mesh/bug25045",
    BUGS,
    Verdict::Ok,
    bug25045
);

fn bug30149(host: &mut ViewerHost) {
    read_as_result(host, "bug30149_comb_bug.step");
    host.tclean("result");
    host.incmesh("result", "1");
    let info = host.tricheck("result");
    if info.contains("Not connected mesh inside face") {
        println!("Error : Mesh contains free triangle");
    }
    host.checktrinfo("result", &["-tri", "32", "-nod", "54"]);
    host.vinit();
    host.vtop();
    host.vsetdispmode_shaded();
    host.vdisplay("result");
    host.vfit();
    host.checkview_screenshot("bug30149");
}
viewer_case!(
    bugs_mesh_bug30149,
    "tests/bugs/mesh/bug30149",
    BUGS,
    Verdict::Ok,
    bug30149
);
