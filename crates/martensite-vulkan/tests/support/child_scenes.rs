//! Scenes of parent objects whose children keep their own placement,
//! shared by the V3d and selection regressions that build them.
use crate::case_view::viewer_session::DisplayRequest;
use crate::primitive_shapes::{box_sized, sphere};
use crate::session_run::SessionRun;
use std::error::Error;

/// A shaded sphere `parent` of radius 3 with shaded unit boxes child1 at x = 10 and child2 at
/// x = 20, added in place after the parent stops propagating its visual state, then fitted.
/// Example: `let mut run = SessionRun::new()?; boxes_beside_sphere(&mut run)?;`.
pub(crate) fn boxes_beside_sphere(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().clear()?;
    run.session().display("parent", &sphere(3.)?, &shaded)?;
    for name in ["child1", "child2"] {
        run.session()
            .display(name, &box_sized([1., 1., 1.])?, &shaded)?;
    }
    run.session()
        .set_location_translation("child1", [10., 0., 0.])?;
    run.session()
        .set_location_translation("child2", [20., 0., 0.])?;
    run.session().ignore_parent_visual_state("parent")?;
    run.session().add_child_in_place("parent", "child1")?;
    run.session().add_child_in_place("parent", "child2")?;
    run.session().fit()?;
    Ok(())
}
