//! Adding the same local plane keeps its existing view association and object location.
use super::prelude::*;

const BUG31650_BANNER: &[&str] = &[
    "============",
    "0031650: Visualization - invalid picking of object with local transformation and per-object clipping plane",
    "============",
    "",
];

pub(crate) fn shared_global_local_plane() -> Result<(), Box<dyn Error>> {
    banner(BUG31650_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    located_box(run.session())?;
    run.session().fit()?;
    let mut planes = initial_view_plane(run.session())?;
    observe_located(run.session())?;
    planes.equation("p", [0., 0., -1., -10.])?;
    object_plane(&mut planes, run.session(), "p", "b", false)?;
    observe_located(run.session())?;
    run.dump("bug31650.png")?;
    run.finish()
}

pub(super) fn located_box(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_axonometric()?;
    session.display(
        "b",
        &box_at([-28., -11., -35.], [20., 20., 42.])?,
        &shaded_highlighted(),
    )?;
    session.set_location("b", [0., 0., 20.])
}

fn observe_located(session: &mut ViewerSession) -> Result<(), Failure> {
    session.move_cursor(205, 205)?;
    pixel(session, 205, 320, "DARKGOLDENROD", "Error")?;
    Ok(())
}

pub(super) fn initial_view_plane(session: &ViewerSession) -> Result<ClippingPlanes, Failure> {
    let mut planes = ClippingPlanes::new(session.view());
    if !planes.delete("p")? {
        println!("Warning: clipping plane p does not exist");
    }
    planes.ensure("p")?;
    planes.equation("p", [0., 0., -1., -10.])?;
    planes.on_view("p", session.view())?;
    Ok(planes)
}
