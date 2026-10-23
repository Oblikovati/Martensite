//! bugs demo regression cases owned by
//! TKShHealing whose inputs are in the public dataset.
use super::bugs_common::banner;
use crate::support::{
    commands::explode_type,
    log,
    restore::restore_as,
    session::{Session, set_exploded},
    swdraw::checkoverlapedges,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// tests/bugs/demo/bug25020.
/// `catch {checkoverlapedges b_1 b_2} info` reports the degenerated edge.
pub fn bug25020(s: &mut dyn Session) {
    banner(&["============", "CR25020", "============", ""]);
    let b = restore_as(s, "bug25020_b.brep", "b").shape();
    let edges = set_exploded(s, "b", explode_type(&b, ShapeEnum::Edge));
    let info = match checkoverlapedges(("b_1", "b_2"), &edges[0], &edges[1], None, None) {
        Ok(text) | Err(text) => text,
    };
    if info.contains("degenerated") {
        log::puts("OK : checkoverlapedges have the check for degenerated edges");
    } else {
        log::puts("Error : checkoverlapedges doesn't have the check for degenerated edges");
    }
}
