//! Default one-shape bopargcheck returns its actual diagnostic without caller policy.
use martensite_tkbo::bop_algo::BOPAlgoArgumentAnalyzer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;

/// The native valid return remains shared with caller-owned grading.
pub(crate) const VALID_ARGUMENT_RESPONSE: &str = "Shape(s) seem(s) to be valid for BOP.\n";

/// Share the source default modes with full diagnostics; `configured_default_argument_analyzer(&shape)`.
pub(crate) fn configured_default_argument_analyzer(shape: &Shape) -> BOPAlgoArgumentAnalyzer {
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.options_mut().set_run_parallel(false);
    checker
        .options_mut()
        .set_fuzzy_value(precision::confusion());
    checker.set_shape1(shape);
    *checker.argument_type_mode() = true;
    *checker.self_inter_mode() = true;
    *checker.small_edge_mode() = true;
    *checker.rebuild_face_mode() = true;
    *checker.continuity_mode() = true;
    *checker.curve_on_surface_mode() = true;
    checker
}

/// Return the bare native diagnostic, without adding a predicate; `default_argument_response(&shape)`.
pub(crate) fn default_argument_response(shape: &Shape) -> &'static str {
    let mut checker = configured_default_argument_analyzer(shape);
    checker.perform();
    // The source's single-shape default branch does not emit ReportAlerts or #F details.
    if checker.has_faulty() {
        return "Faulties, that can not be treated by BOP, are detected.\n";
    }
    VALID_ARGUMENT_RESPONSE
}
