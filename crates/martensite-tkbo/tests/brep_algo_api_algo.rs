//! Behavior of the `BRepAlgoAPIAlgo` base before a derived algorithm builds a result.
use martensite_tkbo::bop_algo::AlertUserBreak;
use martensite_tkbo::brep_algo_api::BRepAlgoAPIAlgo;
use martensite_tkernel::message::{AlertHandle, BaseAlert, Gravity};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::sync::Arc;

#[test]
fn shape_returns_the_stored_null_result_without_building() {
    let mut algorithm = BRepAlgoAPIAlgo::new();
    assert!(!algorithm.is_done());
    assert!(algorithm.shape().is_null());
    assert!(
        BRepBuilderAPIMakeShape::shape(&mut algorithm)
            .expect("BRepAlgoAPIAlgo::Shape returns even when not done")
            .is_null()
    );
    assert!(!algorithm.is_done());
}

#[test]
fn default_constructor_uses_boolean_option_defaults() {
    let algorithm = BRepAlgoAPIAlgo::new();
    assert!(!algorithm.run_parallel());
    assert_eq!(
        algorithm.fuzzy_value(),
        martensite_tkernel::precision::confusion()
    );
}

#[test]
fn public_option_methods_preserve_configuration() {
    let mut algorithm = BRepAlgoAPIAlgo::new();
    algorithm.set_run_parallel(true);
    algorithm.set_fuzzy_value(0.01);
    assert!(algorithm.run_parallel());
    assert_eq!(algorithm.fuzzy_value(), 0.01);
}

#[test]
fn report_access_and_error_methods_forward_to_the_shared_report() {
    let algorithm = BRepAlgoAPIAlgo::new();
    let report = algorithm.get_report();
    report.add_alert(Gravity::Fail, &(Arc::new(AlertUserBreak) as AlertHandle));
    report.add_alert(Gravity::Warning, &(Arc::new(BaseAlert) as AlertHandle));
    assert!(algorithm.has_errors());
    assert!(algorithm.has_error(AlertUserBreak::static_type()));
    assert!(algorithm.has_warnings());
    assert!(algorithm.has_warning(BaseAlert::static_type()));
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    algorithm.dump_errors(&mut errors).unwrap();
    algorithm.dump_warnings(&mut warnings).unwrap();
    assert_eq!(errors, b"BOPAlgoAlertUserBreak\n");
    assert_eq!(warnings, b"Message_Alert\n");
    algorithm.clear_warnings();
    assert!(!algorithm.has_warnings() && algorithm.has_errors());
    algorithm.clear();
    assert!(!algorithm.has_errors());
}
