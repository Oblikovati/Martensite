//! Public overlapping operations and shared readers retain the same owned evidence.
use super::*;
use std::sync::Barrier;
use std::thread::{ScopedJoinHandle, scope};

#[derive(Debug, PartialEq, Eq)]
struct PublicationSnapshot {
    native: Vec<u8>,
    public: String,
    receipts: String,
}
struct ConcurrentReading {
    snapshot: PublicationSnapshot,
    detached: DetachedWrappedBodies,
}
#[test]
fn overlapping_public_cone_operations_and_shared_readers_preserve_owned_evidence() {
    let mut authored = request();
    let original = authored.clone();
    let sequential = check_completed(authored.clone());
    let expected = snapshot(&sequential);
    let concurrent = overlapping_operations(&authored);
    check_workers(&concurrent, &original, &expected);
    assert_eq!(authored, original);
    check_readers(&sequential, &expected);
    check_export_isolation(&sequential, &mut authored, &original);
    check_workers(&concurrent, &original, &expected);
    assert_eq!(snapshot(&sequential), expected);
}
fn overlapping_operations(request: &WrappedConeNormalRequest) -> [WrappedConeNormalResult; 2] {
    let start = Barrier::new(2);
    scope(|workers| {
        let tasks = [0, 1].map(|_| workers.spawn(|| public_worker(request, &start)));
        tasks.map(join)
    })
}
fn public_worker(request: &WrappedConeNormalRequest, start: &Barrier) -> WrappedConeNormalResult {
    start.wait();
    crate::surface_sketch::normal_extrude_cone_profile(request)
        .unwrap_or_else(|cause| panic!("public cone worker refused: {cause:?}"))
}
fn join<T>(task: ScopedJoinHandle<'_, T>) -> T {
    task.join()
        .unwrap_or_else(|cause| std::panic::resume_unwind(cause))
}
fn check_workers(
    results: &[WrappedConeNormalResult; 2],
    request: &WrappedConeNormalRequest,
    expected: &PublicationSnapshot,
) {
    for result in results {
        publication::assert_publication(result, request);
        check_measure(result);
        check_coverage(result);
        assert_eq!(&snapshot(result), expected);
    }
}
fn check_readers(result: &WrappedConeNormalResult, expected: &PublicationSnapshot) {
    let sequential = result.export_native().unwrap();
    let baseline = detached_bytes(&sequential);
    let [first, second] = overlapping_readers(result);
    for reading in [&first, &second] {
        assert_eq!(&reading.snapshot, expected);
        assert_eq!(detached_bytes(&reading.detached), baseline);
        check_disjoint_exports(&sequential, &reading.detached);
    }
    check_disjoint_exports(&first.detached, &second.detached);
    assert_eq!(&snapshot(result), expected);
}
fn overlapping_readers(result: &WrappedConeNormalResult) -> [ConcurrentReading; 2] {
    let start = Barrier::new(2);
    scope(|workers| {
        let tasks = [0, 1].map(|_| workers.spawn(|| read_and_export(result, &start)));
        tasks.map(join)
    })
}
fn read_and_export(result: &WrappedConeNormalResult, start: &Barrier) -> ConcurrentReading {
    start.wait();
    let before = snapshot(result);
    let detached = result.export_native().unwrap();
    normals::check(result, &detached);
    occurrences::check(result, &detached);
    assert_eq!(snapshot(result), before);
    ConcurrentReading {
        snapshot: before,
        detached,
    }
}
fn check_disjoint_exports(first: &DetachedWrappedBodies, second: &DetachedWrappedBodies) {
    assert!(!first.bodies.is_partner(&second.bodies));
    assert_eq!(
        first.entities.keys().collect::<Vec<_>>(),
        second.entities.keys().collect::<Vec<_>>()
    );
    assert!(first.entities.values().all(|shape| {
        second
            .entities
            .values()
            .all(|other| !shape.is_partner(other))
    }));
}
fn detached_bytes(detached: &DetachedWrappedBodies) -> Vec<u8> {
    let bindings = detached
        .entities
        .iter()
        .map(|(key, shape)| (*key, shape.clone()))
        .collect::<Vec<_>>();
    crate::surface_sketch::encoding::encode(&detached.bodies, &bindings).unwrap()
}
fn snapshot(result: &WrappedConeNormalResult) -> PublicationSnapshot {
    PublicationSnapshot {
        native: result.encoded_brep().to_vec(),
        public: format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
            result.history(),
            result.budget(),
            result.reports(),
            result.support(),
            result.depths(),
            result.options(),
            result.proof_options(),
            result.boundary_flux_origin()
        ),
        receipts: receipt_snapshot(result),
    }
}
fn receipt_snapshot(result: &WrappedConeNormalResult) -> String {
    let mut receipts = format!("{:?}", result.reference_body_receipt());
    for region in result.reports().keys() {
        writeln!(
            receipts,
            "{region:?}|{:?}|{:?}|{}",
            result.flux_receipt(*region).unwrap(),
            result.reference_volume_receipt(*region).unwrap(),
            authored_snapshot(result, *region)
        )
        .unwrap();
    }
    receipts
}
fn authored_snapshot(result: &WrappedConeNormalResult, region: SurfaceRegionId) -> String {
    let authored = result.authored_receipt(region).unwrap();
    let clearances = (0..)
        .map_while(|index| authored.clearance(index))
        .collect::<Vec<_>>();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{}|{}|{clearances:?}",
        authored.area(),
        authored.inverse_slant(),
        authored.volume(),
        authored.difference(),
        authored.volume_operations(),
        authored.operations()
    )
}
