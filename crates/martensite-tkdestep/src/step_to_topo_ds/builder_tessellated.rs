//! `StepToTopoDSBuilder`: the four tessellated overloads of `init`.
use super::builder::new_tool;
use super::translate_tessellated::checked_face;
use super::{
    StepToTopoDSBuilder, StepToTopoDSBuilderError, StepToTopoDSNMTool, StepToTopoDSTranslateFace,
    StepToTopoDSTranslateShell, StepToTopoDSTranslateSolid,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_visual::{
    StepVisualTessellatedFace, StepVisualTessellatedShell, StepVisualTessellatedSolid,
    StepVisualTessellatedSurfaceSet,
};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::transfer::TransferTransientProcess;

impl StepToTopoDSBuilder {
    /// The end of a tessellated overload.
    fn finish_tessellated(
        &mut self,
        tp: &Handle<TransferTransientProcess>,
        entity: &Handle<StandardTransient>,
        made: Option<Shape>,
        warning: &str,
    ) -> Result<(), Failure> {
        let Some(shape) = made else {
            step_live(tp, "StepToTopoDSBuilder")?.add_warning_text(entity, warning, "");
            self.my_error = StepToTopoDSBuilderError::Other;
            self.root.done = false;
            return Ok(());
        };
        self.my_result = shape;
        self.my_error = StepToTopoDSBuilderError::Done;
        self.root.done = true;
        Ok(())
    }
    /// `init(t_so, tp, read_tessellated_when_no_b_rep_only, has_geom, factors, progress)`.
    pub fn init_tessellated_solid(
        &mut self,
        solid: &Handle<StepVisualTessellatedSolid>,
        tp: &Handle<TransferTransientProcess>,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let mut translator = StepToTopoDSTranslateSolid::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let mut tool = new_tool(tp);
        let mut nm_tool = StepToTopoDSNMTool::new();
        translator.init(
            solid,
            tp,
            &mut tool,
            &mut nm_tool,
            only_no_brep,
            has_geom,
            factors,
            progress,
        )?;
        let made = if translator.is_done() {
            Some(topods::solid(translator.value()?)?.into_shape())
        } else {
            None
        };
        self.finish_tessellated(
            tp,
            &solid.cast(),
            made,
            " TessellatedSolid not mapped to TopoDS",
        )
    }
    /// `init(t_sh, tp, read_tessellated_when_no_b_rep_only, has_geom, factors, progress)`.
    pub fn init_tessellated_shell(
        &mut self,
        shell: &Handle<StepVisualTessellatedShell>,
        tp: &Handle<TransferTransientProcess>,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let mut translator = StepToTopoDSTranslateShell::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let mut tool = new_tool(tp);
        let mut nm_tool = StepToTopoDSNMTool::new();
        translator.init_tessellated(
            shell,
            &mut tool,
            &mut nm_tool,
            only_no_brep,
            has_geom,
            factors,
            progress,
        )?;
        let made = if translator.is_done() {
            Some(topods::shell(translator.value()?)?.into_shape())
        } else {
            None
        };
        self.finish_tessellated(
            tp,
            &shell.cast(),
            made,
            " TessellatedShell not mapped to TopoDS",
        )
    }
    /// `init(tf, tp, read_tessellated_when_no_b_rep_only, has_geom, factors)`.
    pub fn init_tessellated_face(
        &mut self,
        face: &Handle<StepVisualTessellatedFace>,
        tp: &Handle<TransferTransientProcess>,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let mut tool = new_tool(tp);
        let mut nm_tool = StepToTopoDSNMTool::new();
        translator.init_tessellated_face(
            face,
            &mut tool,
            &mut nm_tool,
            only_no_brep,
            has_geom,
            factors,
        )?;
        let made = if translator.is_done() {
            Some(checked_face(translator.value()?)?)
        } else {
            None
        };
        self.finish_tessellated(
            tp,
            &face.cast(),
            made,
            " TessellatedFace not mapped to TopoDS",
        )
    }
    /// `init(tss, tp, has_geom, factors)`.
    pub fn init_tessellated_surface_set(
        &mut self,
        set: &Handle<StepVisualTessellatedSurfaceSet>,
        tp: &Handle<TransferTransientProcess>,
        has_geom: &mut bool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let mut tool = new_tool(tp);
        let mut nm_tool = StepToTopoDSNMTool::new();
        translator.init_tessellated_surface_set(set, &mut tool, &mut nm_tool, factors)?;
        *has_geom = false;
        let made = if translator.is_done() {
            Some(checked_face(translator.value()?)?)
        } else {
            None
        };
        self.finish_tessellated(
            tp,
            &set.cast(),
            made,
            " TessellatedSurfaceSet not mapped to TopoDS",
        )
    }
}
