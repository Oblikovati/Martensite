use super::*;
use crate::primitive_shapes::{cone, cylinder, sphere, translated};
use martensite_tkbo::bop_algo::Operation;

fn case_fusion_inputs() -> Result<[Shape; 3], BoxError> {
    let sphere = sphere(10.)?;
    let cylinder = cylinder(8., 20.)?;
    let cone = cone(10., 8., 5.)?;
    Ok([
        translated(&sphere, [0., 0., 25.])?,
        cylinder,
        translated(&cone, [0., 0., -5.])?,
    ])
}

pub(super) fn actual_prepared_multi_tool_fuse() -> Result<Shape, BoxError> {
    let [object, first_tool, second_tool] = case_fusion_inputs()?;
    let mut filler = crate::common_operation::configured_intersections(&object, &first_tool);
    filler.add_argument(&second_tool);
    filler.perform();
    crate::common_operation::report_result_alerts(filler.options());
    if filler.options().has_errors() {
        return Err("bfillds errors=true; expected completed source intersections".into());
    }
    let mut operation =
        crate::common_operation::configured_prepared(&object, &first_tool, Operation::Fuse);
    operation.add_tool(&second_tool);
    operation.perform_with_filler(&mut filler);
    completed_fuse_shape(operation)
}

pub(super) fn completed_fuse_shape(
    operation: martensite_tkbo::bop_algo::BOPAlgoBOP,
) -> Result<Shape, BoxError> {
    crate::common_operation::report_result_alerts(operation.options());
    if operation.options().has_errors() || operation.shape().is_null() {
        return Err(format!(
            "bbop Fuse errors={}, null={}; expected completed non-null source result",
            operation.options().has_errors(),
            operation.shape().is_null()
        )
        .into());
    }
    Ok(operation.shape().clone())
}
