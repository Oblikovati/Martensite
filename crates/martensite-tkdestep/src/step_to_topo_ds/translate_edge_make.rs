//! `StepToTopoDSTranslateEdge`: the edge on a 3D curve, the closed and the null-length cases.
use super::StepToTopoDSTranslateEdge;
use super::StepToTopoDSTranslateEdgeError;
use super::translate_edge::{
    EdgeInProgress, cartesian_points, decode_make_edge_error, make_curve, make_edge,
    shift_displaced_line,
};
use super::{StepToTopoDSGeometricTool, StepToTopoDSTool};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomCurve;
use crate::step_shape::{StepShapeEdgeCurve, StepShapeVertex};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Edge, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkmath::gp::{Direction3, Point3, Vector3};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisCurve;
use martensite_tktopalgo::brep_lib::{BRepLibEdgeError, BRepLibMakeEdge};
use martensite_tkxsbase::transfer::TransferTransientProcess;

impl StepToTopoDSTranslateEdge {
    /// `MakeFromCurve3D(C3D, EC, Vend, preci, E, V1, V2, T, theLocalFactors)`: `edge` from a 3D curve
    /// between two vertices, which may change.
    ///
    /// # Errors
    /// The exceptions of the translation of the curve and of the edge.
    #[allow(clippy::too_many_arguments, reason = "the member's parameters")]
    pub fn make_from_curve_3d(
        &mut self,
        c3d: &Handle<StepGeomCurve>,
        edge_curve: &Handle<StepShapeEdgeCurve>,
        v_end: &Handle<StepShapeVertex>,
        preci: f64,
        edge: &mut Edge,
        v1: &mut Vertex,
        v2: &mut Vertex,
        tool: &mut StepToTopoDSTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let mut state = EdgeInProgress {
            entity: edge_curve,
            v_end,
            preci,
            v1: v1.clone(),
            v2: v2.clone(),
            edge: edge.clone(),
        };
        let outcome = self.make_from_curve_3d_in(c3d, &mut state, tool, factors);
        (*v1, *v2, *edge) = (state.v1, state.v2, state.edge);
        outcome
    }

    /// `MakeFromCurve3D`: the case of a 3D curve, alone or in a surface curve. On a null edge
    /// (`done` stays true) the result is nullified and `state.edge` stays null, so
    /// `init` binds the null edge as the code does.
    pub(super) fn make_from_curve_3d_in(
        &mut self,
        c3d: &Handle<StepGeomCurve>,
        state: &mut EdgeInProgress<'_>,
        tool: &mut StepToTopoDSTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let process_handle = tool.transient_process();
        let process = step_live(
            &process_handle,
            "StepToTopoDSTranslateEdge::MakeFromCurve3D",
        )?;
        let entity: Handle<StandardTransient> = c3d.cast();
        let mut curve = make_curve(c3d, process, factors)?;
        let Some(live) = curve.get() else {
            process.add_fail_text(&entity, " Make GeomCurve (3D) failed", "");
            self.my_error = StepToTopoDSTranslateEdgeError::Other;
            self.root.done = false;
            return Ok(());
        };
        // -- Statistics -- -> No Warning message
        tool.add_continuity_curve(live);
        let (pv1, pv2) = (BRepTool::pnt(&state.v1)?, BRepTool::pnt(&state.v2)?);
        let mut points = [pv1, pv2];
        if state.v1.is_same(&state.v2) {
            cartesian_points(
                step_live(state.entity, "StepToTopoDSTranslateEdge")?,
                &mut points,
                factors,
            )?;
        }
        let (mut u1, mut u2) = Self::project_on_curve(&curve, &points, state.preci)?;
        if !StepToTopoDSGeometricTool::update_param_3d(&curve, &mut u1, &mut u2, state.preci)? {
            process.add_warning_text(&entity, "Update of 3D-Parameters has failed", "");
        }
        //:d5: instead of AdjustCurve above which is incorrect if U1 and U2 are not ends
        // The edge maker checks the vertices through the adaptor, whose rounding differs from the
        // curve's own evaluation by more than the margin of the tolerances set below.
        let adaptor = GeomAdaptorCurve::with_curve(&curve)?;
        let temp1 = adaptor.value(u1)?.distance(&pv1);
        let temp2 = adaptor.value(u2)?.distance(&pv2);
        if temp1 > state.preci || temp2 > state.preci {
            let chord = points[0].distance(&points[1]);
            let displaced = (temp1 - temp2).abs() < state.preci
                && (u2 - u1 - chord).abs() < precision::confusion();
            match shift_displaced_line(&curve, points[0], u1).filter(|_| displaced) {
                Some(shifted) => {
                    curve = shifted;
                    process.add_warning_text(
                        &entity,
                        "Poor result from projection vertex / line 3d, line shifted",
                        "",
                    );
                }
                None => process.add_warning_text(
                    &entity,
                    "Poor result from projection vertex / curve 3d",
                    "",
                ),
            }
        }
        let builder = BRepBuilder::default();
        //:h6 abv 14 Jul 98: PRO8845 #2746: *=1.0001
        builder.update_vertex_tolerance(&state.v1, 1.000001 * temp1)?;
        builder.update_vertex_tolerance(&state.v2, 1.000001 * temp2)?;
        let mut maker =
            BRepLibMakeEdge::from_curve_vertices_params(&curve, &state.v1, &state.v2, u1, u2)?;
        if maker.is_done() {
            state.edge = maker.edge()?;
            builder.range(&state.edge, u1, u2, false)?; // abv 14 Mar 00: trj3_pm1-ug.stp #91739, edge 2
            return Ok(());
        }
        if maker.error() != BRepLibEdgeError::DifferentPointsOnClosedCurve {
            decode_make_edge_error(&maker, &entity, &curve, &state.v1, &state.v2, process)?;
            state.edge = make_edge(&curve, &state.v1, &state.v2, u1, u2)?;
            self.my_error = StepToTopoDSTranslateEdgeError::Done;
            self.root.done = true;
            return Ok(());
        }
        if step_live(&curve, "StepToTopoDSTranslateEdge::MakeFromCurve3D")?.is_closed()? {
            return self
                .make_closed_with_two_vertices(&curve, &entity, &mut maker, state, u1, u2, tool);
        }
        self.make_null_length_edge(state, process)
    }
    /// The two parameters of the projected points; the adjustment comes after.
    pub(super) fn project_on_curve(
        curve: &Handle<dyn GeomCurve>,
        points: &[Point3; 2],
        preci: f64,
    ) -> Result<(f64, f64), Failure> {
        let live = step_live(curve, "StepToTopoDSTranslateEdge::MakeFromCurve3D")?;
        let range = (live.first_parameter(), live.last_parameter());
        let (_, _, u1) = ShapeAnalysisCurve.project(curve, points[0], preci, range, false)?;
        let (_, _, u2) = ShapeAnalysisCurve.project(curve, points[1], preci, range, false)?;
        Ok((u1, u2))
    }
    /// The edge could be closed and trimmed by two different vertices: the topology is updated.
    #[allow(
        clippy::too_many_arguments,
        reason = "the locals of one branch of MakeFromCurve3D"
    )]
    pub(super) fn make_closed_with_two_vertices(
        &mut self,
        curve: &Handle<dyn GeomCurve>,
        entity: &Handle<StandardTransient>,
        maker: &mut BRepLibMakeEdge,
        state: &mut EdgeInProgress<'_>,
        u1: f64,
        u2: f64,
        tool: &mut StepToTopoDSTool,
    ) -> Result<(), Failure> {
        let process_handle = tool.transient_process();
        let process = step_live(
            &process_handle,
            "StepToTopoDSTranslateEdge::MakeFromCurve3D",
        )?;
        // Attention: topology updating
        tool.bind(&state.v_end.cast(), &state.v1)?;
        state.v2 = topods::vertex(&state.v1.reversed())?;
        maker.init_curve_vertices_params(curve, &state.v1, &state.v2, u1, u2)?;
        if maker.is_done() {
            process.add_warning_text(
                &state.entity.cast(),
                "Wrong topology corrected : Closed Edge with TWO different Vertices",
                "",
            );
            state.edge = maker.edge()?;
            return Ok(());
        }
        decode_make_edge_error(maker, entity, curve, &state.v1, &state.v2, process)?;
        state.edge = make_edge(curve, &state.v1, &state.v2, u1, u2)?;
        self.my_error = StepToTopoDSTranslateEdgeError::Done;
        self.root.done = true;
        Ok(())
    }
    /// A curve too small to be made: "Bon, on la fait cette petite edge, mais faudra repasser pour
    /// l enlever ET FUSIONNER LES VERTEX, pour tout le shell !".
    pub(super) fn make_null_length_edge(
        &mut self,
        state: &mut EdgeInProgress<'_>,
        process: &TransferTransientProcess,
    ) -> Result<(), Failure> {
        self.my_error = StepToTopoDSTranslateEdgeError::Done;
        self.root.done = true;
        if state.v1.is_same(&state.v2) {
            process.add_fail_text(&state.entity.cast(), "NULL EDGE, SKIPPED", "");
            self.my_result.nullify();
            return Ok(());
        }
        process.add_fail_text(&state.entity.cast(), "This edge has null arc length", "");
        let (p1, p2) = (BRepTool::pnt(&state.v1)?, BRepTool::pnt(&state.v2)?);
        let direction = Direction3::from_vector(Vector3::between(p1, p2))?;
        let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(p1, direction)).cast();
        state.edge = make_edge(&line, &state.v1, &state.v2, 0.0, p1.distance(&p2))?;
        Ok(())
    }
}
