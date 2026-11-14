//! Frozen family checks retain authored identities through one shared table.
use super::super::tests::{contour, request};
use super::*;
pub(super) fn requests() -> Vec<Request> {
    let mut second = request();
    second.region = SurfaceRegionId(100);
    second.loops = vec![contour(
        1000,
        &[
            &[[4., 0.], [5., 0.]],
            &[[5., 0.], [5., 1.]],
            &[[5., 1.], [4., 1.]],
            &[[4., 1.], [4., 0.]],
        ],
    )];
    vec![request(), second]
}
#[test]
fn disconnected_members_share_one_frozen_table_without_publication() {
    let result = construct(&requests()).unwrap();
    assert_eq!(result.coverage.members, 2);
    assert_eq!(result.coverage.bindings, result.keys.len());
    assert_eq!(result.members.len(), 2);
    assert_eq!(result.history.records.len(), result.keys.len());
    assert_eq!(result.selection.prepared.planar.members.len(), 2);
    check_measures(&result);
    let (root, bindings) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    assert_eq!(
        crate::surface_sketch::encoding::encode(&root, &bindings).unwrap(),
        result.bytes
    );
    for member in result.members.values() {
        assert!(!member.storage.is_empty());
        assert!(!member.cones.pieces.is_empty());
    }
    fn send_sync<T: Send + Sync>() {}
    send_sync::<UnqualifiedFamily>();
}
#[test]
fn selection_order_does_not_change_frozen_bytes_or_authored_keys() {
    let mut inputs = requests();
    let first = construct(&inputs).unwrap();
    inputs.reverse();
    let second = construct(&inputs).unwrap();
    assert_eq!(first.keys, second.keys);
    assert_eq!(first.bytes, second.bytes);
    assert_eq!(first.history, second.history);
}
#[test]
fn missing_or_reversed_family_member_is_refused() {
    use martensite_tkbrep::topods::{Builder, ShapeIterator};
    use martensite_tkg3d::top_abs::Orientation;
    let prepared = selection::prepare(&requests()).unwrap();
    let (_, _, mut root, bindings) = freeze::construct(&prepared.prepared).unwrap();
    let member = ShapeIterator::new(&root, true, true)
        .value()
        .unwrap()
        .clone();
    root.set_free(true).unwrap();
    Builder.remove(&mut root, &member).unwrap();
    assert!(coverage::certify_family(&prepared.prepared, &root, &bindings).is_err());
    Builder
        .add(&mut root, &member.oriented(Orientation::Reversed))
        .unwrap();
    assert!(coverage::certify_family(&prepared.prepared, &root, &bindings).is_err());
}

fn frozen_fixture() -> (Vec<Request>, Shape, Vec<(BindingKey, Shape)>) {
    let prepared = selection::prepare(&requests()).unwrap().prepared;
    let (_, _, root, bindings) = freeze::construct(&prepared).unwrap();
    (prepared, root, bindings)
}
#[test]
fn duplicate_unbound_and_noncompound_family_roots_are_refused() {
    use martensite_tkbrep::topods::{Builder, ShapeIterator};
    let (prepared, mut root, bindings) = frozen_fixture();
    let member = ShapeIterator::new(&root, true, true)
        .value()
        .unwrap()
        .clone();
    assert!(coverage::certify_family(&prepared, &member, &bindings).is_err());
    root.set_free(true).unwrap();
    Builder.add(&mut root, &member).unwrap();
    assert!(coverage::certify_family(&prepared, &root, &bindings).is_err());
    Builder.remove(&mut root, &member).unwrap();
    let (_, other_root, _) = frozen_fixture();
    let other = ShapeIterator::new(&other_root, true, true)
        .value()
        .unwrap()
        .clone();
    Builder.add(&mut root, &other).unwrap();
    assert!(coverage::certify_family(&prepared, &root, &bindings).is_err());
}
#[test]
fn aliased_cross_member_bindings_and_duplicate_owners_are_refused() {
    let (mut prepared, root, mut bindings) = frozen_fixture();
    alias_solids(&mut bindings);
    assert!(matches!(
        coverage::certify_family(&prepared, &root, &bindings),
        Err(coverage::CoverageFailure {
            cause: coverage::CoverageCause::AliasedBindings { .. },
            ..
        })
    ));
    let (_, root, bindings) = frozen_fixture();
    prepared.push(prepared[0].clone());
    assert!(coverage::certify_family(&prepared, &root, &bindings).is_err());
}
#[test]
fn retained_global_binding_requires_every_member_and_exact_o2_sources() {
    let originals = requests();
    let selection = selection::certify(&originals, Default::default()).unwrap();
    let prepared = &selection.prepared.prepared;
    let (_, _, root, bindings) = freeze::construct(prepared).unwrap();
    let (_, views) = coverage::certify_family(prepared, &root, &bindings).unwrap();
    let mut stored = store(&originals, prepared, views).unwrap_or_else(|e| panic!("{e}"));
    bind(&originals, &selection, &stored).unwrap();
    let removed = stored.remove(&SurfaceRegionId(100)).unwrap();
    assert!(matches!(
        bind(&originals, &selection, &stored),
        Err(FamilyFailure::Binding(_))
    ));
    stored.insert(SurfaceRegionId(100), removed);
    exchange_profiles(&mut stored);
    assert!(matches!(
        bind(&originals, &selection, &stored),
        Err(FamilyFailure::Binding(_))
    ));
}
#[test]
fn decoded_export_mutation_does_not_change_frozen_family() {
    use martensite_tkbrep::topods::{Builder, ShapeIterator};
    let result = construct(&requests()).unwrap();
    let (mut exported, _) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    let member = ShapeIterator::new(&exported, true, true)
        .value()
        .unwrap()
        .clone();
    exported.set_free(true).unwrap();
    Builder.remove(&mut exported, &member).unwrap();
    let (root, bindings) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    let prepared = &result.selection.prepared.prepared;
    assert!(coverage::certify_family(prepared, &exported, &bindings).is_err());
    assert!(coverage::certify_family(prepared, &root, &bindings).is_ok());
    assert_eq!(
        crate::surface_sketch::encoding::encode(&root, &bindings).unwrap(),
        result.bytes
    );
}

fn check_measures(result: &UnqualifiedFamily) {
    for (region, expected) in [(5, 11.10375), (100, 3.15)] {
        let member = &result.members[&SurfaceRegionId(region)];
        let comparison = &member.measures.comparison;
        let [lo, hi] = comparison.normal_certificate().authored_volume_enclosure();
        assert!(lo <= expected && expected <= hi);
        assert!(comparison.discrepancy_bound() < 1e-4);
        assert!(comparison.representative_error_bound() < 1e-4);
        assert!(member.volume_estimate.is_finite() && member.volume_estimate > 0.);
    }
}

fn alias_solids(bindings: &mut [(BindingKey, Shape)]) {
    let first = bindings
        .iter()
        .position(|(k, _)| k.role == crate::surface_sketch::HistoryRole::Solid)
        .unwrap();
    let last = bindings
        .iter()
        .rposition(|(k, _)| k.role == crate::surface_sketch::HistoryRole::Solid)
        .unwrap();
    bindings[last].1 = bindings[first].1.clone();
}

fn exchange_profiles(stored: &mut BTreeMap<SurfaceRegionId, pipeline::StoredMember>) {
    let proofs = &mut stored.get_mut(&SurfaceRegionId(5)).unwrap().storage;
    let (first, rest) = proofs.split_at_mut(1);
    std::mem::swap(&mut first[0].profile, &mut rest[0].profile);
}
#[test]
fn an_island_in_an_unselected_hole_freezes_as_a_separate_member() {
    let mut inputs = requests();
    for pole in inputs[1]
        .loops
        .iter_mut()
        .flatten()
        .flat_map(|p| &mut p.poles)
    {
        pole[0] = 0.625 + (pole[0] - 4.) * 0.25;
        pole[1] = 0.625 + pole[1] * 0.25;
    }
    let family = construct(&inputs).unwrap();
    assert_eq!(family.coverage.members, 2);
    let measure = family.members[&SurfaceRegionId(100)]
        .measures
        .comparison
        .normal_certificate();
    let [lo, hi] = measure.authored_volume_enclosure();
    assert!(lo <= 0.196875 && 0.196875 <= hi);
}
#[test]
fn a_family_crossing_a_full_turn_keeps_global_periodic_evidence() {
    let mut inputs = requests();
    for pole in inputs[1]
        .loops
        .iter_mut()
        .flatten()
        .flat_map(|p| &mut p.poles)
    {
        pole[0] += 62.;
    }
    let family = construct(&inputs).unwrap();
    let boundary = &family.selection.prepared.boundary_clearance;
    assert_eq!(boundary.periodic.images().len(), 1);
    assert_eq!(family.members.len(), 2);
}
