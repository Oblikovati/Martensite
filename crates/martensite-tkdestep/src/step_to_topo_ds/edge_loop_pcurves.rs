//! `StepToTopoDSTranslateEdgeLoop`: the pcurves of an edge on the face.
use super::StepToTopoDSTranslateEdgeLoop;
use super::translate_edge_loop::{EdgeLoopLocals, FaceContext};
use super::{StepToTopoDSGeometricTool, StepToTopoDSTool, StepToTopoDSTranslateEdge};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomPcurve, StepGeomSurfaceCurve};
use crate::step_shape::StepShapeEdgeCurve;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::{self, Orientation};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisCurve;
use martensite_tkxsbase::transfer::TransferTransientProcess;

impl StepToTopoDSTranslateEdgeLoop {
    /// The pcurves of an edge from its geometry, then the update of the edge. False is a
    /// `continue`: a failure the edge is left out for.
    #[allow(clippy::too_many_arguments, reason = "the locals of the loop body")]
    pub(super) fn update_edge_pcurves(
        &mut self,
        translator: &StepToTopoDSTranslateEdge,
        edge: &Edge,
        edge_curve: &Handle<StepShapeEdgeCurve>,
        context: &FaceContext<'_>,
        locals: &mut EdgeLoopLocals,
        tool: &mut StepToTopoDSTool,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let live_curve = step_live(edge_curve, "StepToTopoDSTranslateEdgeLoop")?;
        let curve = live_curve.edge_geometry();
        let mut is_seam = false;
        let mut is_like_seam = false;
        let mut there_is_like_seam = false;
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateEdgeLoop")?;
        let curve_entity: Handle<StandardTransient> = edge_curve.cast();
        let pcurve = curve.cast::<StepGeomPcurve>();
        let surface_curve = curve.cast::<StepGeomSurfaceCurve>();
        if curve.is_null() {
            tool.set_compute_pcurve(true);
            locals.has_pcurve = false;
        } else if pcurve.is_some() {
            // CASE 1: The Edge Geometry is of Pcurve Type
            locals.c2d = translator.make_pcurve(&pcurve, &context.converted_surface, factors)?;
            // -- Statistics --
            tool.add_continuity_curve_2d(step_live(&locals.c2d, "StepToTopoDSTranslateEdgeLoop")?);
        } else if context.is_c3d_only {
            // CASE 2 with modepcurve = 3: the pcurves are ignored here
            tool.set_compute_pcurve(true);
            locals.has_pcurve = false;
        } else if surface_curve.is_some() {
            // recouvre les cas SeamCurve et IntersectionCurve
            let seams = self.read_surface_curve(
                translator,
                &surface_curve,
                edge_curve,
                context,
                locals,
                tool,
                factors,
            )?;
            (is_seam, is_like_seam, there_is_like_seam) = seams;
            if !locals.has_pcurve {
                // The edge geometry has no 2D representation
                tool.set_compute_pcurve(true);
            }
        } else {
            // CASE 3: a single 3D curve
            tool.set_compute_pcurve(true);
            locals.has_pcurve = false;
        }
        let builder = BRepBuilder::default();
        if locals.has_pcurve && (is_seam || there_is_like_seam) {
            return self.update_seam_edge(edge, &curve_entity, is_seam, context, locals, process);
        }
        if !locals.has_pcurve {
            return Ok(true);
        }
        // The Edge is a "normal" edge
        if locals.c2d.is_null() || is_like_seam {
            process.add_fail_text(&curve_entity, " Edge: Trimming of 2D curve failed", "");
            self.fail_edge();
            return Ok(false);
        }
        builder.update_edge_pcurve_on_face(edge, &locals.c2d, context.face, 0.0)?;
        Ok(true)
    }
    /// A surface curve: its pcurves on the face, with the special mapping of a seam curve or what
    /// is like one (see CATIA cylinder). Gives (is seam, is like seam, there is a like seam).
    #[allow(clippy::too_many_arguments, reason = "the locals of the loop body")]
    pub(super) fn read_surface_curve(
        &mut self,
        translator: &StepToTopoDSTranslateEdge,
        surface_curve: &Handle<StepGeomSurfaceCurve>,
        edge_curve: &Handle<StepShapeEdgeCurve>,
        context: &FaceContext<'_>,
        locals: &mut EdgeLoopLocals,
        tool: &mut StepToTopoDSTool,
        factors: &StepDataFactors,
    ) -> Result<(bool, bool, bool), Failure> {
        let (mut last, mut step_pcurve) =
            StepToTopoDSGeometricTool::pcurve(surface_curve, context.step_surface, 0)?;
        // De toute facon, on recalcule
        locals.has_pcurve = step_pcurve.is_some() && !context.is_plane;
        let step_edge = edge_curve.cast();
        let is_like_seam = StepToTopoDSGeometricTool::is_like_seam(
            surface_curve,
            context.step_surface,
            &step_edge,
            context.edge_loop,
        )?;
        let is_seam = StepToTopoDSGeometricTool::is_seam_curve(
            surface_curve,
            context.step_surface,
            &step_edge,
            context.edge_loop,
        )?;
        if is_seam || is_like_seam {
            let live = step_live(surface_curve, "StepToTopoDSTranslateEdgeLoop")?;
            // isLikeSeam = Two faces on the same Surface
            let first = live.associated_geometry_value(1)?.pcurve();
            let second = live.associated_geometry_value(2)?.pcurve();
            if first.is_null() || second.is_null() {
                locals.has_pcurve = false; // smh: BUC60810
            } else {
                locals.c2d1 =
                    translator.make_pcurve(&first, &context.converted_surface, factors)?;
                locals.c2d2 =
                    translator.make_pcurve(&second, &context.converted_surface, factors)?;
                locals.has_pcurve = locals.c2d1.is_some() && locals.c2d2.is_some();
            }
            if is_like_seam {
                locals.has_pcurve = true;
            }
            return Ok((is_seam, is_like_seam, is_like_seam));
        }
        if !locals.has_pcurve {
            return Ok((false, false, false));
        }
        // GeometricTool: Pcurve a retourne StepPCurve
        while last > 0 {
            locals.c2d1 =
                translator.make_pcurve(&step_pcurve, &context.converted_surface, factors)?;
            if locals.c2d1.is_null() {
                let process = tool.transient_process();
                step_live(&process, "StepToTopoDSTranslateEdgeLoop")?.add_warning_text(
                    &edge_curve.cast(),
                    "Incorrect pcurve is not translated. Pcurve definition is not correct",
                    "",
                );
                locals.has_pcurve = false;
                break;
            }
            locals.c2d = locals.c2d1.clone();
            (last, step_pcurve) =
                StepToTopoDSGeometricTool::pcurve(surface_curve, context.step_surface, last)?;
            // -- Statistics --
            tool.add_continuity_curve_2d(step_live(&locals.c2d, "StepToTopoDSTranslateEdgeLoop")?);
        }
        Ok((false, false, false))
    }
    /// A seam edge: the pcurve which is FORWARD has to be identified. False is a `continue`.
    pub(super) fn update_seam_edge(
        &mut self,
        edge: &Edge,
        curve_entity: &Handle<StandardTransient>,
        is_seam: bool,
        context: &FaceContext<'_>,
        locals: &EdgeLoopLocals,
        process: &TransferTransientProcess,
    ) -> Result<bool, Failure> {
        let not_mapped = |this: &mut Self| {
            process.add_fail_text(curve_entity, " Seam curve not mapped", "");
            this.fail_edge();
            Ok(false)
        };
        if locals.c2d1.is_null() || locals.c2d2.is_null() {
            return not_mapped(self);
        }
        let wire_orientation = if context.forward_wire {
            Orientation::Forward
        } else {
            Orientation::Reversed
        };
        let face_orientation = if context.same_sense {
            Orientation::Forward
        } else {
            Orientation::Reversed
        };
        let cumulated = top_abs::compose(
            top_abs::compose(edge.orientation(), wire_orientation),
            face_orientation,
        );
        let mut forward_pcurve =
            ShapeAnalysisCurve.select_forward_seam(&locals.c2d1, &locals.c2d2)?;
        if forward_pcurve == 0 {
            return not_mapped(self);
        }
        if cumulated != Orientation::Forward {
            forward_pcurve = 3 - forward_pcurve; // inverser 1-2
        }
        let (first, second) = if forward_pcurve == 1 {
            (&locals.c2d1, &locals.c2d2)
        } else {
            (&locals.c2d2, &locals.c2d1)
        };
        let builder = BRepBuilder::default();
        if is_seam {
            // When the edge is a Seam, it is better to find the topological trimming right now.
            builder.update_edge_pcurves_on_face(edge, (first, second), context.face, 0.0)?;
        } else {
            builder.update_edge_pcurve_on_face(edge, first, context.face, 0.0)?;
        }
        Ok(true)
    }
}
