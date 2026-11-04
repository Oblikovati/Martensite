//! CR26973 preserves all four view/object and topology-location combinations.
use super::prelude::*;
use super::selected_state::local_selected_faces;
use crate::primitive_shapes::translated;

const BUG26973_BANNER: &[&str] = &[
    "============",
    "CR26973: Visualization - selection of entities hidden by clipping planes is broken",
    "============",
    "",
];

pub(crate) fn four_plane_targets() -> Result<(), Box<dyn Error>> {
    banner(BUG26973_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    for (view, transformed, image) in PROBES {
        one_probe(&mut run, view, transformed, image)?;
    }
    run.finish()
}

const PROBES: [(bool, bool, &str); 4] = [
    (true, false, "bug26973_view_wo_trsf.png"),
    (true, true, "bug26973_view_with_trsf.png"),
    (false, false, "bug26973_obj_wo_trsf.png"),
    (false, true, "bug26973_obj_with_trsf.png"),
];

pub(super) fn probe_shape(transformed: bool) -> Result<martensite_tkbrep::topods::Shape, Failure> {
    if transformed {
        return translated(&box_at([-100.; 3], [1.; 3])?, [100.; 3]);
    }
    box_sized([1.; 3])
}

fn one_probe(
    run: &mut SessionRun,
    view: bool,
    transformed: bool,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    prepare_probe(run.session(), transformed)?;
    let mut planes = ClippingPlanes::new(run.session().view());
    planes.create("pln")?;
    attach_probe_plane(&mut planes, run.session(), view)?;
    planes.equation("pln", [0., 1., 0., -0.5])?;
    run.session().select_point(200, 140)?;
    check_probe(run.session(), view, transformed)?;
    run.dump(image)?;
    run.session().clear()?;
    planes.delete("pln")?;
    Ok(())
}

pub(super) fn prepare_probe(session: &mut ViewerSession, transformed: bool) -> Result<(), Failure> {
    session.display("b", &probe_shape(transformed)?, &DisplayRequest::default())?;
    session.fit()?;
    session.set_object_selection_mode("b", 4, true)?;
    Ok(())
}

fn check_probe(session: &ViewerSession, view: bool, transformed: bool) -> Result<(), Failure> {
    let info = format!(
        "Test case: clipping planes applied to {}, object trsf is {}",
        if view { "view" } else { "object" },
        if transformed { "on" } else { "off" }
    );
    if session.selected_count() != 1 {
        println!("ERROR: can not select face.\n{info}");
    }
    let selected = local_selected_faces(session.context())?
        .into_iter()
        .next()
        .unwrap_or_default();
    if selected != "Face #1" {
        println!("ERROR: wrong face is selected.\nActually selected face: {selected}\n{info}");
    }
    Ok(())
}

pub(super) fn attach_probe_plane(
    planes: &mut ClippingPlanes,
    session: &ViewerSession,
    view: bool,
) -> Result<(), Failure> {
    if view {
        return planes.on_named_view("pln", "Driver1/Viewer1/View1");
    }
    object_plane(planes, session, "pln", "b", false)
}
