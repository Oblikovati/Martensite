//! The preparation passes of an edge loop: vertices and 3D curves first, so that a closed curve
//! limited by distinct vertices gives one vertex when they have the same coordinates.
use super::{StepToTopoDSNMTool, StepToTopoDSTool, StepToTopoDSTranslateVertex};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomCurve, StepGeomSurfaceCurve};
use crate::step_shape::{
    StepShapeEdgeCurve, StepShapeEdgeLoop, StepShapeOrientedEdge, StepShapeVertex,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// The 3D curve of an edge, made once for the transfer. A failure is recorded, never raised.
fn prepare_curve(
    entity: &Handle<StepGeomCurve>,
    process: &TransferTransientProcess,
    factors: &StepDataFactors,
) -> Handle<dyn GeomCurve> {
    let start: Handle<StandardTransient> = entity.cast();
    let known: Handle<dyn GeomCurve> = process.find_transient(&start).cast();
    if known.is_some() {
        return known;
    }
    let bound = StepToGeom::make_curve(entity, factors).and_then(|made| {
        if made.is_some() {
            process.bind_transient(&start, &made.cast())?;
        } else {
            process.add_warning_text(
                &start,
                "Could not convert a curve. Curve definition is incorrect",
                "",
            );
        }
        Ok(made)
    });
    bound.unwrap_or_else(|_| {
        process.add_fail_text(
            &start,
            "Exception was raised. Curve geometry definition is incorrect",
            "",
        );
        Handle::default()
    })
}

/// The 3D curve entity of an edge curve: the one of a surface curve, or the geometry itself.
fn curve_3d_entity(edge_curve: &StepShapeEdgeCurve) -> Handle<StepGeomCurve> {
    let curve = edge_curve.edge_geometry();
    match curve.cast::<StepGeomSurfaceCurve>().get() {
        Some(surface_curve) => surface_curve.curve3d(),
        None => curve,
    }
}

/// The oriented edge of a list entry that holds an edge curve, with bug #29979 unwrapped (an
/// oriented edge containing another oriented edge). Warns and gives `None` otherwise.
fn usable_edge(
    entry: &Handle<StepShapeOrientedEdge>,
    process: &TransferTransientProcess,
) -> Result<Option<Handle<StepShapeEdgeCurve>>, Failure> {
    let entity: Handle<StandardTransient> = entry.cast();
    let element = entry
        .get()
        .map(|live| live.edge_element())
        .unwrap_or_default();
    if element.is_null() {
        process.add_warning_text(
            &entity,
            "Incorrect Oriented_Edge is not translated. Oriented_Edge definition is not correct",
            "",
        );
        return Ok(None);
    }
    let nested = element.cast::<StepShapeOrientedEdge>();
    let (holder, inner) = match nested.get() {
        Some(live) => (
            nested.clone().cast::<StandardTransient>(),
            live.edge_element(),
        ),
        None => (entity, element),
    };
    let edge_curve = inner.cast::<StepShapeEdgeCurve>();
    if edge_curve.is_null() {
        process.add_warning_text(&holder, "Edge does not contain EDGE_CURVE, skipped", "");
        return Ok(None);
    }
    Ok(Some(edge_curve))
}

/// Vertices of the same coordinates are made one: gka 21.08.1998 bug PRO7656.
fn merge_confused_vertices(
    edge_curve: &StepShapeEdgeCurve,
    curve: &Handle<dyn GeomCurve>,
    list_entry: &Handle<StandardTransient>,
    tool: &mut StepToTopoDSTool,
    nm_tool: &mut StepToTopoDSNMTool,
    process: &TransferTransientProcess,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let (v_start, v_end) = if edge_curve.same_sense() {
        (edge_curve.edge_start(), edge_curve.edge_end())
    } else {
        (edge_curve.edge_end(), edge_curve.edge_start())
    };
    let start_was_bound = tool.is_bound(&v_start.cast());
    let end_was_bound = tool.is_bound(&v_end.cast());
    let first = StepToTopoDSTranslateVertex::with_vertex(&v_start, tool, nm_tool, factors)?;
    let last = StepToTopoDSTranslateVertex::with_vertex(&v_end, tool, nm_tool, factors)?;
    let mut v1 = Vertex::default();
    if first.is_done() {
        v1 = topods::vertex(first.value()?)?;
    }
    if !last.is_done() {
        return Ok(());
    }
    let v2 = topods::vertex(last.value()?)?;
    if BRepTool::pnt(&v1)?.distance(&BRepTool::pnt(&v2)?) > precision::confusion() {
        return Ok(());
    }
    if !end_was_bound || start_was_bound {
        tool.bind(&v_end.cast(), &v1)?;
    } else {
        tool.bind(&v_start.cast(), &v2)?;
    }
    let is_open = match curve.get() {
        Some(live) => !live.is_closed()?,
        None => false,
    };
    if is_open {
        process.add_warning_text(list_entry, "Vertex of same coordinates, set confused", "");
    }
    Ok(())
}

/// The first pass: make the vertices and the 3D curves of every edge of the loop.
pub(super) fn prepare_vertices_and_curves(
    edge_loop: &StepShapeEdgeLoop,
    tool: &mut StepToTopoDSTool,
    nm_tool: &mut StepToTopoDSNMTool,
    process: &TransferTransientProcess,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    for rank in 1..=edge_loop.nb_edge_list() {
        let entry = edge_loop.edge_list_value(rank)?;
        let Some(edge_curve) = usable_edge(&entry, process)? else {
            continue;
        };
        let live = step_live(&edge_curve, "StepToTopoDSTranslateEdgeLoop")?;
        let entity = curve_3d_entity(live);
        let curve = if entity.is_some() {
            prepare_curve(&entity, process, factors)
        } else {
            Handle::default()
        };
        merge_confused_vertices(live, &curve, &entry.cast(), tool, nm_tool, process, factors)?;
    }
    Ok(())
}

/// The vertices at the junction of two oriented edges and at their other ends.
struct Junction {
    end_of_first: Handle<StepShapeVertex>,
    start_of_second: Handle<StepShapeVertex>,
    start_of_first: Handle<StepShapeVertex>,
    end_of_second: Handle<StepShapeVertex>,
}

impl Junction {
    fn new(
        oriented1: &StepShapeOrientedEdge,
        curve1: &StepShapeEdgeCurve,
        oriented2: &StepShapeOrientedEdge,
        curve2: &StepShapeEdgeCurve,
    ) -> Self {
        let (start1, end1) = (curve1.edge_start(), curve1.edge_end());
        let (start2, end2) = (curve2.edge_start(), curve2.edge_end());
        let (start_of_first, end_of_first) = if oriented1.orientation() {
            (start1, end1)
        } else {
            (end1, start1)
        };
        let (start_of_second, end_of_second) = if oriented2.orientation() {
            (start2, end2)
        } else {
            (end2, start2)
        };
        Self {
            end_of_first,
            start_of_second,
            start_of_first,
            end_of_second,
        }
    }
    /// The edges already share a vertex.
    fn is_connected(&self) -> bool {
        self.end_of_first == self.start_of_second
            || self.end_of_first == self.end_of_second
            || self.start_of_second == self.start_of_first
            || self.end_of_second == self.start_of_first
    }
}

/// True when the two confused vertices could be made one: this is only possible while one of the
/// edges is not yet translated, else nothing will help.
fn bind_confused_junction(
    junction: &Junction,
    (v1, v2): (&Vertex, &Vertex),
    (curve1, curve2): (&Handle<StepShapeEdgeCurve>, &Handle<StepShapeEdgeCurve>),
    preci: f64,
    tool: &mut StepToTopoDSTool,
) -> Result<bool, Failure> {
    if BRepTool::pnt(v1)?.distance(&BRepTool::pnt(v2)?) > preci {
        return Ok(false);
    }
    if !tool.is_bound(&curve1.cast()) {
        tool.bind(&junction.end_of_first.cast(), v2)?;
        return Ok(true);
    }
    if !tool.is_bound(&curve2.cast()) {
        tool.bind(&junction.start_of_second.cast(), v1)?;
        return Ok(true);
    }
    Ok(false)
}

/// The second pass: :f6 abv 29 Apr 98: BUC50070 #3815: make sure that each two edges are connected
/// by the same vertex; else check that the vertices confuse and make them one vertex.
pub(super) fn connect_adjacent_edges(
    edge_loop: &StepShapeEdgeLoop,
    loop_entity: &Handle<StandardTransient>,
    preci: f64,
    tool: &mut StepToTopoDSTool,
    nm_tool: &mut StepToTopoDSNMTool,
    process: &TransferTransientProcess,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let count = edge_loop.nb_edge_list();
    for rank in 1..=count {
        let oriented1 = edge_loop.edge_list_value(rank)?;
        let oriented2 = edge_loop.edge_list_value(if rank < count { rank + 1 } else { 1 })?;
        let (Some(live1), Some(live2)) = (oriented1.get(), oriented2.get()) else {
            continue;
        };
        let curve1 = live1.edge_element().cast::<StepShapeEdgeCurve>();
        let curve2 = live2.edge_element().cast::<StepShapeEdgeCurve>();
        // see #29979
        let (Some(edge1), Some(edge2)) = (curve1.get(), curve2.get()) else {
            continue;
        };
        let junction = Junction::new(live1, edge1, live2, edge2);
        if junction.is_connected() {
            continue;
        }
        let first = StepToTopoDSTranslateVertex::with_vertex(
            &junction.end_of_first,
            tool,
            nm_tool,
            factors,
        )?;
        let second = StepToTopoDSTranslateVertex::with_vertex(
            &junction.start_of_second,
            tool,
            nm_tool,
            factors,
        )?;
        if !first.is_done() || !second.is_done() {
            continue; // not treated
        }
        let (v1, v2) = (
            topods::vertex(first.value()?)?,
            topods::vertex(second.value()?)?,
        );
        if v1.is_null() || v2.is_null() || v1.is_same(&v2) {
            continue; // not treated, or OK
        }
        let fixed = bind_confused_junction(&junction, (&v1, &v2), (&curve1, &curve2), preci, tool)?;
        let text = if fixed {
            "Adjacent edges do not have common vertex; set confused"
        } else {
            "Adjacent edges are not connected"
        };
        process.add_warning_text(loop_entity, text, "");
    }
    Ok(())
}
