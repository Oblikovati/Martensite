//! Selection filters, per-mode sensitivity and perspective rectangle picks of small boxes.
use super::prelude::*;
use martensite_tkbrep::topods::Vertex;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use martensite_tkv3d::select_mgr::SelectMgrFilterType;
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// `bug28365`: a vertex filter keeps a whole box from being selected at the neutral point.
pub(crate) fn vertex_filter_blocks_object() -> Result<(), Box<dyn Error>> {
    banner(&["==========="]);
    banner(&[
        "0028365: Visualization, AisInteractiveContext - apply selection filter in AddOrRemoveSelected at Neutral point",
    ]);
    banner(&["===========", ""]);
    let b = box_sized([10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().add_shape_type_filter(ShapeEnum::Vertex);
    run.session().toggle_selected("b")?;
    expect_selected(&mut run, 0, "Error: Vertex filter was not applied");
    run.dump("bug28365.png")?;
    run.finish()
}

/// `bug26462_1`: selection follows a wider edge sensitivity of one box only.
pub(crate) fn sensitivity_follows_width() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26462: Visualization - selection does not adapt to line width change",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    for name in ["b1", "b2"] {
        let shape = box_sized([10., 10., 10.])?;
        run.session()
            .display(name, &shape, &DisplayRequest::default())?;
    }
    run.session().set_location("b1", [10., 10., 0.])?;
    run.session().fit()?;
    run.session().set_line_width("b1", 3.)?;
    run.session().set_line_width("b2", 3.)?;
    run.session().set_selection_mode(2, true)?;
    run.session().select_polyline(&[(305, 322)], Xor)?;
    run.session().select_polyline(&[(103, 322)], Xor)?;
    expect_selected(&mut run, 0, "ERROR: no boxes must be selected!");
    banner(&[
        "0029779: Visualization, SelectMgrSelectableObject - sensitivity change for object not activated in given mode leads to crash",
    ]);
    let b1 = run.session().named_objects(&["b1"])?.remove(0);
    run.session()
        .context()
        .set_selection_sensitivity(&b1, 3, 4)?;
    run.session()
        .context()
        .set_selection_sensitivity(&b1, 2, 4)?;
    run.session().select_point(305, 322)?;
    expect_selected(&mut run, 1, "ERROR: b1 was not selected");
    run.session().select_polyline(&[(103, 322)], Xor)?;
    expect_selected(
        &mut run,
        1,
        "ERROR: b2 is selected after b1's tolerance increased",
    );
    run.dump("bug26462_1.png")?;
    run.finish()
}

/// `bug25624`: a click in perspective selects the box under it, not a box behind.
pub(crate) fn perspective_pick_of_small_boxes() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC25624: Visualization - selection is incorrect in perspective mode in a specific case",
    ]);
    banner(&["========", ""]);
    let mut run = SessionRun::new()?;
    run.session()
        .update_view(|view| view.set_auto_z_fit_mode(true, view.auto_z_fit_scale_factor()))?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().set_display_mode(0)?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    for (name, z) in [("a", 0.), ("b", 0.5), ("c", 1.)] {
        let shape = box_at([0., 0., z], [0.1, 0.1, 0.1])?;
        run.session()
            .display(name, &shape, &DisplayRequest::default())?;
    }
    run.session().fit()?;
    run.session().select_point(210, 210)?;
    run.session().check_color(170, 10, [0.8, 0.8, 0.8])?;
    run.session().check_color(210, 184, [1., 1., 0.])?;
    run.dump("bug25624.png")?;
    run.finish()
}

/// `vertex name x y z`.
fn vertex([x, y, z]: [f64; 3]) -> Result<Vertex, Box<dyn Error>> {
    Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()?)
}

/// Toggle `name` into or out of the selection, then print `message` unless `count` owners
/// are selected; `toggle_expecting(&mut run, "e", 0, "Error")?`.
fn toggle_expecting(
    run: &mut SessionRun,
    name: &str,
    count: i32,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    run.session().toggle_selected(name)?;
    expect_selected(run, count, message);
    Ok(())
}

/// A context filter of `kind` over the vertex and edge type filters.
fn vertex_and_edge_filters(run: &mut SessionRun, kind: SelectMgrFilterType) {
    run.session().context().set_filter_type(kind);
    run.session().add_shape_type_filter(ShapeEnum::Vertex);
    run.session().add_shape_type_filter(ShapeEnum::Edge);
}

/// `bug31221`: vertex and edge filters combined by AND accept nothing, by OR both objects.
pub(crate) fn combined_context_filters() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===========",
        "0031221: Visualization - selection filter in context",
    ]);
    banner(&["===========", ""]);
    let v = vertex([0., 0., 0.])?.into_shape();
    let e = BRepBuilderAPIMakeEdge::from_vertices(&vertex([1., 0., 0.])?, &vertex([10., 0., 0.])?)?;
    let e = e.edge()?.into_shape();
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display("v", &v, &DisplayRequest::default())?;
    run.session().display("e", &e, &DisplayRequest::default())?;
    run.session().fit()?;
    vertex_and_edge_filters(&mut run, SelectMgrFilterType::And);
    let and = "Error: combined AND filter was not applied";
    toggle_expecting(&mut run, "e", 0, and)?;
    toggle_expecting(&mut run, "v", 0, and)?;
    run.session().toggle_selected("e")?;
    run.session().toggle_selected("v")?;
    vertex_and_edge_filters(&mut run, SelectMgrFilterType::Or);
    let or = "Error: combined OR filter was not applied";
    toggle_expecting(&mut run, "e", 1, or)?;
    toggle_expecting(&mut run, "v", 2, or)?;
    run.dump("bug31221.png")?;
    run.finish()
}

/// The pixels of the twelve box edges `bug7186` reads after widening them.
const BUG7186_EDGES: [(i32, i32); 12] = [
    (125, 47),
    (204, 100),
    (283, 47),
    (29, 200),
    (111, 150),
    (298, 150),
    (379, 200),
    (125, 250),
    (204, 300),
    (283, 250),
    (125, 361),
    (283, 361),
];

/// `bug7186`: a rectangle over empty space clears a selection of every box edge.
pub(crate) fn empty_rectangle_clears() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC7186: Selecting an empty space with rectangle doesn't clear",
    ]);
    banner(&[
        "the current selection, when there is an opened local context",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    show_box(&mut run, "b", [10., 10., 10.])?;
    run.session().set_selection_mode(2, true)?;
    run.session().select_pixels(&[(1, 1), (407, 407)], false)?;
    let mut mistake = run.session().selected_count() != 12;
    if mistake {
        println!("Bad number of selected edges after rectangle selection all edges");
    }
    run.session().select_pixels(&[(10, 10), (20, 20)], false)?;
    if run.session().selected_count() != 0 {
        println!("Error : Bad number of selected edges after rectangle selection an empty space");
        mistake = true;
    }
    run.session().set_line_width("b", 5.)?;
    println!();
    for (x, y) in BUG7186_EDGES {
        mistake |= run.session().pixel_color_name(x, y)? != "YELLOW";
    }
    println!();
    println!("OCC7186: {}", if mistake { "Faulty" } else { "OK" });
    run.dump("bug7186.png")?;
    run.finish()
}

/// Display a box of `size` and fit.
fn show_box(run: &mut SessionRun, name: &str, size: [f64; 3]) -> Result<(), Box<dyn Error>> {
    run.session()
        .display(name, &box_sized(size)?, &DisplayRequest::default())?;
    run.session().fit()?;
    Ok(())
}

/// Detect at `cursor`, check the given pixels, then move the cursor away.
fn hover_checks(
    run: &mut SessionRun,
    cursor: (i32, i32),
    checks: &[((i32, i32), [f64; 3])],
) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(cursor.0, cursor.1)?;
    for &((x, y), color) in checks {
        run.session().check_color(x, y, color)?;
    }
    Ok(())
}

/// `bug26413`: the pixel tolerance widens edge and vertex detection without overriding the
/// selection sensitivity.
pub(crate) fn pixel_tolerance_detection() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26413: Visualization, SelectMgrViewerSelector - Pixel tolerance is overridden by selection sensitivity",
    ]);
    banner(&["============", ""]);
    let (edge, vert) = ((320, 340), (383, 306));
    let (cyan, yellow) = ([0., 1., 1.], [1., 1., 0.]);
    let mut run = SessionRun::new()?;
    show_box(&mut run, "b", [1., 1., 1.])?;
    run.session().set_object_selection_mode("b", 2, true)?;
    for (tolerance, far) in [(None, yellow), (Some(8), cyan), (Some(-1), yellow)] {
        if let Some(pixels) = tolerance {
            run.session().context().set_pixel_tolerance(pixels);
        }
        hover_checks(&mut run, (228, 393), &[(edge, cyan)])?;
        run.session().move_cursor(0, 0)?;
        hover_checks(&mut run, (228, 400), &[(edge, far)])?;
        run.session().move_cursor(0, 0)?;
    }
    run.session().set_object_selection_mode("b", 1, true)?;
    for (tolerance, cursor) in [(-1, (374, 309)), (8, (372, 310))] {
        run.session().context().set_pixel_tolerance(tolerance);
        hover_checks(&mut run, cursor, &[(edge, yellow), (vert, cyan)])?;
        if tolerance < 0 {
            run.session().move_cursor(0, 0)?;
        }
    }
    run.dump("bug26413.png")?;
    run.selection_dump("bug26413_sel_ent.png", SelectionImage::COLORED_ENTITY)?;
    run.finish()
}
