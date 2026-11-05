//! Original direct bsection options, independent from the prepared bopsection path.
use martensite_tkbo::{bop_algo::Glue, brep_algo_api::BRepAlgoAPISection};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Preserve the direct command options before caller-owned build/history; `configured_section(&a, &b)`.
pub fn configured_section(first: &Shape, second: &Shape) -> BRepAlgoAPISection {
    let mut operation = BRepAlgoAPISection::from_shapes(first, second, None, false).unwrap();
    operation.approximation(true);
    operation.compute_pcurve_on1(true);
    operation.compute_pcurve_on2(true);
    operation.set_fuzzy_value(precision::confusion());
    operation.set_run_parallel(false);
    operation.set_non_destructive(false);
    operation.set_glue(Glue::Off);
    operation.set_use_obb(false);
    operation
}

/// Execute original bsection defaults; `direct_section(&first, &second)`.
pub fn direct_section(first: &Shape, second: &Shape) -> Shape {
    direct_section_observed(first, second, |_| {})
}

/// Observe the real post-build report without altering construction; `direct_section_observed(&a, &b, |op| inspect(op))`.
pub fn direct_section_observed(
    first: &Shape,
    second: &Shape,
    observe: impl FnOnce(&BRepAlgoAPISection),
) -> Shape {
    let mut operation = configured_section(first, second);
    operation.build().unwrap();
    observe(&operation);
    assert!(
        operation.is_done(),
        "bsection: expected completed operation"
    );
    assert!(
        !operation.shape().is_null(),
        "bsection: unexpected null result"
    );
    operation.shape().clone()
}
