use super::*;
const LINE: [[f64; 2]; 2] = [[80., -18.], [84., 24.]];

fn input<'a>(spans: &'a [&'a [[f64; 2]]]) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans,
        semi_angle: 0.125,
        chart_origin: 0.,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 1_000_000,
        },
    }
}
fn family(
    query: ConicalChartDomainQuery<'_>,
) -> Result<ConicalChartDomainEvidence, ConicalChartDomainError> {
    certify_conical_chart_domain_with_version(query, ConicalChartDomainVersion::FamilyV3, || false)
}

#[test]
fn original_capacity_and_zero_work_refusal_remain_selectable() {
    let spans = vec![LINE.as_slice(); 128];
    let proof = certify_conical_chart_domain(input(&spans), || false).unwrap();
    assert_eq!(proof.version(), ConicalChartDomainVersion::OriginalV2);
    assert_eq!(proof.span_limit(), 128);
    assert!(proof.matches(&input(&spans)));
    let spans = vec![LINE.as_slice(); 239];
    let error = certify_conical_chart_domain(input(&spans), || false)
        .err()
        .unwrap();
    assert_eq!(error.operations, 0);
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Sector(ConicalSectorFailure::SpanCount(239))
    ));
}

#[test]
fn family_authority_owns_all_239_sources_and_its_fixed_capacity() {
    let spans = vec![LINE.as_slice(); 239];
    let query = input(&spans);
    let proof = family(query).unwrap();
    assert_eq!(proof.version(), ConicalChartDomainVersion::FamilyV3);
    assert_eq!(proof.span_limit(), 4096);
    assert_eq!(proof.source_span_count(), 239);
    assert_eq!(proof.options(), query.options);
    assert!(!proof.matches(&query));
    assert!(proof.matches_with_version(&query, ConicalChartDomainVersion::FamilyV3));
}

#[test]
fn family_count_boundaries_are_admitted_before_any_proof_allocation() {
    for count in [1, 128, 129, 239, 4096] {
        let spans = vec![LINE.as_slice(); count];
        assert!(bounds::admit(&input(&spans), ConicalChartDomainVersion::FamilyV3).is_ok());
    }
    for count in [0, 4097] {
        let spans = vec![LINE.as_slice(); count];
        let error = family(input(&spans)).err().unwrap();
        assert_eq!(error.operations, 0);
        assert!(
            matches!(error.cause, ConicalChartDomainFailure::SourceCount { actual, limit: 4096 } if actual == count)
        );
        assert_eq!(
            error.cause.to_string(),
            format!("family source spans={count}; expected 1..=4096")
        );
    }
}

#[test]
fn original_and_family_share_identical_metric_trace_for_the_same_source() {
    let spans = [LINE.as_slice()];
    let old = certify_conical_chart_domain(input(&spans), || false).unwrap();
    let new = family(input(&spans)).unwrap();
    assert_eq!(old.operations(), new.operations());
    assert_eq!(
        old.relative_physical_bounds(),
        new.relative_physical_bounds()
    );
    assert_eq!(old.sine_bounds(), new.sine_bounds());
    assert!(!old.matches_with_version(&input(&spans), ConicalChartDomainVersion::FamilyV3));
}

#[test]
fn family_admission_preserves_policy_count_and_geometry_error_precedence() {
    let spans = vec![LINE.as_slice(); 4097];
    let mut query = input(&spans);
    query.options.fractional_bits = 7;
    let error = family(query).err().unwrap();
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Sector(ConicalSectorFailure::Policy {
            fractional_bits: 7,
            ..
        })
    ));
    query.options.fractional_bits = 128;
    query.semi_angle = f64::NAN;
    let error = family(query).err().unwrap();
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::SourceCount { actual: 4097, .. }
    ));
}

#[test]
fn family_geometry_failure_retains_the_global_original_source_index() {
    let invalid = [[80., -18.], [f64::NAN, 24.]];
    let mut spans = vec![LINE.as_slice(); 239];
    spans[238] = invalid.as_slice();
    let error = family(input(&spans)).err().unwrap();
    assert_eq!(error.operations, 0);
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Sector(ConicalSectorFailure::Point {
            span: 238,
            pole: 1,
            ..
        })
    ));
}

#[test]
fn family_sources_versions_and_stored_capacity_cannot_be_rebound() {
    let mut spans = vec![LINE.as_slice(); 239];
    let mut proof = family(input(&spans)).unwrap();
    assert!(!proof.matches_with_version(&input(&spans), ConicalChartDomainVersion::OriginalV2));
    proof.span_limit -= 1;
    assert!(!proof.matches_with_version(&input(&spans), ConicalChartDomainVersion::FamilyV3));
    proof.span_limit += 1;
    let changed = [[80., -18.], [84., 24_f64.next_up()]];
    spans[238] = changed.as_slice();
    assert!(!proof.matches_with_version(&input(&spans), ConicalChartDomainVersion::FamilyV3));
    assert!(
        !proof.matches_with_version(&input(&spans[..238]), ConicalChartDomainVersion::FamilyV3)
    );
}

#[test]
fn family_budget_is_one_unchanged_global_ledger() {
    let spans = vec![LINE.as_slice(); 239];
    let mut query = input(&spans);
    query.options.max_operations = 17;
    let error = family(query).err().unwrap();
    assert!(error.operations <= 17);
    assert!(!error.is_cancelled());
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Sector(ConicalSectorFailure::Arithmetic(_))
            | ConicalChartDomainFailure::Arithmetic(_)
    ));
}

#[test]
fn family_cancellation_refuses_at_first_middle_and_final_checkpoints() {
    let spans = vec![LINE.as_slice(); 239];
    let query = input(&spans);
    let mut polls = 0;
    certify_conical_chart_domain_with_version(query, ConicalChartDomainVersion::FamilyV3, || {
        polls += 1;
        false
    })
    .unwrap();
    for stop in [1, polls / 2, polls] {
        let mut seen = 0;
        let error = certify_conical_chart_domain_with_version(
            query,
            ConicalChartDomainVersion::FamilyV3,
            || {
                seen += 1;
                seen >= stop
            },
        )
        .err()
        .unwrap();
        assert!(error.is_cancelled(), "checkpoint={stop}, cause={error:?}");
    }
}

#[test]
fn boundary_authority_and_diagnostic_bits_bind_the_domain_version() {
    use super::super::boundary_chart::ChartBinding;
    let spans = [LINE.as_slice()];
    let old = certify_conical_chart_domain(input(&spans), || false).unwrap();
    let new = family(input(&spans)).unwrap();
    let binding = ChartBinding::capture(&new);
    assert!(binding.matches(&new));
    assert!(!binding.matches(&old));
    assert_ne!(
        binding.diagnostic_bits(),
        ChartBinding::capture(&old).diagnostic_bits()
    );
}

#[test]
fn chart_binding_accepts_independent_exact_sources_and_refuses_mutation() {
    use super::super::boundary_chart::ChartBinding;
    let flat = [[80., 0.], [84., 0.]];
    let spans = [LINE.as_slice(), flat.as_slice()];
    let chart = family(input(&spans)).unwrap();
    let binding = ChartBinding::capture(&chart);
    let rebuilt = family(input(&spans)).unwrap();
    assert!(!std::sync::Arc::ptr_eq(
        &chart.sector.source,
        &rebuilt.sector.source
    ));
    drop(chart);
    assert!(binding.matches(&rebuilt));
    assert!(rebuilt.contains_original(&flat));
    let changed_zero = [[80., -0.], [84., 0.]];
    assert!(!rebuilt.contains_original(&changed_zero));
    let reordered = [flat.as_slice(), LINE.as_slice()];
    let reordered = family(input(&reordered)).unwrap();
    assert!(!binding.matches(&reordered));
}
