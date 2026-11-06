//! `STEPCAFControlActorWrite`: the write actor, with assemblies decided from an XDE document.
use crate::step_control::StepControlActorWrite;
use crate::step_data::StepDataStepModel;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::transfer::{
    TransferActor, TransferActorOfFinderProcessOps, TransferActorOps, TransferBinder,
    TransferFinder, TransferFinderKind, TransferFinderProcess, TransferProcessOps,
    TransferShapeProcessing,
};

/// `STEPCAFControlActorWrite`: in standard mode, the default, it behaves exactly as its ancestor.
/// The translation itself runs in the ancestor, which reads the registered shapes through `is_assembly`.
///
/// ```
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlActorWrite;
/// let actor = STEPCAFControlActorWrite::new();
/// actor.set_std_mode(false);
/// actor.clear_map();
/// ```
#[derive(Default)]
pub struct STEPCAFControlActorWrite {
    base: StepControlActorWrite,
}
impl_transient!(STEPCAFControlActorWrite, "STEPCAFControlActorWrite", StepControlActorWrite, base, interfaces: [dyn TransferActorOps<TransferFinderKind>, dyn TransferActorOfFinderProcessOps]);

impl std::ops::Deref for STEPCAFControlActorWrite {
    type Target = StepControlActorWrite;
    fn deref(&self) -> &StepControlActorWrite {
        &self.base
    }
}

impl STEPCAFControlActorWrite {
    /// `STEPCAFControlActorWrite()`: standard mode.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_std_mode`: in standard mode the actor behaves as its ancestor, and the map is cleared.
    pub fn set_std_mode(&self, std_mode: bool) {
        self.base
            .set_registered_assemblies((!std_mode).then(MapOfShape::default));
    }
    /// `clear_map`: of the shapes registered as assemblies.
    pub fn clear_map(&self) {
        self.base.clear_registered_assemblies();
    }
    /// `register_assembly`: a compound to write as an assembly; else nothing.
    pub fn register_assembly(&self, shape: &Shape) -> Result<(), Failure> {
        if shape.shape_type()? == ShapeEnum::Compound {
            self.base.register_assembly_shape(shape);
        }
        Ok(())
    }
    /// `is_assembly`: registered in the map, or as the ancestor decides in standard mode.
    pub fn is_assembly(
        &self,
        model: &Handle<StepDataStepModel>,
        shape: &mut Shape,
    ) -> Result<bool, Failure> {
        self.base.is_assembly(model, shape)
    }
}

impl TransferActorOps<TransferFinderKind> for STEPCAFControlActorWrite {
    fn actor(&self) -> &TransferActor<TransferFinderKind> {
        self.base.actor()
    }
    fn recognize(&self, start: &Handle<dyn TransferFinder>) -> bool {
        self.base.recognize(start)
    }
    fn transferring(
        &self,
        start: &Handle<dyn TransferFinder>,
        tp: &Handle<dyn TransferProcessOps<TransferFinderKind>>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        self.base.transferring(start, tp, progress)
    }
}

impl TransferActorOfFinderProcessOps for STEPCAFControlActorWrite {
    fn shape_processing(&self) -> &TransferShapeProcessing {
        self.base.shape_processing()
    }
    fn mode_trans_state(&self) -> &SharedState<i32> {
        self.base.mode_trans_state()
    }
    fn transfer(
        &self,
        start: &Handle<dyn TransferFinder>,
        fp: &Handle<TransferFinderProcess>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        TransferActorOfFinderProcessOps::transfer(&self.base, start, fp, progress)
    }
}
