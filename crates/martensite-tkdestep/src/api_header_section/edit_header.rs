//! `APIHeaderSectionEditHeader`: the header of a model as an editor of ten values.
use super::APIHeaderSectionMakeHeader;
use crate::step_data::StepDataStepModel;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::{IFSelectEditForm, IFSelectEditValue, IFSelectEditor, IFSelectEditorBase},
    interface::{InterfaceInterfaceModel, InterfaceParamType, InterfaceTypedValue},
};

/// `is_time_stamp`. Defect kept: rank 5 falls through its `-` test to the `0` or `1`
/// test of rank 6, so no value is ever accepted. A time stamp can only be loaded, not edited.
fn is_time_stamp(_val: &Handle<HAsciiString>) -> bool {
    false
}

/// ```
/// use martensite_tkdestep::api_header_section::APIHeaderSectionEditHeader;
/// use martensite_tkxsbase::if_select::IFSelectEditor;
/// let editor = APIHeaderSectionEditHeader::new().unwrap();
/// assert_eq!(editor.get().unwrap().nb_values(), 10);
/// ```
pub struct APIHeaderSectionEditHeader {
    transient: StandardTransient,
    base: IFSelectEditorBase,
}
impl_transient!(
    APIHeaderSectionEditHeader,
    "APIHeaderSectionEditHeader",
    dyn IFSelectEditor,
    transient,
    interfaces: [dyn IFSelectEditor]
);

impl APIHeaderSectionEditHeader {
    /// `APIHeaderSectionEditHeader()`.
    pub fn new() -> Result<Handle<Self>, Failure> {
        let editor = Handle::new(Self {
            transient: StandardTransient::default(),
            base: IFSelectEditorBase::new(10),
        });
        let Some(live) = editor.get() else {
            return Ok(editor);
        };
        live.base.bind_self(&editor.cast());
        const VALUES: [(&str, &str); 10] = [
            ("fn_name", "name"),
            ("fn_time_stamp", "time"),
            ("fn_author", "author"),
            ("fn_organization", "org"),
            ("fn_preprocessor_version", "preproc"),
            ("fn_originating_system", "orig"),
            ("fn_authorization", "autorize"),
            ("fs_schema_identifiers", "schema"),
            ("fd_description", "descr"),
            ("fd_implementation_level", "level"),
        ];
        for (rank, (name, short)) in VALUES.iter().enumerate() {
            let value = InterfaceTypedValue::new(name, InterfaceParamType::ParamText, "")?;
            if rank == 1 {
                value.set_satisfies(Some(is_time_stamp), "IsTimeStamp");
            }
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

impl IFSelectEditor for APIHeaderSectionEditHeader {
    fn editor_base(&self) -> &IFSelectEditorBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        AsciiString::new("Step Header")
    }
    fn recognize(&self, _form: &Handle<IFSelectEditForm>) -> Result<bool, Failure> {
        Ok(true)
    }
    fn string_value(
        &self,
        _form: &Handle<IFSelectEditForm>,
        num: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        Ok(self
            .typed_value(num)?
            .get()
            .map_or_else(Handle::default, |live| live.h_string_value()))
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
        let mkh = APIHeaderSectionMakeHeader::from_model(live)?;
        form.load_value(1, &mkh.name());
        form.load_value(2, &mkh.time_stamp());
        form.load_value(3, &mkh.author_value(1)?);
        form.load_value(4, &mkh.organization_value(1)?);
        form.load_value(5, &mkh.preprocessor_version());
        form.load_value(6, &mkh.originating_system());
        form.load_value(7, &mkh.authorisation());
        form.load_value(8, &mkh.schema_identifiers_value(1)?);
        form.load_value(9, &mkh.description_value(1)?);
        form.load_value(10, &mkh.implementation_level());
        Ok(true)
    }
    fn apply(
        &self,
        form: &Handle<IFSelectEditForm>,
        _ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let step = model.cast::<StepDataStepModel>();
        let (Some(live), Some(form)) = (step.get(), form.get()) else {
            return Ok(false);
        };
        let mkh = APIHeaderSectionMakeHeader::from_model(live)?;
        if form.is_modified(1)? {
            mkh.set_name(&form.edited_value(1)?);
        }
        if form.is_modified(2)? {
            mkh.set_time_stamp(&form.edited_value(2)?);
        }
        if form.is_modified(3)? {
            mkh.set_author_value(1, &form.edited_value(3)?)?;
        }
        if form.is_modified(4)? {
            mkh.set_organization_value(1, &form.edited_value(4)?)?;
        }
        if form.is_modified(5)? {
            mkh.set_preprocessor_version(&form.edited_value(5)?);
        }
        if form.is_modified(6)? {
            mkh.set_originating_system(&form.edited_value(6)?);
        }
        if form.is_modified(7)? {
            mkh.set_authorisation(&form.edited_value(7)?);
        }
        if form.is_modified(8)? {
            mkh.set_schema_identifiers_value(1, &form.edited_value(8)?)?;
        }
        if form.is_modified(9)? {
            mkh.set_description_value(1, &form.edited_value(9)?)?;
        }
        if form.is_modified(10)? {
            mkh.set_implementation_level(&form.edited_value(10)?);
        }
        mkh.apply(live)?;
        Ok(true)
    }
}
