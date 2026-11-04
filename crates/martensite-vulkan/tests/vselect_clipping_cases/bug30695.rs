//! Rectangle selection retains the original four/one/two observations and sticky overlap.
use super::prelude::*;

const BUG30695_BANNER: &[&str] = &[
    "=============",
    "0030695: Visualization - selection by box should use clipping planes set for viewer",
    "=============",
];

pub(crate) fn rectangle_containment_overlap() -> Result<(), Box<dyn Error>> {
    banner(BUG30695_BANNER);
    let mut run = SessionRun::new()?;
    display_boxes(run.session())?;
    run.session().fit()?;
    run.session().zoom(0.5)?;
    checked_rectangle(run.session(), false, 4, INITIAL_SELECTION_ERROR)?;
    let mut planes = ClippingPlanes::new(run.session().view());
    planes.create("pln")?;
    planes.on_named_view("pln", "Driver1/Viewer1/View1")?;
    planes.equation("pln", [-1., 0., 0., 5.])?;
    checked_rectangle(run.session(), false, 1, CONTAINMENT_ERROR)?;
    checked_rectangle(run.session(), true, 2, OVERLAP_ERROR)?;
    run.dump("bug30695.png")?;
    run.finish()
}

pub(super) fn display_boxes(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    for (name, origin, size) in [
        ("b", [0.; 3], [10.; 3]),
        ("b1", [-5., 0., 0.], [2.; 3]),
        ("b2", [13., 0., 0.], [2.; 3]),
        ("b3", [16., 0., 0.], [2.; 3]),
    ] {
        session.display(
            name,
            &box_at(origin, size)?,
            &DisplayRequest::default().with_display_mode(1),
        )?;
    }
    Ok(())
}

pub(super) fn select_box(session: &mut ViewerSession, overlap: bool) -> Result<(), Failure> {
    let rectangle = [(40, 100), (370, 300)];
    session.select_pixels(&rectangle, overlap)
}

const CONTAINMENT_ERROR: &str =
    "ERROR: Bad number of selected objects after clippling plane set on view!";
const OVERLAP_ERROR: &str = "ERROR: Bad number of selected objects after clippling plane set on view with allow overlap selection!";

fn checked_rectangle(
    session: &mut ViewerSession,
    overlap: bool,
    expected: i32,
    label: &str,
) -> Result<(), Failure> {
    select_box(session, overlap)?;
    count(session, expected, label);
    Ok(())
}

const INITIAL_SELECTION_ERROR: &str = "ERROR: Initial objects are not selected!";
