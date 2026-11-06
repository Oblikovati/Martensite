//! Split collection preserves stored parts while applying consumed-edge membership.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildBuilder, TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet,
};
use martensite_tkbool::top_ope_brep_ds::{TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure};
use martensite_tkbool::top_ope_brep_tool::TopOpeBRepToolShapeExplorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

fn prepared_split_builder(source: &Shape) -> Result<TopOpeBRepBuildBuilder, Failure> {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    builder.perform(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    builder
        .change_split(source, State::Out)
        .append(source.clone());
    builder.mark_split(source, State::Out, true);
    Ok(builder)
}

fn collected_parts(
    builder: &mut TopOpeBRepBuildBuilder,
    source: &Shape,
    reverse: bool,
) -> Result<Vec<Shape>, Failure> {
    let mut explorer = TopOpeBRepToolShapeExplorer::new(source, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut result = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    builder.split_shapes(&mut explorer, State::Out, State::In, &mut result, reverse)?;
    assert!(!explorer.more());
    assert!(builder.is_split(source, State::Out));
    assert_eq!(builder.splits(source, State::Out).len(), 1);
    Ok(result.start_elements().iter().cloned().collect())
}

fn expected_orientation(source: Orientation, reverse: bool) -> Orientation {
    if !reverse {
        return source;
    }
    match source {
        Orientation::Forward => Orientation::Reversed,
        Orientation::Reversed => Orientation::Forward,
        other => panic!("orientation {other:?}; expected Forward or Reversed fixture"),
    }
}

#[test]
fn unconsumed_split_parts_keep_identity_and_apply_requested_orientation() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    for orientation in [Orientation::Forward, Orientation::Reversed] {
        let source = edge.oriented(orientation);
        let mut builder = prepared_split_builder(&source)?;
        for reverse in [false, true] {
            let parts = collected_parts(&mut builder, &source, reverse)?;
            assert_eq!(parts.len(), 1);
            assert!(parts[0].is_same(&source));
            assert_eq!(
                parts[0].orientation(),
                expected_orientation(orientation, reverse)
            );
            assert_eq!(source.orientation(), orientation);
            assert_eq!(
                builder
                    .splits(&source, State::Out)
                    .iter()
                    .next()
                    .expect("one part")
                    .orientation(),
                orientation
            );
        }
    }
    Ok(())
}

#[path = "builder_consumed_edges/membership.rs"]
mod membership;
