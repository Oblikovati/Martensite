//! The two semantic checks that walk the sharing graph.
use super::*;
use crate::step_entity::*;
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_rw::*;
use crate::step_shape::{
    StepShapeEdgeCurve, StepShapeEdgeLoop, StepShapeFaceBound, StepShapeOrientedEdge,
    StepShapeVertexPoint,
};

/// The first entity of a type sharing `entity`, null when there is none (the `GetSharing`).
fn sharing_of_type<T: RuntimeTyped>(
    entity: &Handle<StandardTransient>,
    share_tool: &InterfaceShareTool,
) -> Result<Handle<T>, Failure> {
    let mut sharing = share_tool.sharings(entity)?;
    sharing.select_type(T::static_type(), true);
    if sharing.nb_entities() == 0 {
        return Ok(Handle::default());
    }
    Ok(sharing.value()?.cast::<T>())
}

/// The orientation of the face bound sharing an oriented edge, true when there is none.
fn face_bound_orientation(
    oriented_edge: &Handle<StepShapeOrientedEdge>,
    share_tool: &InterfaceShareTool,
) -> Result<bool, Failure> {
    let edge: Handle<StandardTransient> = oriented_edge.cast();
    if !share_tool.is_shared(&edge)? {
        return Ok(true);
    }
    let edge_loop: Handle<StandardTransient> =
        sharing_of_type::<StepShapeEdgeLoop>(&edge, share_tool)?.cast();
    if !share_tool.is_shared(&edge_loop)? {
        return Ok(true);
    }
    let face_bound = sharing_of_type::<StepShapeFaceBound>(&edge_loop, share_tool)?;
    Ok(face_bound.get().is_none_or(|bound| bound.orientation()))
}

/// Whether the ends are two distinct vertices at equal points, within `Precision::confusion`.
fn are_ends_match(edge_curve: &Handle<StepShapeEdgeCurve>) -> Result<bool, Failure> {
    let start = edge_curve.edge_start().cast::<StepShapeVertexPoint>();
    let end = edge_curve.edge_end().cast::<StepShapeVertexPoint>();
    if start == end {
        return Ok(false);
    }
    let where_called = "RwStepShapeRwEdgeCurve::Check";
    let start_point = step_live(&start, where_called)?
        .vertex_geometry()
        .cast::<StepGeomCartesianPoint>();
    let end_point = step_live(&end, where_called)?
        .vertex_geometry()
        .cast::<StepGeomCartesianPoint>();
    let (Some(first), Some(last)) = (start_point.get(), end_point.get()) else {
        return Ok(false);
    };
    let mut squared = 0.0;
    for rank in 1..=3 {
        let delta = first.coordinates_value(rank)? - last.coordinates_value(rank)?;
        squared += delta * delta;
    }
    Ok(squared.sqrt() < martensite_tkernel::precision::confusion())
}

/// Whether the face bound's orientation and the oriented edge's orientation differ.
fn is_cumulated(
    edge: &Handle<StepShapeOrientedEdge>,
    share_tool: &InterfaceShareTool,
) -> Result<bool, Failure> {
    let bound_orientation = face_bound_orientation(edge, share_tool)?;
    let oriented = step_live(edge, "RwStepShapeRwEdgeCurve::Check")?;
    Ok(bound_orientation != oriented.orientation())
}

impl RwStepShapeRwEdgeCurve {
    /// `Check`: coincident end points are a warning; an edge not referenced, or not used in opposite
    /// senses by exactly two oriented edges, is a failure.
    ///
    /// # Errors
    /// A failure of the sharing graph, or the null dereference of a vertex.
    pub fn check(
        &self,
        the_edge_curve: &Handle<StepShapeEdgeCurve>,
        the_share_tool: &InterfaceShareTool,
        the_message_tool: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if are_ends_match(the_edge_curve)? {
            the_message_tool.add_warning_text(
                "Two instances of Vertex have equal (within uncertainty) coordinates",
                "",
            );
        }
        let edge: Handle<StandardTransient> = the_edge_curve.cast();
        if !the_share_tool.is_shared(&edge)? {
            the_message_tool.add_fail_text("ERROR: EdgeCurve not referenced", "");
            return Ok(());
        }
        let mut sharing = the_share_tool.sharings(&edge)?;
        sharing.select_type(StepShapeOrientedEdge::static_type(), true);
        if sharing.nb_entities() != 2 {
            return Ok(());
        }
        let first = sharing.value()?.cast::<StepShapeOrientedEdge>();
        let first_cumulated = is_cumulated(&first, the_share_tool)?;
        sharing.next();
        let second = sharing.value()?.cast::<StepShapeOrientedEdge>();
        if first_cumulated == is_cumulated(&second, the_share_tool)? {
            the_message_tool.add_fail_text("ERROR: non 2-manifold topology", "");
        }
        Ok(())
    }
}

/// The face bound orientation of the other oriented edge: the running value when any link is missing.
fn other_face_bound_orientation(
    other_edge: &Handle<StepShapeOrientedEdge>,
    share_tool: &InterfaceShareTool,
    running: bool,
) -> Result<bool, Failure> {
    let other: Handle<StandardTransient> = other_edge.cast();
    if !share_tool.is_shared(&other)? {
        return Ok(running);
    }
    let mut sharing = share_tool.sharings(&other)?;
    sharing.select_type(StepShapeEdgeLoop::static_type(), true);
    sharing.start();
    let edge_loop = sharing.value()?;
    if !share_tool.is_shared(&edge_loop)? {
        return Ok(running);
    }
    let mut sharing = share_tool.sharings(&edge_loop)?;
    sharing.select_type(StepShapeFaceBound::static_type(), true);
    sharing.start();
    let face_bound = sharing.value()?.cast::<StepShapeFaceBound>();
    Ok(face_bound
        .get()
        .map_or(running, |bound| bound.orientation()))
}

impl RwStepShapeRwFaceBound {
    /// `Check`: each edge of the loop used by two oriented edges must be used in opposite senses,
    /// once the orientation of each face bound is taken into account.
    ///
    /// # Errors
    /// A failure of the sharing graph, or the null dereference of an oriented edge.
    pub fn check(
        &self,
        ent: &Handle<StepShapeFaceBound>,
        a_shto: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let where_called = "RwStepShapeRwFaceBound::Check";
        let bound_orientation = ent.orientation();
        // The variable outlives the loop, so an orientation found for one edge stays for
        // the next edge whose links are missing.
        let mut other_orientation = true;
        let edge_loop = ent.bound().cast::<StepShapeEdgeLoop>();
        let Some(the_loop) = edge_loop.get() else {
            return Ok(());
        };
        for rank in 1..=the_loop.nb_edge_list() {
            let oriented = the_loop.edge_list_value(rank)?;
            let oriented_ref = step_live(&oriented, where_called)?;
            let mut sharing = a_shto.sharings(&oriented_ref.edge_element().cast())?;
            sharing.select_type(StepShapeOrientedEdge::static_type(), true);
            if sharing.nb_entities() != 2 {
                continue;
            }
            let first = sharing.value()?.cast::<StepShapeOrientedEdge>();
            sharing.next();
            let second = sharing.value()?.cast::<StepShapeOrientedEdge>();
            let mut other = Handle::<StepShapeOrientedEdge>::default();
            if oriented == first {
                other = second;
            } else if oriented == second {
                other = first;
            }
            other_orientation = other_face_bound_orientation(&other, a_shto, other_orientation)?;
            let own_sense = bound_orientation == oriented_ref.orientation();
            let other_ref = step_live(&other, where_called)?;
            let other_sense = other_orientation == other_ref.orientation();
            if own_sense == other_sense {
                ach.add_fail_text("ERROR: non 2-manifold topology", "");
            }
        }
        Ok(())
    }
}
