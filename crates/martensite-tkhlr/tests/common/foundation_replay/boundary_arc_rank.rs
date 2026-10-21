//! Stable native domain traversal for boundary arc attribution.
use martensite_tkg3d::adaptor3d::Adaptor3dTopolTool;
use martensite_tkgeomalgo::int_start::TheArc;
pub(crate) fn arc_rank(d: &Adaptor3dTopolTool, arc: &TheArc) -> i32 {
    let mut rank = 0;
    d.init();
    while d.more() {
        rank += 1;
        if d.value().is_ok_and(|a| &a == arc) {
            return rank;
        }
        d.next();
    }
    0
}
