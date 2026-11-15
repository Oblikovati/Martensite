//! Final solid ancestry must survive actual Boolean assembly (#456).
use super::{BOPAlgoBOP, Operation};
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use martensite_tkbrep::topods::Shape;
fn fixture() -> (BOPAlgoBOP, Vec<Shape>) {
    let bytes: [&[u8]; 2] = [
        include_bytes!("../builder/entry_progress/box0.brep"),
        include_bytes!("../builder/entry_progress/box1.brep"),
    ];
    let sources = bytes.map(|bytes| read_owned_progress_shape(bytes, ShapeEnum::Solid));
    let mut bop = BOPAlgoBOP::new();
    bop.set_operation(Operation::Fuse);
    bop.add_argument(&sources[0]);
    bop.add_tool(&sources[1]);
    (bop, sources.to_vec())
}
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;
#[test]
fn overlapping_box_fuse_has_both_actual_solid_parents() {
    let (mut bop, sources) = fixture();
    bop.perform();
    assert!(!bop.options().has_errors());
    let solids: Vec<_> = Explorer::new(bop.shape(), ShapeEnum::Solid, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert_eq!(solids.len(), 1);
    for parent in &sources {
        require_modified_parent(&bop, parent, &solids[0]);
    }
}

#[path = "solid_history/cases.rs"]
mod cases;
#[path = "solid_history/fixtures.rs"]
mod fixtures;

fn require_modified_parent(bop: &BOPAlgoBOP, parent: &Shape, solid: &Shape) {
    assert!(
        bop.modified(parent)
            .unwrap()
            .iter()
            .any(|child| child.is_same(solid)),
        "missing finalsolid sourceparent"
    );
    assert!(!bop.is_deleted(parent).unwrap());
    assert!(
        bop.builder()
            .origins()
            .seek(solid)
            .unwrap()
            .iter()
            .any(|origin| origin.is_same(parent))
    );
}
