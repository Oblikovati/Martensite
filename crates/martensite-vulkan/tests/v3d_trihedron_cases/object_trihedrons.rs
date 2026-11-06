//! Object trihedrons next to shapes: detection after relocation, highlighted points, hidden
//! labels, and a textured box behind a trihedron.
use super::prelude::*;
use super::trihedron_options::{new_trihedron, own_datum_aspect};

/// `bug23422`: a relocated box is detected where it is drawn.
pub(crate) fn moved_box_detection() -> Outcome {
    banner(&[
        "============",
        "CR23422: Selection problems when using SetLocation",
        "============",
        "",
    ]);
    let b = box_sized([50., 50., 50.])?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().display_trihedron("t")?;
    run.session().set_location("b", [30., 30., 130.])?;
    run.session().move_cursor(240, 150)?;
    run.dump("bug23422.png")?;
    run.finish()
}

/// `bug24396`: trihedron points highlight in vertex mode while its axes stay unhighlighted.
pub(crate) fn point_highlight() -> Outcome {
    banner(&[
        "============",
        "CR24396: vselmode - disable auto loading of objects into Local Context",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().display_trihedron("tri")?;
    let b = box_sized([10., 10., 10.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().set_object_selection_mode("b", 1, true)?;
    run.session().fit()?;
    axis_not_highlighted(&mut run)?;
    run.session().set_object_selection_mode("b", 0, false)?;
    run.session().set_selection_mode(0, false)?;
    run.session().erase(&["b"])?;
    run.session().set_selection_mode(1, true)?;
    run.session().move_cursor(29, 305)?;
    if !run.session().check_color(24, 305, [0., 1., 1.])? {
        println!("Error : The trihedron point highlight is incorrect.");
    }
    axis_not_highlighted(&mut run)?;
    run.dump("bug24396.png")?;
    run.finish()
}

fn axis_not_highlighted(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(29, 56)?;
    if !run.session().check_color(29, 56, [0., 0., 0.])? {
        println!("Error : The trihedron should not be highlighted.");
    }
    Ok(())
}

/// `bug24989`: a default trihedron displays.
pub(crate) fn default_trihedron() -> Outcome {
    banner(&[
        "============",
        "OCC24989: Visualization - workaround vc12 32-bit compiler optimizer bug resulted in crash in AisAxis",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().display_trihedron("vtr")?;
    run.dump("bug24989.png")?;
    run.finish()
}

/// `bug26754`: a trihedron without labels, picked as a whole and then by its axes.
pub(crate) fn hidden_label_selection() -> Outcome {
    banner(&[
        "================================================================",
        "CR26754: Visualization - provide API to display AisTrihedron presentation without axes labels",
        "================================================================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let trihedron = new_trihedron()?;
    if let Some(aspect) = own_datum_aspect(&trihedron)?.get() {
        aspect.set_draw_labels(false);
    }
    run.session().display_object("t", trihedron.cast())?;
    run.session().zoom(4.)?;
    run.dump("bug26754_1.png")?;
    run.session().select_point(204, 100)?;
    run.dump("bug26754_2.png")?;
    run.session().set_selection_mode(2, true)?;
    run.session().select_point(204, 100)?;
    run.dump("bug26754_3.png")?;
    run.finish()
}
