//! The late-binding reading: a parameter of any kind into a transient, a select member or
//! a field.
use super::reader_data::c_atoi;
use super::{
    StepDataLogical, StepDataPDescr, StepDataSelectArrReal, StepDataSelectInt,
    StepDataSelectMember, StepDataSelectNamed, StepDataSelectReal, StepDataStepReaderData,
};
use martensite_tkernel::{
    ncollection::{ArrayError, HArray1},
    standard::{Failure, Handle, StandardTransient, exceptions},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileParameter, InterfaceFileReaderDataBase,
    InterfaceFileReaderDataParams, InterfaceParamType as Param,
};

type TransientList = HArray1<Handle<StandardTransient>>;

fn array_failure(error: ArrayError) -> Failure {
    error.into_failure("StepDataStepReaderData")
}

pub(super) fn fastof(text: &[u8]) -> f64 {
    InterfaceFileReaderDataBase::fastof(text)
}

fn set_at<T: Send + Sync + 'static>(list: &HArray1<T>, ip: i32, value: T) -> Result<(), Failure> {
    list.change_array1()
        .set_value(ip, value)
        .map_err(array_failure)
}

/// The logical of `.F.`, `.T.`, `.U.`.
pub(super) fn logical_of(text: &[u8]) -> Option<StepDataLogical> {
    match text {
        b".F." => Some(StepDataLogical::LFalse),
        b".T." => Some(StepDataLogical::LTrue),
        b".U." => Some(StepDataLogical::LUnknown),
        _ => None,
    }
}

/// The arrays `read_sub` fills: the first parameter chooses, a mismatch turns to transients.
#[derive(Default)]
struct SubArrays {
    hin: Handle<HArray1<i32>>,
    hre: Handle<HArray1<f64>>,
    hst: Handle<HArray1<Handle<HAsciiString>>>,
    htr: Handle<TransientList>,
}

fn new_array<T: Default + Clone + Send + Sync + 'static>(
    nbp: i32,
) -> Result<Handle<HArray1<T>>, Failure>
where
    HArray1<T>: martensite_tkernel::standard::RuntimeTyped,
{
    Ok(Handle::new(
        HArray1::<T>::with_bounds(1, nbp).map_err(array_failure)?,
    ))
}

impl SubArrays {
    /// Passes to transients the values read before rank `ip`.
    fn spread_to_transients(
        &mut self,
        nbp: i32,
        ip: i32,
    ) -> Result<Handle<StandardTransient>, Failure> {
        let list = new_array::<Handle<StandardTransient>>(nbp)?;
        let live = list.get().expect("a new array");
        for jp in 1..ip {
            if let Some(ints) = self.hin.get() {
                let member = StepDataSelectInt::default();
                member.set_int(*ints.array1().value(jp).map_err(array_failure)?);
                set_at(live, jp, Handle::new(member).cast())?;
            }
            if let Some(reals) = self.hre.get() {
                let member = StepDataSelectReal::default();
                member.set_real(*reals.array1().value(jp).map_err(array_failure)?);
                set_at(live, jp, Handle::new(member).cast())?;
            }
            if let Some(texts) = self.hst.get() {
                let text = texts.array1().value(jp).map_err(array_failure)?.clone();
                set_at(live, jp, text.cast())?;
            }
        }
        self.htr = list.clone();
        Ok(list.cast())
    }
}

impl StepDataStepReaderData {
    /// `read_sub(numsub, mess, ach, descr, val)`: the content of a sub-list: a `SelectNamed`
    /// for a typed one, an array otherwise. Returns the kind for the field, 0 when empty,
    /// negative when failed. Behaviour kept: a list of mixed types stops at the first
    /// value of another type, with -1.
    pub fn read_sub(
        &self,
        numsub: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
        val: &mut Handle<StandardTransient>,
    ) -> Result<i32, Failure> {
        let nbp = self.nb_params(numsub)?;
        if nbp == 0 {
            return Ok(0);
        }
        let rectyp = self.record_type(numsub);
        if nbp == 1 && rectyp.as_bytes().first() != Some(&b'(') {
            let named = Handle::new(StepDataSelectNamed::default());
            *val = named.cast();
            let live = named.get().expect("a new member");
            live.set_name(&String::from_utf8_lossy(rectyp.as_bytes()));
            let mut member = named.cast::<StandardTransient>();
            if self.read_any(numsub, 1, mess, ach, descr, &mut member)? {
                return Ok(live.kind());
            }
            return Ok(0);
        }
        let mut kod = match self.param(numsub, 1)?.param_type() {
            Param::ParamInteger => 1,
            Param::ParamReal => 5,
            Param::ParamIdent => 7,
            Param::ParamVoid | Param::ParamSub => 0,
            Param::ParamText => 6,
            Param::ParamEnum => 4,
            _ => return Ok(-1),
        };
        let mut arrays = SubArrays::default();
        match kod {
            1 => {
                arrays.hin = new_array::<i32>(nbp)?;
                *val = arrays.hin.cast();
            }
            5 => {
                arrays.hre = new_array::<f64>(nbp)?;
                *val = arrays.hre.cast();
            }
            6 => {
                arrays.hst = new_array::<Handle<HAsciiString>>(nbp)?;
                *val = arrays.hst.cast();
            }
            _ => {
                arrays.htr = new_array::<Handle<StandardTransient>>(nbp)?;
                *val = arrays.htr.cast();
            }
        }
        for ip in 1..=nbp {
            let param = self.param(numsub, ip)?;
            kod = self.read_sub_item(kod, ip, &param, &arrays)?;
            if kod > 0 {
                continue;
            }
            if arrays.htr.is_null() {
                *val = arrays.spread_to_transients(nbp, ip)?;
            }
            self.read_mixed_item(ip, &param, &arrays, mess, ach, descr)?;
            return Ok(-1);
        }
        Ok(8)
    }
    /// One value in the array of the list; the kind becomes 0 when it does not fit.
    fn read_sub_item(
        &self,
        kod: i32,
        ip: i32,
        param: &InterfaceFileParameter,
        arrays: &SubArrays,
    ) -> Result<i32, Failure> {
        let text = param.c_value();
        let found = param.param_type();
        match kod {
            1 if found == Param::ParamInteger => {
                set_at(arrays.hin.get().expect("the list"), ip, c_atoi(text))?;
            }
            4 if found == Param::ParamEnum => {
                let named = StepDataSelectNamed::default();
                named.set_enum(-1, text);
                set_at(
                    arrays.htr.get().expect("the list"),
                    ip,
                    Handle::new(named).cast(),
                )?;
            }
            5 if found == Param::ParamReal => {
                set_at(arrays.hre.get().expect("the list"), ip, fastof(text))?;
            }
            6 if found == Param::ParamText => {
                set_at(
                    arrays.hst.get().expect("the list"),
                    ip,
                    self.clean_text(text)?,
                )?;
            }
            7 => {
                let entity = self.bound_entity(param.entity_number());
                set_at(arrays.htr.get().expect("the list"), ip, entity)?;
            }
            _ => return Ok(0),
        }
        Ok(kod)
    }
    /// The first value out of the list's type, into the transients. The
    /// real and the void do not reach the list.
    fn read_mixed_item(
        &self,
        ip: i32,
        param: &InterfaceFileParameter,
        arrays: &SubArrays,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
    ) -> Result<(), Failure> {
        let list = arrays.htr.get().expect("the transients of the list");
        let text = param.c_value();
        match param.param_type() {
            Param::ParamInteger => {
                let member = StepDataSelectInt::default();
                member.set_integer(c_atoi(text));
                set_at(list, ip, Handle::new(member).cast())?;
            }
            Param::ParamIdent => {
                set_at(list, ip, self.bound_entity(param.entity_number()))?;
            }
            Param::ParamEnum => {
                if let Some(logical) = logical_of(text) {
                    let member = StepDataSelectInt::default();
                    member.set_logical(logical);
                    set_at(list, ip, Handle::new(member).cast())?;
                } else {
                    let member = StepDataSelectNamed::default();
                    member.set_enum(-1, text);
                    set_at(list, ip, Handle::new(member).cast())?;
                }
            }
            Param::ParamText => set_at(list, ip, self.clean_text(text)?.cast())?,
            Param::ParamSub => {
                let mut sub = Handle::default();
                let kind = self.read_sub(param.entity_number(), mess, ach, descr, &mut sub)?;
                if kind >= 0 {
                    set_at(list, ip, sub)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// `read_any(num, nump, mess, ach, descr, val)`: a parameter into a transient: named or
    /// immediate member, text or the referenced entity. An existing member in `val` is
    /// filled. An integer read into an existing member goes by `set_real`.
    pub fn read_any(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
        val: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let param = self.param(num, nump)?;
        let text = param.c_value();
        match param.param_type() {
            Param::ParamInteger | Param::ParamReal if val.is_some() => {
                existing_member(val)?.set_real(fastof(text));
                Ok(true)
            }
            Param::ParamInteger => {
                let member = StepDataSelectInt::default();
                member.set_integer(c_atoi(text));
                *val = Handle::new(member).cast();
                Ok(true)
            }
            Param::ParamReal => {
                let member = StepDataSelectReal::default();
                member.set_real(fastof(text));
                *val = Handle::new(member).cast();
                Ok(true)
            }
            Param::ParamIdent => {
                if param.entity_number() > 0 {
                    *val = self.bound_entity(param.entity_number());
                }
                Ok(val.is_some())
            }
            Param::ParamEnum => {
                read_any_enum(text, val);
                Ok(true)
            }
            Param::ParamText => {
                let cleaned = self.clean_text(text)?;
                if val.is_some() {
                    let bytes = cleaned.string().as_bytes().to_vec();
                    existing_member(val)?.set_string(&bytes);
                    return Ok(true);
                }
                *val = cleaned.cast();
                Ok(true)
            }
            Param::ParamSub => self.read_any_sub(num, nump, mess, ach, descr, val),
            _ => Ok(false),
        }
    }
    fn read_any_sub(
        &self,
        num: i32,
        nump: i32,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
        val: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let numsub = self.sub_list_number(num, nump, false)?;
        if self.nb_params(numsub)? != 1 {
            return Ok(false);
        }
        let rectyp = self.record_type(numsub);
        if rectyp.as_bytes().first() == Some(&b'(') {
            return Ok(false);
        }
        let name = String::from_utf8_lossy(rectyp.as_bytes()).into_owned();
        if self.read_array_member(numsub, &name, mess, ach, descr, val)? {
            return Ok(true);
        }
        let mut member = val.cast::<dyn StepDataSelectMember>();
        if member.is_null() {
            member = Handle::new(StepDataSelectNamed::default()).cast();
            *val = member.cast();
        }
        if !member.get().is_some_and(|live| live.set_name(&name)) {
            return Ok(false);
        }
        self.read_any(numsub, 1, mess, ach, descr, val)
    }
    /// The member with an array of reals; false when `val` is not one or the list does not
    /// suit it.
    fn read_array_member(
        &self,
        numsub: i32,
        name: &str,
        mess: &str,
        ach: &Handle<InterfaceCheck>,
        descr: &Handle<StepDataPDescr>,
        val: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let target = val.cast::<StepDataSelectArrReal>();
        let Some(live) = target.get() else {
            return Ok(false);
        };
        let numsub2 = self.sub_list_number(numsub, 1, false)?;
        let nbp2 = self.nb_params(numsub2)?;
        if nbp2 <= 1 || self.param(numsub2, 1)?.param_type() != Param::ParamReal {
            return Ok(false);
        }
        if !val
            .cast::<dyn StepDataSelectMember>()
            .get()
            .is_some_and(|member| member.set_name(name))
        {
            return Ok(false);
        }
        let mut values = Vec::new();
        for rank in 1..=nbp2 {
            if self.param(numsub2, rank)?.param_type() != Param::ParamReal {
                continue;
            }
            let mut member = Handle::new(StepDataSelectReal::default()).cast();
            if self.read_any(numsub2, rank, mess, ach, descr, &mut member)?
                && let Some(real) = member.cast::<StepDataSelectReal>().get()
            {
                values.push(real.real());
            }
        }
        let array = new_array::<f64>(values.len() as i32)?;
        for (rank, value) in values.into_iter().enumerate() {
            set_at(array.get().expect("a new array"), rank as i32 + 1, value)?;
        }
        live.set_arr_real(&array);
        Ok(true)
    }
}

fn existing_member(
    val: &Handle<StandardTransient>,
) -> Result<Handle<dyn StepDataSelectMember>, Failure> {
    let member = val.cast::<dyn StepDataSelectMember>();
    if member.is_null() {
        return Err(exceptions::null_object(
            "StepDataStepReaderData::ReadAny: the given value is not a select member",
        ));
    }
    Ok(member)
}

/// The enumeration case of `read_any`: a logical or a named enumeration. Quirk kept:
/// the enumeration value is -1, its text carries the meaning.
fn read_any_enum(text: &[u8], val: &mut Handle<StandardTransient>) {
    let member = val.cast::<dyn StepDataSelectMember>();
    let logical = logical_of(text);
    if let Some(live) = member.get() {
        match logical {
            Some(logical) => live.set_logical(logical),
            None => live.set_enum(-1, text),
        }
        return;
    }
    if let Some(logical) = logical {
        let created = StepDataSelectInt::default();
        created.set_logical(logical);
        *val = Handle::new(created).cast();
        return;
    }
    let created = StepDataSelectNamed::default();
    created.set_enum(-1, text);
    *val = Handle::new(created).cast();
}
