//! The reading of the parameters of a record, by type. Each member fills a check with the
//! message and original text. Every member returns `result` because a record out of
//! range is an out of range failure, raised by the parameter access.
use super::reader_data::c_atoi;
use super::{StepDataLogical, StepDataSelectType, StepDataStepReaderData, StepDataUndefinedEntity};
use martensite_tkernel::{
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileParameter, InterfaceFileReaderDataBase,
    InterfaceFileReaderDataParams, InterfaceParamType as Param,
};

/// Adds the parameter failure to a check: `errmess` holds `%d` and `%s`, for
/// the parameter rank and the description.
pub(super) fn add_param_fail(ach: &Handle<InterfaceCheck>, errmess: &str, nump: i32, mess: &str) {
    ach.add_fail_text(&format_param_message(errmess, nump, mess), errmess);
}

pub(super) fn format_param_message(errmess: &str, nump: i32, mess: &str) -> String {
    errmess
        .replacen("%d", &nump.to_string(), 1)
        .replacen("%s", mess, 1)
}

fn fastof(param: &InterfaceFileParameter) -> f64 {
    InterfaceFileReaderDataBase::fastof(param.c_value())
}

impl StepDataStepReaderData {
    /// The parameter `nump`, `None` when it is absent.
    pub(super) fn present_param(
        &self,
        num: i32,
        nump: i32,
    ) -> Result<Option<InterfaceFileParameter>, Failure> {
        if nump > 0 && nump <= self.nb_params(num)? {
            return Ok(Some(self.param(num, nump)?));
        }
        Ok(None)
    }
    /// `read_sub_list(num, nump, mess, ach, numsub, optional = false, lenmin = 0, lenmax = 0)`.
    /// An absent optional returns false with no message and `numsub` 0.
    #[allow(clippy::too_many_arguments)]
    pub fn read_sub_list(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        numsub: &mut i32,
        optional: bool,
        _lenmin: i32,
        _lenmax: i32,
    ) -> Result<bool, Failure> {
        *numsub = self.sub_list_number(num, nump, false)?;
        if *numsub > 0 {
            if self.nb_params(*numsub)? == 0 {
                let errmess = "Parameter n0.%d (%s) is an empty LIST";
                ach.add_warning_text(&format_param_message(errmess, nump, mess), errmess);
            }
            return Ok(true);
        }
        *numsub = 0;
        if self.param(num, nump)?.param_type() == Param::ParamVoid && optional {
            return Ok(false);
        }
        add_param_fail(ach, "Parameter n0.%d (%s) not a LIST", nump, mess);
        Ok(false)
    }
    /// One real of a coordinate list; the message when it is not one.
    fn read_coordinate(
        &self,
        numsub: i32,
        rank: i32,
        target: &mut f64,
        errmess: &'static str,
    ) -> Result<Option<&'static str>, Failure> {
        let param = self.param(numsub, rank)?;
        if param.param_type() == Param::ParamReal {
            *target = fastof(&param);
            return Ok(None);
        }
        Ok(Some(errmess))
    }
    /// `read_xy(num, nump, mess, ach, X, Y)`.
    pub fn read_xy(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        x: &mut f64,
        y: &mut f64,
    ) -> Result<bool, Failure> {
        let numsub = self.sub_list_number(num, nump, false)?;
        let mut errmess = None;
        if numsub == 0 {
            errmess = Some("Parameter n0.%d (%s) : (X,Y) not a SubList");
        } else if self.nb_params(numsub)? != 2 {
            errmess = Some("Parameter n0.%d (%s) : (X,Y) has not 2 params");
        } else {
            let coords: [(&mut f64, &'static str); 2] = [
                (x, "Parameter n0.%d (%s) : (X,Y) X not a Real"),
                (y, "Parameter n0.%d (%s) : (X,Y) Y not a Real"),
            ];
            for (rank, (target, text)) in coords.into_iter().enumerate() {
                let found = self.read_coordinate(numsub, rank as i32 + 1, target, text)?;
                errmess = found.or(errmess);
            }
        }
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_xyz(num, nump, mess, ach, X, Y, Z)`.
    #[allow(clippy::too_many_arguments)]
    pub fn read_xyz(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> Result<bool, Failure> {
        let numsub = self.sub_list_number(num, nump, false)?;
        let mut errmess = None;
        if numsub == 0 {
            errmess = Some("Parameter n0.%d (%s) : (X,Y,Z) not a SubList");
        } else if self.nb_params(numsub)? != 3 {
            errmess = Some("Parameter n0.%d (%s) : (X,Y,Z) has not 3 params");
        } else {
            let coords: [(&mut f64, &'static str); 3] = [
                (x, "Parameter n0.%d (%s) : (X,Y,Z) X not a Real"),
                (y, "Parameter n0.%d (%s) : (X,Y,Z) Y not a Real"),
                (z, "Parameter n0.%d (%s) : (X,Y,Z) Z not a Real"),
            ];
            for (rank, (target, text)) in coords.into_iter().enumerate() {
                let found = self.read_coordinate(numsub, rank as i32 + 1, target, text)?;
                errmess = found.or(errmess);
            }
        }
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// The ending: true without an error, else the fail.
    pub(super) fn conclude(
        &self,
        errmess: Option<&str>,
        ach: &Handle<InterfaceCheck>,
        nump: i32,
        mess: &str,
    ) -> bool {
        match errmess {
            None => true,
            Some(errmess) => {
                add_param_fail(ach, errmess, nump, mess);
                false
            }
        }
    }
    /// `read_real(num, nump, mess, ach, val)`: an integer parameter is accepted.
    pub fn read_real(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        val: &mut f64,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) => match param.param_type() {
                Param::ParamReal | Param::ParamInteger => {
                    *val = fastof(&param);
                    None
                }
                _ => Some("Parameter n0.%d (%s) not a Real"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_entity(num, nump, mess, ach, atype, ent)` for a transient: `ent` receives an
    /// undefined entity even when the type does not match; false then.
    pub fn read_entity_transient(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        atype: &RuntimeType,
        ent: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) if param.param_type() != Param::ParamIdent => {
                Some("Parameter n0.%d (%s) not an Entity")
            }
            Some(param) if param.entity_number() <= 0 => {
                Some("Parameter n0.%d (%s) : Unresolved reference")
            }
            Some(param) => {
                let found = self.bound_entity(param.entity_number());
                if found.is_some() && found.is_kind(atype) {
                    *ent = found;
                    None
                } else {
                    if found.is_kind(StepDataUndefinedEntity::static_type()) {
                        *ent = found;
                    }
                    Some("Parameter n0.%d (%s) : Entity has illegal type")
                }
            }
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `ReadEntity<T>(num, nump, mess, ach, atype, ent)`: `ent` is only assigned on success,
    /// and false when the entity does not cast to `T`.
    pub fn read_entity<T: ?Sized + RuntimeTyped>(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        atype: &RuntimeType,
        ent: &mut Handle<T>,
    ) -> Result<bool, Failure> {
        let mut found = ent.cast::<StandardTransient>();
        if !self.read_entity_transient(num, nump, mess, ach, atype, &mut found)? {
            return Ok(false);
        }
        *ent = found.cast();
        Ok(ent.is_some())
    }
    /// `read_entity(num, nump, mess, ach, sel)`: the select checks the type and records the
    /// entity; other parameters are read as a member of the select.
    pub fn read_entity_select(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        sel: &mut dyn StepDataSelectType,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) => match param.param_type() {
                Param::ParamIdent => self.select_ident(&param, sel)?,
                Param::ParamVoid => Some("Parameter n0.%d (%s) not an Entity"),
                _ => self.select_member_param(num, nump, mess, ach, sel)?,
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    fn select_ident(
        &self,
        param: &InterfaceFileParameter,
        sel: &mut dyn StepDataSelectType,
    ) -> Result<Option<&'static str>, Failure> {
        let nent = param.entity_number();
        if nent <= 0 {
            return Ok(Some("Parameter n0.%d (%s) : Unresolved reference"));
        }
        let found = self.bound_entity(nent);
        if sel.matches(&found) {
            sel.set_value(&found)?;
            return Ok(None);
        }
        if found.is_kind(StepDataUndefinedEntity::static_type()) {
            sel.set_value(&found)?;
        }
        Ok(Some("Parameter n0.%d (%s) : Entity has illegal type"))
    }
    fn select_member_param(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        sel: &mut dyn StepDataSelectType,
    ) -> Result<Option<&'static str>, Failure> {
        let mut member = sel.new_member().cast::<StandardTransient>();
        let mut errmess = None;
        if !self.read_any(num, nump, mess, ach, &sel.description(), &mut member)? {
            errmess = Some("Parameter n0.%d (%s) : could not be read");
        }
        if !sel.matches(&member) {
            return Ok(Some("Parameter n0.%d (%s) : illegal parameter type"));
        }
        sel.set_value(&member)?;
        Ok(errmess)
    }
    /// `read_integer(num, nump, mess, ach, val)`: a real is rounded, with a fail.
    pub fn read_integer(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        val: &mut i32,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) => match param.param_type() {
                Param::ParamInteger => {
                    *val = c_atoi(param.c_value());
                    None
                }
                Param::ParamReal => {
                    *val = fastof(&param).round() as i32;
                    Some("Parameter n0.%d (%s) was rounded")
                }
                _ => Some("Parameter n0.%d (%s) not an Integer"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_boolean(num, nump, mess, ach, flag)`: any failure leaves `flag` true.
    pub fn read_boolean(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        flag: &mut bool,
    ) -> Result<bool, Failure> {
        *flag = true;
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent.It was set to true"),
            Some(param) if param.param_type() != Param::ParamEnum => {
                Some("Parameter n0.%d (%s) not a Boolean. It was set to true")
            }
            Some(param) => match param.c_value() {
                b".T." => None,
                b".F." => {
                    *flag = false;
                    None
                }
                _ => Some("Parameter n0.%d (%s) : Incorrect Boolean Value. It was set to true"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_logical(num, nump, mess, ach, flag)`.
    pub fn read_logical(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        flag: &mut StepDataLogical,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) if param.param_type() != Param::ParamEnum => {
                Some("Parameter n0.%d (%s) not a Logical")
            }
            Some(param) => match param.c_value() {
                b".T." => {
                    *flag = StepDataLogical::LTrue;
                    None
                }
                b".F." => {
                    *flag = StepDataLogical::LFalse;
                    None
                }
                b".U." => {
                    *flag = StepDataLogical::LUnknown;
                    None
                }
                _ => Some("Parameter n0.%d (%s) : Incorrect Logical Value"),
            },
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `read_string(num, nump, mess, ach, val)`: the text without its quotes, its control
    /// directives decoded.
    pub fn read_string(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        val: &mut Handle<HAsciiString>,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => Some("Parameter n0.%d (%s) absent"),
            Some(param) if param.param_type() == Param::ParamText => {
                *val = self.clean_text(param.c_value())?;
                None
            }
            Some(_) => Some("Parameter n0.%d (%s) not a quoted String"),
        };
        Ok(self.conclude(errmess, ach, nump, mess))
    }
    /// `check_derived(num, nump, mess, ach, errstat = false)`: true for `*`. The test
    /// is kept: the message is a warning when `errstat` is false.
    pub fn check_derived(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        errstat: bool,
    ) -> Result<bool, Failure> {
        let errmess = match self.present_param(num, nump)? {
            None => "Parameter n0.%d (%s) absent",
            Some(param) if param.c_value() == b"*" => return Ok(true),
            Some(_) => "Parameter n0.%d (%s) not Derived",
        };
        let text = format_param_message(errmess, nump, mess);
        if errstat {
            ach.add_fail_text(&text, errmess);
        } else {
            ach.add_warning_text(&text, errmess);
        }
        Ok(false)
    }
}
