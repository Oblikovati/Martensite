//! Selected-owner counts after point, polygon and rectangle picks.
use super::prelude::*;

/// `bug727`: an object picked with xor a second time is deselected.
pub(crate) fn deselect_second_object() -> Result<(), Box<dyn Error>> {
    banner(&["============", "OCC727", "============", ""]);
    let a = box_sized([10., 10., 10.])?;
    let b = box_at([50., 50., 50.], [10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().select_point(0, 0)?;
    run.session().select_point(60, 300)?;
    run.session().select_polyline(&[(350, 120)], Xor)?;
    expect_selected(&mut run, 2, "Error : (case 1)");
    run.session().select_polyline(&[(350, 120)], Xor)?;
    expect_selected(&mut run, 1, "Error : (case 2)");
    run.dump("bug727.png")?;
    run.finish()
}

/// `bug23539_1`: a polygon around the view selects the 12 box edges, one in empty space none.
pub(crate) fn polygon_edge_counts() -> Result<(), Box<dyn Error>> {
    banner(&["CR23539: Test case for polygonal selection"]);
    let b = box_sized([10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_selection_mode(2, true)?;
    run.session()
        .select_polyline(&[(1, 1), (1, 407), (407, 407), (407, 1)], Replace)?;
    let all = "Error : Bad number of selected edges after polygonal selection of all edges";
    expect_selected(&mut run, 12, all);
    run.session()
        .select_polyline(&[(10, 10), (10, 20), (20, 20), (20, 10)], Replace)?;
    let empty = "Error : Bad number of selected edges after polyginal selection of empty space";
    expect_selected(&mut run, 0, empty);
    run.dump("bug23539_1.png")?;
    run.finish()
}

/// `bug23539_2`: a polygon picked with xor adds edges to a point selection.
pub(crate) fn polygon_xor_counts() -> Result<(), Box<dyn Error>> {
    banner(&["CR23539: Test case for polygonal shift selection"]);
    let a = box_sized([10., 10., 10.])?;
    let b = box_at([50., 50., 50.], [10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_selection_mode(2, true)?;
    run.session().select_point(0, 0)?;
    run.session().select_point(58, 300)?;
    run.session()
        .select_polyline(&[(300, 200), (300, 60), (400, 60), (407, 150)], Xor)?;
    expect_selected(
        &mut run,
        13,
        "Error : Polygonal shift selection doesn't work properly",
    );
    run.session().select_polyline(&[(350, 121)], Xor)?;
    expect_selected(&mut run, 12, "Error : (case 2)");
    run.dump("bug23539_2.png")?;
    run.finish()
}

/// `bug26147`: rectangles keep only fully included entities unless overlap is allowed.
pub(crate) fn overlapping_rectangle_counts() -> Result<(), Box<dyn Error>> {
    banner(&[
        "CR26147: Visualization - restore the ability to pick only fully included objects in rectangular selection",
    ]);
    let b1 = box_at([0., 0., 0.], [1., 1., 1.])?;
    let b2 = box_at([2., 2., 2.], [1., 1., 1.])?;
    let b3 = box_at([4., 4., 4.], [1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    for (name, shape) in [("b1", &b1), ("b2", &b2), ("b3", &b3)] {
        run.session()
            .display(name, shape, &DisplayRequest::default())?;
    }
    run.session().fit()?;
    for (name, mode) in [("b1", 1), ("b1", 2), ("b2", 1), ("b2", 2)] {
        run.session().set_object_selection_mode(name, mode, true)?;
    }
    run.session()
        .select_rectangle((75, 230), (235, 320), Replace)?;
    let inside =
        "ERROR: Wrong number of entities in vertice-edge selection without overlap allowed!";
    expect_selected(&mut run, 4, inside);
    run.session()
        .select_rectangle_overlapping((75, 230), (235, 320), Replace)?;
    let overlap = "ERROR: Wrong number of entities in vertice-edge selection with overlap allowed!";
    expect_selected(&mut run, 11, overlap);
    run.session().select_point(0, 0)?;
    run.session().set_object_selection_mode("b3", 4, true)?;
    // "Overlap off" only keeps the corner order; the selector's overlap detection stays on.
    run.session().select_rectangle((75, 230), (235, 320), Xor)?;
    run.session()
        .select_rectangle((350, 150), (380, 170), Xor)?;
    let shift = "ERROR: Wrong number of entities in vertice-edge-face shift selection without overlap allowed!";
    expect_selected(&mut run, 4, shift);
    run.session().select_point(0, 0)?;
    run.session()
        .select_rectangle_overlapping((75, 230), (235, 320), Xor)?;
    run.session()
        .select_rectangle_overlapping((350, 150), (380, 170), Xor)?;
    let shift_overlap = "ERROR: Wrong number of entities in vertice-edge-face shift selection with overlap allowed!";
    expect_selected(&mut run, 13, shift_overlap);
    run.dump("bug26147.png")?;
    run.finish()
}
