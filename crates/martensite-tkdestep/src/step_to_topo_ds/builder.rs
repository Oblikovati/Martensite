//! `StepToTopoDSBuilder`: the solids and the surface and wireframe models. `init` on a geometric
//! set needs `StepControlActorRead` and is left to that package; the overloads on tessellated
//! items wait for StepVisual.
use super::{
    StepToTopoDSBuilderError, StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool,
    StepToTopoDSTranslateShell,
};
use crate::de_step::DeStepReadModeMaxPrecision;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeConnectedFaceSet, StepShapeFacetedBrep,
    StepShapeFacetedBrepAndBrepWithVoids, StepShapeManifoldSolidBrep, StepShapeOrientedClosedShell,
    StepShapeShellBasedSurfaceModel,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Compound, Shape, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// `ResetPreci`: :S4136: the tolerances of the result are limited when the mode asks for it.
fn reset_precision(
    process: &TransferTransientProcess,
    shape: &Shape,
    max_tol: f64,
) -> Result<(), Failure> {
    let model = process.model().cast::<StepDataStepModel>();
    let live = step_live(&model, "StepToTopoDSBuilder ResetPreci")?;
    let mode = live.internal_parameters().read_max_precision_mode;
    if mode != DeStepReadModeMaxPrecision::PREFERRED {
        ShapeFixShapeTolerance.limit_tolerance(
            shape,
            precision::confusion(),
            max_tol,
            ShapeEnum::Shape,
        )?;
    }
    Ok(())
}

/// A new tool for one translation.
pub(super) fn new_tool(process: &Handle<TransferTransientProcess>) -> StepToTopoDSTool {
    StepToTopoDSTool::with_map(&DataMap::default(), process)
}

/// The geometric statistics, sent at trace levels above 2.
fn send_statistics(process: &TransferTransientProcess, tool: &StepToTopoDSTool) {
    if process.trace_level() <= 2 {
        return;
    }
    process.messenger().send(&format!(
        "Geometric Statistics : \n   Surface Continuity : - C0 : {}\n                        - C1 : {}\n                        - C2 : {}\n   Curve Continuity :   - C0 : {}\n                        - C1 : {}\n                        - C2 : {}\n   PCurve Continuity :  - C0 : {}\n                        - C1 : {}\n                        - C2 : {}\n",
        tool.c0_surf(),
        tool.c1_surf(),
        tool.c2_surf(),
        tool.c0_cur3(),
        tool.c1_cur3(),
        tool.c2_cur3(),
        tool.c0_cur2(),
        tool.c1_cur2(),
        tool.c2_cur2(),
    ));
}

/// Builder.
#[derive(Default)]
pub struct StepToTopoDSBuilder {
    /// Root.
    pub root: StepToTopoDSRoot,
    pub(super) my_result: Shape,
    pub(super) my_error: StepToTopoDSBuilderError,
}

impl StepToTopoDSBuilder {
    /// `StepToTopoDSBuilder()`.
    pub fn new() -> Self {
        Self::default()
    }
    pub(super) fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSBuilderError::Done;
        self.root.done = true;
    }
    pub(super) fn fail(&mut self) {
        self.my_error = StepToTopoDSBuilderError::Other;
        self.root.done = false;
    }
    /// A shell translator with the tolerances of this builder.
    fn shell_translator(&self) -> StepToTopoDSTranslateShell {
        let mut translator = StepToTopoDSTranslateShell::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        translator
    }
    /// The solid of a closed outer shell, or `None` after the warning that it is not mapped.
    #[allow(
        clippy::too_many_arguments,
        reason = "the locals of the four solid overloads"
    )]
    fn outer_solid(
        &mut self,
        translator: &mut StepToTopoDSTranslateShell,
        outer: &Handle<StepShapeConnectedFaceSet>,
        warning: &str,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        process: &TransferTransientProcess,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Option<Solid>, Failure> {
        translator.init(outer, tool, nm_tool, factors, progress)?;
        if !translator.is_done() {
            let entity: Handle<StandardTransient> = outer.cast();
            process.add_warning_text(&entity, warning, "");
            self.fail();
            return Ok(None);
        }
        let shape = translator.value()?.clone();
        shape.set_closed(true)?;
        let builder = BRepBuilder::default();
        let mut solid = Solid::default();
        builder.make_solid(&mut solid);
        builder.add(&mut solid, &shape)?;
        Ok(Some(solid))
    }
    /// `init(StepShapeManifoldSolidBrep, TP, factors, progress)`.
    pub fn init_manifold_solid(
        &mut self,
        solid_brep: &Handle<StepShapeManifoldSolidBrep>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let mut tool = new_tool(tp);
        let mut nm_tool = StepToTopoDSNMTool::new();
        let mut translator = self.shell_translator();
        let outer = step_live(solid_brep, "StepToTopoDSBuilder")?.outer().cast();
        let warning = " OuterShell from ManifoldSolidBrep not mapped to TopoDS";
        let Some(solid) = self.outer_solid(
            &mut translator,
            &outer,
            warning,
            &mut tool,
            &mut nm_tool,
            process,
            factors,
            progress,
        )?
        else {
            return Ok(());
        };
        let shape = solid.into_shape();
        self.finish(shape.clone());
        send_statistics(process, &tool);
        reset_precision(process, &shape, self.root.max_tol())
    }
    /// `init(StepShapeBrepWithVoids, TP, factors, progress)`.
    pub fn init_brep_with_voids(
        &mut self,
        brep: &Handle<StepShapeBrepWithVoids>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let live = step_live(brep, "StepToTopoDSBuilder")?;
        let mut tool = new_tool(tp);
        // Non-manifold topology is not referenced by BrepWithVoids (ssv; 14.11.2010)
        let mut nm_tool = StepToTopoDSNMTool::new();
        let mut translator = self.shell_translator();
        let count = live.nb_voids();
        let scope = ProgressScope::with_literal(progress, "Shell", f64::from(count + 1), false);
        let outer = live.outer().cast();
        let warning = " OuterShell from BrepWithVoids not mapped to TopoDS";
        let range = scope.next(1.0);
        let Some(mut solid) = self.outer_solid(
            &mut translator,
            &outer,
            warning,
            &mut tool,
            &mut nm_tool,
            process,
            factors,
            &range,
        )?
        else {
            return Ok(());
        };
        drop(range);
        self.my_error = StepToTopoDSBuilderError::Done;
        self.root.done = true;
        let builder = BRepBuilder::default();
        let mut rank = 1;
        while rank <= count && scope.more() {
            let shell = live.voids_value(rank)?;
            let range = scope.next(1.0);
            translator.init(&shell.cast(), &mut tool, &mut nm_tool, factors, &range)?;
            if translator.is_done() {
                let mut shape = translator.value()?.clone();
                shape.set_closed(true)?;
                //:e0 abv 25 Mar 98: void should be an OrientedShell
                if shell.get().is_some_and(|oriented| !oriented.orientation()) {
                    shape.reverse();
                }
                builder.add(&mut solid, &shape)?;
            } else {
                let entity: Handle<StandardTransient> = shell.cast();
                process.add_warning_text(
                    &entity,
                    " A Void from BrepWithVoids not mapped to TopoDS",
                    "",
                );
            }
            rank += 1;
        }
        let shape = solid.into_shape();
        self.my_result = shape.clone();
        send_statistics(process, &tool);
        //:S4136 ShapeFix::SameParameter (S,false);
        reset_precision(process, &shape, self.root.max_tol())
    }
    /// `init(StepShapeFacetedBrep, TP, factors, progress)`.
    pub fn init_faceted_brep(
        &mut self,
        brep: &Handle<StepShapeFacetedBrep>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let mut tool = new_tool(tp);
        // Non-manifold topology is not referenced by FacetedBrep (ss; 14.11.2010)
        let mut nm_tool = StepToTopoDSNMTool::new();
        let mut translator = self.shell_translator();
        let outer = step_live(brep, "StepToTopoDSBuilder")?.outer().cast();
        let warning = " OuterShell from FacetedBrep not mapped to TopoDS";
        let Some(solid) = self.outer_solid(
            &mut translator,
            &outer,
            warning,
            &mut tool,
            &mut nm_tool,
            process,
            factors,
            progress,
        )?
        else {
            return Ok(());
        };
        tool.clear_edge_map();
        tool.clear_vertex_map();
        self.finish(solid.into_shape());
        Ok(())
    }
    /// `init(StepShapeFacetedBrepAndBrepWithVoids, TP, factors, progress)`.
    pub fn init_faceted_brep_with_voids(
        &mut self,
        brep: &Handle<StepShapeFacetedBrepAndBrepWithVoids>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let live = step_live(brep, "StepToTopoDSBuilder")?;
        let mut tool = new_tool(tp);
        // Non-manifold topology is not referenced by FacetedBrepAndBrepWithVoids (ss; 14.11.2010)
        let mut nm_tool = StepToTopoDSNMTool::new();
        let mut translator = self.shell_translator();
        let root_scope = ProgressScope::unnamed(progress, 2.0, false);
        let outer = live.outer().cast();
        let warning = " OuterShell from FacetedBrepAndBrepWithVoids not mapped to TopoDS";
        let range = root_scope.next(1.0);
        let Some(mut solid) = self.outer_solid(
            &mut translator,
            &outer,
            warning,
            &mut tool,
            &mut nm_tool,
            process,
            factors,
            &range,
        )?
        else {
            return Ok(());
        };
        drop(range);
        let count = live.nb_voids()?;
        let voids_range = root_scope.next(1.0);
        let scope = ProgressScope::unnamed(&voids_range, f64::from(count), false);
        let builder = BRepBuilder::default();
        let mut rank = 1;
        while rank <= count && scope.more() {
            let shell: Handle<StepShapeOrientedClosedShell> = live.voids_value(rank)?;
            let range = scope.next(1.0);
            translator.init(&shell.cast(), &mut tool, &mut nm_tool, factors, &range)?;
            if translator.is_done() {
                // Unlike BrepWithVoids, this overload does not reverse a void by its orientation.
                let shape = translator.value()?.clone();
                shape.set_closed(true)?;
                builder.add(&mut solid, &shape)?;
            } else {
                let entity: Handle<StandardTransient> = shell.cast();
                process.add_warning_text(
                    &entity,
                    " A Void from FacetedBrepAndBrepWithVoids not mapped to TopoDS",
                    "",
                );
            }
            rank += 1;
        }
        self.finish(solid.into_shape());
        Ok(())
    }
    /// `init(StepShapeShellBasedSurfaceModel, TP, NMTool, factors, progress)`: more than one
    /// boundary gives a compound, else the shell.
    pub fn init_shell_based_surface_model(
        &mut self,
        model: &Handle<StepShapeShellBasedSurfaceModel>,
        tp: &Handle<TransferTransientProcess>,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let live = step_live(model, "StepToTopoDSBuilder")?;
        let mut tool = new_tool(tp);
        let count = live.nb_sbsm_boundary()?;
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut shell_shape = Shape::default();
        let mut translator = self.shell_translator();
        let scope = ProgressScope::with_literal(progress, "Shell", f64::from(count), false);
        let mut rank = 1;
        while rank <= count && scope.more() {
            let range = scope.next(1.0);
            let boundary = live.sbsm_boundary_value(rank)?;
            let (open, closed) = (boundary.open_shell(), boundary.closed_shell());
            let (shell_entity, is_closed, warning): (
                Handle<StepShapeConnectedFaceSet>,
                bool,
                &str,
            ) = if open.is_some() {
                (
                    open.cast(),
                    false,
                    " OpenShell from ShellBasedSurfaceModel not mapped to TopoDS",
                )
            } else {
                (
                    closed.cast(),
                    true,
                    " ClosedShell from ShellBasedSurfaceModel not mapped to TopoDS",
                )
            };
            rank += 1;
            if shell_entity.is_null() {
                continue;
            }
            translator.init(&shell_entity, &mut tool, nm_tool, factors, &range)?;
            if translator.is_done() {
                shell_shape = topods::shell(translator.value()?)?.into_shape();
                shell_shape.set_closed(is_closed)?;
                builder.add(&mut compound, &shell_shape)?;
            } else {
                let entity: Handle<StandardTransient> = shell_entity.cast();
                process.add_warning_text(&entity, warning, "");
            }
        }
        let compound = compound.into_shape();
        self.finish(if count > 1 {
            compound.clone()
        } else {
            shell_shape.clone()
        });
        send_statistics(process, &tool);
        //:S4136 ShapeFix::SameParameter (S,false);
        reset_precision(process, &compound, self.root.max_tol())?;
        reset_precision(process, &shell_shape, self.root.max_tol()) // skl
    }
    /// The end of the two model overloads. The error code is inverted: a result gives
    /// `other` and no result gives `done`; `is_done` is the reliable answer.
    pub(super) fn finish_model(
        &mut self,
        process: &TransferTransientProcess,
    ) -> Result<(), Failure> {
        let has_result = !self.my_result.is_null();
        self.my_error = if has_result {
            StepToTopoDSBuilderError::Other
        } else {
            StepToTopoDSBuilderError::Done
        };
        self.root.done = has_result;
        reset_precision(process, &self.my_result, self.root.max_tol())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure without a result.
    pub fn value(&self) -> Result<&Shape, Failure> {
        self.root.check_done("StepToTopoDSBuilder::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSBuilderError {
        self.my_error
    }
}
