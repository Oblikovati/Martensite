//! `bsection` with the flags that turn off its p-curves or its approximation.
use super::options_bop::report_api_alerts;
use crate::support::{commands::bound_shape, session::Session};
use martensite_tkbo::{bop_algo::Glue, brep_algo_api::BRepAlgoAPISection};
use martensite_tkernel::precision;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// The options of `bsection`: `-n2d`, `-n2d1`, `-n2d2` and `-na` turn these off.
#[derive(Clone, Copy, Debug)]
pub struct SectionFlags {
    /// No `-n2d` and no `-n2d1`.
    pub pcurve_on1: bool,
    /// No `-n2d` and no `-n2d2`.
    pub pcurve_on2: bool,
    /// No `-na`.
    pub approximation: bool,
}

/// `bsection result first second [flags]`, as the plain command otherwise.
/// Example: `bsection_with(s, "r", ("a", "f"), flags)`.
pub fn bsection_with(
    s: &mut dyn Session,
    name: &str,
    (first, second): (&str, &str),
    flags: SectionFlags,
) {
    let (first, second) = (bound_shape(s, first), bound_shape(s, second));
    let mut operation = BRepAlgoAPISection::from_shapes(&first, &second, None, false)
        .unwrap_or_else(|failure| panic!("bsection {name}: {failure:?}"));
    operation.approximation(flags.approximation);
    operation.compute_pcurve_on1(flags.pcurve_on1);
    operation.compute_pcurve_on2(flags.pcurve_on2);
    operation.set_fuzzy_value(precision::confusion());
    operation.set_run_parallel(false);
    operation.set_non_destructive(false);
    operation.set_glue(Glue::Off);
    operation.set_use_obb(false);
    operation.build().unwrap();
    report_api_alerts(&operation);
    if !operation.is_done() {
        return;
    }
    let result = operation.shape().clone();
    if result.is_null() {
        println!(" null shape");
        return;
    }
    s.bind(name, &result);
}
