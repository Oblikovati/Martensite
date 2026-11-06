//! The modifiers of the header of a model, and those that apply the edits of the model.
use super::IgesSelectModelModifier;
use super::modifier_bases::{add_fail, add_warning, iges_model_modifier, modifier_base};
use crate::iges_data::{
    IgesDataBasicEditor, IgesDataGlobalSection, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataProtocolOps, iges_atoi, iges_live,
};
use martensite_tkernel::{
    osd::Process,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::IFSelectContextModif,
    interface::{
        InterfaceCheck, InterfaceCopyTool, InterfaceInterfaceModel, InterfaceModelEntities,
    },
};

/// Declares a modifier without own field, with a constructor; `$may_change` tells whether it may
/// change the graph of dependences.
macro_rules! plain_modifier {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $may_change:literal, $label:literal) => {
        iges_model_modifier! {
            $(#[$meta])*
            $class, $recorded, {}, self => AsciiString::new($label)
        }
        impl Default for $class {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $class {
            #[doc = concat!("A `", $recorded, "`.")]
            pub fn new() -> Self {
                Self {
                    transient: StandardTransient::default(),
                    base: modifier_base($may_change),
                }
            }
        }
    };
}

/// The current date as the file says it, with the year on two digits before 2000.
fn date_now() -> Handle<HAsciiString> {
    let mut process = Process::new();
    process.system_date().map_or_else(
        |_| IgesDataGlobalSection::new_date_string(0, 0, 0, 0, 0, 0, -1),
        |date| {
            let (month, day, year, hour, minute, second, _, _) = date.values();
            let mode = if year < 2000 { 0 } else { -1 };
            IgesDataGlobalSection::new_date_string(year, month, day, hour, minute, second, mode)
        },
    )
}

/// A check of the header of the model after a change.
fn verified(target: &IgesDataIgesModel) -> Handle<InterfaceCheck> {
    let mut check = Handle::new(InterfaceCheck::new());
    target.verify_check(&mut check);
    check
}

plain_modifier! {
    /// Corrects the selected entities.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectAutoCorrect;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// assert_eq!(IgesSelectAutoCorrect::new().label().as_bytes(), b"Auto-Correction of IGES Entities");
    /// ```
    IgesSelectAutoCorrect, "IgesSelectAutoCorrect", false, "Auto-Correction of IGES Entities"
}

impl IgesSelectModelModifier for IgesSelectAutoCorrect {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let protocol = ctx.protocol().cast::<dyn IgesDataProtocolOps>();
        if protocol.is_null() {
            return add_fail(ctx, "IGES Auto Correct, not called with Protocol");
        }
        let corrector = IgesDataBasicEditor::with_model(target, &protocol);
        ctx.start();
        while ctx.more() {
            let entity = ctx.value_result()?.cast::<IgesDataIgesEntity>();
            if corrector.auto_correct(&entity)? {
                ctx.trace("")?;
            }
            ctx.next();
        }
        Ok(())
    }
}

plain_modifier! {
    /// Computes the status of the entities of the model.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectComputeStatus;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// assert!(!IgesSelectComputeStatus::new().may_change_graph());
    /// ```
    IgesSelectComputeStatus, "IgesSelectComputeStatus", false,
    "Compute Status of IGES Entities (Directory Part)"
}

impl IgesSelectModelModifier for IgesSelectComputeStatus {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let protocol = ctx.protocol().cast::<dyn IgesDataProtocolOps>();
        if protocol.is_null() {
            return add_fail(ctx, "IGES Compute Status, Protocol incorrect");
        }
        IgesDataBasicEditor::with_model(target, &protocol).compute_status()
    }
}

iges_model_modifier! {
    /// Sets one parameter of the global section, by its rank, from a text.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSetGlobalParameter;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// let modifier = IgesSelectSetGlobalParameter::new(3);
    /// assert_eq!(modifier.global_number(), 3);
    /// assert_eq!(modifier.label().as_bytes(), b"Set IGES Global Parameter (undefined)");
    /// ```
    IgesSelectSetGlobalParameter, "IgesSelectSetGlobalParameter",
    { thenum: i32, theval: Handle<HAsciiString> },
    self => self.parameter_label()
}

impl IgesSelectSetGlobalParameter {
    /// A modifier of the parameter `numpar`.
    pub fn new(numpar: i32) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(false),
            thenum: SharedState::new(numpar),
            theval: SharedState::new(Handle::default()),
        }
    }
    /// The rank of the parameter.
    pub fn global_number(&self) -> i32 {
        *self.thenum.read()
    }
    /// Sets the text of the parameter.
    pub fn set_value(&self, text: &Handle<HAsciiString>) {
        *self.theval.write() = text.clone();
    }
    /// The text of the parameter.
    pub fn value(&self) -> Handle<HAsciiString> {
        self.theval.read().clone()
    }
    fn parameter_label(&self) -> AsciiString {
        let value = self.value();
        let Some(text) = value.get() else {
            return AsciiString::new("Set IGES Global Parameter (undefined)");
        };
        let mut label = format!(
            "Set IGES Global Parameter Number {} to ",
            self.global_number()
        )
        .into_bytes();
        label.extend_from_slice(text.string().as_bytes());
        AsciiString::from_bytes(label)
    }
}

impl IgesSelectModelModifier for IgesSelectSetGlobalParameter {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let value = self.value();
        let Some(text) = value.get() else {
            return add_warning(ctx, "Set IGES Global Parameter, no value defined, ignored");
        };
        let model = iges_live(target, "set global parameter")?;
        let mut header = model.global_section();
        let params = header.params()?;
        let set = iges_live(&params, "set global parameter")?;
        let thenum = self.global_number();
        if thenum <= 0 || thenum > set.nb_params() {
            let mess = format!("Set IGES Global Parameter : Number {thenum} incorrect");
            return add_fail(ctx, &mess);
        }
        let content = text.string().as_bytes().to_vec();
        set.change_param(thenum, |parameter| {
            let kind = parameter.param_type();
            parameter.init_text(&content, kind);
        })?;
        let check = Handle::new(InterfaceCheck::new());
        header.init(&params, &check)?;
        ctx.add_check(&check);
        if !iges_live(&check, "set global parameter")?.has_failed() {
            model.set_global_section(&header);
        }
        Ok(())
    }
}

iges_model_modifier! {
    /// Clears the short labels of the selected entities, or sets them to their directory number.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSetLabel;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// let label = IgesSelectSetLabel::new(1, true).label();
    /// assert_eq!(label.as_bytes(), b"Set Short Label to DE Number (enforced)");
    /// ```
    IgesSelectSetLabel, "IgesSelectSetLabel",
    { themode: i32, theforce: bool },
    self => self.mode_label()
}

impl IgesSelectSetLabel {
    /// A modifier of the mode 0 to clear, 1 for the directory number; `enforce` replaces any label.
    pub fn new(mode: i32, enforce: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(false),
            themode: SharedState::new(mode),
            theforce: SharedState::new(enforce),
        }
    }
    fn mode_label(&self) -> AsciiString {
        let mode = match *self.themode.read() {
            0 => "Clear Short Label",
            1 => "Set Short Label to DE Number",
            _ => "",
        };
        let enforced = if *self.theforce.read() {
            " (enforced)"
        } else {
            ""
        };
        AsciiString::new(&format!("{mode}{enforced}"))
    }
}

/// Whether a label is one this modifier wrote: `DE`, then a positive number.
fn is_directory_label(label: &HAsciiString) -> bool {
    let content = label.string();
    let text = content.as_bytes();
    text.len() > 2 && text.starts_with(b"DE") && iges_atoi(&text[2..]) > 0
}

impl IgesSelectModelModifier for IgesSelectSetLabel {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let model = iges_live(target, "set label")?;
        let mode = *self.themode.read();
        let force = *self.theforce.read();
        ctx.start();
        while ctx.more() {
            let entity = ctx.value_result()?.cast::<IgesDataIgesEntity>();
            if let Some(iges) = entity.get() {
                if mode == 0 {
                    iges.set_label(&Handle::default(), -1);
                } else {
                    let mut lab = iges.short_label();
                    if force || lab.get().is_some_and(is_directory_label) {
                        lab.nullify();
                    }
                    if lab.is_null() {
                        let number = 2 * model.number(&entity.cast()) - 1;
                        lab = Handle::new(HAsciiString::new(format!("DE{number}")));
                    }
                    iges.set_label(&lab, -1);
                }
            }
            ctx.next();
        }
        Ok(())
    }
}

plain_modifier! {
    /// Updates the version of the norm to 5.1, when it is older.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSetVersion5;
    /// let _modifier = IgesSelectSetVersion5::new();
    /// ```
    IgesSelectSetVersion5, "IgesSelectSetVersion5", false,
    "Update IGES Version in Global Section to 5.1"
}

impl IgesSelectModelModifier for IgesSelectSetVersion5 {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let model = iges_live(target, "set version")?;
        let mut header = model.global_section();
        if header.iges_version() >= 9 {
            return Ok(());
        }
        header.set_iges_version(9);
        header.set_last_change_date_now();
        model.set_global_section(&header);
        let check = verified(model);
        let live = iges_live(&check, "set version")?;
        if live.has_failed()
            && let Some(general) = ctx.c_check(0)?.get()
        {
            general.get_messages(live);
        }
        Ok(())
    }
}

plain_modifier! {
    /// Sets the creation date of the header to now.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectUpdateCreationDate;
    /// let _modifier = IgesSelectUpdateCreationDate::new();
    /// ```
    IgesSelectUpdateCreationDate, "IgesSelectUpdateCreationDate", false,
    "Update Creation Date in IGES Global Section"
}

impl IgesSelectModelModifier for IgesSelectUpdateCreationDate {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let model = iges_live(target, "update creation date")?;
        model.change_global_section(|header| header.set_date(&date_now()));
        ctx.add_check(&verified(model));
        Ok(())
    }
}

plain_modifier! {
    /// Sets the file name of the header to the one of the context.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectUpdateFileName;
    /// let _modifier = IgesSelectUpdateFileName::new();
    /// ```
    IgesSelectUpdateFileName, "IgesSelectUpdateFileName", false,
    "Updates IGES File Name to new current one"
}

impl IgesSelectModelModifier for IgesSelectUpdateFileName {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        if !ctx.has_file_name() {
            return add_warning(ctx, "New File Name unknown, former one is kept");
        }
        let model = iges_live(target, "update file name")?;
        let name = Handle::new(HAsciiString::new(ctx.file_name().as_bytes()));
        model.change_global_section(|header| header.set_file_name(&name));
        ctx.add_check(&verified(model));
        Ok(())
    }
}

plain_modifier! {
    /// Sets the date of the header to now, with the version it needs. The field set is the
    /// creation date, despite the label.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectUpdateLastChange;
    /// let _modifier = IgesSelectUpdateLastChange::new();
    /// ```
    IgesSelectUpdateLastChange, "IgesSelectUpdateLastChange", false,
    "Update Last Change Date in IGES Global Section"
}

impl IgesSelectModelModifier for IgesSelectUpdateLastChange {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let model = iges_live(target, "update last change")?;
        model.change_global_section(|header| {
            if header.iges_version() < 9 {
                header.set_iges_version(9);
            }
            header.set_date(&date_now());
        });
        ctx.add_check(&verified(model));
        Ok(())
    }
}
