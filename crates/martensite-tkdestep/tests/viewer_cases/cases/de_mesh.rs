//! `tests/de_mesh`: the grids' `begin` and `end` are the scripts' test.
use crate::commands::face_of_mesh;
use crate::grading::DE_MESH;
use crate::host::ViewerHost;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;
use martensite_tkdestl::rw_stl::{DEFAULT_MERGE_ANGLE, RwStl};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::tcollection::AsciiString;

/// `tests/de_mesh/step_read/begin` and `step_write/begin`.
fn tessellated_parameters(host: &ViewerHost) {
    host.session
        .pilot_command("param write.step.schema AP242DIS");
    host.session.pilot_command("param read.step.tessellated On");
    host.session
        .pilot_command("param write.step.tessellated On");
    host.session.pilot_command("newmodel");
}

/// The end of both grids: the document explored, axonometric, with the grids' lights.
fn show_explored(host: &mut ViewerHost, image: &str) {
    host.vclear();
    host.vinit_named("View1");
    host.xdisplay_shaded("D", true);
    host.vaxo();
    host.vfit();
    host.vrenderparams_phong();
    host.vlight_intensity(0, 2.5);
    host.vlight_intensity(1, 0.3);
    host.vcamera_orthographic();
    host.vdump(image);
    println!("Close D -silent ");
    host.session.close_document("D");
}

fn step_read_a2(host: &mut ViewerHost) {
    tessellated_parameters(host);
    println!("0030839: Data Exchange - triangulation-only STEP file is read as empty shape");
    let path = host
        .session
        .path_of(Place::Data("nist_ftc_08_asme1_ap242-2.stp"));
    println!("ReadStep D {path} ");
    assert!(host.session.read_step("D", &path).is_ok(), "ReadStep D");
    println!("XGetOneShape S D ");
    assert!(
        host.session.get_one_shape("S", "D").is_ok(),
        "XGetOneShape S D"
    );
    host.checktrinfo("S", &["-face", "273", "-nod", "3788", "-tri", "3377"]);
    show_explored(host, "A2");
    host.session
        .pilot_command("param write.step.schema AP214CD");
    host.session
        .pilot_command("param read.step.tessellated Off");
    host.session
        .pilot_command("param write.step.tessellated Off");
}
viewer_case!(
    de_mesh_step_read_a2,
    "tests/de_mesh/step_read/A2",
    DE_MESH,
    Verdict::Ok,
    step_read_a2
);

/// `readstl S file`: a single face over the mesh.
fn read_stl_as_face(host: &mut ViewerHost, file: &'static str) {
    let path = host.session.path_of(Place::Data(file));
    println!("readstl S {path} ");
    let file_name = AsciiString::from(path.as_str());
    let mesh =
        RwStl::read_file_with_angle(&file_name, DEFAULT_MERGE_ANGLE, &ProgressRange::default())
            .unwrap_or_else(|failure| panic!("readstl {path}: {failure:?}"));
    host.bind("S", &face_of_mesh(&mesh));
}

fn step_write_a1(host: &mut ViewerHost) {
    tessellated_parameters(host);
    read_stl_as_face(host, "b4.stl");
    let saved = host.session.path_of(Place::Image("A1.stp"));
    println!("stepwrite a S {saved} ");
    host.session.stepwrite("a", "S", &saved);
    println!("ReadStep D {saved} ");
    assert!(host.session.read_step("D", &saved).is_ok(), "ReadStep D");
    println!("XGetOneShape S1 D ");
    assert!(
        host.session.get_one_shape("S1", "D").is_ok(),
        "XGetOneShape S1 D"
    );
    host.checktrinfo("S1", &["-face", "1", "-nod", "8", "-tri", "12"]);
    show_explored(host, "A1");
    println!("Close D1 -silent ");
    host.session.close_document("D1");
    host.session
        .pilot_command("param write.step.schema AP214CD");
    host.session.pilot_command("param read.step.tessellated On");
    host.session
        .pilot_command("param write.step.tessellated OnNoBRep");
}
viewer_case!(
    de_mesh_step_write_a1,
    "tests/de_mesh/step_write/A1",
    DE_MESH,
    Verdict::Ok,
    step_write_a1
);

/// One merge result on its own, with the face boundaries.
fn show_merged(host: &mut ViewerHost, name: &str, image: &str) {
    host.vclear();
    host.vdisplay_shaded(name, &format!("-dispMode 1 {name}"));
    host.vaspects_face_boundary(name);
    host.vdump(image);
}

fn stl_read_d3(host: &mut ViewerHost) {
    let path = host.session.path_of(Place::Data("as1-oc-214-mat.stp"));
    println!("testreadstep {path} ss ");
    host.session.testreadstep(&path, "ss");
    host.incmesh("ss", "1.0");
    host.vinit_named("View1");
    host.vdisplay_shaded("ss", "-dispMode 1 ss");
    host.vaspects_face_boundary("ss");
    host.vfit();
    host.vdump("D3_stp");
    host.trmergenodes_one_face("ss", 0.0, "m0");
    host.trmergenodes_one_face("ss", 45.0, "m45");
    host.trmergenodes_one_face("ss", 90.0, "m90");
    show_merged(host, "m0", "D3_m0");
    show_merged(host, "m45", "D3_m45");
    show_merged(host, "m90", "D3_m90");
}
viewer_case!(
    de_mesh_stl_read_d3,
    "tests/de_mesh/stl_read/D3",
    DE_MESH,
    Verdict::Ok,
    stl_read_d3
);
