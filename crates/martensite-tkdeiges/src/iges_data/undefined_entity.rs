//! The entities the norm does not recognise: their parameters are kept as they are.
use super::{
    IgesDataColorEntity, IgesDataLabelDisplayEntity, IgesDataLevelListEntity,
    IgesDataLineFontEntity, IgesDataTransfEntity, IgesDataViewKindEntity,
};
use super::{
    IgesDataDirPart, IgesDataEntityKind, IgesDataIgesEntity, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader, iges_live, iges_msg,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, SharedState},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataParams, InterfaceParamType,
    InterfaceUndefinedContent,
};

/// An entity the norm does not recognise, or whose directory entry is wrong.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataUndefinedEntity;
/// let entity = IgesDataUndefinedEntity::new();
/// assert!(entity.is_ok_dir_part());
/// ```
pub struct IgesDataUndefinedEntity {
    parent: IgesDataIgesEntity,
    thecont: SharedState<Handle<InterfaceUndefinedContent>>,
}
impl_transient!(
    IgesDataUndefinedEntity,
    "IgesDataUndefinedEntity",
    IgesDataIgesEntity,
    parent
);

impl Default for IgesDataUndefinedEntity {
    fn default() -> Self {
        Self {
            parent: IgesDataIgesEntity::default(),
            thecont: SharedState::new(Handle::new(InterfaceUndefinedContent::new())),
        }
    }
}

impl std::ops::Deref for IgesDataUndefinedEntity {
    type Target = IgesDataIgesEntity;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl IgesDataEntityKind for IgesDataUndefinedEntity {
    fn iges_entity(&self) -> &IgesDataIgesEntity {
        &self.parent
    }
}

/// Whether the reference of a directory field goes to an entity of the required kind.
fn field_kind_is(ir: &IgesDataIgesReaderData, num: i32, kind: &RuntimeType) -> bool {
    ir.bound_entity(num).is_kind(kind)
}

impl IgesDataUndefinedEntity {
    /// An entity with empty content and no directory error.
    pub fn new() -> Self {
        Self::default()
    }
    /// The content.
    pub fn undefined_content(&self) -> Handle<InterfaceUndefinedContent> {
        self.thecont.read().clone()
    }
    /// The content, to change it.
    pub fn changeable_content(&self) -> Handle<InterfaceUndefinedContent> {
        self.undefined_content()
    }
    /// Sets the content.
    pub fn set_new_content(&self, cont: &Handle<InterfaceUndefinedContent>) {
        *self.thecont.write() = cont.clone();
    }
    /// Whether the directory entry was without error.
    pub fn is_ok_dir_part(&self) -> bool {
        self.dir_status() == 0
    }
    /// The errors of the directory entry, by bits.
    pub fn dir_status(&self) -> i32 {
        self.parent.dir_status_bits()
    }
    /// Forgets the errors of the directory entry.
    pub fn set_ok_dir_part(&self) {
        self.parent.set_dir_status_bits(0);
    }
    /// Checks a directory entry; what is wrong is noted in the status, reported to `ach` and
    /// removed from `dp`. True when nothing was wrong.
    pub fn read_dir(
        &self,
        ir: &Handle<IgesDataIgesReaderData>,
        dp: &mut IgesDataDirPart,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<bool, Failure> {
        let data = iges_live(ir, "undefined entity: read dir")?;
        let mut fields = dp.values();
        let v = &mut fields.values;
        let max = 2 * data.nb_records();
        let mut status = 0;
        let fail = |key: &str| {
            if let Some(check) = ach.get() {
                check.send_fail(&iges_msg(key));
            }
        };
        let negative_wrong = |value: i32, kind: &RuntimeType| {
            value < -max || (value < 0 && !field_kind_is(data, (1 - value) / 2, kind))
        };
        let positive_wrong = |value: i32, kind: &RuntimeType| {
            value < 0 || value > max || (value > 0 && !field_kind_is(data, (1 + value) / 2, kind))
        };
        if negative_wrong(v[3], <dyn IgesDataLineFontEntity>::static_type()) {
            fail("XSTEP_60");
            status += 8;
            v[3] = 0;
        }
        if negative_wrong(v[4], <dyn IgesDataLevelListEntity>::static_type()) {
            fail("XSTEP_61");
            status += 32;
            // The field reset here is the one of the line font, whatever was found on the level.
            v[3] = 0;
        }
        if positive_wrong(v[5], <dyn IgesDataViewKindEntity>::static_type()) {
            fail("XSTEP_62");
            status += 128;
            v[5] = 0;
        }
        if positive_wrong(v[6], <dyn IgesDataTransfEntity>::static_type()) {
            fail("XSTEP_63");
            status |= 1;
            v[6] = 0;
        }
        if positive_wrong(v[7], <dyn IgesDataLabelDisplayEntity>::static_type()) {
            fail("XSTEP_64");
            status |= 1;
            v[7] = 0;
        }
        if v[14] < -max || v[14] > max {
            fail("XSTEP_70");
            status += 512;
            v[14] = 0;
        } else if v[14] < 0
            && !field_kind_is(
                data,
                (1 - v[14]) / 2,
                <dyn IgesDataColorEntity>::static_type(),
            )
        {
            status += 512;
            v[14] = 0;
        }
        let digits_only = fields
            .subscript
            .iter()
            .take(8)
            .take_while(|byte| **byte != 0)
            .all(|byte| *byte == b' ' || byte.is_ascii_digit());
        if !digits_only {
            fail("XSTEP_72");
            status += 1024;
            fields.subscript[..8].fill(b' ');
        }
        self.parent.set_dir_status_bits(status);
        if status != 0 {
            dp.init(&fields);
        }
        Ok(status == 0)
    }
    /// Keeps the parameters as literals; associativities and properties are not told apart.
    ///
    /// # Errors
    /// A failure of the parameter list.
    pub fn read_own_params(
        &self,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let nb = pr.nb_params();
        let content = self.undefined_content();
        let cont = iges_live(&content, "undefined entity: read")?;
        cont.reservate(nb, nb)?;
        for i in 1..=nb {
            let value = Handle::new(HAsciiString::new(pr.param_value(i)?.as_bytes()));
            cont.add_literal(pr.param_type(i)?, &value)?;
        }
        pr.set_current_number(nb + 1);
        Ok(())
    }
    /// Writes the parameters kept.
    ///
    /// # Errors
    /// A failure of the writer.
    pub fn write_own_params(&self, iw: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        write_content(&self.undefined_content(), iw)
    }
}

/// Writes the parameters of a content: void, entity or literal.
pub(super) fn write_content(
    content: &Handle<InterfaceUndefinedContent>,
    iw: &mut IgesDataIgesWriter,
) -> Result<(), Failure> {
    let cont = iges_live(content, "undefined entity: write")?;
    for i in 1..=cont.nb_params() {
        if cont.param_type(i)? == InterfaceParamType::ParamVoid {
            iw.send_void()?;
        } else if cont.is_param_entity(i)? {
            iw.send_entity(&cont.param_entity(i)?.cast::<IgesDataIgesEntity>(), false)?;
        } else {
            iw.send_string(&cont.param_value(i)?)?;
        }
    }
    Ok(())
}
