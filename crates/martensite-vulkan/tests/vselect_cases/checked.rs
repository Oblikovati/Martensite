//! Selection regressions that check rendered pixel colours after their commands.
use super::prelude::*;
use crate::session_run::banner;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::select_mgr::SelectMgrPickingStrategy;

/// Pixels on edges 10, 2 and 1 of the 10 mm box, in the order the edges cycle.
const EDGE_PIXELS: [(i32, i32); 3] = [(94, 140), (103, 60), (29, 208)];

/// True when edge `index` is highlighted and the two others keep the plain colour.
fn edge_highlighted(run: &mut SessionRun, index: usize) -> Result<bool, Box<dyn Error>> {
    let name = |run: &mut SessionRun, i: usize| {
        let (x, y) = EDGE_PIXELS[i % 3];
        run.session().pixel_rgba_name(x, y)
    };
    let highlighted = name(run, index)? == "CYAN 1";
    let next = name(run, index + 1)? == "YELLOW 1";
    let next_next = name(run, index + 2)? == "YELLOW 1";
    Ok(highlighted && next && next_next)
}

fn expect_edge(run: &mut SessionRun, index: usize, message: &str) -> Result<(), Box<dyn Error>> {
    if !edge_highlighted(run, index)? {
        println!("{message}");
    }
    Ok(())
}

/// `edge/A10`: cycling through the overlapping detected edges forwards and backwards.
pub(crate) fn edge_a10() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25436",
        "Test case check work of the new commands vselnext and vselprev",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let b = box_sized([10., 10., 10.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(30, 101)?;
    expect_edge(&mut run, 0, "ERROR: incorrect highlighting of edge 10")?;
    let steps: [(bool, usize, &str); 6] = [
        (
            true,
            2,
            "ERROR: incorrect highlighting of edge 1 after vselnext call",
        ),
        (
            true,
            1,
            "ERROR: incorrect highlighting of edge 2 after vselnext call",
        ),
        (
            true,
            0,
            "ERROR: incorrect highlighting of edge 10 after vselnext call",
        ),
        (
            false,
            1,
            "ERROR: incorrect highlighting of edge 2 after vselprev call",
        ),
        (
            false,
            2,
            "ERROR: incorrect highlighting of edge 1 after vselprev call",
        ),
        (
            false,
            0,
            "ERROR: incorrect highlighting of edge 10 after vselprev call",
        ),
    ];
    for (forward, index, message) in steps {
        if forward {
            run.session().highlight_next_detected()?;
        } else {
            run.session().highlight_previous_detected()?;
        }
        expect_edge(&mut run, index, message)?;
    }
    run.dump("A10.png")?;
    run.finish()
}

fn expect_rgb(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    expected: &str,
    equal: bool,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if (run.session().pixel_color_name(x, y)? == expected) != equal {
        println!("{message}");
    }
    Ok(())
}

/// `face/F3`: a face filter and the picking strategy decide which of two boxes highlights.
pub(crate) fn face_f3() -> Result<(), Box<dyn Error>> {
    let b1 = box_at([0., 0., 0.], [1., 1., 1.])?;
    let b2 = box_at([2., 0., 0.], [1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(404.),
        proj: Some([0.8, -0.16, 0.5]),
        up: Some([-0.4, 0.4, 0.8]),
        at: Some([1.5, 0.5, 0.5]),
        ..ViewParameters::default()
    })?;
    run.session().clear_selection_filters();
    run.session()
        .set_picking_strategy(SelectMgrPickingStrategy::FirstAcceptable);
    let shaded = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    run.session().display("b1", &b1, &shaded)?;
    run.session().display("b2", &b2, &shaded)?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b1", 4, true)?;
    run.session().move_cursor(220, 220)?;
    let b2_lit = "Error: box b2 should be highlighted";
    expect_rgb(&mut run, (220, 220), "TURQUOISE2", true, b2_lit)?;
    run.session().add_shape_type_filter(ShapeEnum::Face);
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(220, 220)?;
    let face_lit = "Error: face of box b1 should be highlighted";
    expect_rgb(&mut run, (150, 150), "CYAN", true, face_lit)?;
    run.session()
        .set_picking_strategy(SelectMgrPickingStrategy::OnlyTopmost);
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(220, 220)?;
    let face_unlit = "Error: face of box b1 should NOT be highlighted";
    expect_rgb(&mut run, (150, 150), "CYAN", false, face_unlit)?;
    run.session().move_cursor(150, 150)?;
    expect_rgb(&mut run, (150, 150), "CYAN", true, face_lit)?;
    run.dump("F3.png")?;
    run.finish()
}

/// `wire_solid/L1`: showing only one of two shaded boxes erases the other.
pub(crate) fn wire_solid_l1() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0029739: Test Commands - vdonly does not hide displayed objects",
        "========",
        "",
    ]);
    let b1 = box_at([0., 0., 0.], [1., 2., 3.])?;
    let b2 = box_at([2., 0., 0.], [3., 2., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b1", &b1, &shaded)?;
    run.session().display("b2", &b2, &shaded)?;
    run.session().fit()?;
    run.session().display_only(&["b1"])?;
    let erased = "Error: wrong prs erased";
    expect_rgb(&mut run, (100, 100), "BLACK", false, erased)?;
    expect_rgb(&mut run, (300, 300), "BLACK", true, erased)?;
    run.dump("L1.png")?;
    run.finish()
}
