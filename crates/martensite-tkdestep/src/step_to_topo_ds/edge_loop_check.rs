//! The check of the pcurves of a wire: the topological trimming parameters and the deviation
//! between 2D and 3D.
use crate::step_entity::step_live;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::elclib;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;

/// `RemoveSinglePCurve`.
pub(super) fn remove_single_pcurve(edge: &Edge, face: &Face) -> Result<(), Failure> {
    ShapeBuildEdge.remove_pcurve_on_face(edge, face)
}

/// `RemovePCurves`: every edge of the wire.
fn remove_pcurves(wire: &Wire, face: &Face) -> Result<(), Failure> {
    let mut explorer = Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while explorer.more() {
        remove_single_pcurve(&topods::edge(explorer.current())?, face)?;
        explorer.next()?;
    }
    Ok(())
}

/// The range of one edge on the face: w1 == w2 is illegal and drops the pcurve; on a bounded curve
/// the range is brought back between the ends; false when the pcurve is dropped.
fn check_range(edge: &Edge, face: &Face) -> Result<bool, Failure> {
    let builder = BRepBuilder::default();
    let (pcurve, mut w1, mut w2) = ShapeAnalysisEdge::new().pcurve_on_face(edge, face, false)?;
    let Some(live) = pcurve.get() else {
        return Ok(false);
    };
    let (cf, cl) = (live.first_parameter(), live.last_parameter());
    if w1 == w2 {
        remove_single_pcurve(edge, face)?;
        return Ok(false);
    }
    // check parameters of range only for not periodic curves
    if !live.is_periodic() {
        if w1 < cf {
            builder.range_on_face(edge, face, cf, w2)?;
            w1 = cf;
        }
        if w2 > cl {
            builder.range_on_face(edge, face, w1, cl)?;
            w2 = cl;
        }
    }
    let surface = BRepTool::surface(face)?;
    let surface = step_live(&surface, "StepToTopoDSTranslateEdgeLoop CheckPCurves")?;
    if w1 > w2 && surface.is_u_periodic() {
        let (u1, u2, _, _) = surface.bounds();
        let tolerance = ((w2 - w1).abs() / 2.0).min(precision::p_confusion());
        let (adjusted1, adjusted2) = elclib::adjust_periodic(u1, u2, tolerance, w1, w2);
        builder.range_on_face(edge, face, adjusted1, adjusted2)?;
    }
    Ok(true)
}

/// `CheckPCurves`: on a plane the pcurves are simply removed.
pub(super) fn check_pcurves(
    wire: &Wire,
    face: &Face,
    is_plane: bool,
    preci: f64,
) -> Result<(), Failure> {
    if is_plane {
        return remove_pcurves(wire, face);
    }
    let wire_data = ShapeExtendWireData::from_wire(wire, true, true)?;
    for rank in 1..=wire_data.nb_edges() {
        let edge = wire_data.edge(rank)?;
        if !check_range(&edge, face)? {
            continue;
        }
        // advanced check
        XSAlgoShapeProcessor::check_pcurve(&edge, face, preci, wire_data.is_seam(rank))?;
    }
    Ok(())
}
