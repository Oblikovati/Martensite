use super::*;
use martensite_tkbo::bop_algo::{BOPAlgoBuilder, BOPAlgoPaveFiller};
use martensite_tkernel::{ncollection::List, precision};

fn prepared_parallel_intersections(shapes: &[Shape]) -> Result<BOPAlgoPaveFiller, BoxError> {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&shapes.iter().cloned().collect::<List<_>>());
    filler.options_mut().set_run_parallel(true);
    filler.options_mut().set_fuzzy_value(precision::confusion());
    filler.options_mut().set_use_obb(false);
    filler.set_non_destructive(false);
    filler.set_glue(martensite_tkbo::bop_algo::Glue::Off);
    filler.perform();
    crate::common_operation::report_result_alerts(filler.options());
    if filler.options().has_errors() {
        return Err("bfillds parallel=true errors=true; expected completed intersections".into());
    }
    Ok(filler)
}

pub(super) fn parallel_general_build(shapes: &[Shape]) -> Result<Shape, BoxError> {
    let mut filler = prepared_parallel_intersections(shapes)?;
    let mut builder = BOPAlgoBuilder::new();
    for shape in shapes {
        builder.add_argument(shape);
    }
    builder.options_mut().set_run_parallel(true);
    builder.set_check_inverted(true);
    builder.set_to_fill_history(true);
    builder.perform_with_filler(&mut filler);
    crate::common_operation::report_result_alerts(builder.options());
    completed_parallel_shape(builder)
}

fn completed_parallel_shape(builder: BOPAlgoBuilder) -> Result<Shape, BoxError> {
    if builder.options().has_errors() || builder.shape().is_null() {
        return Err(format!(
            "bbuild parallel=true errors={},null={}; expected complete non-null result",
            builder.options().has_errors(),
            builder.shape().is_null()
        )
        .into());
    }
    Ok(builder.shape().clone())
}
