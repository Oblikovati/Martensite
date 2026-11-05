//! Curve-indexed output lists remain live across extension reads and rebuilds.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::{TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};

const CURVE_KEYS: [i32; 5] = [i32::MIN, -7, 0, 1, i32::MAX];

fn stored_edges(builder: &mut TopOpeBRepBuildBuilder, key: i32, edge: &Shape) {
    assert!(builder.new_edges(key).is_empty());
    builder.change_new_edges(key).append(edge.clone());
    let shared = std::ptr::from_ref(builder.new_edges(key));
    let mutable = std::ptr::from_mut(builder.change_new_edges(key));
    assert_eq!(shared, mutable.cast_const());
    let edges = builder.new_edges(key);
    assert_eq!(edges.len(), 1);
    assert!(edges.iter().next().expect("one edge").is_same(edge));
}

#[test]
fn indexed_edge_lists_accept_signed_keys_and_return_the_stored_payload() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for key in CURVE_KEYS {
        stored_edges(&mut builder, key, &edge);
    }
    assert!(builder.new_edges(2).is_empty());
    Ok(())
}

#[test]
fn rebuilding_an_empty_structure_removes_prior_curve_edge_lists() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for key in CURVE_KEYS {
        stored_edges(&mut builder, key, &edge);
    }
    builder.build_edges(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    for key in CURVE_KEYS {
        assert!(builder.new_edges(key).is_empty());
    }
    Ok(())
}

#[path = "builder_curve_edges/map.rs"]
mod map;
