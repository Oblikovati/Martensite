//! `STEPEditEditSDR`: an editor fit for a Shape Definition Representation which designates a Product
//! Definition.
use crate::step_construct::StepConstructPart;
use crate::step_data::StepDataStepModel;
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
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
/// use martensite_tkdestep::step_edit::STEPEditEditSDR;
/// use martensite_tkxsbase::if_select::IFSelectEditor;
/// let editor = STEPEditEditSDR::new().unwrap();
/// assert_eq!(editor.get().unwrap().nb_values(), 11);
/// assert_eq!(editor.get().unwrap().label().as_bytes(), b"STEP : Product Data (SDR)");
/// ```
pub struct STEPEditEditSDR {
    transient: StandardTransient,
    base: IFSelectEditorBase,
}
impl_transient!(
    STEPEditEditSDR,
    "STEPEditEditSDR",
    dyn IFSelectEditor,
    transient,
    interfaces: [dyn IFSelectEditor]
);

impl STEPEditEditSDR {
    /// `STEPEditEditSDR()`.
    pub fn new() -> Result<Handle<Self>, Failure> {
        let editor = Handle::new(Self {
            transient: StandardTransient::default(),
            base: IFSelectEditorBase::new(11),
        });
        let Some(live) = editor.get() else {
            return Ok(editor);
        };
        live.base.bind_self(&editor.cast());
        const VALUES: [(&str, &str); 11] = [
            ("PDC_Name", "PDC_Name"),
            ("PDC_LifeCycleStage", "PDC_Stage"),
            ("PDC_Description", "PDC_Descr"),
            ("P_Version_Id", "PV_Id"),
            ("P_Version_Description", "PV_Descr"),
            ("Product_Id", "P_Id"),
            ("Product_Name", "P_Name"),
            ("Product_Description", "P_Descr"),
            ("P_Context_Name", "PC_Name"),
            ("P_Context_Discipline", "PC_Disc"),
            ("Application", "Appli"),
        ];
        for (rank, (name, short)) in VALUES.iter().enumerate() {
            let value = InterfaceTypedValue::new(name, InterfaceParamType::ParamText, "")?;
            live.set_value(
                rank as i32 + 1,
                &Handle::new(value),
                short,
                IFSelectEditValue::Editable,
            )?;
        }
        Ok(editor)
    }
}

/// The part tool of an SDR of a STEP model, `None` for other entities or models.
fn part_of(
    ent: &Handle<StandardTransient>,
    model: &Handle<dyn InterfaceInterfaceModel>,
) -> Option<StepConstructPart> {
    let sdr = ent.cast::<StepShapeShapeDefinitionRepresentation>();
    if sdr.is_null() || model.cast::<StepDataStepModel>().is_null() {
        return None;
    }
    let mut part = StepConstructPart::new();
    part.read_sdr(&sdr);
    Some(part)
}

impl IFSelectEditor for STEPEditEditSDR {
    fn editor_base(&self) -> &IFSelectEditorBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        AsciiString::new("STEP : Product Data (SDR)")
    }
    fn recognize(&self, _form: &Handle<IFSelectEditForm>) -> Result<bool, Failure> {
        // il faut 10 parametres
        Ok(true)
    }
    /// The default values, in which the name of the application is Martensite.
    fn string_value(
        &self,
        _form: &Handle<IFSelectEditForm>,
        num: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        let value = match num {
            1 => "as proposed",
            2 => "First_Design",
            3 | 5 | 8 => "void",
            4 => "ID",
            9 => "Mechanical",
            10 => "Assembly",
            11 => "Martensite",
            _ => return Ok(Handle::default()),
        };
        Ok(Handle::new(HAsciiString::new(value)))
    }
    fn load(
        &self,
        form: &Handle<IFSelectEditForm>,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let (Some(ctx), Some(form)) = (part_of(ent, model), form.get()) else {
            return Ok(false);
        };
        form.load_value(1, &ctx.pdc_name()?);
        form.load_value(2, &ctx.pdc_stage()?);
        form.load_value(3, &ctx.pd_description()?);
        form.load_value(4, &ctx.pdf_id()?);
        form.load_value(5, &ctx.pdf_description()?);
        form.load_value(6, &ctx.pid()?);
        form.load_value(7, &ctx.pname()?);
        form.load_value(8, &ctx.pdescription()?);
        form.load_value(9, &ctx.pc_name()?);
        form.load_value(10, &ctx.pc_discipline_type()?);
        form.load_value(11, &ctx.ac_application()?);
        Ok(true)
    }
    fn apply(
        &self,
        form: &Handle<IFSelectEditForm>,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let (Some(ctx), Some(form)) = (part_of(ent, model), form.get()) else {
            return Ok(false);
        };
        type Setter = fn(&StepConstructPart, &Handle<HAsciiString>) -> Result<(), Failure>;
        const SETTERS: [Setter; 11] = [
            StepConstructPart::set_pdc_name,
            StepConstructPart::set_pdc_stage,
            StepConstructPart::set_pd_description,
            StepConstructPart::set_pdf_id,
            StepConstructPart::set_pdf_description,
            StepConstructPart::set_pid,
            StepConstructPart::set_pname,
            StepConstructPart::set_pdescription,
            StepConstructPart::set_pc_name,
            StepConstructPart::set_pc_discipline_type,
            StepConstructPart::set_ac_application,
        ];
        for (rank, setter) in SETTERS.iter().enumerate() {
            let num = rank as i32 + 1;
            if form.is_modified(num)? {
                setter(&ctx, &form.edited_value(num)?)?;
            }
        }
        Ok(true)
    }
}
