//! Binding presence controls real SplitShapes filtering, including stored false values.
use super::{TopOpeBRepDSHDataStructure, collected_parts, connected_edges, prepared_split_builder};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::Orientation;

fn assert_consumed_case(
    edge: &Shape,
    consumed: bool,
    orientation: Orientation,
) -> Result<(), Failure> {
    let source = edge.oriented(orientation);
    let mut builder = prepared_split_builder(&source)?;
    builder
        .change_consumed_face_edge_membership()
        .bind(source.reversed(), consumed);
    for reverse in [false, true] {
        assert!(collected_parts(&mut builder, &source, reverse)?.is_empty());
    }
    assert_eq!(
        *builder.consumed_face_edge_membership().find(&source)?,
        consumed
    );
    Ok(())
}

#[test]
fn both_boolean_payloads_suppress_parts_across_source_and_result_orientations()
-> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    for consumed in [false, true] {
        for orientation in [Orientation::Forward, Orientation::Reversed] {
            assert_consumed_case(&edge, consumed, orientation)?;
        }
    }
    Ok(())
}

#[test]
fn changing_boolean_payload_does_not_unconsume_an_edge_but_unbinding_does() -> Result<(), Failure> {
    let [source, _] = connected_edges()?;
    let mut builder = prepared_split_builder(&source)?;
    builder
        .change_consumed_face_edge_membership()
        .bind(source.clone(), false);
    let shared = std::ptr::from_ref(builder.consumed_face_edge_membership());
    let mutable = std::ptr::from_mut(builder.change_consumed_face_edge_membership());
    assert_eq!(shared, mutable.cast_const());
    assert!(collected_parts(&mut builder, &source, false)?.is_empty());
    *builder
        .change_consumed_face_edge_membership()
        .change_find(&source)? = true;
    assert!(collected_parts(&mut builder, &source, false)?.is_empty());
    assert!(
        builder
            .change_consumed_face_edge_membership()
            .unbind(&source.reversed())
    );
    assert_eq!(collected_parts(&mut builder, &source, false)?.len(), 1);
    Ok(())
}

#[test]
fn replacing_the_whole_membership_map_restores_only_the_unbound_source() -> Result<(), Failure> {
    let [source, other] = connected_edges()?;
    let mut builder = prepared_split_builder(&source)?;
    builder
        .change_consumed_face_edge_membership()
        .bind(source.clone(), false);
    assert!(collected_parts(&mut builder, &source, false)?.is_empty());
    let mut replacement = DataMap::default();
    replacement.bind(other, false);
    *builder.change_consumed_face_edge_membership() = replacement;
    assert_eq!(collected_parts(&mut builder, &source, false)?.len(), 1);
    assert_eq!(builder.consumed_face_edge_membership().len(), 1);
    Ok(())
}

#[test]
fn normal_preparation_clears_the_same_consumed_edge_membership_map() -> Result<(), Failure> {
    let [source, _] = connected_edges()?;
    let mut builder = prepared_split_builder(&source)?;
    builder
        .change_consumed_face_edge_membership()
        .bind(source, false);
    let before = std::ptr::from_ref(builder.consumed_face_edge_membership());
    builder.perform(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    assert_eq!(
        before,
        std::ptr::from_ref(builder.consumed_face_edge_membership())
    );
    assert!(builder.consumed_face_edge_membership().is_empty());
    Ok(())
}
