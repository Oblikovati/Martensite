//! `bugs_modalg_6`: the hidden-line cases.
use crate::support::checks::{NbShapesCheck, checknbshapes, compound};
use crate::support::hlr::HlrSession;
use crate::support::restore::{Restored, restore};
use crate::support::script::{Reply, Script, ScriptError, ScriptStep};
use crate::support::session::{Session, checkview_display};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;

fn header(draw: &mut Script, title: &str, subject: &str) {
    for line in ["============", title, "============", ""] {
        draw.puts(line);
    }
    draw.puts("###############################");
    draw.puts(&format!("## {subject}"));
    draw.puts("###############################");
}

fn failure(error: martensite_tkernel::standard::Failure) -> ScriptError {
    ScriptError(format!("An exception was caught {error}"))
}

/// The HLR pipeline both scripts run on `shape`, with the projector `<prefix>_proj` on
/// the `hprj` frame and the outliner `<prefix>_outl`: hprj, houtl, hfill (no isolines),
/// hload, hsetprj, hupdate, hhide, hres2d.
fn project(
    draw: &mut Script,
    view: &mut dyn Session,
    shape: &str,
    prefix: &str,
    frame: [&str; 9],
) -> ScriptStep {
    let (projector, outliner) = (format!("{prefix}_proj"), format!("{prefix}_outl"));
    let mut hlr = HlrSession::new().map_err(failure)?;
    let mut args = vec![projector.as_str()];
    args.extend(frame);
    hlr.hprj(draw, &args)?;
    hlr.houtl(draw, &outliner, shape)?;
    hlr.hfill(draw, &outliner, &projector, "0")?;
    hlr.hload(draw, &outliner)?;
    hlr.hsetprj(draw, &projector)?;
    hlr.hupdate(draw)?;
    hlr.hhide(draw)?;
    hlr.hres2d(draw, view)
}

/// bugs/modalg_6/bug25214: HLR fails to project edges. The visible sharp edges of the
/// diagonal polyline (0,0,0)-(10,10,10) seen along Z must be one edge.
pub fn bug25214(draw: &mut Script, view: &mut dyn Session) -> ScriptStep {
    header(draw, "OCC25214", "HLR fails to project edges");
    draw.command(
        &["polyline", "Line", "0", "0", "0", "10", "10", "10"],
        |draw| {
            // BRepTest polyline: one BRepBuilderAPI_MakePolygon::Add per point.
            let mut polygon = BRepBuilderAPIMakePolygon::new();
            polygon.add_point(Point3::new(0., 0., 0.))?;
            polygon.add_point(Point3::new(10., 10., 10.))?;
            let wire = polygon.wire()?.into();
            draw.set_shape("Line", &wire);
            view.set("Line", &Restored::Shape(wire));
            Ok(Reply::ok(""))
        },
    )?;
    let frame = ["0", "0", "0", "0", "0", "1", "1", "0", "0"];
    project(draw, view, "Line", "Shape2DView", frame)?;
    let check = NbShapesCheck {
        expected: [None, Some(1), None, None, None, None, None, None, None],
        total: false,
        message: "HLRToShape",
        reference: None,
        args_text: "-edge 1 -m HLRToShape",
    };
    checknbshapes(draw, "vl", &check)
}

/// The `nbshapes_expected` reference text of bug25908, as the script writes it.
const BUG25908_REFERENCE: &str = "
Number of shapes in shape
 VERTEX    : 109
 EDGE      : 55
 WIRE      : 0
 FACE      : 0
 SHELL     : 0
 SOLID     : 0
 COMPSOLID : 0
 COMPOUND  : 1
 SHAPE     : 165
";

/// bugs/modalg_6/bug25908: B-spline (U2 < U1) in HLRBRep_HLRToShape. The visible outputs
/// of the hidden-line projection of the original B-spline shape along Y have the exact
/// reference counts (`nbshapes -t`), and the result is displayed and captured in 2D.
pub fn bug25908(draw: &mut Script, view: &mut dyn Session) -> ScriptStep {
    header(draw, "OCC25908", "BSpline (U2 < U1) in HLRBRep_HLRToShape");
    let file = "bug25908_hlr-bspline-clone2-tcl-h1.brep";
    draw.command(&["restore", file, "h1"], |draw| {
        let shape = restore(file).shape();
        draw.set_shape("h1", &shape);
        view.set("h1", &Restored::Shape(shape));
        Ok(Reply::ok("h1"))
    })?;
    let frame = ["0", "0", "0", "0", "1", "0", "0", "0", "1"];
    project(draw, view, "h1", "h1", frame)?;
    compound(draw, view, &["vl", "v1l", "vnl", "vol", "vil"], "result")?;
    let args_text = format!("-ref {{{BUG25908_REFERENCE}}} -t -m HLRToShape");
    let check = NbShapesCheck {
        expected: [None; 9],
        total: true,
        message: "HLRToShape",
        reference: Some(BUG25908_REFERENCE),
        args_text: &args_text,
    };
    checknbshapes(draw, "result", &check)?;
    draw.puts("checkview -display result -2d -path bug25908.png");
    let shown = (!draw.shape("result").is_null()).then_some("result");
    checkview_display(view, shown, "bug25908.png");
    Ok(())
}
