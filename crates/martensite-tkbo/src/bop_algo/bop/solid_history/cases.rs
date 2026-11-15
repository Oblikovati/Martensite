use super::fixtures::*;
use crate::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;
#[test]
fn nested_original_cell_retains_self_and_other_recorded_origins() {
    let sources = [cuboid([0.; 3], [6.; 3]), cuboid([2.; 3], [1.; 3])];
    let bop = operation(&sources, Operation::Fuse, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 1);
    assert!(covered(&bop, &sources[0], &solids[0]));
    assert_eq!(selected_cells_from(&bop, &sources[1]), 1);
    assert!(covered(&bop, &sources[1], &solids[0]));
    assert!(!bop.is_deleted(&sources[1]).unwrap());
    require_coverage(&bop, &sources);
}
#[test]
fn disconnected_fuse_preserves_exact_survival_without_foreign_parents() {
    let sources = [cuboid([0.; 3], [2.; 3]), cuboid([4.; 3], [2.; 3])];
    let bop = operation(&sources, Operation::Fuse, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 2);
    for source in &sources {
        assert!(solids.iter().any(|solid| solid.is_same(source)));
        assert!(bop.modified(source).unwrap().is_empty());
        assert!(!bop.is_deleted(source).unwrap());
    }
    require_coverage(&bop, &sources);
}
#[test]
fn face_touching_fuse_reports_both_real_parents() {
    let sources = [cuboid([0.; 3], [2.; 3]), cuboid([2., 0., 0.], [2.; 3])];
    let bop = operation(&sources, Operation::Fuse, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 1);
    for source in &sources {
        assert!(covered(&bop, source, &solids[0]));
    }
    require_coverage(&bop, &sources);
}
#[test]
fn cut_keeps_only_object_solid_parent_and_all_entity_coverage() {
    let sources = [cuboid([0.; 3], [2.; 3]), cuboid([1., 0., 0.], [2.; 3])];
    let bop = operation(&sources, Operation::Cut, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 1);
    assert!(covered(&bop, &sources[0], &solids[0]));
    assert!(!covered(&bop, &sources[1], &solids[0]));
    assert!(bop.is_deleted(&sources[1]).unwrap());
    require_coverage(&bop, &sources);
}
#[test]
fn split_cut_retains_both_children_of_one_object_solid() {
    let sources = [
        cuboid([0.; 3], [4.; 3]),
        cuboid([1., -1., -1.], [2., 6., 6.]),
    ];
    let bop = operation(&sources, Operation::Cut, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 2);
    for solid in &solids {
        assert!(covered(&bop, &sources[0], solid));
        assert!(!covered(&bop, &sources[1], solid));
    }
    require_coverage(&bop, &sources);
}
#[test]
fn empty_cut_reports_deleted_sources_and_no_final_images() {
    let sources = [cuboid([0.; 3], [2.; 3]), cuboid([-1.; 3], [4.; 3])];
    let bop = operation(&sources, Operation::Cut, true, false);
    assert!(entities(bop.shape(), ShapeEnum::Solid).is_empty());
    for source in &sources {
        assert!(bop.is_deleted(source).unwrap());
        assert!(bop.modified(source).unwrap().is_empty());
    }
}
#[test]
fn ranged_unranged_and_disabled_history_preserve_identical_geometry() {
    let sources = [cuboid([0.; 3], [2.; 3]), cuboid([1., 0., 0.], [2.; 3])];
    let plain = operation(&sources, Operation::Fuse, true, false);
    let ranged = operation(&sources, Operation::Fuse, true, true);
    let disabled = operation(&sources, Operation::Fuse, false, false);
    assert_eq!(encode(plain.shape()), encode(ranged.shape()));
    assert_eq!(encode(plain.shape()), encode(disabled.shape()));
    for bop in [&plain, &ranged] {
        require_coverage(bop, &sources);
    }
}
#[test]
fn three_contributing_solids_keep_all_actual_parents() {
    let sources = [
        cuboid([0.; 3], [2.; 3]),
        cuboid([1., 0., 0.], [2.; 3]),
        cuboid([2., 0., 0.], [2.; 3]),
    ];
    let bop = operation(&sources, Operation::Fuse, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 1);
    for source in &sources {
        assert!(covered(&bop, source, &solids[0]));
    }
    require_coverage(&bop, &sources);
}

#[test]
fn rebuilt_component_does_not_inherit_disconnected_source() {
    let sources = [
        cuboid([0.; 3], [2.; 3]),
        cuboid([1., 0., 0.], [2.; 3]),
        cuboid([8.; 3], [2.; 3]),
    ];
    let bop = operation(&sources, Operation::Fuse, true, false);
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 2);
    let rebuilt = solids
        .iter()
        .find(|solid| !solid.is_same(&sources[2]))
        .unwrap();
    assert!(covered(&bop, &sources[0], rebuilt));
    assert!(covered(&bop, &sources[1], rebuilt));
    assert!(!covered(&bop, &sources[2], rebuilt));
    assert!(solids.iter().any(|solid| solid.is_same(&sources[2])));
    require_coverage(&bop, &sources);
}

#[test]
fn interior_selected_cells_keep_parent_after_all_their_faces_cancel() {
    let sources = [
        cuboid([0.; 3], [2.; 3]),
        cuboid([2., 0., 0.], [2.; 3]),
        cuboid([1., 0.5, 0.5], [2., 1., 1.]),
    ];
    let bop = operation(&sources, Operation::Fuse, true, false);
    assert!(
        selected_cells_from(&bop, &sources[2]) > 0,
        "interior fixture must contain actualselectedsourcecells"
    );
    let solids = entities(bop.shape(), ShapeEnum::Solid);
    assert_eq!(solids.len(), 1);
    for source in &sources {
        assert!(covered(&bop, source, &solids[0]));
    }
    require_coverage(&bop, &sources);
}
