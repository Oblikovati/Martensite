// SPDX-License-Identifier: GPL-2.0-only
//! Actual binary64 source pairs retained before #544's structural extraction.
use super::super::RootIsolationOptions;
use super::{
    operation::compute_with_signs, sign_policy::PairSignOptions, source::OwnedBezierSource,
};
fn sources(index: usize) -> [OwnedBezierSource; 2] {
    fixtures()[index].map(|bits| {
        let controls = bits.map(|point| point.map(f64::from_bits));
        OwnedBezierSource::capture(&controls, None, [0., 1.]).unwrap()
    })
}
fn fixtures() -> [[[[u64; 2]; 4]; 2]; 3] {
    [
        [
            [
                [4603237913123563864, 4598289708100205852],
                [4603978300341532360, 4598205734477790689],
                [4604719241952117104, 4598077357117753595],
                [4605460656772548168, 4597926204597406302],
            ],
            [
                [4605460656772548168, 4597926204597406302],
                [4605226525776622568, 4596011606006340593],
                [4605068737209341896, 4594065713500000611],
                [4604989835895586328, 4590545986757817334],
            ],
        ],
        [
            [
                [4607294728714364864, 4594742132459288744],
                [4607369416199682356, 4596587889308414102],
                [4607481158285681336, 4598295177716044227],
                [4607628363044536280, 4599191434647590675],
            ],
            [
                [4607628363044536280, 4599191434647590675],
                [4607501398940023892, 4599292263552389650],
                [4607356253925869556, 4599390591653194283],
                [4607197140039268884, 4599485800461781378],
            ],
        ],
        [
            [
                [4605460656772548168, 4597926204597406302],
                [4605226525776622568, 4596011606006340593],
                [4605068737209341896, 4594065713500000611],
                [4604989835895586328, 4590545986757817334],
            ],
            [
                [4604989835895586328, 4590545986757817334],
                [4603490710934230520, 4590751977301200385],
                [4601304782701941952, 4590979294749579945],
                [4598307563971912336, 4591236427772699478],
            ],
        ],
    ]
}
#[test]
fn actual_graph_sources_pin_complete_candidates_signs_work_and_polls() {
    for index in 0..3 {
        let mut options = RootIsolationOptions::new(0.001);
        options.max_bisections = [3910, 4024, 3910][index];
        let mut polls = 0;
        let outcome = compute_with_signs(
            sources(index),
            (false, true),
            options,
            PairSignOptions::derived(),
            1e-4,
            &mut || {
                polls += 1;
                false
            },
        );
        let proof = outcome.completion.unwrap();
        let graph = proof.graph().unwrap();
        let candidate = &graph.candidate_certificate;
        assert_eq!(proof.contacts.len(), 1);
        assert_eq!(outcome.work.completed, [0, 29, 0][index]);
        assert_eq!(polls, [694, 1687, 694][index]);
        assert_eq!(
            candidate.membership.accepted,
            if index == 1 {
                vec![false, false, true]
            } else {
                vec![true]
            }
        );
        let expected = expected_signs(index);
        for (query, signs) in candidate.membership.queries.iter().zip(expected) {
            assert_eq!(query.evaluation.signs(), signs);
        }
        assert_roots(index, &candidate.isolation);
    }
}
fn expected_signs(index: usize) -> Vec<Vec<std::cmp::Ordering>> {
    use std::cmp::Ordering::{Equal as Z, Greater as P, Less as N};
    match index {
        0 => vec![vec![P], vec![Z], vec![Z], vec![N]],
        1 => vec![vec![P, N, N], vec![Z, Z, Z], vec![N, P, Z], vec![N, P, N]],
        2 => vec![vec![N], vec![Z], vec![Z], vec![N]],
        _ => panic!("fixture {index}; expected 0..3"),
    }
}
fn assert_roots(index: usize, isolation: &super::super::RootIsolation) {
    let bounds = if index == 1 {
        vec![
            [353. / 1024., 354. / 1024.],
            [814. / 1024., 815. / 1024.],
            [1., 1.],
        ]
    } else {
        vec![[1., 1.]]
    };
    assert_eq!(isolation.roots().len(), bounds.len());
    assert_eq!(isolation.bisections(), if index == 1 { 19 } else { 0 });
    for (root, expected) in isolation.roots().iter().zip(bounds) {
        assert!(root.bounds().0.compare_f64(expected[0]).unwrap().is_eq());
        assert!(root.bounds().1.compare_f64(expected[1]).unwrap().is_eq());
        assert_eq!(root.multiplicity(), 1);
    }
}
