//! Authored directed graph expectations do not obtain expected edges from splitter output.
use super::{fixture::input_identity_records, observations::selected_receiver};
use crate::bop_algo::{ProgressCompletion, WireSplitter};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::Orientation;
use std::collections::BTreeMap;

type Incidence = (usize, Orientation);
type DirectedEdges = Vec<Incidence>;

#[test]
fn authored_regular_boundary_has_four_forward_incidences() {
    for parallel in [false, true] {
        let (mut receiver, inputs) = selected_receiver(parallel, 4);
        assert_eq!(
            receiver.perform_with_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        let rows = input_identity_records(&receiver, &inputs);
        assert_eq!(rows.len(), 1);
        assert_membership(&rows[0], &[0, 1, 2, 3], &[]);
        assert_closed_graph(&rows[0]);
    }
}

#[test]
fn authored_split_squares_have_exact_opposite_diagonal_triangles() {
    for parallel in [false, true] {
        let (mut receiver, inputs) = super::fixture::split_square_splitter(parallel);
        assert_eq!(
            receiver.perform_with_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        assert_triangles(&receiver, &inputs);
    }
}

fn assert_triangles(receiver: &WireSplitter, inputs: &[Shape]) {
    let rows = input_identity_records(receiver, inputs);
    assert_eq!(rows.len(), 4);
    for offset in [0, 6] {
        let lower = triangle_with(&rows, offset);
        let upper = triangle_with(&rows, offset + 2);
        assert_membership(lower, &[offset, offset + 1], &[offset + 4]);
        assert_membership(upper, &[offset + 2, offset + 3, offset + 4], &[]);
        assert_closed_graph(lower);
        assert_closed_graph(upper);
    }
}

fn triangle_with(rows: &[DirectedEdges], boundary: usize) -> &[Incidence] {
    let matches: Vec<&DirectedEdges> = rows
        .iter()
        .filter(|row| row.iter().any(|(edge, _)| *edge == boundary))
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "authored edge {boundary}; expected one output incidence"
    );
    assert_eq!(matches[0].len(), 3);
    matches[0]
}

fn assert_membership(actual: &[Incidence], forward: &[usize], reversed: &[usize]) {
    let mut expected: DirectedEdges = forward
        .iter()
        .map(|edge| (*edge, Orientation::Forward))
        .collect();
    expected.extend(reversed.iter().map(|edge| (*edge, Orientation::Reversed)));
    let mut actual = actual.to_vec();
    actual.sort_by_key(|(edge, _)| *edge);
    expected.sort_by_key(|(edge, _)| *edge);
    assert_eq!(actual, expected);
}

fn authored_endpoints(edge: usize) -> (usize, usize) {
    let component = edge / 6;
    let (start, end) = match edge % 6 {
        0 => (0, 1),
        1 => (1, 2),
        2 => (2, 3),
        3 => (3, 0),
        4 => (0, 2),
        other => panic!("authored edge slot {other}; expected unique identity 0..=4"),
    };
    (component * 4 + start, component * 4 + end)
}

fn assert_closed_graph(edges: &[Incidence]) {
    let mut degrees: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (edge, orientation) in edges {
        let (mut start, mut end) = authored_endpoints(*edge);
        if *orientation == Orientation::Reversed {
            std::mem::swap(&mut start, &mut end);
        }
        degrees.entry(start).or_default().1 += 1;
        degrees.entry(end).or_default().0 += 1;
    }
    assert_eq!(degrees.len(), edges.len());
    assert!(
        degrees.values().all(|degree| *degree == (1, 1)),
        "authored directed cycle {edges:?}; expected one incoming/outgoing incidence per vertex"
    );
}

#[test]
#[should_panic(expected = "expected one incoming/outgoing")]
fn reversed_boundary_breaks_the_authored_closed_cycle() {
    assert_closed_graph(&[
        (0, Orientation::Forward),
        (1, Orientation::Forward),
        (4, Orientation::Forward),
    ]);
}

#[test]
fn legacy_default_range_preserves_authored_irregular_graph() {
    let (mut receiver, inputs) = super::fixture::split_square_splitter(false);
    receiver.perform().unwrap();
    assert_triangles(&receiver, &inputs);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn duplicate_incidence_cannot_replace_an_authored_edge() {
    let repeated = [(0, Orientation::Forward), (0, Orientation::Forward)];
    assert_membership(&repeated, &[0, 1], &[]);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn reversed_diagonal_cannot_be_silently_reoriented() {
    let wrong = [
        (0, Orientation::Forward),
        (1, Orientation::Forward),
        (4, Orientation::Forward),
    ];
    assert_membership(&wrong, &[0, 1], &[4]);
}
