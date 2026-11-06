//! `TopoDSToStepBuilder`: a shell or a face to a connected_face_set or a face, with a tessellated
//! item alongside as the write mode asks.
use super::{
    TopoDSToStepBuilderError, TopoDSToStepMakeStepFace, TopoDSToStepMakeTessellatedItem,
    TopoDSToStepRoot, TopoDSToStepTool,
};
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_shape::{
    StepShapeClosedShell, StepShapeConnectedFaceSet, StepShapeFace, StepShapeOpenShell,
    StepShapeTopologicalRepresentationItem,
};
use crate::step_visual::StepVisualTessellatedItem;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape, Shell};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::transfer::TransferFinderProcess;

/// Builder.
#[derive(Clone, Default)]
pub struct TopoDSToStepBuilder {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: Handle<StepShapeTopologicalRepresentationItem>,
    my_tessellated_result: Handle<StepVisualTessellatedItem>,
    my_error: TopoDSToStepBuilderError,
}

/// The faces a shape explores to.
fn count_faces(shape: &Shape) -> Result<i32, Failure> {
    let mut count = 0;
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        count += 1;
        faces.next()?;
    }
    Ok(count)
}

/// The closed or open shell over the given faces.
fn connected_face_set(
    faces: Vec<Handle<StepShapeFace>>,
    closed: bool,
) -> Handle<StepShapeConnectedFaceSet> {
    let (name, set) = (empty_step_name(), shared_array1(faces));
    if closed {
        let shell = Handle::new(StepShapeClosedShell::new());
        shell.init(&name, &set);
        return shell.cast();
    }
    let shell = Handle::new(StepShapeOpenShell::new());
    shell.init(&name, &set);
    shell.cast()
}

impl TopoDSToStepBuilder {
    /// `TopoDSToStepBuilder()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepBuilder(aShape, aTool, FP, theTessellatedGeomParam, theLocalFactors, theProgress)`.
    pub fn with_shape(
        shape: &Shape,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        tessellated_geom_param: i32,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(shape, tool, fp, tessellated_geom_param, factors, progress)?;
        Ok(made)
    }
    /// `init(shape, tool, FP, tessellated_geom_param, local_factors, progress)`.
    pub fn init(
        &mut self,
        shape: &Shape,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        tessellated_geom_param: i32,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        if tool.is_bound(shape) {
            self.my_error = TopoDSToStepBuilderError::BuilderDone;
            self.root.done = true;
            self.my_result = tool.find(shape)?;
            return Ok(());
        }
        match shape.shape_type()? {
            ShapeEnum::Shell => {
                let shell = topods::shell(shape)?;
                self.init_shell(&shell, tool, fp, tessellated_geom_param, factors, progress)
            }
            ShapeEnum::Face => {
                let face = topods::face(shape)?;
                self.init_face(&face, tool, fp, tessellated_geom_param, factors, progress)
            }
            _ => Ok(()),
        }
    }
    /// CKY  9-DEC-1997 (PRO9824 et consorts)
    /// Pour passer les orientations: ELLES SONT DONNEES EN RELATIF
    /// Donc, pour SHELL, on doit l ecrire en direct en STEP (pas le choix)
    /// -> il faut repercuter en dessous, donc explorer le Shell TEL QUEL
    /// Pour FACE WIRE, d une part on ECRIT SON ORIENTATION relative au contenant
    ///   (puisqu on peut), d autre part on EXPLORE EN FORWARD: ainsi les
    ///   orientations des sous-shapes sont relatives a leur contenant immediat
    ///   et la recombinaison en lecture est sans malice
    fn init_shell(
        &mut self,
        shell: &Shell,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        tessellated_geom_param: i32,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        tool.set_current_shell(shell);
        let steps = if tessellated_geom_param != 0 {
            2.0
        } else {
            1.0
        };
        let outer = ProgressScope::unnamed(progress, steps, false);
        let count = count_faces(shell)?;
        let faces_range = outer.next(1.0);
        let scope = ProgressScope::unnamed(&faces_range, f64::from(count), false);
        let mut faces = Vec::new();
        let mut maker = TopoDSToStepMakeStepFace::new();
        let mut explorer = Explorer::new(shell, ShapeEnum::Face, ShapeEnum::Shape)?;
        while explorer.more() && scope.more() {
            let face = topods::face(explorer.current())?;
            maker.init(&face, tool, fp, factors)?;
            // MakeFace Error Handling: warning only
            if maker.is_done() {
                faces.push(maker.value()?.cast::<StepShapeFace>());
            }
            explorer.next()?;
            let _ = scope.next(1.0);
        }
        if !scope.more() {
            return Ok(());
        }
        if faces.is_empty() {
            // Builder Error handling;
            self.my_error = TopoDSToStepBuilderError::NoFaceMapped;
            self.root.done = false;
        } else {
            let entity = connected_face_set(faces, shell.closed()?);
            // --------------------------------------------------------------
            // To add later: if not facetted context & shell is reversed
            //                then shall create an oriented_shell with
            //                orientation flag to false.
            // --------------------------------------------------------------
            tool.bind(shell, &entity.cast());
            self.my_result = entity.cast();
            self.root.done = true;
        }
        if tessellated_geom_param == 1 || (tessellated_geom_param == 2 && self.my_result.is_null())
        {
            let range = outer.next(1.0);
            let tessellated =
                TopoDSToStepMakeTessellatedItem::with_shell(shell, tool, fp, factors, &range)?;
            if tessellated.is_done() {
                self.my_tessellated_result = tessellated.value()?;
                self.my_error = TopoDSToStepBuilderError::BuilderDone;
                self.root.done = true;
            }
        }
        Ok(())
    }
    fn init_face(
        &mut self,
        face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        tessellated_geom_param: i32,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let maker = TopoDSToStepMakeStepFace::with_face(face, tool, fp, factors)?;
        let mut tessellated = TopoDSToStepMakeTessellatedItem::new();
        if tessellated_geom_param == 1 || (tessellated_geom_param == 2 && !maker.is_done()) {
            let scope = ProgressScope::unnamed(progress, 1.0, false);
            // fourth parameter is true in order to create a tessellated_surface_set entity
            // or put false to create a triangulated_face instead
            tessellated.init_face(face, tool, fp, true, factors, &scope.next(1.0))?;
        }
        if !maker.is_done() && !tessellated.is_done() {
            // MakeFace Error Handling: Face not Mapped
            self.my_error = TopoDSToStepBuilderError::BuilderOther;
            self.root.done = false;
            return Ok(());
        }
        if maker.is_done() {
            self.my_result = maker.value()?;
        }
        if tessellated.is_done() {
            self.my_tessellated_result = tessellated.value()?;
        }
        self.my_error = TopoDSToStepBuilderError::BuilderDone;
        self.root.done = true;
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`: the topological item.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepShapeTopologicalRepresentationItem>, Failure> {
        self.root.check_done("TopoDSToStepBuilder::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `tessellated_value`: the optional tessellated item.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn tessellated_value(&self) -> Result<Handle<StepVisualTessellatedItem>, Failure> {
        self.root
            .check_done("TopoDSToStepBuilder::TessellatedValue()")?;
        Ok(self.my_tessellated_result.clone())
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepBuilderError {
        self.my_error
    }
}
