// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::conical_profile::sector::source_bits;

#[test]
fn one_coefficient_index_preserves_raw_membership_and_numeric_coverage() {
    let positive_zero = [[4., 0.], [5., 1.]];
    let negative_zero = [[4., -0.], [5., 1.]];
    let other = [[5., 1.], [6., 0.]];
    let source = source_bits(&[&positive_zero, &other, &negative_zero, &other]);
    let index = OriginalSourceIndex::from_sources(&source);
    assert!(index.contains_exact(&source, &positive_zero));
    assert!(index.contains_exact(&source, &negative_zero));
    assert!(index.covers(
        &source,
        [other.as_slice(), &negative_zero, &other].into_iter()
    ));
    assert!(!index.covers(&source, [other.as_slice(), &other].into_iter()));

    let source = source_bits(&[&positive_zero, &other]);
    let index = OriginalSourceIndex::from_sources(&source);
    assert!(!index.contains_exact(&source, &negative_zero));
    assert!(index.covers(&source, [negative_zero.as_slice(), &other].into_iter()));
}

#[test]
fn large_refined_family_refuses_a_missing_source_and_a_one_ulp_change() {
    let polygons: Vec<_> = (0..4096)
        .map(|i| [[f64::from(i) + 4., 0.], [f64::from(i) + 5., 1.]])
        .collect();
    let sources: Vec<_> = polygons.iter().map(<[_; 2]>::as_slice).collect();
    let source = source_bits(&sources);
    let index = OriginalSourceIndex::from_sources(&source);
    let fitted = sources.iter().copied().rev().flat_map(|poles| [poles; 8]);
    assert!(index.covers(&source, fitted));
    assert!(!index.covers(&source, sources.iter().copied().skip(1)));

    let mut changed = polygons[0];
    changed[0][0] = changed[0][0].next_up();
    let fitted = sources.iter().copied().skip(1).chain([changed.as_slice()]);
    assert!(!index.covers(&source, fitted));
}
