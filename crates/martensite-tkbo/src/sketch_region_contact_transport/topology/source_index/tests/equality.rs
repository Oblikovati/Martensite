// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::gp::{Transform3, Vector3};
#[test]
fn full_source_lineage_topology_orientation_and_location_identity_are_required() {
    let original = parent(1);
    let variants = changed_parents(&original);
    for changed in variants {
        assert_distinct(&original, changed);
    }
}
fn changed_parents(original: &OriginalMemberIdentity) -> Vec<OriginalMemberIdentity> {
    let mut source = original.clone();
    source.source = RegionSourceId(2);
    let mut topology = original.clone();
    topology.topology_address = 2;
    let mut orientation = original.clone();
    orientation.orientation = Orientation::Reversed;
    let mut location = original.clone();
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(1., 2., 3.));
    location.location = Location::new(transform);
    vec![source, topology, orientation, location]
}
fn assert_distinct(original: &OriginalMemberIdentity, changed: OriginalMemberIdentity) {
    let inputs = vec![vec![original.clone()], vec![changed.clone()]];
    let sources = vec![
        source(1, vec![original.clone()]),
        source(1, vec![changed.clone()]),
    ];
    let domains = vec![(1, Some(original.clone()), true), (1, Some(changed), true)];
    let indexed = index(&inputs, &sources, &domains);
    assert_eq!(indexed.matching_source_rows(0), &[0]);
    assert_eq!(indexed.matching_source_rows(1), &[1]);
}
#[test]
fn separately_allocated_equal_transform_locations_do_not_collapse() {
    let mut first = parent(1);
    let mut second = first.clone();
    first.location = Location::new(Transform3::default());
    second.location = Location::new(Transform3::default());
    assert_ne!(first, second);
    assert_distinct(&first, second);
}
#[test]
fn domain_parent_identity_changes_cannot_admit_an_original_parent() {
    let original = parent(1);
    let inputs = vec![vec![original.clone()]];
    let sources = vec![source(1, vec![original.clone()])];
    for changed in changed_parents(&original) {
        let domains = vec![(1, Some(changed), true)];
        assert!(
            index(&inputs, &sources, &domains)
                .matching_source_rows(0)
                .is_empty()
        );
    }
}
