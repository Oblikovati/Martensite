//! The reading of a select member, a field and a list of fields.
use super::reader_any::{fastof, logical_of};
use super::reader_data::c_atoi;
use super::reader_read::add_param_fail;
use super::{
    StepDataESDescr, StepDataField, StepDataFieldList, StepDataPDescr, StepDataSelectMember,
    StepDataStepReaderData,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderDataParams, InterfaceParamType as Param,
};

impl StepDataStepReaderData {
    /// `read_member(num, nump, mess, ach, val)`: into an existing member, which must stay the
    /// same, or a new one.
    pub fn read_member(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        val: &mut Handle<dyn StepDataSelectMember>,
    ) -> Result<bool, Failure> {
        let mut value = val.cast::<StandardTransient>();
        let nuldescr = Handle::default();
        if value.is_null() {
            if !self.read_any(num, nump, mess, ach, &nuldescr, &mut value)? {
                return Ok(false);
            }
            *val = value.cast();
            return Ok(val.is_some());
        }
        let result = self.read_any(num, nump, mess, ach, &nuldescr, &mut value)?;
        if value == val.cast::<StandardTransient>() {
            return Ok(result);
        }
        add_param_fail(
            ach,
            "Parameter n0.%d (%s) : does not match SELECT clause",
            nump,
            mess,
        );
        Ok(false)
    }
    /// `ReadMember<T>(num, nump, mess, ach, val)`: the safe variant for a specific member.
    pub fn read_member_typed<T>(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        val: &mut Handle<T>,
    ) -> Result<bool, Failure>
    where
        T: ?Sized + martensite_tkernel::standard::RuntimeTyped,
    {
        let mut member = val.cast::<dyn StepDataSelectMember>();
        if !self.read_member(num, nump, mess, ach, &mut member)? {
            return Ok(false);
        }
        *val = member.cast();
        Ok(val.is_some())
    }
    /// `read_field(num, nump, mess, ach, descr, fild)`: always true.
    pub fn read_field(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
        fild: &mut StepDataField,
    ) -> Result<bool, Failure> {
        let param = self.param(num, nump)?;
        let text = param.c_value();
        let mut read = true;
        match param.param_type() {
            Param::ParamInteger => fild.set_integer(c_atoi(text)),
            Param::ParamReal => fild.set_real(fastof(text)),
            Param::ParamIdent => {
                if param.entity_number() > 0 {
                    fild.set_entity(&self.bound_entity(param.entity_number()));
                }
            }
            Param::ParamVoid => {}
            Param::ParamText => fild.set(&self.clean_text(text)?.cast()),
            Param::ParamEnum => match logical_of(text) {
                Some(logical) => fild.set_logical(logical),
                None => fild.set_enum(-1, text),
            },
            Param::ParamSub => {
                let mut sub = Handle::default();
                let kind = self.read_sub(param.entity_number(), mess, ach, descr, &mut sub)?;
                if kind >= 0 {
                    fild.clear_int(kind);
                    fild.set(&sub);
                }
            }
            _ => read = false,
        }
        if !read && text == b"*" {
            fild.set_derived();
        }
        Ok(true)
    }
    /// `read_list(num, ach, descr, list)`: the fields controlled by a description.
    pub fn read_list(
        &self,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataESDescr>,
        list: &mut dyn StepDataFieldList,
    ) -> Result<bool, Failure> {
        let live = descr.get().ok_or_else(|| {
            exceptions::null_object("StepDataStepReaderData::ReadList: null description")
        })?;
        let nb = list.nb_fields();
        if !self.check_nb_params(num, nb, ach, live.type_name())? {
            return Ok(false);
        }
        for rank in 1..=nb {
            let pde = live.field(rank);
            let name = pde.get().map(StepDataPDescr::name).unwrap_or_default();
            self.read_field(num, rank, &name, ach, &pde, list.c_field(rank)?)?;
        }
        Ok(true)
    }
}
