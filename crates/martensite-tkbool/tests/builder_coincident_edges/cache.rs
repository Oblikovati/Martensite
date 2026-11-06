//! The normal edge builder consumes externally supplied live coincident-edge state.
use super::{
    TopOpeBRepBuildBuilder, TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure, assert_copied_line,
    connected_edges, curves_using, result_edge,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::Orientation;

struct PreparedCacheCase {
    source: Shape,
    structure: Handle<TopOpeBRepDSHDataStructure>,
    cached: Shape,
    builder: TopOpeBRepBuildBuilder,
}

impl PreparedCacheCase {
    fn new() -> Result<Self, Failure> {
        let [source, _] = connected_edges()?;
        let structure = curves_using(&source)?;
        let mut producer = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
        producer.build_edges_of(1, &structure)?;
        let cached = result_edge(&producer, 1).reversed();
        let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
        builder
            .change_coincident_edge_cache()
            .bind(source.clone(), cached.clone());
        Ok(Self {
            source,
            structure,
            cached,
            builder,
        })
    }

    fn build(&mut self, index: i32) -> Result<Shape, Failure> {
        self.builder.build_edges_of(index, &self.structure)?;
        Ok(result_edge(&self.builder, index))
    }
}

#[test]
fn indexed_edge_building_reuses_a_seeded_cache_and_preserves_its_orientation() -> Result<(), Failure>
{
    let mut case = PreparedCacheCase::new()?;
    let shared = std::ptr::from_ref(case.builder.coincident_edge_cache());
    let mutable = std::ptr::from_mut(case.builder.change_coincident_edge_cache());
    assert_eq!(shared, mutable.cast_const());
    let first = case.build(1)?;
    assert!(first.is_same(&case.cached));
    assert!(case.build(2)?.is_same(&case.cached));
    assert_copied_line(&case.source, &first)?;
    let cached = case
        .builder
        .coincident_edge_cache()
        .find(&case.source.reversed())?;
    assert_eq!(cached.orientation(), Orientation::Reversed);
    assert_eq!(case.cached.orientation(), Orientation::Reversed);
    assert_eq!(case.builder.coincident_edge_cache().len(), 1);
    Ok(())
}

#[test]
fn removing_or_replacing_the_cache_changes_future_copies_without_rewriting_results()
-> Result<(), Failure> {
    let mut case = PreparedCacheCase::new()?;
    let first = case.build(1)?;
    assert!(
        case.builder
            .change_coincident_edge_cache()
            .unbind(&case.source.reversed())
    );
    let second = case.build(2)?;
    assert!(!second.is_same(&first));
    *case.builder.change_coincident_edge_cache() = DataMap::default();
    let third = case.build(3)?;
    assert!(!third.is_same(&first) && !third.is_same(&second));
    assert!(result_edge(&case.builder, 1).is_same(&first));
    assert!(result_edge(&case.builder, 2).is_same(&second));
    assert_copied_line(&case.source, &second)?;
    assert_copied_line(&case.source, &third)?;
    Ok(())
}

#[test]
fn full_edge_rebuild_clears_the_same_cache_and_future_builds_repopulate_it() -> Result<(), Failure>
{
    let mut case = PreparedCacheCase::new()?;
    let previous = case.build(1)?;
    let address = std::ptr::from_ref(case.builder.coincident_edge_cache());
    case.builder
        .build_edges(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    assert_eq!(
        address,
        std::ptr::from_ref(case.builder.coincident_edge_cache())
    );
    assert!(case.builder.coincident_edge_cache().is_empty());
    assert!(case.builder.new_edges(1).is_empty());
    let rebuilt = case.build(1)?;
    assert!(!rebuilt.is_same(&previous));
    assert_eq!(case.builder.coincident_edge_cache().len(), 1);
    assert_copied_line(&case.source, &rebuilt)?;
    assert_copied_line(&case.source, &previous)?;
    Ok(())
}
