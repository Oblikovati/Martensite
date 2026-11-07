//! `IgesControlController`: the IGES norm of the work sessions.
use super::{IgesControlActorWrite, IgesControlAlgoContainer};
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_data::{IgesDataIgesModel, iges_data_init, iges_live};
use crate::iges_select::{
    IgesSelectAutoCorrect, IgesSelectClassType, IgesSelectComputeStatus,
    IgesSelectCounterOfLevelNumber, IgesSelectEditDirPart, IgesSelectEditHeader,
    IgesSelectFloatFormat, IgesSelectIGESName, IgesSelectIGESTypeForm, IgesSelectRemoveCurves,
    IgesSelectSelectBasicGeom, IgesSelectSelectBypassGroup, IgesSelectSelectBypassSubfigure,
    IgesSelectSelectFaces, IgesSelectSelectPCurves, IgesSelectSelectSubordinate,
    IgesSelectSelectVisibleStatus, IgesSelectSetGlobalParameter, IgesSelectSetLabel,
    IgesSelectSignColor, IgesSelectSignLevelNumber, IgesSelectSignStatus, IgesSelectUpdateFileName,
    IgesSelectWorkLibrary,
};
use crate::iges_to_brep::{IgesToBRep, IgesToBRepActor};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::if_select::{
    IFSelectEditor, IFSelectSelectDeduct, IFSelectSelectExtract, IFSelectSelectModelEntities,
    IFSelectSelectModelRoots, IFSelectSelectType, IFSelectSelection, IFSelectSignCounter,
    IFSelectSignMultiple, IFSelectSignature,
};
use martensite_tkxsbase::interface::{
    InterfaceInterfaceModel, InterfaceModelBase, InterfaceStatic,
};
use martensite_tkxsbase::transfer::TransferActorOfTransientProcessOps;
use martensite_tkxsbase::xs_algo::XSAlgo;
use martensite_tkxsbase::xs_control::{
    XSControlController, XSControlControllerBase, XSControlSelectForTransfer, XSControlWorkSession,
};
use std::sync::OnceLock;

const WHERE: &str = "IgesControlController";

/// The IGES norm.
pub struct IgesControlController {
    transient: StandardTransient,
    base: XSControlControllerBase,
}
impl_transient!(
    IgesControlController,
    "IgesControlController",
    StandardTransient,
    transient,
    interfaces: [dyn XSControlController]
);

/// The parent's members, on the base of this controller.
struct InheritedMembers {
    transient: StandardTransient,
    controller: Handle<dyn XSControlController>,
}
impl_transient!(
    InheritedMembers,
    "InheritedMembers",
    StandardTransient,
    transient,
    interfaces: [dyn XSControlController]
);

impl XSControlController for InheritedMembers {
    fn controller_base(&self) -> &XSControlControllerBase {
        self.controller.controller_base()
    }
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        self.controller.new_model()
    }
}

fn static_text(name: &str) -> Result<Handle<HAsciiString>, Failure> {
    let parameter = InterfaceStatic::static_named(name);
    Ok(iges_live(&parameter, WHERE)?.h_string_value())
}

/// A header item: its value by name, and the modifier that sets it.
fn add_header_item(
    controller: &Handle<dyn XSControlController>,
    (parameter, rank): (&str, i32),
    (value_name, modifier_name): (&str, &str),
) -> Result<(), Failure> {
    let modifier = Handle::new(IgesSelectSetGlobalParameter::new(rank));
    let value = static_text(parameter)?;
    iges_live(&modifier, WHERE)?.set_value(&value);
    controller.add_session_item(&value.cast(), value_name, false);
    controller.add_session_item(&modifier.cast(), modifier_name, true);
    Ok(())
}

fn add_session_items(controller: &Handle<dyn XSControlController>) -> Result<(), Failure> {
    let items: Vec<(Handle<StandardTransient>, &str, bool)> = vec![
        (
            Handle::new(IgesSelectRemoveCurves::new(true)).cast(),
            "iges-remove-pcurves",
            false,
        ),
        (
            Handle::new(IgesSelectRemoveCurves::new(false)).cast(),
            "iges-remove-curves-3d",
            false,
        ),
        (
            Handle::new(IgesSelectSetLabel::new(0, true)).cast(),
            "iges-clear-label",
            false,
        ),
        (
            Handle::new(IgesSelectSetLabel::new(1, false)).cast(),
            "iges-set-label-dnum",
            false,
        ),
        (
            Handle::new(IgesSelectAutoCorrect::new()).cast(),
            "iges-auto-correct",
            true,
        ),
        (
            Handle::new(IgesSelectComputeStatus::new()).cast(),
            "iges-compute-status",
            true,
        ),
    ];
    for (item, name, applied) in items {
        controller.add_session_item(&item, name, applied);
    }
    let format = IgesSelectFloatFormat::new();
    format.set_default(12);
    controller.add_session_item(&Handle::new(format).cast(), "iges-float-digits-12", true);
    add_header_item(
        controller,
        ("write.iges.header.product", 3),
        ("iges-header-val-sender", "iges-header-set-sender"),
    )?;
    controller.add_session_item(
        &Handle::new(IgesSelectUpdateFileName::new()).cast(),
        "iges-update-file-name",
        true,
    );
    add_header_item(
        controller,
        ("write.iges.header.receiver", 12),
        ("iges-header-val-receiver", "iges-header-set-receiver"),
    )?;
    add_header_item(
        controller,
        ("write.iges.header.author", 21),
        ("iges-header-val-author", "iges-header-set-author"),
    )?;
    add_header_item(
        controller,
        ("write.iges.header.company", 22),
        ("iges-header-val-company", "iges-header-set-company"),
    )
}

fn trace_statics(base: &XSControlControllerBase) {
    for (name, used) in [
        ("read.iges.bspline.approxd1.mode", 5),
        ("read.iges.bspline.continuity", 5),
        ("write.iges.header.receiver", 2),
        ("write.iges.header.author", 2),
        ("write.iges.header.company", 2),
        ("write.iges.unit", 6),
        ("write.iges.brep.mode", 6),
    ] {
        base.trace_static(name, used);
    }
}

/// The selections of a session with model roots.
fn add_selections(
    session: &XSControlWorkSession,
    all: &Handle<dyn IFSelectSelection>,
    roots: &Handle<dyn IFSelectSelection>,
    transferable: &Handle<dyn IFSelectSelection>,
) -> Result<(), Failure> {
    let add = |name: &str, item: Handle<StandardTransient>| -> Result<(), Failure> {
        session.add_named_item(name, &item, true).map(|_| ())
    };
    let visible = |input: &Handle<dyn IFSelectSelection>, direct: bool| {
        let selection = IgesSelectSelectVisibleStatus::default();
        selection.set_direct(direct);
        selection.set_input(input);
        Handle::new(selection).cast::<StandardTransient>()
    };
    add("iges-visible-roots", visible(roots, true))?;
    add("iges-visible-transf-roots", visible(transferable, true))?;
    add("iges-blanked-roots", visible(roots, false))?;
    add("iges-blanked-transf-roots", visible(transferable, false))?;
    let independent = IgesSelectSelectSubordinate::new(0);
    independent.set_input(all);
    add("iges-status-independant", Handle::new(independent).cast())?;
    let groups = IgesSelectSelectBypassGroup::new(0);
    groups.set_input(roots);
    add("iges-bypass-group", Handle::new(groups).cast())?;
    let subfigures = Handle::new(IgesSelectSelectBypassSubfigure::new(0));
    subfigures.set_input(roots);
    let subfigures_input: Handle<dyn IFSelectSelection> = subfigures.cast();
    add("iges-bypass-subfigure", subfigures.cast())?;
    let both = IgesSelectSelectBypassGroup::new(0);
    both.set_input(&subfigures_input);
    add("iges-bypass-group-subfigure", Handle::new(both).cast())?;
    for (mode, name) in [
        (1, "iges-curves-3d"),
        (2, "iges-basic-curves-3d"),
        (0, "iges-basic-geom"),
        (-1, "iges-surfaces"),
    ] {
        let basic = IgesSelectSelectBasicGeom::new(mode);
        basic.set_input(&subfigures_input);
        add(name, Handle::new(basic).cast())?;
    }
    let faces = Handle::new(IgesSelectSelectFaces::new());
    faces.set_input(&subfigures_input);
    let curves = IgesSelectSelectPCurves::new(true);
    curves.set_input(&faces.cast());
    add("iges-faces", faces.cast())?;
    add("iges-pcurves", Handle::new(curves).cast())?;
    let no_definition = IFSelectSelectType::with_type(IgesBasicSubfigureDef::static_type());
    no_definition.set_direct(false);
    no_definition.set_input(roots);
    add(
        "iges-no-indep-subfigure-def",
        Handle::new(no_definition).cast(),
    )
}

/// The signatures of a session with model roots.
fn add_signatures(session: &XSControlWorkSession) -> Result<(), Failure> {
    let add = |name: &str, item: Handle<StandardTransient>| -> Result<(), Failure> {
        session.add_named_item(name, &item, true).map(|_| ())
    };
    let type_form = Handle::new(IgesSelectIGESTypeForm::new(true));
    let as_signature: Handle<dyn IFSelectSignature> = type_form.cast();
    add("iges-type", type_form.cast())?;
    let by_status = IFSelectSignMultiple::new("IGES Type Form    Status");
    by_status.add(&as_signature, 15, false);
    by_status.add(&Handle::new(IgesSelectSignStatus::new()).cast(), 0, false);
    add("iges-type-status", Handle::new(by_status).cast())?;
    let by_name = Handle::new(IFSelectSignMultiple::new("IGES Type Form  TypeName"));
    by_name.add(&as_signature, 4, false);
    by_name.add(&Handle::new(IgesSelectClassType::new()).cast(), 0, false);
    add("iges-type-name", by_name.cast())?;
    let counter = IFSelectSignCounter::with_signature(&as_signature, false, true)?;
    add("iges-types", Handle::new(counter).cast())?;
    add(
        "iges-levels",
        Handle::new(IgesSelectCounterOfLevelNumber::new(false, true)).cast(),
    )?;
    add(
        "iges-level-number",
        Handle::new(IgesSelectSignLevelNumber::new(false)).cast(),
    )?;
    add("iges-name", Handle::new(IgesSelectIGESName::new()).cast())?;
    for (mode, name) in [
        (1, "iges-color-number"),
        (2, "iges-color-name"),
        (3, "iges-color-rgb"),
        (4, "iges-color-red"),
        (5, "iges-color-green"),
        (6, "iges-color-blue"),
    ] {
        add(name, Handle::new(IgesSelectSignColor::new(mode)).cast())?;
    }
    let header = IgesSelectEditHeader::new()?;
    add("iges-header-edit", header.cast())?;
    add("iges-header", header.form(false, true)?.cast())?;
    let dir_part = IgesSelectEditDirPart::new()?;
    add("iges-dir-part-edit", dir_part.cast())?;
    add("iges-dir-part", dir_part.form(false, true)?.cast())?;
    session.set_sign_type(&by_name.cast());
    Ok(())
}

impl IgesControlController {
    /// A new controller named `IGES` and `iges`, or `FNES` and `fnes`, bound to its handle;
    /// `IgesControlController::new(false)?`.
    ///
    /// # Errors
    /// A failure of the init of the entity packages or of the settings.
    #[allow(
        clippy::new_ret_no_self,
        reason = "a controller lives behind its trait handle"
    )]
    pub fn new(fnes: bool) -> Result<Handle<dyn XSControlController>, Failure> {
        let (long_name, short_name) = if fnes {
            ("FNES", "fnes")
        } else {
            ("IGES", "iges")
        };
        let base = XSControlControllerBase::new(long_name, short_name)?;
        iges_data_init()?;
        trace_statics(&base);
        base.set_adaptor_library(&Handle::new(IgesSelectWorkLibrary::new(fnes)).cast());
        base.set_adaptor_protocol(&IgesSelectWorkLibrary::define_protocol().cast());
        let reading = IgesToBRepActor::new();
        reading.set_continuity(0);
        base.set_adaptor_read(&Handle::new(reading).cast());
        base.set_adaptor_write(&Handle::new(IgesControlActorWrite::default()).cast());
        let controller: Handle<dyn XSControlController> = Handle::new(Self {
            transient: StandardTransient::default(),
            base,
        })
        .cast();
        controller.controller_base().bind_self(&controller);
        add_session_items(&controller)?;
        controller.set_mode_write(0, 1, true);
        controller.set_mode_write_help(0, "Faces", true);
        controller.set_mode_write_help(1, "BRep", true);
        Ok(controller)
    }
    /// Records the controller under its names, once in the process;
    /// `IgesControlController::init()?`.
    ///
    /// # Errors
    /// A failure of the construction of the controller.
    pub fn init() -> Result<bool, Failure> {
        static DONE: OnceLock<()> = OnceLock::new();
        if DONE.get().is_none() {
            let controller = Self::new(false)?;
            controller.auto_record()?;
            XSAlgo::init()?;
            IgesToBRep::init()?;
            IgesToBRep::set_algo_container(
                &Handle::new(IgesControlAlgoContainer::new_container()),
            )?;
            let _ = DONE.set(());
        }
        Ok(true)
    }
}

impl XSControlController for IgesControlController {
    fn controller_base(&self) -> &XSControlControllerBase {
        &self.base
    }
    /// The template of the session, with the header of the settings.
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        let template = InterfaceModelBase::template("iges").unwrap_or_default();
        let model: Handle<IgesDataIgesModel> = template.cast();
        if let Some(live) = model.get() {
            let mut section = live.global_section();
            if let (Ok(receiver), Ok(author), Ok(company)) = (
                static_text("write.iges.header.receiver"),
                static_text("write.iges.header.author"),
                static_text("write.iges.header.company"),
            ) {
                section.set_receive_name(&receiver);
                section.set_author_name(&author);
                section.set_company_name(&company);
            }
            section.set_unit_flag(InterfaceStatic::i_val("write.iges.unit"));
            section.set_unit_name(&Handle::new(HAsciiString::from(InterfaceStatic::c_val(
                "write.iges.unit",
            ))));
            live.set_global_section(&section);
        }
        model.cast()
    }
    fn actor_read(
        &self,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Handle<dyn TransferActorOfTransientProcessOps> {
        let reading: Handle<IgesToBRepActor> = self.base.adaptor_read().cast();
        if let Some(live) = reading.get() {
            let iges = if model.cast::<IgesDataIgesModel>().is_some() {
                model.clone()
            } else {
                self.new_model()
            };
            // A model without a global section keeps the tolerance of the actor.
            let _ = live.set_model(&iges);
            live.set_continuity(InterfaceStatic::i_val("read.iges.bspline.continuity"));
        }
        self.base.adaptor_read()
    }
    fn customise(&self, ws: &Handle<XSControlWorkSession>) -> Result<(), Failure> {
        InheritedMembers {
            transient: StandardTransient::default(),
            controller: self.base.this()?,
        }
        .customise(ws)?;
        let session = iges_live(ws, WHERE)?;
        let mut all: Handle<dyn IFSelectSelection> = session.named_item("xst-model-all").cast();
        if all.is_null() {
            all = Handle::new(IFSelectSelectModelEntities::new()).cast();
        }
        let mut roots: Handle<dyn IFSelectSelection> = session.named_item("xst-model-roots").cast();
        if roots.is_null() {
            let made = Handle::new(IFSelectSelectModelRoots::new());
            session.add_named_item("xst-model-roots", &made.cast(), true)?;
            roots = made.cast();
        }
        let mut transferable: Handle<dyn IFSelectSelection> =
            session.named_item("xst-transferrable-roots").cast();
        if transferable.is_null() {
            let made = Handle::new(XSControlSelectForTransfer::new());
            made.set_reader(&session.transfer_reader());
            session.add_named_item("xst-transferrable-roots", &made.cast(), true)?;
            transferable = made.cast();
        }
        add_selections(session, &all, &roots, &transferable)?;
        add_signatures(session)
    }
}
