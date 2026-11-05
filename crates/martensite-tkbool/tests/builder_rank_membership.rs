//! Shape ranks preserve first-input precedence and stable duplicate filtering.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbrep::{
    topexp,
    topods::{self, Shape},
};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::Orientation;

fn rank_builder(first: &Shape, second: &Shape) -> Result<TopOpeBRepBuildBuilder, Failure> {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    builder.map_shapes(first, second)?;
    Ok(builder)
}

fn shared_vertex(first: &Shape, second: &Shape) -> Result<Shape, Failure> {
    let (_, shared) = topexp::vertices(&topods::edge(first)?, false);
    let (next_start, _) = topexp::vertices(&topods::edge(second)?, false);
    assert!(shared.is_same(&next_start));
    Ok(shared.into_shape())
}

#[test]
fn mapped_ranks_prefer_the_first_shape_for_shared_topology() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let shared = shared_vertex(&first, &second)?;
    let mut builder = rank_builder(&first, &second)?;
    assert_eq!(builder.shape_rank(&first), 1);
    assert_eq!(builder.shape_rank(&second), 2);
    assert_eq!(builder.shape_rank(&shared), 1);
    assert_eq!(builder.shape_rank(&first.reversed()), 1);
    builder.clear_maps();
    assert_eq!(builder.shape_rank(&first), 0);
    assert_eq!(builder.shape_rank(&second), 0);
    builder.map_shapes(&Shape::default(), &first)?;
    assert_eq!(builder.shape_rank(&first), 2);
    assert_eq!(builder.shape_rank(&second), 0);
    Ok(())
}

fn rank_filter_input(first: &Shape, second: &Shape, shared: &Shape) -> List<Shape> {
    List::from_iter([
        second.clone(),
        shared.clone(),
        first.clone(),
        second.reversed(),
        shared.reversed(),
    ])
}

fn assert_first_rank_result(result: &List<Shape>, first: &Shape, shared: &Shape) {
    let parts: Vec<&Shape> = result.iter().collect();
    assert_eq!(parts.len(), 2);
    assert!(parts[0].is_same(first));
    assert_eq!(parts[0].orientation(), Orientation::Reversed);
    assert!(parts[1].is_same(shared));
}

#[test]
fn same_rank_filters_preserve_existing_orientation_and_append_matches_once() -> Result<(), Failure>
{
    let [first, second] = connected_edges()?;
    let shared = shared_vertex(&first, &second)?;
    let builder = rank_builder(&first, &second)?;
    let input = rank_filter_input(&first, &second, &shared);
    let mut rank1 = List::from_iter([first.reversed()]);
    builder.find_same_rank(&input, 1, &mut rank1);
    assert_first_rank_result(&rank1, &first, &shared);
    let mut rank2 = List::default();
    builder.find_same_rank(&input, 2, &mut rank2);
    assert_eq!(rank2.len(), 1);
    assert!(rank2.iter().next().expect("one edge").is_same(&second));
    let mut grid_rank2 = List::default();
    builder.g_find_same_rank(&input, 2, &mut grid_rank2);
    assert_eq!(grid_rank2.len(), 1);
    assert!(grid_rank2.iter().next().expect("one edge").is_same(&second));
    Ok(())
}

#[path = "builder_rank_membership/maps.rs"]
mod maps;
