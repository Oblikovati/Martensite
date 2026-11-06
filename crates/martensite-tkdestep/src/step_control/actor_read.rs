//! `StepControlActorRead`: STEP product and shape representation entities to shapes. This file
//! holds the actor's state, `recognize`, `transfer` and `transfer_shape`; each overload family of
//! `transfer_entity` has a file of its own.
use crate::header_section::HeaderSectionFileName;
use crate::step_basic::StepBasicProductDefinition;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::StepGeomGeometricRepresentationItem;
use crate::step_repr::{
    StepReprMappedItem, StepReprNextAssemblyUsageOccurrence, StepReprRepresentation,
    StepReprShapeRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeContextDependentShapeRepresentation,
    StepShapeEdgeBasedWireframeModel, StepShapeFaceBasedSurfaceModel, StepShapeFaceSurface,
    StepShapeFacetedBrep, StepShapeFacetedBrepAndBrepWithVoids, StepShapeGeometricSet,
    StepShapeManifoldSolidBrep, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation, StepShapeShellBasedSurfaceModel,
};
use crate::step_to_topo_ds::StepToTopoDSNMTool;
use crate::step_visual::{
    StepVisualTessellatedFace, StepVisualTessellatedShapeRepresentation,
    StepVisualTessellatedShell, StepVisualTessellatedSolid, StepVisualTessellatedSurfaceSet,
};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::{
    TransferActor, TransferActorOfTransientProcess, TransferActorOfTransientProcessOps,
    TransferActorOps, TransferBinder, TransferProcessOps, TransferShapeProcessing,
    TransferTransientKind, TransferTransientProcess, transferring_by_transient_process,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeBinder;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;

/// What the transfers of one actor share.
#[derive(Default)]
pub(super) struct ActorReadState {
    pub(super) nm_tool: StepToTopoDSNMTool,
    pub(super) precision: f64,
    pub(super) max_tol: f64,
    pub(super) sr_context: Handle<StepReprRepresentation>,
    pub(super) model: Handle<dyn InterfaceInterfaceModel>,
    /// `NM_DETECTED`: a file static shared by every actor, here one per actor so
    /// transfers on other threads cannot change each other's result.
    pub(super) nm_detected: bool,
}

/// ```
/// use martensite_tkdestep::step_control::StepControlActorRead;
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkxsbase::transfer::TransferActorOps;
/// let actor = StepControlActorRead::new(&Handle::default());
/// assert!(!actor.recognize(&Handle::default()));
/// ```
#[derive(Default)]
pub struct StepControlActorRead {
    base: TransferActorOfTransientProcess,
    pub(super) state: SharedState<ActorReadState>,
}
impl_transient!(StepControlActorRead, "StepControlActorRead", TransferActorOfTransientProcess, base, interfaces: [dyn TransferActorOps<TransferTransientKind>, dyn TransferActorOfTransientProcessOps]);

/// The classes `recognize` takes whatever the mode.
fn is_recognized_item(start: &Handle<StandardTransient>) -> bool {
    start.cast::<StepShapeFacetedBrep>().is_some()
        || start.cast::<StepShapeBrepWithVoids>().is_some()
        || start.cast::<StepShapeManifoldSolidBrep>().is_some()
        || start.cast::<StepShapeShellBasedSurfaceModel>().is_some()
        || start
            .cast::<StepShapeFacetedBrepAndBrepWithVoids>()
            .is_some()
        || start.cast::<StepShapeGeometricSet>().is_some()
        || start.cast::<StepReprMappedItem>().is_some()
        || start.cast::<StepShapeFaceSurface>().is_some()
        || start.cast::<StepShapeEdgeBasedWireframeModel>().is_some()
        || start.cast::<StepShapeFaceBasedSurfaceModel>().is_some()
}

/// The classes `recognize` takes when tessellated geometry is read.
fn is_tessellated_item(start: &Handle<StandardTransient>) -> bool {
    start.cast::<StepVisualTessellatedFace>().is_some()
        || start.cast::<StepVisualTessellatedSurfaceSet>().is_some()
        || start.cast::<StepVisualTessellatedShell>().is_some()
        || start.cast::<StepVisualTessellatedSolid>().is_some()
        || start
            .cast::<StepVisualTessellatedShapeRepresentation>()
            .is_some()
}

impl StepControlActorRead {
    /// `StepControlActorRead(theModel)`.
    pub fn new(model: &Handle<dyn InterfaceInterfaceModel>) -> Self {
        let actor = Self::default();
        actor.set_model(model);
        actor
    }
    /// `set_model(model)`.
    pub fn set_model(&self, model: &Handle<dyn InterfaceInterfaceModel>) {
        self.state.write().model = model.clone();
    }
    /// The model the actor recognizes with.
    fn step_model(&self) -> Handle<StepDataStepModel> {
        self.state.read().model.cast()
    }
    /// The transfer process's model.
    pub(super) fn tp_model(
        tp: &Handle<TransferTransientProcess>,
    ) -> Result<Handle<StepDataStepModel>, Failure> {
        let model: Handle<StepDataStepModel> =
            step_live(tp, "StepControlActorRead")?.model().cast();
        step_live(
            &model,
            "StepControlActorRead: the model of the transient process",
        )?;
        Ok(model)
    }
    /// The shape fix parameters with the tolerances of the current representation, for a
    /// processor (`set_parameter` on a copy of `GetShapeFixParameters`).
    pub(super) fn new_shape_processor(&self, non_manifold: bool) -> XSAlgoShapeProcessor {
        let mut parameters = self.shape_processing().shape_fix_parameters();
        let (precision, max_tol) = {
            let state = self.state.read();
            (state.precision, state.max_tol)
        };
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.Tolerance3d",
            precision,
            true,
            &mut parameters,
        );
        XSAlgoShapeProcessor::set_parameter_real(
            "FixShape.MaxTolerance3d",
            max_tol,
            true,
            &mut parameters,
        );
        if non_manifold {
            let one = martensite_tkernel::tcollection::AsciiString::new("1");
            XSAlgoShapeProcessor::set_parameter(
                "FixShape.NonManifold",
                &one,
                true,
                &mut parameters,
            );
        }
        XSAlgoShapeProcessor::new(&parameters, &Default::default())
    }
    /// True when the file's preprocessor is I-DEAS.
    fn is_ideas_file(model: &StepDataStepModel) -> Result<bool, Failure> {
        let header = model.header();
        header.start();
        while header.more() {
            let file_name: Handle<HeaderSectionFileName> = header.value()?.cast();
            if let Some(live) = file_name.get() {
                let version = live.preprocessor_version();
                if version
                    .get()
                    .is_some_and(|text| text.search("I-DEAS") != -1)
                {
                    return Ok(true);
                }
            }
            header.next();
        }
        Ok(false)
    }
    /// `transfer_shape(start, TP, local_factors, isManifold, use_trsf, progress)`.
    pub fn transfer_shape(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        is_manifold: bool,
        use_trsf: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        if start.is_null() {
            return Ok(self.null_result());
        }
        let model = Self::tp_model(tp)?;
        let prod_mode = model.internal_parameters().read_product_mode;
        let binder = if !prod_mode
            && start
                .cast::<StepShapeShapeDefinitionRepresentation>()
                .is_some()
        {
            self.old_way(start, tp, progress)?
        } else if start.cast::<StepBasicProductDefinition>().is_some() {
            self.transfer_product_definition(&start.cast(), tp, factors, use_trsf, progress)?
        } else if start
            .cast::<StepReprNextAssemblyUsageOccurrence>()
            .is_some()
        {
            self.transfer_nauo(&start.cast(), tp, factors, progress)?
        } else if start.cast::<StepShapeShapeRepresentation>().is_some() {
            let mut is_bound = false;
            self.transfer_shape_representation(
                &start.cast(),
                tp,
                factors,
                &mut is_bound,
                false,
                progress,
            )?
        } else if start
            .cast::<StepShapeContextDependentShapeRepresentation>()
            .is_some()
        {
            self.transfer_cdsr(&start.cast(), tp, factors, progress)?
        } else if start
            .cast::<StepReprShapeRepresentationRelationship>()
            .is_some()
        {
            self.transfer_srr(&start.cast(), tp, factors, 0, false, progress)?
        } else if start
            .cast::<StepGeomGeometricRepresentationItem>()
            .is_some()
        {
            self.transfer_geometric_item(&start.cast(), tp, factors, is_manifold, progress)?
        } else if start.cast::<StepReprMappedItem>().is_some() {
            self.transfer_mapped_item(&start.cast(), tp, factors, progress)?
        } else if start.cast::<StepShapeFaceSurface>().is_some() {
            self.transfer_face_surface(&start.cast(), tp, factors, progress)?
        } else {
            Handle::<TransferBRepShapeBinder>::default()
        };
        Ok(binder.cast())
    }
}

impl TransferActorOps<TransferTransientKind> for StepControlActorRead {
    fn actor(&self) -> &TransferActor<TransferTransientKind> {
        self.base.actor()
    }
    fn recognize(&self, start: &Handle<StandardTransient>) -> bool {
        if start.is_null() {
            return false;
        }
        if start.cast::<StepBasicProductDefinition>().is_some()
            || start
                .cast::<StepReprNextAssemblyUsageOccurrence>()
                .is_some()
        {
            return true;
        }
        let model = self.step_model();
        let Some(live) = model.get() else {
            return false;
        };
        let (prod_mode, tessellated) = {
            let parameters = live.internal_parameters();
            (
                parameters.read_product_mode,
                parameters.read_tessellated.as_raw() != 0,
            )
        };
        if !prod_mode
            && start
                .cast::<StepShapeShapeDefinitionRepresentation>()
                .is_some()
        {
            return true;
        }
        let representation: Handle<StepShapeShapeRepresentation> = start.cast();
        if let Some(live_rep) = representation.get() {
            let count = live_rep.nb_items();
            return (1..=count).any(|rank| {
                live_rep
                    .items_value(rank)
                    .is_ok_and(|item| self.recognize(&item.cast()))
            });
        }
        if is_recognized_item(start) || (tessellated && is_tessellated_item(start)) {
            return true;
        }
        let relationship: Handle<StepReprShapeRepresentationRelationship> = start.cast();
        if let Some(live_rel) = relationship.get() {
            return self.recognize(&live_rel.rep1().cast())
                || self.recognize(&live_rel.rep2().cast());
        }
        start
            .cast::<StepShapeContextDependentShapeRepresentation>()
            .is_some()
    }
    fn transferring(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<dyn TransferProcessOps<TransferTransientKind>>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        transferring_by_transient_process(self, start, tp, progress)
    }
}

impl TransferActorOfTransientProcessOps for StepControlActorRead {
    fn shape_processing(&self) -> &TransferShapeProcessing {
        self.base.shape_processing()
    }
    fn transfer(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let mut factors = StepDataFactors::new();
        let model = Self::tp_model(tp)?;
        if !model.is_initialized_unit() {
            XSAlgoShapeProcessor::prepare_for_transfer(); // update unit info
            let unit =
                martensite_tkxsbase::xs_algo::xs_algo_units().get_cascade_length_unit_default();
            model.set_local_length_unit(unit);
        }
        factors.set_cascade_unit(model.local_length_unit());
        if Self::is_ideas_file(&model)? {
            self.state.write().nm_tool.set_ideas_case(true);
        }
        let use_trsf = model.internal_parameters().read_root_transformation;
        self.transfer_shape(start, tp, &factors, true, use_trsf, progress)
    }
}
