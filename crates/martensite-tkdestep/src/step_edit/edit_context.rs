//! `STEPEditEditContext`: an editor fit for the Product Definition Context, one per model:
//! ProductDefinition, ApplicationProtocolDefinition, ProductRelatedProductCategory.
use crate::step_construct::StepConstructContextTool;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::{IFSelectEditForm, IFSelectEditValue, IFSelectEditor, IFSelectEditorBase},
    interface::{InterfaceInterfaceModel, InterfaceParamType, InterfaceTypedValue},
};

/// ```
/// use martensite_tkdestep::step_edit::STEPEditEditContext;
/// use martensite_tkxsbase::if_select::IFSelectEditor;
/// let editor = STEPEditEditContext::new().unwrap();
/// assert_eq!(editor.get().unwrap().nb_values(), 5);
/// assert_eq!(editor.get().unwrap().label().as_bytes(), b"STEP : Product Definition Context");
/// ```
pub struct STEPEditEditContext {
    transient: StandardTransient,
    base: IFSelectEditorBase,
}
impl_transient!(
    STEPEditEditContext,
    "STEPEditEditContext",
    dyn IFSelectEditor,
    transient,
    interfaces: [dyn IFSelectEditor]
);

impl STEPEditEditContext {
    /// `STEPEditEditContext()`.
    pub fn new() -> Result<Handle<Self>, Failure> {
        let editor = Handle::new(Self {
            transient: StandardTransient::default(),
            base: IFSelectEditorBase::new(5),
        });
        let Some(live) = editor.get() else {
            return Ok(editor);
        };
        live.base.bind_self(&editor.cast());
        const VALUES: [(&str, InterfaceParamType, &str, IFSelectEditValue); 5] = [
            (
                "Application Context",
                InterfaceParamType::ParamIdent,
                "AC",
                IFSelectEditValue::EditRead,
            ),
            (
                "AppliContext_Status",
                InterfaceParamType::ParamText,
                "AC_Status",
                IFSelectEditValue::Editable,
            ),
            (
                "AppliContext_Schema",
                InterfaceParamType::ParamText,
                "AC_Schema",
                IFSelectEditValue::Editable,
            ),
            (
                "AppliContext_Year",
                InterfaceParamType::ParamInteger,
                "AC_Year",
                IFSelectEditValue::Editable,
            ),
            (
                "AppliContext_Name",
                InterfaceParamType::ParamText,
                "AC_Name",
                IFSelectEditValue::Editable,
            ),
        ];
        for (rank, (name, kind, short, access)) in VALUES.iter().enumerate() {
            let value = InterfaceTypedValue::new(name, *kind, "")?;
            live.set_value(rank as i32 + 1, &Handle::new(value), short, *access)?;
        }
        Ok(editor)
    }
}

/// The context tool of a model, with default factors.
fn context_of(model: &Handle<StepDataStepModel>) -> Result<StepConstructContextTool, Failure> {
    let mut ctx = StepConstructContextTool::with_model(model)?;
    ctx.set_global_factor(&StepDataFactors::default());
    Ok(ctx)
}

impl IFSelectEditor for STEPEditEditContext {
    fn editor_base(&self) -> &IFSelectEditorBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        AsciiString::new("STEP : Product Definition Context")
    }
    fn recognize(&self, _form: &Handle<IFSelectEditForm>) -> Result<bool, Failure> {
        // il faut 17 parametres
        Ok(true)
    }
    /// The default values, in which the name of the application is Martensite.
    fn string_value(
        &self,
        _form: &Handle<IFSelectEditForm>,
        num: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        let value = match num {
            2 => "DIS",
            3 => "automotive_design",
            4 => "1998",
            5 => "Martensite",
            7 => "Undefined Category",
            8 => "Undefined Description",
            _ => return Ok(Handle::default()),
        };
        Ok(Handle::new(HAsciiString::new(value)))
    }
    fn load(
        &self,
        form: &Handle<IFSelectEditForm>,
        _ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let step = model.cast::<StepDataStepModel>();
        let (Some(live), Some(form)) = (step.get(), form.get()) else {
            return Ok(false);
        };
        let ctx = context_of(&step)?;
        form.load_value(1, &live.string_label(&ctx.get_apd().cast()));
        form.load_value(2, &ctx.get_ac_status());
        form.load_value(3, &ctx.get_ac_schema_name());
        form.load_value(
            4,
            &Handle::new(HAsciiString::from_integer(ctx.get_ac_year()?)),
        );
        form.load_value(5, &ctx.get_ac_name());
        Ok(true)
    }
    fn apply(
        &self,
        form: &Handle<IFSelectEditForm>,
        _ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let step = model.cast::<StepDataStepModel>();
        let (Some(_), Some(form)) = (step.get(), form.get()) else {
            return Ok(false);
        };
        let mut ctx = context_of(&step)?;
        ctx.add_apd(false)?; // on ne sait jamais
        if form.is_modified(2)? {
            ctx.set_ac_status(&form.edited_value(2)?);
        }
        if form.is_modified(3)? {
            ctx.set_ac_schema_name(&form.edited_value(3)?);
        }
        if form.is_modified(4)? {
            let year = form.edited_value(4)?;
            let year = crate::step_entity::step_live(&year, "STEPEditEditContext::Apply")?;
            ctx.set_ac_year(year.integer_value()?);
        }
        if form.is_modified(5)? {
            ctx.set_ac_name(&form.edited_value(5)?)?;
        }
        Ok(true)
    }
}
