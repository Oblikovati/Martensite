//! Analytic references retain source-specific projection while certifying C1 interpolation separately.
use super::specification::Tokens;
use super::*;
struct AnalyticReference {
    axis: usize,
    piece: usize,
    envelope: f64,
    ideal_bound: f64,
    midpoint: f64,
    vectors: [Vec<f64>; 5],
}
fn references(spec: &CaseSpec) -> Vec<AnalyticReference> {
    let source = include_str!("../../fixtures/ordinary_curve/independent.txt");
    source
        .lines()
        .filter(|line| line.split_whitespace().next() == Some(&spec.name))
        .map(parse_reference)
        .collect()
}
fn parse_reference(line: &str) -> AnalyticReference {
    let mut r = Tokens::new(line);
    let _name = r.text();
    let (axis, piece, envelope, ideal_bound, midpoint) =
        (r.size(), r.size(), r.real(), r.real(), r.real());
    let vectors = std::array::from_fn(|_| (0..r.size()).map(|_| r.real()).collect());
    AnalyticReference {
        axis,
        piece,
        envelope,
        ideal_bound,
        midpoint,
        vectors,
    }
}
fn slot(spec: &CaseSpec, reference: &AnalyticReference, table: usize, index: usize) -> usize {
    let pair = reference.piece * 2 + reference.axis;
    match table {
        0 => PREFIX + pair * spec.limit + index,
        1 | 2 => PREFIX + pair * (spec.roots / 2 + 1) + index,
        _ => PREFIX + reference.piece * 2 * (spec.order + 2) as usize + reference.axis + 2 * index,
    }
}
fn compare_reference(spec: &CaseSpec, reference: &AnalyticReference, tables: &[Vec<f64>; 8]) {
    for (table, expected) in reference.vectors.iter().enumerate() {
        for (index, value) in expected.iter().enumerate() {
            let actual = tables[table][slot(spec, reference, table, index)];
            assert!(
                (actual - value).abs() <= reference.envelope,
                "{} independent table={table}, axis={}, piece={}, index={index}, actual={actual:.17e}, expected={value:.17e}, bound={:.17e}",
                spec.name,
                reference.axis,
                reference.piece,
                reference.envelope
            );
        }
    }
}
fn coefficient_error(
    spec: &CaseSpec,
    reference: &AnalyticReference,
    tables: &[Vec<f64>; 8],
) -> f64 {
    let (mut difference, mut magnitude) = (0., 0.);
    for (index, expected) in reference.vectors[0].iter().enumerate() {
        difference += (tables[0][slot(spec, reference, 0, index)] - expected).abs();
        magnitude += expected.abs();
    }
    // Four rounded reference coefficients and their summed differences need a rounding allowance.
    difference + 8. * f64::EPSILON * (1. + magnitude)
}
fn certify_interpolant(spec: &CaseSpec, reference: &AnalyticReference, tables: &[Vec<f64>; 8]) {
    let coefficient_bound = coefficient_error(spec, reference, tables);
    let total_bound = reference.ideal_bound.next_up() + coefficient_bound;
    if spec.name.starts_with("CE_") && spec.maximum == 32 {
        assert!(
            total_bound <= spec.precision,
            "{} piece={} independent bound={total_bound:.17e}",
            spec.name,
            reference.piece
        );
    }
}
pub(super) fn check(spec: &CaseSpec, tables: &[Vec<f64>; 8]) {
    let references = references(spec);
    assert!(!references.is_empty());
    let mut failure_witness = false;
    for reference in references {
        compare_reference(spec, &reference, tables);
        if spec.kind == 1 {
            certify_interpolant(spec, &reference, tables);
            let center = tables[0][slot(spec, &reference, 0, 0)];
            failure_witness |= (center - reference.midpoint).abs()
                - coefficient_error(spec, &reference, tables)
                > spec.precision;
        }
    }
    if spec.kind == 1 && spec.maximum == 1 {
        assert!(
            failure_witness,
            "{} independent midpoint failure",
            spec.name
        );
    }
}
