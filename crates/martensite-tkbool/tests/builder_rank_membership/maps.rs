//! Direct rank-map edits feed normal rank filtering and mapping lifecycle.
use super::{TopOpeBRepBuildBuilder, TopOpeBRepDSBuildTool, connected_edges, rank_builder};
use martensite_tkbrep::{topods::Shape, toptools::ShapeMapHasher};
use martensite_tkernel::{
    ncollection::{IndexedMap, List},
    standard::Failure,
};

type RankMap = IndexedMap<Shape, ShapeMapHasher>;

fn rank_map_mut(builder: &mut TopOpeBRepBuildBuilder, rank: i32) -> &mut RankMap {
    builder
        .change_shape_rank_membership(rank)
        .expect("rank 1 or 2 fixture")
}

fn rank_map_address(builder: &mut TopOpeBRepBuildBuilder, rank: i32) -> *const RankMap {
    let shared = std::ptr::from_ref(builder.shape_rank_membership(rank).expect("supported rank"));
    let mutable = std::ptr::from_mut(rank_map_mut(builder, rank));
    assert_eq!(shared, mutable.cast_const());
    shared
}

#[test]
fn rank_map_references_are_live_and_invalid_ranks_have_no_map() -> Result<(), Failure> {
    let [first, _] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    assert_ne!(
        rank_map_address(&mut builder, 1),
        rank_map_address(&mut builder, 2)
    );
    rank_map_mut(&mut builder, 1).add(first.clone());
    assert!(builder.is_shape_of(&first, 1));
    for rank in [i32::MIN, -1, 0, 3, i32::MAX] {
        assert!(builder.shape_rank_membership(rank).is_none());
        assert!(builder.change_shape_rank_membership(rank).is_none());
        assert!(!builder.is_shape_of(&first, rank));
        assert!(!builder.g_is_shape_of(&first, rank));
    }
    Ok(())
}

fn populate_overlapping_maps(builder: &mut TopOpeBRepBuildBuilder, first: &Shape, second: &Shape) {
    assert_eq!(rank_map_mut(builder, 1).add(first.clone()), 1);
    assert_eq!(rank_map_mut(builder, 1).add(first.reversed()), 1);
    assert_eq!(rank_map_mut(builder, 2).add(second.clone()), 1);
    assert_eq!(rank_map_mut(builder, 2).add(first.clone()), 2);
    assert_eq!(builder.shape_rank(first), 1);
    rank_map_mut(builder, 1).clear(true);
    assert_eq!(builder.shape_rank(first), 2);
}

fn substitute_rank_membership(
    builder: &mut TopOpeBRepBuildBuilder,
    first: &Shape,
    second: &Shape,
) -> Result<(), Failure> {
    rank_map_mut(builder, 1).add(second.clone());
    assert_eq!(builder.shape_rank(second), 1);
    rank_map_mut(builder, 1).substitute(1, first.clone())?;
    assert_eq!(builder.shape_rank(first), 1);
    assert_eq!(builder.shape_rank(second), 2);
    rank_map_mut(builder, 1).remove_last()?;
    assert_eq!(builder.shape_rank(first), 2);
    assert_eq!(builder.shape_rank(second), 2);
    let mut replacement = RankMap::default();
    replacement.add(first.clone());
    *rank_map_mut(builder, 1) = replacement;
    Ok(())
}

#[test]
fn whole_map_edits_feed_rank_precedence_and_real_same_rank_filtering() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    populate_overlapping_maps(&mut builder, &first, &second);
    substitute_rank_membership(&mut builder, &first, &second)?;
    assert_eq!(builder.shape_rank(&first), 1);
    assert_eq!(builder.shape_rank(&second), 2);
    let input = List::from_iter([first.clone(), second.clone(), first.reversed()]);
    let mut selected = List::default();
    builder.find_same_rank(&input, 2, &mut selected);
    assert_eq!(selected.len(), 1);
    assert!(selected.iter().next().expect("one match").is_same(&second));
    assert_eq!(
        builder.shape_rank_membership(2).expect("second rank").len(),
        2
    );
    Ok(())
}

fn assert_mapping_map(
    builder: &TopOpeBRepBuildBuilder,
    rank: i32,
    source: &Shape,
) -> Result<(), Failure> {
    let map = builder.shape_rank_membership(rank).expect("supported rank");
    assert_eq!(map.len(), 3);
    assert!(map.find_key(1)?.is_same(source));
    Ok(())
}

#[test]
fn normal_mapping_and_clear_reuse_both_stored_rank_map_objects() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut builder = rank_builder(&first, &second)?;
    let pointers = [
        rank_map_address(&mut builder, 1),
        rank_map_address(&mut builder, 2),
    ];
    assert_mapping_map(&builder, 1, &first)?;
    assert_mapping_map(&builder, 2, &second)?;
    builder.clear_maps();
    for rank in [1, 2] {
        assert_eq!(
            pointers[(rank - 1) as usize],
            rank_map_address(&mut builder, rank)
        );
        assert!(
            builder
                .shape_rank_membership(rank)
                .expect("supported rank")
                .is_empty()
        );
    }
    builder.map_shapes(&Shape::default(), &first)?;
    assert!(
        builder
            .shape_rank_membership(1)
            .expect("first rank")
            .is_empty()
    );
    assert_mapping_map(&builder, 2, &first)?;
    Ok(())
}

#[test]
fn direct_null_membership_remains_distinct_from_grid_null_rejection() -> Result<(), Failure> {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    let null = Shape::default();
    assert_eq!(rank_map_mut(&mut builder, 2).add(null.clone()), 1);
    assert!(builder.is_shape_of(&null, 2));
    assert_eq!(builder.shape_rank(&null), 2);
    assert!(!builder.g_is_shape_of(&null, 2));
    assert_eq!(builder.g_shape_rank(&null), 0);
    Ok(())
}
