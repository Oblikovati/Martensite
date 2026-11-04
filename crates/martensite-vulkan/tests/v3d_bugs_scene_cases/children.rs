//! Children that keep or inherit their parent's transformation, and erased parents that do
//! not propagate their visual state.
use super::prelude::*;
use crate::child_scenes::boxes_beside_sphere;

/// The shaded sphere `m` of radius 3 at (20, 10, 20), unit boxes b1 and b2 at x = -10 and five
/// unit spheres c1..c5 along X, as `bug30561` displays them.
fn parent_sphere_scene(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().clear()?;
    run.session().display("m", &sphere(3.)?, &shaded)?;
    run.session()
        .set_location_translation("m", [20., 10., 20.])?;
    for name in ["b1", "b2"] {
        run.session()
            .display(name, &box_sized([1., 1., 1.])?, &shaded)?;
    }
    for name in ["b1", "b2"] {
        run.session()
            .set_location_translation(name, [-10., 0., 0.])?;
    }
    let names = ["c1", "c2", "c3", "c4", "c5"];
    for name in names {
        run.session().display(name, &sphere(1.)?, &shaded)?;
    }
    for (index, name) in names.into_iter().enumerate() {
        let x = 20. + 10. * (index as f64 + 1.);
        run.session()
            .set_location_translation(name, [x, 10., 20.])?;
    }
    Ok(())
}

/// `bug30561`: children added in place stay put when the parent moves; an inheriting child
/// follows it.
pub(crate) fn children_keep_placement() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0030561: Visualization, PrsMgrPresentableObject - Keep the local transformation for child in AddChild()",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    parent_sphere_scene(&mut run)?;
    for name in ["c1", "c2", "c3", "c4", "c5"] {
        run.session().add_child_in_place("m", name)?;
    }
    run.session().add_child("m", "b1")?;
    run.session()
        .set_location_translation("m", [100., 20., 20.])?;
    run.session()
        .set_location_translation("c4", [40., 10., 0.])?;
    run.session().fit()?;
    run.dump("bug30561.png")?;
    run.finish()
}

/// `bug30823`: erasing a parent that does not propagate its visual state keeps its children.
pub(crate) fn erased_parent_keeps_children() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0030823: Visualization, PrsMgrPresentableObject - A new flag to disable automatic display/erase of children",
    ]);
    banner(&["============="]);
    let mut run = SessionRun::new()?;
    boxes_beside_sphere(&mut run)?;
    run.session().erase(&["parent"])?;
    run.dump("bug30823.png")?;
    run.finish()
}
