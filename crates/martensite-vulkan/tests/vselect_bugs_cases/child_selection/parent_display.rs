//! OCC27757 activates its sphere child through the parent-only display operation.
use super::*;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObject as _};

pub(super) fn discarded_sphere_location() -> Location {
    let mut transform = Transform3::default();
    transform.set_translation_part(Vector3::new(20., 20., 0.));
    Location::new(transform)
}

pub(super) fn origin_sphere_after_discard() -> Result<Shape, Failure> {
    let sphere = crate::primitive_shapes::sphere(10.)?;
    // RegressionCommands OCC27757 discards Located's copy; it never moves the source sphere.
    drop(sphere.located(&discarded_sphere_location(), false)?);
    Ok(sphere)
}

pub(super) fn parent_only_display(
    context: &AisInteractiveContext,
) -> Result<ChildSelectionScene, Failure> {
    let box_shape = crate::primitive_shapes::box_sized([20.; 3])?;
    let sphere_shape = origin_sphere_after_discard()?;
    let parent = AisShape::new(&box_shape);
    let child = AisShape::new(&sphere_shape);
    parent.add_child(&child.cast())?;
    context.display_with_modes(&parent.cast(), 1, 0, false, PrsMgrDisplayStatus::None)?;
    context.update_current_viewer();
    Ok(ChildSelectionScene {
        parent,
        children: vec![child],
    })
}

fn complete_parent_display(context: &AisInteractiveContext) -> Outcome {
    let scene = parent_only_display(context)?;
    // Native command handles die on return; the real context/hierarchy retain the scene.
    drop(scene);
    Ok(())
}

fn check_parent_child_selection(run: &mut SessionRun) -> Outcome {
    // These are independent checks: read the pixel even after a bad count.
    if run.session().selected_count() != 1 {
        println!("Error: no object was selected; the sphere must be selected!");
    }
    if run.session().pixel_rgba_name(145, 285)? != "GRAY80 1" {
        println!("Error: the sphere is not highlighted with selection color!");
    }
    Ok(())
}

pub(crate) fn child_selected_through_parent() -> Outcome {
    banner(&[
        "============",
        "CR27757",
        "Visualization - handle child objects in selection manager regardless of HasOwnPresentations() flag",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    complete_parent_display(run.session().context())?;
    run.session().fit()?;
    run.session().select_point(120, 245)?;
    check_parent_child_selection(&mut run)?;
    run.dump("bug27757.png")?;
    run.finish()
}
