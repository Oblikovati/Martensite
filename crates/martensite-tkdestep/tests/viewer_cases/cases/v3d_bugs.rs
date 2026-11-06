//! `tests/v3d/bugs`.
use crate::grading::V3D;
use crate::host::ViewerHost;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;
use martensite_tkservice::aspect::AspectTypeOfLine;

fn bug23123(host: &mut ViewerHost) {
    let path = host.session.path_of(Place::Data("OCC23123-test.step"));
    println!("ReadStep D {path} ");
    assert!(host.session.read_step("D", &path).is_ok(), "ReadStep D");
    host.xshow("D");
    host.vfit();
    host.vsetdispmode_shaded();
    println!("Point color test:");
    host.checkcolor(310, 225, [0.04, 0.04, 0.67]);
    host.vdump("bug23123");
}
viewer_case!(
    v3d_bugs_bug23123,
    "tests/v3d/bugs/bug23123",
    V3D,
    Verdict::Ok,
    bug23123
);

fn bug23407_1(host: &mut ViewerHost) {
    println!("XNewDoc Doc1 ");
    host.session.new_document("Doc1");
    let path = host.session.path_of(Place::Data("screw.step"));
    println!("ReadStep Doc1 {path} ");
    assert!(
        host.session.read_step("Doc1", &path).is_ok(),
        "ReadStep Doc1"
    );
    host.xshow_label("Doc1", "0:1:1:1");
    host.xshow_face_boundary(
        "Doc1",
        "0:1:1:1",
        [255., 0., 0.],
        10.0,
        AspectTypeOfLine::Dash,
    );
    host.vfit();
    host.vsetdispmode_shaded();
    if !host.checkcolor(178, 36, [1., 0., 0.]) {
        println!("Error : Boundary of face is not changed");
    }
    if !host.checkcolor(244, 89, [1., 0., 0.]) {
        println!("Error : Boundary of face is not changed");
    }
    host.vdump("bug23407_1");
}
viewer_case!(
    v3d_bugs_bug23407_1,
    "tests/v3d/bugs/bug23407_1",
    V3D,
    Verdict::Ok,
    bug23407_1
);
