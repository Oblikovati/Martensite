//! An undefined entity given by hand: its type, form and parameters are set one by one.
use super::{
    IgesDataEntityKind, IgesDataIgesEntity, IgesDataIgesWriter, IgesDataUndefinedEntity,
    iges_array1_item, iges_live,
};
use martensite_tkernel::{
    impl_transient,
    ncollection::{HArray1, HSequence},
    standard::{Failure, Handle, SharedState},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{InterfaceParamType, InterfaceUndefinedContent};

/// An undefined entity whose content is written through this class.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataFreeFormatEntity;
/// use martensite_tkxsbase::interface::InterfaceParamType;
/// let entity = IgesDataFreeFormatEntity::new();
/// entity.set_type_number(5000);
/// entity.add_literal_text(InterfaceParamType::ParamInteger, "12").unwrap();
/// assert_eq!(entity.nb_params().unwrap(), 1);
/// ```
#[derive(Default)]
pub struct IgesDataFreeFormatEntity {
    parent: IgesDataUndefinedEntity,
    thenegptrs: SharedState<Handle<HSequence<i32>>>,
}
impl_transient!(
    IgesDataFreeFormatEntity,
    "IgesDataFreeFormatEntity",
    IgesDataUndefinedEntity,
    parent
);

impl std::ops::Deref for IgesDataFreeFormatEntity {
    type Target = IgesDataUndefinedEntity;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl IgesDataEntityKind for IgesDataFreeFormatEntity {
    fn iges_entity(&self) -> &IgesDataIgesEntity {
        self.parent.iges_entity()
    }
}

impl IgesDataFreeFormatEntity {
    /// An entity with no parameter.
    pub fn new() -> Self {
        Self::default()
    }
    fn content(&self) -> Handle<InterfaceUndefinedContent> {
        self.parent.undefined_content()
    }
    /// Sets the type number; the form goes back to 0.
    pub fn set_type_number(&self, typenum: i32) {
        self.init_type_and_form(typenum, 0);
    }
    /// Sets the form number.
    pub fn set_form_number(&self, formnum: i32) {
        self.init_type_and_form(self.type_number(), formnum);
    }
    /// The count of parameters.
    ///
    /// # Errors
    /// A null object failure for a null content.
    pub fn nb_params(&self) -> Result<i32, Failure> {
        let content = self.content();
        Ok(iges_live(&content, "free format entity")?.nb_params())
    }
    /// Whether a parameter is an IGES entity, its type, the entity and the literal.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_data(
        &self,
        num: i32,
    ) -> Result<
        (
            bool,
            InterfaceParamType,
            Handle<IgesDataIgesEntity>,
            Handle<HAsciiString>,
        ),
        Failure,
    > {
        let content = self.content();
        let cont = iges_live(&content, "free format entity")?;
        let (is_entity, ptype, entity, literal) = cont.param_data(num)?;
        let entity = entity.cast::<IgesDataIgesEntity>();
        Ok((is_entity && entity.is_some(), ptype, entity, literal))
    }
    /// The type of a parameter.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_type(&self, num: i32) -> Result<InterfaceParamType, Failure> {
        let content = self.content();
        iges_live(&content, "free format entity")?.param_type(num)
    }
    /// Whether a parameter is an entity.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn is_param_entity(&self, num: i32) -> Result<bool, Failure> {
        let content = self.content();
        iges_live(&content, "free format entity")?.is_param_entity(num)
    }
    /// The entity of a parameter.
    ///
    /// # Errors
    /// An interface failure when the parameter is no entity.
    pub fn param_entity(&self, num: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let content = self.content();
        Ok(iges_live(&content, "free format entity")?
            .param_entity(num)?
            .cast())
    }
    /// Whether a parameter is to be written as a negative pointer.
    pub fn is_negative_pointer(&self, num: i32) -> bool {
        let pointers = self.thenegptrs.read();
        pointers
            .get()
            .is_some_and(|list| list.sequence(|items| items.iter().any(|item| *item == num)))
    }
    /// The literal of a parameter.
    ///
    /// # Errors
    /// An interface failure when the parameter is an entity.
    pub fn param_value(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
        let content = self.content();
        iges_live(&content, "free format entity")?.param_value(num)
    }
    /// The parameters to write as negative pointers; null when none.
    pub fn negative_pointers(&self) -> Handle<HSequence<i32>> {
        self.thenegptrs.read().clone()
    }
    /// Adds a literal parameter.
    ///
    /// # Errors
    /// A failure of the content.
    pub fn add_literal(
        &self,
        ptype: InterfaceParamType,
        val: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        let content = self.content();
        iges_live(&content, "free format entity")?.add_literal(ptype, val)
    }
    /// Adds a literal parameter from a text.
    ///
    /// # Errors
    /// A failure of the content.
    pub fn add_literal_text(
        &self,
        ptype: InterfaceParamType,
        val: impl AsRef<[u8]>,
    ) -> Result<(), Failure> {
        self.add_literal(ptype, &Handle::new(HAsciiString::new(val)))
    }
    /// Adds an entity parameter, to write as a negative pointer when `negative`.
    ///
    /// # Errors
    /// A failure of the content.
    pub fn add_entity(
        &self,
        ptype: InterfaceParamType,
        ent: &Handle<IgesDataIgesEntity>,
        negative: bool,
    ) -> Result<(), Failure> {
        let content = self.content();
        iges_live(&content, "free format entity")?.add_entity(ptype, &ent.cast())?;
        if !negative {
            return Ok(());
        }
        let count = self.nb_params()?;
        let mut pointers = self.thenegptrs.write();
        if pointers.is_null() {
            *pointers = Handle::new(HSequence::default());
        }
        if let Some(list) = pointers.get() {
            list.append(count);
        }
        Ok(())
    }
    /// Adds the count of a list, then its entities; a null list adds the count 0.
    ///
    /// # Errors
    /// A failure of the content.
    pub fn add_entities(
        &self,
        ents: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        let Some(list) = ents.get() else {
            return self.add_literal_text(InterfaceParamType::ParamInteger, "0");
        };
        let (lower, upper, length) = {
            let items = list.array1();
            (items.lower(), items.upper(), items.length())
        };
        self.add_literal(
            InterfaceParamType::ParamInteger,
            &Handle::new(HAsciiString::from_integer(length)),
        )?;
        for i in lower..=upper {
            let entity = iges_array1_item(ents, i, "add_entities")?;
            self.add_entity(InterfaceParamType::ParamIdent, &entity, false)?;
        }
        Ok(())
    }
    /// Adds parameters to write as negative pointers.
    pub fn add_negative_pointers(&self, list: &Handle<HSequence<i32>>) {
        let added = list.get().map_or_else(Vec::new, |live| {
            live.sequence(|items| items.iter().copied().collect())
        });
        let mut pointers = self.thenegptrs.write();
        if pointers.is_null() {
            *pointers = Handle::new(HSequence::default());
        }
        if let Some(own) = pointers.get() {
            for item in added {
                own.append(item);
            }
        }
    }
    /// Forgets the negative pointers.
    pub fn clear_negative_pointers(&self) {
        self.thenegptrs.write().nullify();
    }
    /// Writes the parameters, the negative pointers negated.
    ///
    /// # Errors
    /// A failure of the writer.
    pub fn write_own_params(&self, iw: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        let negatives: Vec<i32> = self.thenegptrs.read().get().map_or_else(Vec::new, |list| {
            list.sequence(|items| items.iter().copied().collect())
        });
        let mut next_negative = 0;
        let content = self.content();
        let cont = iges_live(&content, "free format entity")?;
        for i in 1..=cont.nb_params() {
            if cont.param_type(i)? == InterfaceParamType::ParamVoid {
                iw.send_void()?;
            } else if cont.is_param_entity(i)? {
                let negative = negatives.get(next_negative) == Some(&i);
                next_negative += usize::from(negative);
                let entity = cont.param_entity(i)?.cast::<IgesDataIgesEntity>();
                iw.send_entity(&entity, negative)?;
            } else {
                iw.send_string(&cont.param_value(i)?)?;
            }
        }
        Ok(())
    }
}
