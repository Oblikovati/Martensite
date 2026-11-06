//! `StepControlController`: the STEP norm of the work sessions.
use super::controller_statics::define_statics;
use super::shape_fix_parameters::default_shape_fix_parameters;
use super::{StepControlActorRead, StepControlActorWrite};
use crate::api_header_section::APIHeaderSectionEditHeader;
use crate::rw_header_section::RWHeaderSection;
use crate::rw_step_ap214::RwStepAP214;
use crate::step_data::StepDataStepModel;
use crate::step_edit::{STEPEdit, STEPEditEditContext, STEPEditEditSDR};
use crate::step_entity::step_live;
use crate::step_select::StepSelectWorkLibrary;
use crate::step_selections::{
    STEPSelectionsSelectAssembly, STEPSelectionsSelectDerived, STEPSelectionsSelectFaces,
    STEPSelectionsSelectForTransfer, STEPSelectionsSelectGSCurves, STEPSelectionsSelectInstances,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkshhealing::shape_process::{
    ShapeProcessOperation, ShapeProcessOperationsFlags, ShapeProcessShapeMap,
};
use martensite_tkxsbase::if_select::{
    IFSelectEditForm, IFSelectReturnStatus, IFSelectSelectDeduct, IFSelectSelectModelRoots,
    IFSelectSelection, IFSelectSignAncestor, IFSelectSignCounter, IFSelectSignature,
};
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::{
    TransferActorOfFinderProcessOps, TransferActorOfTransientProcessOps, TransferFinderProcess,
};
use martensite_tkxsbase::xs_algo::{XSAlgo, XSAlgoParameterMap, XSAlgoShapeProcessor};
use martensite_tkxsbase::xs_control::{
    XSControlController, XSControlControllerBase, XSControlWorkSession,
};
use std::sync::{Mutex, OnceLock};

/// Controller.
pub struct StepControlController {
    transient: StandardTransient,
    base: XSControlControllerBase,
}
impl_transient!(StepControlController, "StepControlController", StandardTransient, transient, interfaces: [dyn XSControlController]);

/// The members `StepControlController` calls on its parent. The provided members of the trait
/// cannot be reached from the overrides in the impl below, so this second implementor, on the
/// same base, keeps them: `InheritedMembers::customise` and `transfer_write_shape`.
struct InheritedMembers {
    transient: StandardTransient,
    controller: Handle<dyn XSControlController>,
}
impl_transient!(InheritedMembers, "InheritedMembers", StandardTransient, transient, interfaces: [dyn XSControlController]);

impl XSControlController for InheritedMembers {
    fn controller_base(&self) -> &XSControlControllerBase {
        self.controller.controller_base()
    }
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        self.controller.new_model()
    }
}

impl StepControlController {
    /// The parent's members, on this controller's base.
    fn inherited(&self) -> Result<InheritedMembers, Failure> {
        Ok(InheritedMembers {
            transient: StandardTransient::default(),
            controller: self.base.this()?,
        })
    }
}

/// The items the constructor and `customise` both add: selections on `roots`, and the editors.
fn norm_items(
    roots: &Handle<dyn IFSelectSelection>,
) -> Result<Vec<(&'static str, Handle<StandardTransient>)>, Failure> {
    let faces = Handle::new(STEPSelectionsSelectFaces::default());
    faces.set_input(roots);
    let curves = Handle::new(STEPSelectionsSelectGSCurves::default());
    curves.set_input(roots);
    let assembly = Handle::new(STEPSelectionsSelectAssembly::default());
    assembly.set_input(roots);
    let mut items: Vec<(&'static str, Handle<StandardTransient>)> = vec![
        ("step-faces", faces.cast()),
        (
            "step-instances",
            Handle::new(STEPSelectionsSelectInstances::default()).cast(),
        ),
        ("step-GS-curves", curves.cast()),
        ("step-assembly", assembly.cast()),
    ];
    let edit_header = APIHeaderSectionEditHeader::new()?;
    items.push(("step-header-edit", edit_header.cast()));
    items.push((
        "step-header",
        IFSelectEditForm::new(&edit_header.cast(), false, true, "Step Header")?.cast(),
    ));
    let edit_context = STEPEditEditContext::new()?;
    items.push(("step-context-edit", edit_context.cast()));
    let context_label = "STEP Product Definition Context";
    items.push((
        "step-context",
        IFSelectEditForm::new(&edit_context.cast(), false, true, context_label)?.cast(),
    ));
    let edit_sdr = STEPEditEditSDR::new()?;
    items.push(("step-SDR-edit", edit_sdr.cast()));
    let sdr_label = "STEP Product Data (SDR)";
    items.push((
        "step-SDR-data",
        IFSelectEditForm::new(&edit_sdr.cast(), false, true, sdr_label)?.cast(),
    ));
    Ok(items)
}

/// The selections and signatures a session with model roots gets.
fn items_on_roots(
    roots: &Handle<dyn IFSelectSelection>,
    sign_type: &Handle<dyn IFSelectSignature>,
) -> Result<Vec<(&'static str, Handle<StandardTransient>)>, Failure> {
    let mut derived = STEPSelectionsSelectDerived::new();
    derived.set_protocol(&STEPEdit::protocol())?;
    let select_sdr = STEPEdit::new_select_sdr()?;
    select_sdr.set_input(roots);
    Ok(vec![
        ("step-type", sign_type.cast()),
        (
            "step-types",
            Handle::new(IFSelectSignCounter::with_signature(sign_type, false, true)?).cast(),
        ),
        // pdn S4133 18.02.99
        (
            "xst-derived",
            Handle::new(IFSelectSignAncestor::new(false)).cast(),
        ),
        ("step-derived", Handle::new(derived).cast()),
        ("step-shape-def-repr", select_sdr.cast()),
        (
            "step-placed-items",
            STEPEdit::new_select_placed_item()?.cast(),
        ),
        // input deja pret avec ModelAll
        ("step-shape-repr", STEPEdit::new_select_shape_repr()?.cast()),
    ])
}

/// The operations the writing actor runs by default.
fn default_write_flags() -> ShapeProcessOperationsFlags {
    let mut flags = ShapeProcessOperationsFlags::default();
    flags.set(ShapeProcessOperation::SplitCommonVertex);
    flags.set(ShapeProcessOperation::DirectFaces);
    flags
}

/// `init` of the statics and the modules, once in the process.
fn init_process_wide() -> Result<(), Failure> {
    static DONE: Mutex<bool> = Mutex::new(false);
    let mut done = DONE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if !*done {
        RWHeaderSection::init();
        RwStepAP214::init();
        define_statics()?;
        *done = true;
    }
    Ok(())
}

impl StepControlController {
    /// `StepControlController()`: a new controller named `STEP` and `step`, bound to its handle.
    #[allow(
        clippy::new_ret_no_self,
        reason = "a controller lives behind its trait handle"
    )]
    pub fn new() -> Result<Handle<dyn XSControlController>, Failure> {
        let base = XSControlControllerBase::new("STEP", "step")?;
        init_process_wide()?;
        let actor_write = Handle::new(StepControlActorWrite::new());
        let processing = actor_write.shape_processing();
        processing.set_shape_fix_parameters_from(
            &default_shape_fix_parameters(),
            &XSAlgoParameterMap::default(),
        );
        processing.set_flags(default_write_flags());
        base.set_adaptor_write(&actor_write.cast());
        let library = StepSelectWorkLibrary::new(false);
        library.set_dump_label(1);
        base.set_adaptor_library(&Handle::new(library).cast());
        base.set_adaptor_protocol(&STEPEdit::protocol());
        let controller: Handle<dyn XSControlController> = Handle::new(Self {
            transient: StandardTransient::default(),
            base,
        })
        .cast();
        controller.controller_base().bind_self(&controller);
        controller.set_mode_write(0, 4, true);
        controller.set_mode_write_help(0, "As Is", true);
        controller.set_mode_write_help(1, "Faceted Brep", true);
        controller.set_mode_write_help(2, "Shell Based", true);
        controller.set_mode_write_help(3, "Manifold Solid", true);
        controller.set_mode_write_help(4, "Wireframe", true);
        controller
            .controller_base()
            .trace_static("read.surfacecurve.mode", 5);
        Ok(controller)
    }
    /// `init`: records the controller under its names, once in the process.
    pub fn init() -> Result<bool, Failure> {
        static DONE: OnceLock<()> = OnceLock::new();
        if DONE.get().is_none() {
            let controller = Self::new()?;
            controller.auto_record()?; // avec les noms donnes a la construction
            XSAlgo::init()?;
            let _ = DONE.set(());
        }
        Ok(true)
    }
}

impl XSControlController for StepControlController {
    fn controller_base(&self) -> &XSControlControllerBase {
        &self.base
    }
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        STEPEdit::new_model().unwrap_or_default().cast()
    }
    fn actor_read(
        &self,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Handle<dyn TransferActorOfTransientProcessOps> {
        let known: Handle<StepControlActorRead> = self.base.adaptor_read().cast();
        if known.is_some() {
            return known.cast();
        }
        // A new actor each time: myAdaptorRead is never set, and the session keeps it.
        Handle::new(StepControlActorRead::new(model)).cast()
    }
    /// `customise(WS)`.
    fn customise(&self, ws: &Handle<XSControlWorkSession>) -> Result<(), Failure> {
        self.inherited()?.customise(ws)?;
        let session = step_live(ws, "StepControlController::Customise")?;
        let mut roots: Handle<dyn IFSelectSelection> = session.named_item("xst-model-roots").cast();
        if roots.is_null() {
            let made = Handle::new(IFSelectSelectModelRoots::new());
            session.add_named_item("xst-model-roots", &made.cast(), true)?;
            roots = made.cast();
        }
        let for_transfer = Handle::new(STEPSelectionsSelectForTransfer::new());
        for_transfer.set_reader(&session.transfer_reader());
        session.add_named_item("xst-transferrable-roots", &for_transfer.cast(), true)?;
        let sign_type = STEPEdit::sign_type()?;
        session.set_sign_type(&sign_type);
        for (name, item) in items_on_roots(&roots, &sign_type)? {
            session.add_named_item(name, &item, true)?;
        }
        for (name, item) in norm_items(&roots)? {
            session.add_named_item(name, &item, true)?;
        }
        Ok(())
    }
    fn transfer_write_shape(
        &self,
        shape: &Shape,
        fp: &Handle<TransferFinderProcess>,
        model: &Handle<dyn InterfaceInterfaceModel>,
        modetrans: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        if !(0..=4).contains(&modetrans) {
            return Ok(IFSelectReturnStatus::RetError);
        }
        let write_actor: Handle<StepControlActorWrite> = self.base.adaptor_write().cast();
        let step_model: Handle<StepDataStepModel> = model.cast();
        let live_model = step_live(&step_model, "StepControlController::TransferWriteShape")?;
        if let Some(live) = write_actor.get() {
            live.set_group_mode(live_model.internal_parameters().write_assembly.as_raw());
        }
        let mut written = shape.clone();
        let mut modified = ShapeProcessShapeMap::default();
        if live_model.internal_parameters().write_nonmanifold {
            let remover = martensite_tkshhealing::shape_upgrade::ShapeUpgradeRemoveLocations::new();
            remover.set_remove_level(martensite_tkg3d::top_abs::ShapeEnum::Compound);
            remover.remove(&written)?;
            written = remover.get_result();
            modified = remover.get_modified_shapes_map(Clone::clone);
        }
        let status = self
            .inherited()?
            .transfer_write_shape(&written, fp, model, modetrans, progress)?;
        XSAlgoShapeProcessor::merge_shape_transfer_info_finder(fp, &modified, &Handle::default())?;
        Ok(status)
    }
}
