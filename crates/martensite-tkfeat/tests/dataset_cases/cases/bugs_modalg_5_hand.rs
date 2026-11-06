//! `bugs/modalg_5/bug24086`: the split of a face and a wire, shown on the real renderer.
use crate::support::{
    feat_script::{FeatScript, checked},
    raster_session::KEEP_IMAGES_ENV,
    restore::restore,
    session::Session,
    viewer_host::ViewerHost,
};
use martensite_tkbool::brep_algo;
use martensite_tkbrep::topods;
use martensite_tkfeat::brep_feat::BRepFeatSplitShape;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::path::PathBuf;

/// `bug24086 face wire`: the split shape, checked, then displayed in the open viewer.
fn wire_on_face_verdict(
    s: &FeatScript<'_>,
    host: &mut ViewerHost,
    face: &str,
    wire: &str,
) -> String {
    let face = checked(topods::face(&s.shape_or_null(face)));
    let wire = checked(topods::wire(&s.shape_or_null(wire)));
    let mut split = BRepFeatSplitShape::with_shape(face.as_ref());
    checked(split.add_wire(&wire, &face));
    let result = checked(split.shape()).clone();
    let verdict = if checked(brep_algo::is_valid(&result)) {
        "VALID"
    } else {
        "INVALID"
    };
    let answer = format!("Result was checked and it is {verdict}");
    println!("{answer}");
    host.display("24086", &result);
    answer
}

pub fn bug24086(s: &mut FeatScript<'_>) {
    println!("============");
    println!("24086");
    println!("============");
    println!();
    restore(s, "bug24086_face.brep", "face");
    restore(s, "bug24086_wire.brep", "wire");
    let mut host = ViewerHost::open();
    if wire_on_face_verdict(s, &mut host, "face", "wire").contains("INVALID") {
        println!("Error: result shape is invalid");
    } else {
        println!("OK: result shape is valid");
    }
    host.fit();
    host.display("face", &s.shape_or_null("face"));
    host.display("wire", &s.shape_or_null("wire"));
    finish(host, "bug24086");
}

/// `checkview -screenshot -3d`: the image must show something, and validation must stay clean.
pub fn finish(host: ViewerHost, image: &str) {
    let keep = std::env::var_os(KEEP_IMAGES_ENV)
        .map(|folder| PathBuf::from(folder).join(format!("{image}.ppm")));
    let drawn = host.dump(keep.as_deref());
    assert!(
        drawn > 0,
        "checkview {image}: expected a drawn image, got {drawn} pixels"
    );
    let errors = host.close();
    assert!(errors.is_empty(), "validation errors: {errors:?}");
}
