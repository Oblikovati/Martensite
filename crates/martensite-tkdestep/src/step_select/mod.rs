//! `StepSelect`: STEP in the work session: reading and writing files, the signature by STEP
//! type, and the modifiers of a model and of a file.
use crate::step_data::{
    StepDataProtocolOps, StepDataStepDumper, StepDataStepModel, StepDataStepWriter,
    StepDataUndefinedEntity, StepDataWriterLib,
};
use crate::step_file::step_file_read;
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    message::Message,
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectContextModif, IFSelectContextWrite, IFSelectGeneralModifier, IFSelectModifier,
        IFSelectSignature, IFSelectSignatureBase, IFSelectWorkLibrary, IFSelectWorkLibraryBase,
    },
    interface::{
        InterfaceCopyTool, InterfaceEntityIterator, InterfaceInterfaceModel,
        InterfaceModelEntities, InterfaceModelReports, InterfaceProtocol, InterfaceSignType,
        interface_error, interface_sign_type_text,
    },
    moni_tool::MoniToolSignText,
};
use std::fmt::Write as _;
use std::io::{Read, Write};

/// `StepSelectFileModifier`, abstract: a modifier of the file being written.
///
/// ```
/// use martensite_tkdestep::step_select::StepSelectFileModifier;
/// fn may_change(modifier: &dyn StepSelectFileModifier) -> bool {
///     use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
///     modifier.may_change_graph()
/// }
/// ```
pub trait StepSelectFileModifier: IFSelectGeneralModifier {
    /// `perform(ctx, writer)`.
    fn perform(
        &self,
        ctx: &mut IFSelectContextWrite,
        writer: &mut StepDataStepWriter,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn StepSelectFileModifier,
    "StepSelectFileModifier",
    dyn IFSelectGeneralModifier
);

/// `StepSelectModelModifier`, abstract: a modifier of a STEP model. The `perform` is
/// [`step_model_modifier_perform`], which an implementation's `IFSelectModifier::perform` calls.
pub trait StepSelectModelModifier: IFSelectModifier {
    /// `perform_protocol(ctx, target, proto, TC)`: by default `performing`.
    fn perform_protocol(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<StepDataStepModel>,
        _proto: &Handle<dyn StepDataProtocolOps>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        self.performing(ctx, target, tc)
    }
    /// `performing(ctx, target, TC)`.
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<StepDataStepModel>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn StepSelectModelModifier,
    "StepSelectModelModifier",
    dyn IFSelectModifier
);

/// The `StepSelectModelModifier::perform(ctx, target, protocol, TC)`.
pub fn step_model_modifier_perform<M: StepSelectModelModifier + RuntimeTyped + ?Sized>(
    modifier: &Handle<M>,
    ctx: &mut IFSelectContextModif,
    target: &Handle<dyn InterfaceInterfaceModel>,
    protocol: &Handle<dyn InterfaceProtocol>,
    tc: &mut InterfaceCopyTool,
) -> Result<(), Failure> {
    let Some(live) = modifier.get() else {
        return Ok(());
    };
    ctx.trace_modifier(&modifier.cast());
    let step = target.cast::<StepDataStepModel>();
    if step.is_null() {
        ctx.c_check(0)?
            .add_fail_text("Model to Modify : unproper type", "");
        return Ok(());
    }
    live.perform_protocol(ctx, &step, &protocol.cast(), tc)
}

/// `StepSelectStepType`: the signature by STEP type, for the schema of a protocol.
///
/// ```
/// use martensite_tkdestep::{step_data::StepData, step_select::StepSelectStepType};
/// use martensite_tkxsbase::moni_tool::MoniToolSignText;
/// let mut sign = StepSelectStepType::new();
/// sign.set_protocol(&StepData::protocol().cast()).unwrap();
/// assert_eq!(sign.name(), "Step Type (Schema (DEFAULT))");
/// ```
pub struct StepSelectStepType {
    transient: StandardTransient,
    base: IFSelectSignatureBase,
    thelib: StepDataWriterLib,
    theproto: Handle<dyn StepDataProtocolOps>,
}
impl_transient!(
    StepSelectStepType,
    "StepSelectStepType",
    dyn IFSelectSignature,
    transient,
    interfaces: [dyn MoniToolSignText, dyn InterfaceSignType, dyn IFSelectSignature]
);

impl Default for StepSelectStepType {
    fn default() -> Self {
        Self::new()
    }
}

impl StepSelectStepType {
    /// `StepSelectStepType()`.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new("Step Type"),
            thelib: StepDataWriterLib::empty(),
            theproto: Handle::default(),
        }
    }
    /// `set_protocol(proto)`: `an interface error` when not a STEP protocol.
    pub fn set_protocol(&mut self, proto: &Handle<dyn InterfaceProtocol>) -> Result<(), Failure> {
        let step = proto.cast::<dyn StepDataProtocolOps>();
        let live = step.get().ok_or_else(|| {
            interface_error("StepSelectStepType: SetProtocol expected a STEP protocol")
        })?;
        self.thelib.clear();
        self.thelib.add_protocol(&proto.cast());
        let schema = live.schema_name(&Handle::default());
        self.base.thename = AsciiString::new(&format!("Step Type (Schema {schema})"));
        self.theproto = step;
        Ok(())
    }
    fn complex_of_undefined(ent: &Handle<StandardTransient>) -> String {
        let mut current = ent.cast::<StepDataUndefinedEntity>();
        let Some(first) = current.get() else {
            return String::new();
        };
        if !first.is_complex() {
            return String::from_utf8_lossy(first.step_type().as_bytes()).into_owned();
        }
        let mut names = Vec::new();
        while let Some(live) = current.get() {
            names.push(String::from_utf8_lossy(live.step_type().as_bytes()).into_owned());
            let next = live.next();
            current = next;
        }
        format!("({})", names.join(","))
    }
}

impl MoniToolSignText for StepSelectStepType {
    fn name(&self) -> String {
        String::from_utf8_lossy(self.base.thename.as_bytes()).into_owned()
    }
    fn text(
        &self,
        ent: &Handle<StandardTransient>,
        context: &Handle<StandardTransient>,
    ) -> AsciiString {
        interface_sign_type_text(self, ent, context)
    }
}

impl InterfaceSignType for StepSelectStepType {
    /// `Value(ent, model)`.
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let Some((module, cn)) = self.thelib.select(ent) else {
            let schema = self
                .theproto
                .get()
                .map_or_else(String::new, |live| live.schema_name(model));
            return format!("..NOT FROM SCHEMA {schema}..");
        };
        if !module.is_complex(cn) {
            return module.step_type(cn).to_owned();
        }
        let mut list = Vec::new();
        if module.complex_type(cn, &mut list) {
            if list.is_empty() {
                return "(..COMPLEX TYPE..)".to_owned();
            }
            let names: Vec<String> = list
                .iter()
                .map(|name| String::from_utf8_lossy(name.as_bytes()).into_owned())
                .collect();
            return format!("({})", names.join(","));
        }
        Self::complex_of_undefined(ent)
    }
}

impl IFSelectSignature for StepSelectStepType {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}

/// ```
/// use martensite_tkdestep::step_select::StepSelectWorkLibrary;
/// use martensite_tkxsbase::if_select::IFSelectWorkLibrary;
/// assert_eq!(StepSelectWorkLibrary::new(true).dump_levels(), (1, 2));
/// ```
pub struct StepSelectWorkLibrary {
    transient: StandardTransient,
    base: IFSelectWorkLibraryBase,
    thecopymode: bool,
    thelabmode: martensite_tkernel::standard::SharedState<i32>,
}
impl_transient!(
    StepSelectWorkLibrary,
    "StepSelectWorkLibrary",
    dyn IFSelectWorkLibrary,
    transient,
    interfaces: [dyn IFSelectWorkLibrary]
);

impl StepSelectWorkLibrary {
    /// `StepSelectWorkLibrary(copymode = true)`.
    pub fn new(copymode: bool) -> Self {
        let library = Self {
            transient: StandardTransient::default(),
            base: IFSelectWorkLibraryBase::new(),
            thecopymode: copymode,
            thelabmode: martensite_tkernel::standard::SharedState::new(0),
        };
        library.set_dump_levels(1, 2);
        library.set_dump_help(0, "#id + Step Type");
        library.set_dump_help(1, "Entity as in file");
        library.set_dump_help(2, "Entity + shareds (level 1) as in file");
        library
    }
    /// `set_dump_label(mode)`.
    pub fn set_dump_label(&self, mode: i32) {
        *self.thelabmode.write() = mode;
    }
    fn read(
        &self,
        name: &str,
        stream: Option<&mut dyn Read>,
        model: &Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<i32, Failure> {
        let step = protocol.cast::<dyn StepDataProtocolOps>();
        if step.is_null() {
            return Ok(1);
        }
        step_file_read(name, stream, &model.cast(), &step)
    }
}

fn file_failure(error: &std::io::Error) -> Failure {
    Failure::new(&format!(
        "StepSelectWorkLibrary: the stream failed: {error}"
    ))
}

impl IFSelectWorkLibrary for StepSelectWorkLibrary {
    fn work_library_base(&self) -> &IFSelectWorkLibraryBase {
        &self.base
    }
    fn read_file(
        &self,
        name: &str,
        model: &mut Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<i32, Failure> {
        self.read(name, None, model, protocol)
    }
    fn read_stream(
        &self,
        name: &str,
        istream: &mut dyn Read,
        model: &mut Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<i32, Failure> {
        self.read(name, Some(istream), model, protocol)
    }
    fn write_file(&self, ctx: &mut IFSelectContextWrite) -> Result<bool, Failure> {
        if ctx.model().cast::<StepDataStepModel>().is_null()
            || ctx.protocol().cast::<dyn StepDataProtocolOps>().is_null()
        {
            return Ok(false);
        }
        let name = String::from_utf8_lossy(ctx.file_name().as_bytes()).into_owned();
        let Ok(mut file) = std::fs::File::create(&name) else {
            ctx.c_check(0)?
                .add_fail_text("Step File could not be created", "");
            let _ = writeln!(
                Message::send_info(),
                " Step File could not be created : {name}"
            );
            return Ok(false);
        };
        let written = self.write_stream(ctx, &mut file)?;
        let flushed = file.flush();
        if let Err(error) = &flushed {
            let _ = writeln!(Message::send_info(), "{error}");
        }
        Ok(written && flushed.is_ok())
    }
    fn write_stream(
        &self,
        ctx: &mut IFSelectContextWrite,
        ostream: &mut dyn Write,
    ) -> Result<bool, Failure> {
        let model = ctx.model().cast::<StepDataStepModel>();
        let protocol = ctx.protocol().cast::<dyn StepDataProtocolOps>();
        let Some(live) = model.get() else {
            return Ok(false);
        };
        if protocol.is_null() {
            return Ok(false);
        }
        let mut log = format!(
            " Step File Name : {}({} ents) ",
            String::from_utf8_lossy(ctx.file_name().as_bytes()),
            live.nb_entities()
        );
        let mut writer = StepDataStepWriter::new(&model);
        for rank in 1..=ctx.nb_modifiers() {
            ctx.set_modifier(rank)?;
            let general = ctx.file_modifier();
            if general.is_null() {
                continue;
            }
            let modifier = general.cast::<dyn StepSelectFileModifier>();
            let Some(file_modifier) = modifier.get() else {
                ctx.c_check(0)?
                    .add_fail_text("STEP file modifier has an incompatible type", "");
                return Ok(false);
            };
            file_modifier.perform(ctx, &mut writer)?;
            let label = String::from_utf8_lossy(file_modifier.label().as_bytes()).into_owned();
            let _ = write!(log, " .. FileMod.{rank} {label}");
            if ctx.is_for_all()? {
                log.push_str(" (all model)");
            } else {
                let _ = write!(log, " ({} entities)", ctx.nb_entities());
            }
        }
        writer.send_model(&protocol, false)?;
        let checks = writer.check_list();
        checks.start();
        while checks.more() {
            ctx.c_check(checks.number()?)?
                .get_messages(&*checks.value()?);
            checks.next();
        }
        log.push_str(" Write ");
        let printed = writer.print(ostream).map_err(|e| file_failure(&e))?;
        log.push_str(" Done\n");
        let _ = Message::send_info().write_str(&log);
        Ok(printed)
    }
    fn copy_model(
        &self,
        _original: &Handle<dyn InterfaceInterfaceModel>,
        newmodel: &Handle<dyn InterfaceInterfaceModel>,
        list: &InterfaceEntityIterator,
        tc: &mut InterfaceCopyTool,
    ) -> Result<bool, Failure> {
        if !self.thecopymode {
            return Ok(false);
        }
        for ent in list.content() {
            tc.transfer_entity(&ent)?;
        }
        tc.fill_model(newmodel)?;
        Ok(true)
    }
    fn dump_entity(
        &self,
        model: &Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
        entity: &Handle<StandardTransient>,
        s: &mut dyn Write,
        level: i32,
    ) -> Result<(), Failure> {
        let Some(live) = model.get() else {
            return Ok(());
        };
        let nument = live.number(entity);
        if nument <= 0 || nument > live.nb_entities() {
            return Ok(());
        }
        let io_error = |error: std::io::Error| file_failure(&error);
        s.write_all(b" --- (STEP) Entity ").map_err(io_error)?;
        live.print(entity, s, 0).map_err(io_error)?;
        let name = entity.dynamic_type().map_or("", |kind| kind.name());
        writeln!(s, " Type cdl : {name}").map_err(io_error)?;
        if live.is_redefined_content(nument) {
            writeln!(s, " ***  NOT WELL LOADED : CONTENT FROM FILE  ***").map_err(io_error)?;
        } else if live.is_unknown_entity(nument) {
            writeln!(s, " ***  UNKNOWN TYPE  ***").map_err(io_error)?;
        }
        let mut dump =
            StepDataStepDumper::new(&model.cast(), &protocol.cast(), *self.thelabmode.read());
        dump.dump(s, entity, level)?;
        Ok(())
    }
}
