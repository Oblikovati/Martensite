//! tests/heal/elementary_to_revolution case bodies at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871: each restores `a` and sets `nb_revol`, then
//! the grid end points `CSF_dataDefaults` to tests/heal/data, runs `DT_ApplySeq result a
//! data data1` (the ElementaryToRevolution operator), explodes the faces of `a` and
//! `result`, dumps the surface of each pair and counts the faces whose type changed to
//! SurfaceOfRevolution, which must be `nb_revol`; tests/heal/end shows `result` with
//! `checkview -display result -2d`.
use crate::support::{
    commands::{dump_surface, explode_type, mksurface},
    log,
    restore::{Restored, restore_as},
    session::{Session, checkview_display, set, set_exploded},
    swdraw::{dt_apply_seq, heal_data_directory, set_resource_defaults},
    text::lindex,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `explode name f`: the face variables, and the names the command returns.
fn explode_faces(
    s: &mut dyn Session,
    name: &str,
    shape: &martensite_tkbrep::topods::Shape,
) -> (Vec<martensite_tkbrep::topods::Shape>, String) {
    let faces = set_exploded(s, name, explode_type(shape, ShapeEnum::Face));
    let names: Vec<String> = (1..=faces.len()).map(|i| format!("{name}_{i}")).collect();
    let text = names.join(" ");
    log::result(&text);
    (faces, text)
}

/// `mksurface name face` then `set q [dump name]` (logged), returning the dump.
fn dumped_surface(
    s: &mut dyn Session,
    name: &str,
    face: &martensite_tkbrep::topods::Shape,
) -> String {
    let surface = mksurface(face);
    s.set(name, &Restored::Surface(surface.clone()));
    let dump = dump_surface(name, &surface);
    log::result(&dump);
    dump
}

/// `restore file a`, `set nb_revol n`, then the grid end.
fn elementary_to_revolution(s: &mut dyn Session, file: &str, nb_revol: usize, image: &str) {
    let a = restore_as(s, file, "a").shape();
    let data = heal_data_directory();
    set_resource_defaults("data", &data);
    log::puts(&data.display().to_string());
    let result = set(s, "result", dt_apply_seq(&a, "data", "data1"));
    let (b, b_names) = explode_faces(s, "a", &a);
    let (b1, b1_names) = explode_faces(s, "result", &result);
    log::puts(&b_names);
    log::puts(&b1_names);
    let mut nb_surf = 0;
    for (i, t) in b.iter().enumerate() {
        // `lindex $b1 $i` past the end is empty, and `mksurface su$i` of it fails.
        let t1 = b1.get(i).expect("mksurface: result has fewer faces than a");
        let q = dumped_surface(s, &format!("s{i}"), t);
        let q1 = dumped_surface(s, &format!("su{i}"), t1);
        log::puts(&q);
        log::puts(&q1);
        if lindex(&q, 5) != lindex(&q1, 5) && lindex(&q1, 5) == "SurfaceOfRevolution" {
            nb_surf += 1;
        }
    }
    if nb_surf == 0 || nb_revol != nb_surf {
        log::puts(&format!(
            "Error : {nb_revol} faces should be converted instead of {nb_surf}."
        ));
    }
    checkview_display(s, Some("result"), image);
}

/// tests/heal/elementary_to_revolution/A1.
pub fn a1(s: &mut dyn Session) {
    elementary_to_revolution(s, "DSE3.rle", 4, "A1");
}

/// tests/heal/elementary_to_revolution/A2.
pub fn a2(s: &mut dyn Session) {
    elementary_to_revolution(s, "coneETR.rle", 1, "A2");
}

/// tests/heal/elementary_to_revolution/A3.
pub fn a3(s: &mut dyn Session) {
    elementary_to_revolution(s, "sphereETR.rle", 1, "A3");
}

/// tests/heal/elementary_to_revolution/A4.
pub fn a4(s: &mut dyn Session) {
    elementary_to_revolution(s, "cylinderETR.rle", 1, "A4");
}

/// tests/heal/elementary_to_revolution/A5.
pub fn a5(s: &mut dyn Session) {
    elementary_to_revolution(s, "torusETR.rle", 1, "A5");
}
