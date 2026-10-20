//! `bugs_modalg_7` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::checklength2d,
    commands::{
        crvtpoints_count, explode_type, mkcurve, mksurface, offset_surface, pcurve, project, tclean,
    },
    restore::{binrestore, restore_as},
    session::{Session, ViewStep, checkview_display, set, set_exploded},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;

/// 0026558: `uiso icur o 0` of the offset of the face surface must be built; then
/// `smallview`, `fit` and the screenshot.
fn offset_isoline(s: &mut dyn Session, distance: f64, image: &str) {
    let face = restore_as(s, "bug26558_s.brep", "s").shape();
    let sur = set(s, "sur", mksurface(&face));
    let o = set(
        s,
        "o",
        offset_surface(&sur, distance).expect("offset o sur"),
    );
    let icur = o.get().unwrap().u_iso(0.).expect("uiso icur o 0");
    assert!(!icur.is_null(), "uiso icur o 0 gave no curve");
    set(s, "icur", icur);
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd(image));
}

pub fn bug26558_1(s: &mut dyn Session) {
    offset_isoline(s, 0.5, "bug26558_1-3d");
}

pub fn bug26558_2(s: &mut dyn Session) {
    offset_isoline(s, -0.5, "bug26558_2-3d");
}

/// `checkview -display face -2d` of the face.
fn isolines_checkview(s: &mut dyn Session, file: &str, name: &str, image: &str) {
    let face = restore_as(s, file, name).shape();
    assert!(!face.is_null(), "restore of {file}");
    checkview_display(s, Some(name), image);
}

/// `smallview`, `isos face 50`, `fit` and the screenshot.
fn isolines_50(s: &mut dyn Session, file: &str, image: &str) {
    let face = restore_as(s, file, "face").shape();
    assert!(!face.is_null(), "restore of {file}");
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Isos("face", 50));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd(image));
}

pub fn bug28385_1(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_face.brep", "face", "bug28385_1-2d");
}

pub fn bug28385_2(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_planar_face.brep", "face", "bug28385_2-2d");
}

pub fn bug28385_3(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_two_lines_face.brep", "face", "bug28385_3-2d");
}

/// `smallview`, `don face; fit`, `isos face 50` (no fit after it) and the screenshot.
pub fn bug28385_4(s: &mut dyn Session) {
    let face = restore_as(s, "bug28385_circ_face.brep", "face").shape();
    assert!(!face.is_null(), "restore of bug28385_circ_face.brep");
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Display(&["face"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Isos("face", 50));
    s.step(ViewStep::Xwd("bug28385_4"));
}

pub fn bug28385_5(s: &mut dyn Session) {
    isolines_50(s, "bug28385_circ_face_hole.brep", "bug28385_5");
}

pub fn bug28385_6(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_step4_A8_face.brep", "face", "bug28385_6-2d");
}

pub fn bug28385_7(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_27170_face.brep", "face", "bug28385_7-2d");
}

pub fn bug28385_8(s: &mut dyn Session) {
    isolines_checkview(s, "bug28385_bug763_faces.brep", "faces", "bug28385_8-2d");
}

pub fn bug28385_9(s: &mut dyn Session) {
    isolines_50(s, "bug28385_two_circ_face.brep", "bug28385_9");
}

pub fn bug28385_10(s: &mut dyn Session) {
    isolines_50(s, "bug28385_two_circ_face_gap.brep", "bug28385_10");
}

/// `binrestore` of the BinTools file, `smallview`, `isos a 2`, `repeat 9 u`, `fit` and
/// the screenshot.
pub fn bug28385_11(s: &mut dyn Session) {
    let a = binrestore("bug28385_de_iges_1_J9_faces.bin");
    assert!(
        !a.is_null(),
        "binrestore of bug28385_de_iges_1_J9_faces.bin"
    );
    set(s, "a", a);
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Isos("a", 2));
    for _ in 0..9 {
        s.step(ViewStep::RotateUp);
    }
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug28385_11"));
}

/// 0030203: `catch {project cc c s}` must give a curve whose checklength is 1.
pub fn bug30203(s: &mut dyn Session) {
    let c = restore_as(s, "bug30203_curv.draw", "c").curve();
    let surface = restore_as(s, "bug30203_surf.draw", "s").surface();
    let cc = project(&c, &surface, precision::confusion()).expect("Error: projection fails");
    checklength2d(&set(s, "cc", cc), 1.);
}

/// 0030679: `REQUIRED ALL: Geom_OffsetSurface::EvalD0`: evaluating the face's offset
/// surface at the end of its fourth p-curve must raise that exception (the case catches
/// it), not hang or return a point.
pub fn bug30679(s: &mut dyn Session) {
    let a = restore_as(s, "bug30679_face.brep", "a").shape();
    let pcurves = pcurve(&a);
    for (index, curve) in pcurves.iter().enumerate() {
        set(s, &format!("a_{}", index + 1), curve.clone());
    }
    let uv = pcurves[3]
        .get()
        .unwrap()
        .d0(1.)
        .expect("2dcvalue a_4 1 u v");
    let surface = set(s, "s", mksurface(&a));
    let failure = surface
        .get()
        .unwrap()
        .d0(uv.x(), uv.y())
        .expect_err("svalue s u v");
    let message = format!("{failure:?}");
    assert!(message.contains("Geom_OffsetSurface::EvalD0"), "{message}");
}

/// 0031043: GCPnts_TangentialDeflection honours MinimumOfPoints 1..6 on the edge curve.
pub fn bug31043(s: &mut dyn Session) {
    let result = restore_as(s, "bug28500_shape_mesh_artifact.brep", "result").shape();
    tclean(&result, false);
    let faces = set_exploded(s, "result", explode_type(&result, ShapeEnum::Face));
    let edges = set_exploded(s, "result_7", explode_type(&faces[6], ShapeEnum::Edge));
    let c = set(s, "c", mkcurve(&edges[0]));
    for (min_points, at_least) in [(1, 2), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)] {
        let pnts = crvtpoints_count(&c, 0.01, 1.57, min_points);
        assert!(
            pnts >= at_least,
            "Error : Incorrect number of points {pnts}"
        );
    }
}
