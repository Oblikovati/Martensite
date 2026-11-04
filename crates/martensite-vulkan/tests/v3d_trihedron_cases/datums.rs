//! Datum objects built directly on the context: a selected circle and trihedron, plane
//! trihedrons on a box face, a trihedron with one selection mode, and a type filter.
use super::prelude::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::resource::ResourceHost;
use martensite_tkernel::standard::{Failure, SharedState};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkernel::units::Units;
use martensite_tkernel::units_api::UnitsApi;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve as _;
use martensite_tkg3d::geom::GeomCircle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gc::MakePlane;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource as _;
use martensite_tkv3d::ais::{
    AisCircle, AisInteractiveObject, AisKindOfInteractive, AisPlaneTrihedron, AisTypeFilter,
};
use std::sync::Arc;

/// `buc60814`: a trihedron and a circle both selected show the blue selection colour.
pub(crate) fn selected_circle_and_trihedron() -> Outcome {
    banner(&["========", "BUC60814", "========", ""]);
    let mut run = SessionRun::new()?;
    selected_datums(run.session().context())?;
    run.session().fit()?;
    if !run.session().check_color(131, 352, [0., 0., 1.])? {
        println!("Error : Circle has NOT blue color.");
    }
    if !run.session().check_color(170, 319, [0., 0., 1.])? {
        println!("Error : Trihedron has NOT blue color.");
    }
    run.dump("buc60814.png")?;
    run.finish()
}

/// What the `BUC60814` command builds: an XOY trihedron and a circle of radius 20 about X at
/// (10, 10, 10), both added to the selection after the selection colour becomes BLUE1.
fn selected_datums(context: &martensite_tkv3d::ais::AisInteractiveContext) -> Result<(), Failure> {
    let trihedron: Handle<dyn AisInteractiveObject> =
        super::trihedron_options::new_trihedron()?.cast();
    context.display(&trihedron, false)?;
    let axis = Axis2::new(Point3::new(10., 10., 10.), Direction3::new(1., 0., 0.)?)?;
    let circle: Handle<dyn AisInteractiveObject> =
        AisCircle::new(&Handle::new(GeomCircle::new(axis, 20.)?)).cast();
    context.display(&circle, false)?;
    if let Some(style) = context.selection_style().get() {
        style.set_color(Color::named(NamedColor::BLUE1));
    }
    context.add_or_remove_selected_object(&trihedron, false)?;
    context.add_or_remove_selected_object(&circle, true)
}

/// A units context for plane trihedrons, as the viewer creates once per session.
fn session_units() -> Arc<SharedState<UnitsApi>> {
    let units = UnitsApi::new(Arc::new(Units::new()), ResourceHost::native());
    Arc::new(SharedState::new(units))
}

/// The plane of a face through two points of its second edge and the middle of its first.
fn face_plane_points(face: &Shape) -> Result<[Point3; 3], Failure> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let first = BRepAdaptorCurve::with_edge(&topods::edge(explorer.current())?)?;
    if !explorer.more() {
        return Ok([first.value(0.1)?, first.value(0.9)?, first.value(0.5)?]);
    }
    explorer.next()?;
    let second = BRepAdaptorCurve::with_edge(&topods::edge(explorer.current())?)?;
    Ok([second.value(0.1)?, second.value(0.9)?, first.value(0.5)?])
}

/// A plane trihedron on the plane of `face`, as `OCC10`/`OCC218` build it.
fn plane_trihedron(face: &Shape) -> Result<Handle<AisPlaneTrihedron>, Failure> {
    let [a, b, c] = face_plane_points(face)?;
    let plane = MakePlane::through_points(a, b, c)?.value()?;
    AisPlaneTrihedron::new(&plane, session_units())
}

/// `bug10`: a plane trihedron of length 5 on a box face reports the length it was given.
pub(crate) fn face_plane_trihedron() -> Outcome {
    banner(&["================", "OCC10", "================", ""]);
    let mut run = SessionRun::new()?;
    let face = face_of(&box_sized([10., 10., 10.])?, 1);
    run.session()
        .display("b_1", &face, &DisplayRequest::default())?;
    run.session().fit()?;
    let trihedron = plane_trihedron(&face)?;
    trihedron.set_length(5.);
    run.session().bind_object("trihedron1", trihedron.cast())?;
    run.session()
        .update_context(|context| context.display(&trihedron.cast(), true))?;
    let length = trihedron.get_length();
    println!("Length = 5\ngetLength = {length}");
    println!(
        "{}",
        if length == 5. {
            "OCC10: OK"
        } else {
            "OCC10: ERROR"
        }
    );
    // Coincident-line locus controls (the recording tool): the plane
    // trihedron's axis lies on the face edge at equal depth.
    let controls = [
        ImageControl {
            suffix: "locus-1",
            erased: &["trihedron1"],
        },
        ImageControl {
            suffix: "locus-2",
            erased: &["b_1"],
        },
    ];
    run.dump_with_controls("bug10.png", &controls)?;
    run.finish()
}

/// `bug218`: plane trihedron labels drawn near the axes once the labels are renamed.
pub(crate) fn labelled_plane_trihedron() -> Outcome {
    banner(&["================", "OCC218", "================", ""]);
    let mut run = SessionRun::new()?;
    let face = face_of(&box_sized([10., 10., 10.])?, 1);
    run.session()
        .display("b_1", &face, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_view_parameters(&bug218_view())?;
    let trihedron = plane_trihedron(&face)?;
    set_labels(&trihedron, "X", "Y");
    run.session().bind_object("trihedron1", trihedron.cast())?;
    run.session()
        .update_context(|context| context.display(&trihedron.cast(), true))?;
    run.session().check_color(388, 28, [0., 0., 0.])?;
    set_labels(&trihedron, "my___axis___1", "my___axis2");
    run.session().update_context(|context| {
        context.redisplay(&trihedron.cast(), false, false)?;
        context.update_current_viewer();
        Ok(())
    })?;
    if !run.session().check_color(388, 28, [0.85, 0.85, 0.])? {
        println!("There is no messages near axes");
    }
    run.dump("bug218.png")?;
    run.finish()
}

fn set_labels(trihedron: &AisPlaneTrihedron, x: &str, y: &str) {
    trihedron.set_x_label(&AsciiString::from(x));
    trihedron.set_y_label(&AsciiString::from(y));
}

/// The literal view parameters (scale, projection, up, at) of `bug218`.
fn bug218_view() -> crate::case_view::viewer_session::ViewParameters {
    crate::case_view::viewer_session::ViewParameters {
        scale: Some(7674.87317785833),
        proj: Some([0.966540098190308, -0.24304473400116, 0.0820330902934074]),
        up: Some([-0.0460147373378277, 0.150333747267723, 0.987563848495483]),
        at: Some([0.0466426680664981, 0.147133996816294, 9.95295385008357]),
        ..Default::default()
    }
}

/// `bug22368`: the highlight of a trihedron selected in mode 3 follows its relocation.
pub(crate) fn single_mode_trihedron() -> Outcome {
    banner(&[
        "========================",
        "OCC22368: AisTrihedron's highlight presentation isn't updated on trihedron relocation",
        "========================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display_trihedron("trihedron")?;
    redisplay_with_one_mode(run.session(), "trihedron", 3)?;
    run.session().select_point(210, 190)?;
    run.session()
        .set_location("trihedron", [100., 100., 100.])?;
    for x in 200..=220 {
        run.session().check_color(x, 190, [0., 0., 0.])?;
    }
    run.dump("bug22368.png")?;
    run.finish()
}

/// What `OCC74_set name mode` does: erase, stop automatic activation, display again and
/// activate only `mode`.
fn redisplay_with_one_mode(
    session: &crate::case_view::viewer_session::ViewerSession,
    name: &str,
    mode: i32,
) -> Result<(), Failure> {
    let object = session.object(name).cloned().unwrap_or_default();
    let context = session.context();
    crate::context_mode_transition::redisplay_selection_mode_on_context(context, &object, mode)
}

/// `bug25340`: with a shape-type filter, picking the trihedron leaves its colour unchanged.
pub(crate) fn shape_type_filter() -> Outcome {
    banner(&[
        "========",
        "0025340: Visualization, AisInteractiveContext - properly apply selection filters at Neutral point",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session()
        .display("b", &box_sized([1., 2., 3.])?, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().display_trihedron("t")?;
    let before = run.session().pixel_color_name(87, 25)?;
    let filter = AisTypeFilter::new(AisKindOfInteractive::Shape);
    run.session()
        .context()
        .add_filter(&Handle::new(filter).cast());
    run.session().select_point(87, 25)?;
    let after = run.session().pixel_color_name(87, 25)?;
    report_filter_colours(&before, &after);
    // Coincident-line locus controls: the trihedron axes lie on the box edges at equal depth.
    let controls = [
        ImageControl {
            suffix: "locus-1",
            erased: &["t"],
        },
        ImageControl {
            suffix: "locus-2",
            erased: &["b"],
        },
    ];
    run.dump_with_controls("bug25340.png", &controls)?;
    run.finish()
}

fn report_filter_colours(before: &str, after: &str) {
    if after != before {
        println!("ERROR: OCC25340 is reproduced");
        println!("       initial color of trihedron is: {before}");
        println!("       final color of trihedron is:   {after}");
    }
    if after != "LIGHTSTEELBLUE4" {
        println!("ERROR: OCC25340 is reproduced");
        println!("       additional investigation is needed");
        println!("       expected color is: LIGHTSTEELBLUE4");
        println!("       current color is:  {after}");
    }
}

#[cfg(test)]
#[path = "context_mode_baseline.rs"]
mod context_mode_baseline;
