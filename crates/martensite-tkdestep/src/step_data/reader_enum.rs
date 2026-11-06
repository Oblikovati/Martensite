//! The reading of enumerations and of parameters given as `TYPE(val)`.
use super::reader_read::add_param_fail;
use super::{StepDataEnumTool, StepDataStepReaderData};
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderDataParams, InterfaceParamType as Param,
};

impl StepDataStepReaderData {
    /// `read_enum_param(num, nump, mess, ach, text)`: the dotted text; compare it with
    /// `text.as_str()`.
    pub fn read_enum_param(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        text: &mut String,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) => match param.param_type() {
                Param::ParamEnum => {
                    *text = String::from_utf8_lossy(param.c_value()).into_owned();
                    None
                }
                Param::ParamVoid => {
                    Some("Parameter n0.%d (%s) : Undefined Enumeration not allowed")
                }
                _ => Some("Parameter n0.%d (%s) not an Enumeration"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `fail_enum_value(num, nump, mess, ach)`.
    pub fn fail_enum_value(&self, _num: i32, nump: i32, mess: &str, ach: &Handle<InterfaceCheck>) {
        add_param_fail(
            ach,
            "Parameter n0.%d (%s) : Incorrect Enumeration Value",
            nump,
            mess,
        );
    }
    /// `read_enum(num, nump, mess, ach, enumtool, val)`.
    pub fn read_enum(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        enumtool: &StepDataEnumTool,
        val: &mut i32,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) => match param.param_type() {
                Param::ParamEnum => {
                    *val = enumtool.value(param.c_value());
                    (*val < 0).then_some("Parameter n0.%d (%s) : Incorrect Enumeration Value")
                }
                Param::ParamVoid => {
                    *val = enumtool.null_value();
                    (*val < 0).then_some("Parameter n0.%d (%s) : Undefined Enumeration not allowed")
                }
                _ => Some("Parameter n0.%d (%s) not an Enumeration"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_typed_param(num, nump, mustbetyped, mess, ach, numr, numrp, typ)`: resolves a
    /// parameter given as `TYPE(val)`. `numr`, `numrp` are where to read it.
    #[allow(clippy::too_many_arguments)]
    pub fn read_typed_param(
        &self,
        num: i32,
        nump: i32,
        mustbetyped: bool,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        numr: &mut i32,
        numrp: &mut i32,
        typ: &mut AsciiString,
    ) -> Result<bool, Failure> {
        let Some(param) = self.present_param(num, nump)? else {
            add_param_fail(ach, "Parameter n0.%d (%s) absent", nump, mess);
            return Ok(false);
        };
        if param.param_type() != Param::ParamSub {
            *numr = num;
            *numrp = nump;
            *typ = AsciiString::default();
            if mustbetyped {
                add_param_fail(ach, "Parameter n0.%d (%s) : single, not typed", nump, mess);
                return Ok(false);
            }
            return Ok(true);
        }
        *numr = param.entity_number();
        *numrp = 1;
        let errmess =
            (self.nb_params(*numr)? != 1).then_some("Parameter n0.%d (%s) : SubList, not typed");
        *typ = self.record_type(*numr);
        Ok(self.conclude(errmess, ach, nump, mess))
    }
}
