//! Selection schemes and fitting: replace-extra picks, strict polyline inclusion, the picked
//! sub-shape of a box and fitting to a single selected object.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::dump_diff::{DiffOptions, diff_dumps};
use crate::primitive_shapes::{sphere, translated};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::ais::AisSelectionScheme::ReplaceExtra;

/// Print `message` when the selection state of `name` is not `selected`.
fn expect_state(run: &mut SessionRun, name: &str, selected: bool, message: &str) {
    if run.session().is_selected(name) != selected {
        println!("{message}");
    }
}

/// `bug27805`: fitting to the only selected object frames the view as fitting all does.
pub(crate) fn fit_selected_matches_fit_all() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR27805",
        "AisInteractiveContext::FitSelected() is broken for global selection",
        "============",
        "",
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    run.session().select_point(200, 200)?;
    run.dump("bug27805_fitall.png")?;
    run.session().fit_selected()?;
    run.dump("bug27805_fitsel.png")?;
    let options = DiffOptions {
        tolerance: 0.,
        black_white: false,
        border_filter: false,
        diff_image: Some("bug27805_diff.png"),
    };
    if diff_dumps("bug27805_fitsel.png", "bug27805_fitall.png", &options)? != 0 {
        println!("Error: FitSelected() does not match FitAll() for single object");
    }
    run.finish()
}

/// `bug25492`: the face picked in face selection mode equals one of the box's faces.
pub(crate) fn picked_face_in_exploded_faces() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25492: The selected subshape does not have topological relationship with original shape in OCC680",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let shape = box_at([5., 5., 5.], [3., 3., 3.])?;
    run.session()
        .display("aShape", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_selection_mode(4, true)?;
    run.session().select_point(100, 200)?;
    // Display the selected sub-shape as its own object.
    let context = run.session().context().clone();
    context.init_selected();
    let picked = context.selected_shape()?;
    if !all_distinct(&shape, ShapeEnum::Face)
        .iter()
        .any(|face| face.is_equal(&picked))
    {
        println!("Error: No selected sub-shape was found in exploded sub-shapes");
    }
    run.dump("bug25492.png")?;
    run.finish()
}

/// `bug32205`: replace-extra picks toggle the picked object and drop the others.
pub(crate) fn replace_extra_toggles() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0032205:  Visualization - implementing new selection scheme in context - replace extra",
        "========",
        "",
    ]);
    let b1 = box_at([0., 0., 0.], [10., 10., 10.])?;
    let b2 = box_at([20., 20., 20.], [30., 30., 30.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b1", &b1, &shaded)?;
    run.session().display("b2", &b2, &shaded)?;
    run.session().fit()?;
    run.session().set_selection_display_mode(-1)?;
    run.session().select_rectangle((0, 0), (400, 400), Xor)?;
    expect_state(&mut run, "b1", true, "Error: b1 should be selected.");
    expect_state(&mut run, "b2", true, "Error: b2 should be selected.");
    for b2_selected in [true, false, true] {
        run.session().select_polyline(&[(200, 200)], ReplaceExtra)?;
        expect_state(&mut run, "b1", false, "Error: b1 should not be selected.");
        let message = if b2_selected {
            "Error: b2 should be selected."
        } else {
            "Error: b2 should not be selected."
        };
        expect_state(&mut run, "b2", b2_selected, message);
    }
    run.dump("bug32205.png")?;
    run.finish()
}

/// `bug_polyline_inclusion`: a polyline around both spheres selects them in strict inclusion,
/// a polyline away from them selects nothing in either mode.
pub(crate) fn polyline_strict_inclusion() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================================",
        "Visualization - polyline (polygonal) selection in strict-inclusion mode",
        "Regression guard for SelectMgrTriangularFrustumSet::OverlapsPoint /",
        "OverlapsBox: the single-arg OverlapsPoint(gp_Pnt) used to return false",
        "unconditionally, which made strict-inclusion polyline selection of",
        "SensitiveTriangle / SensitivePoly / SensitiveTriangulation /",
        "SensitivePrimitiveArray / MeshVS_* impossible.",
        "Pixel coordinates are reused from tests/vselect/sphere/rectangle_selection.",
        "========================================",
        "",
    ]);
    let s1 = translated(&sphere(1.)?, [2., 0., 0.])?;
    let s2 = translated(&sphere(1.)?, [-2., 0., 0.])?;
    let mut run = SessionRun::with_view_size(600, 400)?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("s1", &s1, &shaded)?;
    run.session().display("s2", &s2, &shaded)?;
    run.session().fit()?;
    run.session()
        .select_pixels(&[(15, 15), (585, 15), (585, 385), (15, 385)], false)?;
    if !run.session().is_selected("s1") || !run.session().is_selected("s2") {
        println!(
            "Error: polyline enclosing both spheres must select s1 and s2 in strict inclusion mode"
        );
    }
    let away = [(5, 5), (40, 5), (40, 40), (5, 40)];
    for (overlap, mode) in [(false, "strict"), (true, "overlap")] {
        run.session().select_point(0, 0)?;
        run.session().select_pixels(&away, overlap)?;
        if run.session().is_selected("s1") || run.session().is_selected("s2") {
            println!("Error: polyline not covering any sphere must not select anything ({mode})");
        }
    }
    run.dump("bug_polyline_inclusion.png")?;
    run.finish()
}
