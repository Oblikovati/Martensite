//! `StepToTopoDSTranslateShell`: a connected_face_set to a shell. The overload on a tessellated
//! shell is in `translate_tessellated`.
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateFace,
    StepToTopoDSTranslateShellError,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_shape::{StepShapeConnectedFaceSet, StepShapeFaceSurface};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Shape, Shell};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};

/// Translate shell.
#[derive(Default)]
pub struct StepToTopoDSTranslateShell {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslateShellError,
}

impl StepToTopoDSTranslateShell {
    /// `StepToTopoDSTranslateShell()`.
    pub fn new() -> Self {
        Self::default()
    }
    pub(super) fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateShellError::Done;
        self.root.done = true;
    }
    /// `init(CFS, tool, NMTool, factors, progress)`: bug15697: nothing for a null entity. A
    /// face that fails only gives a warning, so the shell can come out with a face missing.
    pub fn init(
        &mut self,
        face_set: &Handle<StepShapeConnectedFaceSet>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let Some(live) = face_set.get() else {
            return Ok(());
        };
        if tool.is_bound(&face_set.cast()) {
            self.finish(topods::shell(tool.find(&face_set.cast())?)?.into_shape());
            return Ok(());
        }
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateShell")?;
        let builder = BRepBuilder::default();
        let mut shell = Shell::default();
        builder.make_shell(&mut shell);
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision()); // gka
        translator.root.set_max_tol(self.root.max_tol());
        let count = live.nb_cfs_faces()?;
        let scope = ProgressScope::with_literal(progress, "Face", f64::from(count), false);
        let mut rank = 1;
        while rank <= count && scope.more() {
            let face = live.cfs_faces_value(rank)?;
            let face_surface = face.cast::<StepShapeFaceSurface>();
            if face_surface.is_null() {
                let entity: Handle<StandardTransient> = face.cast();
                process.add_warning_text(
                    &entity,
                    " Face is not of FaceSurface Type; not mapped to TopoDS",
                    "",
                );
            } else {
                translator.init(&face_surface, tool, nm_tool, factors)?;
                if translator.is_done() {
                    let made = topods::face(translator.value()?)?;
                    builder.add(&mut shell, &made)?;
                } else {
                    // Warning only + add FaceSurface file Identifier
                    let entity: Handle<StandardTransient> = face_surface.cast();
                    process.add_warning_text(
                        &entity,
                        " a Face from Shell not mapped to TopoDS",
                        "",
                    );
                }
            }
            rank += 1;
            drop(scope.next(1.0));
        }
        shell.set_closed(BRepTool::is_closed(&shell)?)?;
        let shape = shell.into_shape();
        tool.bind(&face_set.cast(), &shape)?;
        self.finish(shape);
        Ok(())
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
        self.root
            .check_done("StepToTopoDSTranslateShell::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateShellError {
        self.my_error
    }
}
