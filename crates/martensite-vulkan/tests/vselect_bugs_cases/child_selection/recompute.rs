//! OCC26284 visits separately displayed child objects when recomputing their parent.
use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObject as _;

fn centered_sphere(center: [f64; 3]) -> Result<Handle<AisShape>, Failure> {
    let [x, y, z] = center;
    let mut maker = BRepPrimAPIMakeSphere::at_center(Point3::new(x, y, z), 1.)?;
    let shape = maker.shape()?;
    Ok(AisShape::new(shape))
}

pub(super) fn displayed_sphere_children(
    context: &AisInteractiveContext,
) -> Result<ChildSelectionScene, Failure> {
    let parent = centered_sphere([0.; 3])?;
    context.display(&parent.cast(), false)?;
    let mut children = Vec::new();
    for index in 0..5 {
        let coordinate = 1. + f64::from(index);
        let child = centered_sphere([coordinate; 3])?;
        parent.add_child(&child.cast())?;
        context.display(&child.cast(), false)?;
        children.push(child);
    }
    Ok(ChildSelectionScene { parent, children })
}

pub(super) fn recompute_parent(
    context: &AisInteractiveContext,
    scene: &ChildSelectionScene,
) -> Result<(), Failure> {
    context.recompute_selection_only(&scene.parent.cast())?;
    context.update_current_viewer();
    Ok(())
}

fn complete_recomputation(context: &AisInteractiveContext) -> Outcome {
    let scene = displayed_sphere_children(context)?;
    recompute_parent(context, &scene)?;
    // The anonymous operands leave scope before the view is fitted.
    Ok(())
}

pub(crate) fn child_selection_recomputed() -> Outcome {
    let mut run = child_selection_begin(
        "========",
        "OCC26284: Visualization - eliminate recursion in SelectMgrSelectionManager",
    )?;
    complete_recomputation(run.session().context())?;
    run.session().fit()?;
    run.dump("bug26284.png")?;
    run.finish()
}
