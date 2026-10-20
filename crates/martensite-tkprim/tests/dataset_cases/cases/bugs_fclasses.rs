//! `bugs_fclasses` case bodies.
use crate::support::{
    checkshape::checkshape,
    curves::{imagedir, shape_set},
    log,
    logged_checks::checknbshapes,
    misc::{brestore, bsave},
    prim::box_sizes,
    props_view::{Expected, checkprops},
    session::{Session, checkview_display},
};

/// A box saved with `bsave` under a file name with Unicode characters and read
/// back with `brestore` keeps its area, validity and sub-shapes.
pub fn bug25367_brep(s: &mut dyn Session) {
    log::puts("==========");
    log::puts("OCC25367");
    log::puts("==========");
    log::puts("");
    // `encoding convertfrom unicode "\xDE\x30\xF9\x30\xF1\x30"`: U+30DE U+30F9 U+30F1.
    let unicode = "\u{30DE}\u{30F9}\u{30F1}";
    let name_file = imagedir().join(format!("OCC25367_{unicode}.brep"));
    let b = shape_set(s, "b", box_sizes(1., 1., 1.));
    bsave(&b, &name_file);
    log::result("b");
    let result = shape_set(s, "result", brestore(&name_file));
    log::result("result");
    checkprops(s, "result", &result, Expected::s(6.));
    checkshape(&result);
    checknbshapes(&result, [8, 12, 6, 6, 1, 1, 0, 0, 34].map(Some), None);
    std::fs::remove_file(&name_file).unwrap();
    checkview_display(s, Some("result"), "bug25367_brep");
}
