//! `pcylinder cc 10 30; pcylinder cc2 8 50; ttranslate cc2 0 0 2; bfuse a cc cc2`.
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Transform3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform};
use std::error::Error;

/// A cylinder of radius 10 and height 30 fused with one of radius 8 and height 50 raised by 2.
/// Example: `fused_cylinders()?`.
pub(crate) fn fused_cylinders() -> Result<Shape, Box<dyn Error>> {
    let outer = BRepPrimAPIMakeCylinder::new(10., 30.).shape()?.clone();
    let inner = BRepPrimAPIMakeCylinder::new(8., 50.).shape()?.clone();
    // ttranslate is not a basic transform: BRepBuilderAPITransform without copy.
    let mut raise = BRepBuilderAPITransform::new(Transform3::translation([0., 0., 2.]));
    raise.perform(&inner, false, false)?;
    let inner = raise.shape()?.clone();
    draw_bfuse(&outer, &inner)
}

/// the `bfuse` command (BOPTest bsmt): BOPAlgoBOP with the default session options.
fn draw_bfuse(object: &Shape, tool: &Shape) -> Result<Shape, Box<dyn Error>> {
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(object);
    operation.add_tool(tool);
    operation.set_operation(Operation::Fuse);
    operation.options_mut().set_run_parallel(false);
    operation.set_check_inverted(true);
    operation.perform();
    if operation.options().has_errors() || operation.shape().is_null() {
        return Err(
            std::io::Error::other("bfuse of the two cylinders; expected a fused solid").into(),
        );
    }
    Ok(operation.shape().clone())
}
