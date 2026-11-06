//! `TopoDSToStepMakeTessellatedItem`: the triangulation of a face or a shell to tessellated
//! entities.
use super::tessellated_items::face_triangulation_item;
use super::{TopoDSToStepRoot, TopoDSToStepTool, add_warning};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_shape::StepShapeConnectedFaceSet;
use crate::step_visual::{
    StepVisualTessellatedItem, StepVisualTessellatedShell, StepVisualTessellatedStructuredItem,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shell};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkxsbase::transfer::TransferFinderProcess;

/// Make tessellated item.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeTessellatedItem {
    /// Root.
    pub root: TopoDSToStepRoot,
    the_tessellated_item: Handle<StepVisualTessellatedItem>,
}

impl TopoDSToStepMakeTessellatedItem {
    /// `TopoDSToStepMakeTessellatedItem()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepMakeTessellatedItem(theFace, theTool, theFP, theToPreferSurfaceSet, theLocalFactors, theProgress)`.
    pub fn with_face(
        face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        to_prefer_surface_set: bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init_face(face, tool, fp, to_prefer_surface_set, factors, progress)?;
        Ok(made)
    }
    /// `TopoDSToStepMakeTessellatedItem(theShell, theTool, theFP, theLocalFactors, theProgress)`.
    pub fn with_shell(
        shell: &Shell,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init_shell(shell, tool, fp, factors, progress)?;
        Ok(made)
    }
    /// `init(face, ...)`: a triangulated_face, or a triangulated_surface_set when the face has
    /// no entity and the caller prefers one.
    pub fn init_face(
        &mut self,
        face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        to_prefer_surface_set: bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.root.done = false;
        if progress.user_break() {
            return Ok(());
        }
        let (mesh, _) = BRepTool::triangulation(face, mesh_purpose::NONE)?;
        if mesh.is_null() {
            return add_warning(fp, face, " Face not mapped to TessellatedItem");
        }
        self.the_tessellated_item =
            face_triangulation_item(&mesh, face, tool, to_prefer_surface_set, factors)?;
        self.root.done = true;
        Ok(())
    }
    /// `init(shell, ...)`: a tessellated_shell over the faces' items.
    pub fn init_shell(
        &mut self,
        shell: &Shell,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.root.done = false;
        self.the_tessellated_item = Handle::default();
        if progress.user_break() {
            return Ok(());
        }
        let mut count = 0;
        let mut faces = Explorer::new(shell, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() {
            count += 1;
            faces.next()?;
        }
        let scope = ProgressScope::unnamed(progress, f64::from(count), false);
        let mut items: Vec<Handle<StepVisualTessellatedStructuredItem>> = Vec::new();
        faces.init(shell, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() && scope.more() {
            let face = topods::face(faces.current())?;
            // Advances the scope once for the range and once for the loop.
            let range = scope.next(1.0);
            let maker = Self::with_face(&face, tool, fp, false, factors, &range)?;
            if maker.is_done() {
                items.push(maker.value()?.cast());
            }
            faces.next()?;
            let _ = scope.next(1.0);
        }
        if items.is_empty() {
            return Ok(());
        }
        let mut face_set = Handle::<StepShapeConnectedFaceSet>::default();
        if tool.is_bound(shell) {
            face_set = tool.find(shell)?.cast();
        }
        let tess_shell = Handle::new(StepVisualTessellatedShell::new());
        tess_shell.init(
            &empty_step_name(),
            &shared_array1(items),
            face_set.is_some(),
            &face_set,
        );
        self.the_tessellated_item = tess_shell.cast();
        self.root.done = true;
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepVisualTessellatedItem>, Failure> {
        self.root
            .check_done("TopoDSToStepMakeTessellatedItem::Value()")?;
        Ok(self.the_tessellated_item.clone())
    }
}
