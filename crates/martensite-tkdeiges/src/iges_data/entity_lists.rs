//! The notions derived from the directory part, and the optional lists of an entity.
use super::entity::{IGES_FLAG_ASSOCS, IGES_FLAG_PROPS};
use super::{
    IgesDataEntityKind, IgesDataIgesEntity, IgesDataNameEntity, IgesDataSingleParentEntity,
    iges_entity_of,
};
use martensite_tkernel::{
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::{GeneralTransform3, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceEntityList, interface_error,
};

impl IgesDataIgesEntity {
    /// Whether one single parent entity is among the properties.
    pub fn has_one_parent(&self) -> bool {
        self.nb_typed_properties(<dyn IgesDataSingleParentEntity>::static_type()) == 1
    }
    /// The parent of the single parent entity among the properties.
    ///
    /// # Errors
    /// An interface failure unless exactly one is there.
    pub fn unique_parent(&self) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let count = self.nb_typed_properties(<dyn IgesDataSingleParentEntity>::static_type());
        if count != 1 {
            return Err(interface_error(&format!(
                "IGES entity: unique parent asked with {count} single parent properties; expected 1"
            )));
        }
        let link = self
            .typed_property(<dyn IgesDataSingleParentEntity>::static_type(), 0)?
            .cast::<dyn IgesDataSingleParentEntity>();
        Ok(link
            .get()
            .map_or_else(Handle::default, IgesDataSingleParentEntity::single_parent))
    }
    /// The location the transformation gives, the identity without one.
    ///
    /// # Errors
    /// A failure of the transformation entity.
    pub fn location(&self) -> Result<GeneralTransform3, Failure> {
        match self.transf().get() {
            Some(transf) => transf.value(),
            None => Ok(GeneralTransform3::default()),
        }
    }
    /// [`Self::location`] without its translation part, for vectors.
    ///
    /// # Errors
    /// A failure of the transformation entity.
    pub fn vector_location(&self) -> Result<GeneralTransform3, Failure> {
        let mut location = self.location()?;
        if self.has_transf() {
            location.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
        }
        Ok(location)
    }
    /// The location combined with the one of the single parent, when there is one.
    ///
    /// # Errors
    /// A failure of a transformation entity.
    pub fn compound_location(&self) -> Result<GeneralTransform3, Failure> {
        let mut location = self.location()?;
        if !self.has_one_parent() {
            return Ok(location);
        }
        if let Some(parent) = self.unique_parent()?.get() {
            location.pre_multiply(parent.compound_location()?);
        }
        Ok(location)
    }
    /// Whether the entity has a short label, or one name among its properties.
    pub fn has_name(&self) -> bool {
        self.has_short_label() || self.nb_typed_properties(name_kind()) == 1
    }
    /// The first name among the properties, else the short label with its subscript between
    /// parentheses; null when neither is there.
    ///
    /// # Errors
    /// A failure of the property list.
    pub fn name_value(&self) -> Result<Handle<HAsciiString>, Failure> {
        if self.nb_typed_properties(name_kind()) > 0 {
            let name = self
                .typed_property(name_kind(), 1)?
                .cast::<dyn IgesDataNameEntity>();
            return Ok(name
                .get()
                .map_or_else(Handle::default, IgesDataNameEntity::value));
        }
        let label = self.short_label();
        let Some(text) = label.get() else {
            return Ok(Handle::default());
        };
        let subscript = self.state.read().the_sub_script_n;
        if subscript < 0 {
            return Ok(label.clone());
        }
        let mut named = text.string().as_bytes().to_vec();
        named.extend_from_slice(format!("({subscript})").as_bytes());
        Ok(Handle::new(HAsciiString::new(named)))
    }
    /// Whether associativities are there, or were read as such.
    pub fn are_present_associativities(&self) -> bool {
        let state = self.state.read();
        !state.the_assocs.is_empty() || state.the_status_num & IGES_FLAG_ASSOCS != 0
    }
    /// The count of associativities.
    pub fn nb_associativities(&self) -> i32 {
        self.state.read().the_assocs.nb_entities()
    }
    /// The associativities.
    pub fn associativities(&self) -> InterfaceEntityIterator {
        let mut iter = InterfaceEntityIterator::new();
        self.state.read().the_assocs.fill_iterator(&mut iter);
        iter
    }
    /// The count of associativities of a type.
    pub fn nb_typed_associativities(&self, atype: &RuntimeType) -> i32 {
        self.state.read().the_assocs.nb_typed_entities(atype)
    }
    /// The associativity of a type.
    ///
    /// # Errors
    /// An interface failure unless exactly one is there.
    pub fn typed_associativity(
        &self,
        atype: &RuntimeType,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let found = self.state.read().the_assocs.typed_entity(atype, 0)?;
        Ok(iges_entity_of(&found))
    }
    /// Adds an associativity. Reserved to the reading and copying of entities.
    pub(super) fn add_associativity(&self, ent: &Handle<StandardTransient>) -> Result<(), Failure> {
        self.state.write().the_assocs.append(ent)
    }
    /// Removes an associativity.
    fn remove_associativity(&self, ent: &Handle<StandardTransient>) -> Result<(), Failure> {
        self.state.write().the_assocs.remove(ent)
    }
    /// Sets the associativities as read, and marks them as present.
    pub(crate) fn load_associativities(&self, list: &InterfaceEntityList) {
        let mut state = self.state.write();
        state.the_assocs = list.clone();
        state.the_status_num |= IGES_FLAG_ASSOCS;
    }
    /// Forgets the associativities.
    pub fn clear_associativities(&self) {
        self.state.write().the_assocs.clear();
    }
    /// Records `this` as an associativity of `ent`, when `ent` is not null.
    ///
    /// # Errors
    /// A null object failure when `this` is null.
    pub fn associate<T: ?Sized + RuntimeTyped + IgesDataEntityKind>(
        this: &Handle<T>,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<(), Failure> {
        let Some(target) = ent.get() else {
            return Ok(());
        };
        target.add_associativity(&this.cast())
    }
    /// Removes `this` from the associativities of `ent`, when `ent` is not null.
    ///
    /// # Errors
    /// A failure when `this` is null or not there.
    pub fn dissociate<T: ?Sized + RuntimeTyped + IgesDataEntityKind>(
        this: &Handle<T>,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<(), Failure> {
        let Some(target) = ent.get() else {
            return Ok(());
        };
        target.remove_associativity(&this.cast())
    }
    /// Whether properties are there, or were read as such.
    pub fn are_present_properties(&self) -> bool {
        let state = self.state.read();
        !state.the_props.is_empty() || state.the_status_num & IGES_FLAG_PROPS != 0
    }
    /// The count of properties.
    pub fn nb_properties(&self) -> i32 {
        self.state.read().the_props.nb_entities()
    }
    /// The properties.
    pub fn properties(&self) -> InterfaceEntityIterator {
        let mut iter = InterfaceEntityIterator::new();
        self.state.read().the_props.fill_iterator(&mut iter);
        iter
    }
    /// The count of properties of a type.
    pub fn nb_typed_properties(&self, atype: &RuntimeType) -> i32 {
        self.state.read().the_props.nb_typed_entities(atype)
    }
    /// The property of a type: the one at rank `anum`, or when 0 the only one.
    ///
    /// # Errors
    /// An interface failure when `anum` is 0 and several are there, or when none is at the rank.
    pub fn typed_property(
        &self,
        atype: &RuntimeType,
        anum: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let found = self.state.read().the_props.typed_entity(atype, anum)?;
        Ok(iges_entity_of(&found))
    }
    /// Adds a property.
    ///
    /// # Errors
    /// A null object failure for a null entity.
    pub fn add_property<T: ?Sized + RuntimeTyped + IgesDataEntityKind>(
        &self,
        ent: &Handle<T>,
    ) -> Result<(), Failure> {
        self.state.write().the_props.append(&ent.cast())
    }
    /// Removes a property.
    ///
    /// # Errors
    /// A failure for a null entity, or one that is not there.
    pub fn remove_property<T: ?Sized + RuntimeTyped + IgesDataEntityKind>(
        &self,
        ent: &Handle<T>,
    ) -> Result<(), Failure> {
        self.state.write().the_props.remove(&ent.cast())
    }
    /// Sets the properties as read, and marks them as present.
    pub(crate) fn load_properties(&self, list: &InterfaceEntityList) {
        let mut state = self.state.write();
        state.the_props = list.clone();
        state.the_status_num |= IGES_FLAG_PROPS;
    }
    /// Forgets the properties.
    pub fn clear_properties(&self) {
        self.state.write().the_props.clear();
    }
}

fn name_kind() -> &'static RuntimeType {
    <dyn IgesDataNameEntity>::static_type()
}
