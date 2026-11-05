//! Preparation retains indexed state and expands point bounds without resetting them.
#[path = "preparation_state_contracts/support.rs"]
mod preparation_support;
use preparation_support::*;

#[test]
fn vertex_gap_matrix_uses_half_the_larger_requested_or_confusion_value() {
    let confusion = martensite_tkernel::precision::confusion();
    for fuzz in [-1., -0., 0., confusion / 2., confusion, 1. / 16.] {
        for tolerance in [1. / 128., 1. / 32., 1. / 8.] {
            check_single_vertex_gap(fuzz, tolerance, confusion);
        }
    }
}

#[test]
fn duplicate_oriented_arguments_share_indices_and_skip_ranges() {
    let (first, second, reversed, arguments) = duplicate_arguments();
    let structure = initialized(&arguments, 1. / 16.);
    assert_eq!(structure.arguments().len(), 4);
    assert_eq!((structure.nb_shapes(), structure.nb_ranges()), (2, 2));
    assert_indices(&structure, &[&first, &reversed, &second], &[0, 0, 1]);
    assert_eq!(structure.range(0).unwrap().indices(), (0, 0));
    assert_eq!(structure.range(1).unwrap().indices(), (1, 1));
    assert_forward_root(&structure);
}

#[test]
fn later_initialization_retains_prior_ranges_and_overlaps_the_new_first_range() {
    let first = vertex_at(1., 1. / 128.).into_shape();
    let second = vertex_at(4., 1. / 128.).into_shape();
    let mut structure = initialized(std::slice::from_ref(&first), 1. / 16.);
    structure.set_arguments(&[first, second.clone()]);
    structure.init(1. / 16.).unwrap();
    assert_population(&structure, [2, 2, 2]);
    assert_eq!(structure.range(0).unwrap().indices(), (0, 0));
    assert_eq!(structure.range(1).unwrap().indices(), (0, 1));
    assert_eq!((structure.rank(0), structure.rank(1)), (0, 1));
    structure.init(1. / 16.).unwrap();
    assert_eq!((structure.nb_shapes(), structure.nb_ranges()), (2, 2));
}

#[test]
fn point_mutation_expands_existing_raw_bounds_and_empty_arguments_leave_state_untouched() {
    let vertex = vertex_at(1., 1. / 128.);
    let mut structure = initialized(&[vertex.clone().into_shape()], 1. / 16.);
    move_vertex(&vertex);
    structure.init(1. / 8.).unwrap();
    let gap = 5. / 64.;
    let expected = [1. - gap, 2. - gap, -3. - gap, 4. + gap, 2. + gap, -3. + gap];
    assert_vertex_bounds(&structure, expected);
    structure.add_interference(7, 9);
    structure.set_arguments(&[]);
    structure.init(100.).unwrap();
    assert_population(&structure, [1, 1, 1]);
    assert_vertex_bounds(&structure, expected);
    assert!(structure.has_interference_between(7, 9));
}

#[test]
fn shared_degenerated_endpoints_have_unique_incidence_across_repeated_initialization() {
    let vertex = vertex_at(1., 1. / 128.);
    let first = degenerated_edge(&vertex).into_shape();
    let second = degenerated_edge(&vertex).into_shape();
    let mut structure = initialized(&[first.clone(), second.clone(), first], 1. / 16.);
    for _ in 0..3 {
        assert_population(&structure, [3, 3, 2]);
        assert_eq!(structure.index(&second), 2);
        assert_eq!(structure.edges_at_vertex(1), [0, 2]);
        assert_degenerated_children(&structure);
        assert_eq!(structure.range(0).unwrap().indices(), (0, 1));
        assert_eq!(structure.range(1).unwrap().indices(), (2, 2));
        structure.init(1. / 16.).unwrap();
    }
}
