// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkbrep::{
    bintools,
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::Shape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::{geom::GeomLine, top_abs::Orientation};
use martensite_tkmath::predicates::SegmentContact3d;
use martensite_tksketch3d::{
    Edit, MinimumLength, PointRole, Snapshot,
    wire::{
        DetachedWire, DirectedLineUse, LineDirection, NativeEdgeBinding, NativePointBinding,
        NativeWireValidation, WireClosure, WireFailure, WirePhase, WireRecipe,
        WireRecipeValidation, WireSegment,
    },
};

mod e10;

fn sketch(points: &[[f64; 3]], uses: &[(u64, u64, u64)]) -> Snapshot {
    let edits = points
        .iter()
        .enumerate()
        .map(|(i, xyz)| seed(i as u64 + 1, *xyz, PointRole::Defining))
        .chain(uses.iter().map(|&(id, first, last)| line(id, first, last)))
        .collect::<Vec<_>>();
    Snapshot::empty(MinimumLength::new(1e-5).unwrap())
        .apply(&edits, || false)
        .unwrap()
}

fn forward(line: u64) -> DirectedLineUse {
    DirectedLineUse {
        line: lid(line),
        direction: LineDirection::Forward,
    }
}

fn sample_path() -> (Snapshot, [DirectedLineUse; 3]) {
    let source = sketch(
        &[[0., 0., 0.], [4., 0., 0.], [4., 4., 0.], [4., 4., 4.]],
        &[(10, 1, 2), (12, 3, 2), (11, 3, 4)],
    );
    let back = DirectedLineUse {
        line: lid(12),
        direction: LineDirection::Reversed,
    };
    (source, [forward(10), back, forward(11)])
}

fn archive(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    let options = bintools::WriteOptions {
        with_triangles: false,
        with_normals: false,
        ..Default::default()
    };
    bintools::write(shape, &mut stream, options, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; usize::try_from(stream.tellp()).unwrap()];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

#[test]
fn recipe_and_bindings_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<WireRecipe>();
    send_sync::<WireRecipeValidation>();
    send_sync::<WireSegment>();
    send_sync::<DirectedLineUse>();
    send_sync::<LineDirection>();
    send_sync::<WireClosure>();
    send_sync::<WireFailure>();
    send_sync::<WirePhase>();
    send_sync::<DetachedWire>();
    send_sync::<NativeEdgeBinding>();
    send_sync::<NativePointBinding>();
    send_sync::<NativeWireValidation>();
}

#[test]
fn ordered_open_wire_retains_reversed_occurrence_and_shared_vertices() {
    let (source, uses) = sample_path();
    let recipe = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    assert_eq!(recipe.validation().pair_checks, 3);
    assert_eq!(recipe.validation().pair_limit, 3);
    assert_eq!(recipe.closure(), WireClosure::Open);
    assert_eq!(recipe.segments()[1].endpoints(), [pid(2), pid(3)]);
    let wire = recipe.materialize(|| false).unwrap();
    assert_eq!(
        wire.edges
            .iter()
            .map(|edge| edge.source)
            .collect::<Vec<_>>(),
        uses
    );
    assert_eq!(wire.edges[1].edge.orientation(), Orientation::Reversed);
    assert_eq!(
        wire.points
            .iter()
            .map(|entry| entry.point)
            .collect::<Vec<_>>(),
        [pid(1), pid(2), pid(3), pid(4)]
    );
    assert!(!wire.wire.closed().unwrap());
    assert!(!BRepTool::is_closed(&wire.wire).unwrap());
    assert_eq!(wire.validation.stored_tolerance, 1e-7);
    assert!(wire.validation.curve_error_bound <= 2e-15);
    for edge in &wire.edges {
        let (first, last) = topexp::vertices(&edge.edge, true);
        for (vertex, point) in [first, last].iter().zip(edge.endpoints) {
            let index = usize::try_from(point.get().get()).unwrap() - 1;
            let traced = &wire.points[index];
            assert_eq!(traced.point, point);
            assert!(traced.vertex.is_same(vertex));
            assert_eq!(BRepTool::vertex_tolerance(vertex).unwrap(), 1e-7);
        }
    }
}

#[test]
fn disconnected_ids_are_refused_even_when_coordinates_match() {
    let source = sketch(
        &[[0., 0., 0.], [1., 0., 0.], [1., 0., 0.], [2., 0., 0.]],
        &[(10, 1, 2), (11, 3, 4)],
    );
    let uses = [forward(10), forward(11)];
    assert!(matches!(
        source.wire_recipe(&uses, WireClosure::Open, 1, || false),
        Err(WireFailure::DisjointUses { lines, points })
            if lines == [lid(10), lid(11)] && points == [pid(2), pid(3)]
    ));
}

#[test]
fn absent_duplicate_empty_and_wrong_closure_are_typed() {
    let (source, uses) = sample_path();
    assert!(matches!(
        source.wire_recipe(&[], WireClosure::Open, 0, || false),
        Err(WireFailure::Empty)
    ));
    assert!(matches!(
        source.wire_recipe(&[forward(123)], WireClosure::Open, 0, || false),
        Err(WireFailure::MissingLine(line)) if line == lid(123)
    ));
    assert!(matches!(
        source.wire_recipe(&[uses[0], uses[0]], WireClosure::Open, 0, || false),
        Err(WireFailure::RepeatedLine(line)) if line == lid(10)
    ));
    assert!(matches!(
        source.wire_recipe(&uses, WireClosure::Closed, 3, || false),
        Err(WireFailure::WrongClosure { expected: WireClosure::Closed, points })
            if points == [pid(1), pid(4)]
    ));
}

#[test]
fn self_contact_and_overlap_are_refused_in_wire_order() {
    let crossed = [[0., 0., 0.], [4., 4., 0.], [0., 4., 0.], [4., 0., 0.]];
    let touched = [[0., 0., 0.], [4., 4., 0.], [0., 4., 0.], [0., 0., 0.]];
    for points in [crossed, touched] {
        let source = sketch(&points, &[(10, 1, 2), (11, 2, 3), (12, 3, 4)]);
        assert!(matches!(
            source.wire_recipe(&[forward(10), forward(11), forward(12)], WireClosure::Open, 3, || false),
            Err(WireFailure::SegmentIncidence { lines, adjacent: false, contact: SegmentContact3d::Point })
                if lines == [lid(10), lid(12)]
        ));
    }
    let source = sketch(
        &[[0., 0., 0.], [4., 4., 4.], [1., 1., 1.]],
        &[(10, 1, 2), (11, 2, 3)],
    );
    assert!(matches!(
        source.wire_recipe(&[forward(10), forward(11)], WireClosure::Open, 1, || false),
        Err(WireFailure::SegmentIncidence {
            adjacent: true,
            contact: SegmentContact3d::Overlap,
            ..
        })
    ));
}

#[test]
fn skew_nonadjacent_segments_pass_exact_checks_and_work_is_bounded() {
    let source = sketch(
        &[[0., 0., 0.], [4., 4., 4.], [0., 3., 1.], [4., 1., 4.]],
        &[(10, 1, 2), (11, 2, 3), (12, 3, 4)],
    );
    let uses = [forward(10), forward(11), forward(12)];
    assert!(matches!(
        source.wire_recipe(&uses, WireClosure::Open, 2, || false),
        Err(WireFailure::PairLimit { lines, limit: 2 }) if lines == [lid(11), lid(12)]
    ));
    let wire = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    assert_eq!(wire.validation().pair_checks, 3);
    assert!(wire.materialize(|| false).is_ok());
    let one = source
        .wire_recipe(&uses[..1], WireClosure::Open, 0, || false)
        .unwrap();
    assert_eq!(one.validation().pair_checks, 0);
}

#[test]
fn cancellation_refuses_both_an_incomplete_recipe_and_a_private_native_draft() {
    let (source, uses) = sample_path();
    assert!(matches!(
        source.wire_recipe(&uses, WireClosure::Open, 3, || true),
        Err(WireFailure::Cancelled(WirePhase::Resolve))
    ));
    let before_first_pair = 1 + uses.len() + uses.len() - 1;
    assert!(matches!(
        source.wire_recipe(
            &uses,
            WireClosure::Open,
            3,
            cancel_at(before_first_pair + 1)
        ),
        Err(WireFailure::Cancelled(WirePhase::Incidence))
    ));
    let recipe = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    assert!(matches!(
        recipe.materialize(cancel_at(3)),
        Err(WireFailure::Cancelled(WirePhase::Materialize))
    ));
    assert!(matches!(
        recipe.materialize(cancel_at(uses.len() + 2)),
        Err(WireFailure::Cancelled(WirePhase::VerifyNative))
    ));
    assert_eq!(
        recipe.materialize(|| false).unwrap().edges.len(),
        uses.len()
    );
}

#[test]
fn cancellation_after_trace_stops_binding_collection_and_publication() {
    let (source, uses) = sample_path();
    let recipe = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    // One opening and one per-edge poll for materialization. The trace adds
    // its opening, per-edge and completion polls before preparing bindings.
    let after_trace = 2 * uses.len() + 3;
    let points = uses.len() + 1;
    for offset in 1..=points + 1 {
        assert!(
            matches!(
                recipe.materialize(cancel_at(after_trace + offset)),
                Err(WireFailure::Cancelled(WirePhase::VerifyNative)),
            ),
            "expected cancellation on export poll {offset}"
        );
    }
    assert_eq!(recipe.materialize(|| false).unwrap().points.len(), points);
}

fn cancel_at(target: usize) -> impl FnMut() -> bool {
    let mut polls = 0;
    move || {
        polls += 1;
        polls == target
    }
}

#[test]
fn native_export_refuses_a_carrier_bound_above_its_fixed_tolerance() {
    let source = sketch(&[[0., 0., 0.], [1e16, 1e16, 0.]], &[(10, 1, 2)]);
    let recipe = source
        .wire_recipe(&[forward(10)], WireClosure::Open, 0, || false)
        .unwrap();
    assert!(matches!(
        recipe.materialize(|| false),
        Err(WireFailure::NativeErrorBound { line, bound, limit })
            if line == lid(10) && bound > limit && limit == 1e-7
    ));
}

#[test]
fn local_and_cross_thread_exports_do_not_alias_or_follow_sketch_edits() {
    let (source, uses) = sample_path();
    let recipe = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    let (first, sibling) = std::thread::scope(|scope| {
        let sibling = scope.spawn(|| recipe.materialize(|| false).unwrap());
        (
            recipe.materialize(|| false).unwrap(),
            sibling.join().unwrap(),
        )
    });
    let original = archive(&first.wire);
    assert_eq!(archive(&sibling.wire), original);
    assert!(!first.points[0].vertex.is_same(&sibling.points[0].vertex));
    assert!(!first.edges[0].edge.is_same(&sibling.edges[0].edge));
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&first.points[0].vertex, point([1., 1., 1.]), 1e-7)
        .unwrap();
    let (carrier, _, _) = BRepTool::curve(&first.edges[0].edge).unwrap();
    let carrier = carrier.downcast_ref::<GeomLine>().unwrap();
    carrier.set_location(point([5., 5., 5.]));
    assert_ne!(archive(&first.wire), original);
    assert_eq!(archive(&sibling.wire), original);
    let edited = source
        .apply(
            &[Edit::MovePoint {
                point: pid(4),
                position: point([4., 4., 5.]),
            }],
            || false,
        )
        .unwrap();
    let updated = edited
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    assert_ne!(
        archive(&updated.materialize(|| false).unwrap().wire),
        original
    );
    assert_eq!(
        archive(&recipe.materialize(|| false).unwrap().wire),
        original
    );
}

#[test]
fn independent_processes_replay_the_same_native_wire_bytes() {
    let first = replay_archive();
    let second = replay_archive();
    assert_eq!(first, second);
}

fn replay_archive() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "wire::emit_wire_replay_archive",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .split_once("WIRE_ARCHIVE=")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_wire_replay_archive() {
    use std::fmt::Write;
    let (source, uses) = sample_path();
    let recipe = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    let bytes = archive(&recipe.materialize(|| false).unwrap().wire);
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex, "{byte:02x}").unwrap();
    }
    println!("WIRE_ARCHIVE={hex}");
}
