#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, IgesDataNameEntity};
use crate::iges_data::{iges_array1_item, iges_array1_set, iges_new_array1};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines AssocGroupType, Type `406` Form `23`
/// Used to assign an unambiguous identification to a Group
/// Associativity.
IgesBasicAssocGroupType, IgesBasicAssocGroupTypeState, "IgesBasicAssocGroupType", parent IgesDataIgesEntity, { nb_data: i32, type_: i32, name: Handle<HAsciiString> }
}
impl IgesBasicAssocGroupType {
    iges_methods! {
    get nb_data = nb_data: i32;
    get assoc_type = type_: i32;
    get name = name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// AssocGroupType
    /// - nbDataFields : number of parameter data fields = 2
    /// - aType        : type of attached associativity
    /// - aName        : identifier of associativity of type AType
    pub fn init(&self, nb_data_fields: i32, a_type: i32, a_name: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.nb_data = nb_data_fields;
            state.type_ = a_type;
            state.name = a_name.clone();
        }
        self.init_type_and_form(406, 23);
    }
}
iges_entity! {
/// defines ExternalRefFile, Type `416` Form `1`
/// Used when entire reference file is to be instanced
IgesBasicExternalRefFile, IgesBasicExternalRefFileState, "IgesBasicExternalRefFile", parent IgesDataIgesEntity, { ext_ref_file_identifier: Handle<HAsciiString> }
}
impl IgesBasicExternalRefFile {
    iges_methods! {
    get file_id = ext_ref_file_identifier: Handle<HAsciiString>;
    }
    /// This method is used to set the field of the class
    /// ExternalRefFile
    /// - aFileIdent : External Reference File Identifier
    pub fn init(&self, a_file_ident: &Handle<HAsciiString>) {
        self.state.write().ext_ref_file_identifier = a_file_ident.clone();
        self.init_type_and_form(416, 1);
    }
}
iges_entity! {
/// defines ExternalRefFileIndex, Type `402` Form `12`
/// Contains a list of the symbolic names used by the
/// referencing files and the DE pointers to the
/// corresponding definitions within the referenced file
IgesBasicExternalRefFileIndex, IgesBasicExternalRefFileIndexState, "IgesBasicExternalRefFileIndex", parent IgesDataIgesEntity, { names: Handle<HArray1<Handle<HAsciiString>>>, entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> }
}
impl IgesBasicExternalRefFileIndex {
    iges_methods! {
    len_checked nb_entries = names;
    item name = names: Handle<HAsciiString>;
    item entity = entities: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// ExternalRefFileIndex
    /// - aNameArray  : External Reference Entity symbolic names
    /// - allEntities : External Reference Entities
    ///   raises exception if array lengths are not equal
    ///   if size of aNameArray is not equal to size of allEntities
    pub fn init(
        &self,
        a_name_array: &Handle<HArray1<Handle<HAsciiString>>>,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(a_name_array, "IgesBasicExternalRefFileIndex::init")? != 1
            || iges_array1_lower(all_entities, "IgesBasicExternalRefFileIndex::init")? != 1
            || iges_array1_length(a_name_array, "IgesBasicExternalRefFileIndex::init")?
                != iges_array1_length(all_entities, "IgesBasicExternalRefFileIndex::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesBasicExternalRefFileIndex: Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.names = a_name_array.clone();
            state.entities = all_entities.clone();
        }
        self.init_type_and_form(402, 12);
        Ok(())
    }
}
iges_entity! {
/// defines ExternalRefFileName, Type `416` Form `0-2`
/// Used when single definition from the reference file is
/// required or for external logical references where an
/// entity in one file relates to an entity in another file
IgesBasicExternalRefFileName, IgesBasicExternalRefFileNameState, "IgesBasicExternalRefFileName", parent IgesDataIgesEntity, { ext_ref_file_identifier: Handle<HAsciiString>, ext_ref_entity_symb_name: Handle<HAsciiString> }
}
impl IgesBasicExternalRefFileName {
    iges_methods! {
    get file_id = ext_ref_file_identifier: Handle<HAsciiString>;
    get reference_name = ext_ref_entity_symb_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// ExternalRefFileName
    /// - aFileIdent : External Reference File Identifier
    /// - anExtName  : External Reference Entity Symbolic Name
    pub fn init(&self, a_file_ident: &Handle<HAsciiString>, an_ext_name: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.ext_ref_file_identifier = a_file_ident.clone();
            state.ext_ref_entity_symb_name = an_ext_name.clone();
        }
        self.init_type_and_form(416, self.form_number());
    }
    /// Changes FormNumber to be 2 if `mode` is True (For Entity)
    /// or 0 if `mode` is False (For Definition)
    pub fn set_for_entity(&self, f: bool) {
        self.init_type_and_form(416, if f { 2 } else { 0 });
    }
}
iges_entity! {
/// defines ExternalRefLibName, Type `416` Form `4`
/// Used when it is assumed that a copy of the subfigure
/// exists in native form in a library on the receiving
/// system
IgesBasicExternalRefLibName, IgesBasicExternalRefLibNameState, "IgesBasicExternalRefLibName", parent IgesDataIgesEntity, { lib_name: Handle<HAsciiString>, ext_ref_entity_symb_name: Handle<HAsciiString> }
}
impl IgesBasicExternalRefLibName {
    iges_methods! {
    get library_name = lib_name: Handle<HAsciiString>;
    get reference_name = ext_ref_entity_symb_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// ExternalRefLibName
    /// - aLibName  : Name of library in which ExtName resides
    /// - anExtName : External Reference Entity Symbolic Name
    pub fn init(&self, a_lib_name: &Handle<HAsciiString>, an_ext_name: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.lib_name = a_lib_name.clone();
            state.ext_ref_entity_symb_name = an_ext_name.clone();
        }
        self.init_type_and_form(416, 4);
    }
}
iges_entity! {
/// defines ExternalRefName, Type `416` Form `3`
/// Used when it is assumed that a copy of the subfigure
/// exists in native form on the receiving system
IgesBasicExternalRefName, IgesBasicExternalRefNameState, "IgesBasicExternalRefName", parent IgesDataIgesEntity, { ext_ref_entity_symb_name: Handle<HAsciiString> }
}
impl IgesBasicExternalRefName {
    iges_methods! {
    get reference_name = ext_ref_entity_symb_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// ExternalRefName
    /// - anExtName : External Reference Entity Symbolic Name
    pub fn init(&self, an_ext_name: &Handle<HAsciiString>) {
        self.state.write().ext_ref_entity_symb_name = an_ext_name.clone();
        self.init_type_and_form(416, 3);
    }
}
iges_entity! {
/// defines ExternalReferenceFile, Type `406` Form `12`
/// References definitions residing in another file
IgesBasicExternalReferenceFile, IgesBasicExternalReferenceFileState, "IgesBasicExternalReferenceFile", parent IgesDataIgesEntity, { names: Handle<HArray1<Handle<HAsciiString>>> }
}
impl IgesBasicExternalReferenceFile {
    iges_methods! {
    len_checked nb_list_entries = names;
    item name = names: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// ExternalReferenceFile
    /// - aNameArray : External Reference File Names
    pub fn init(
        &self,
        a_name_array: &Handle<HArray1<Handle<HAsciiString>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(a_name_array, "IgesBasicExternalReferenceFile::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesBasicExternalReferenceFile : Init",
            ));
        }
        self.state.write().names = a_name_array.clone();
        self.init_type_and_form(406, 12);
        Ok(())
    }
}
iges_entity! {
/// defines Group, Type `402` Form `1`
/// The Group Associativity allows a collection of a set
/// of entities to be maintained as a single, logical
/// entity
/// Group, OrderedGroup, GroupWithoutBackP, OrderedGroupWithoutBackP
/// share the same definition (class Group), form number changes
/// non Ordered, non WithoutBackP : form  1
/// non Ordered,     WithoutBackP : form  7
/// Ordered, non WithoutBackP : form 14
/// Ordered,     WithoutBackP : form 15
custom_default IgesBasicGroup, IgesBasicGroupState, "IgesBasicGroup", parent IgesDataIgesEntity, { entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> }
}
impl Default for IgesBasicGroup {
    fn default() -> Self {
        let this = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        this.init_type_and_form(402, 1);
        this
    }
}
impl IgesBasicGroup {
    #[doc = "`IgesBasicGroup()`."]
    pub fn new() -> Self {
        Self::default()
    }
}
impl IgesBasicGroup {
    iges_methods! {
    len nb_entities = entities;
    item entity = entities: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class Group
    /// - allEntities : Used to store pointers to members of
    ///   the Group.
    pub fn init(
        &self,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_entities.is_null() && iges_array1_lower(all_entities, "IgesBasicGroup::init")? != 1
        {
            return Err(exceptions::dimension_mismatch("IgesBasicGroup : Init"));
        }
        self.state.write().entities = all_entities.clone();
        if self.form_number() == 0 {
            self.init_type_and_form(402, 1);
        }
        Ok(())
    }
    /// Sets a Group to be, or not to be Ordered (according mode)
    pub fn set_ordered(&self, mode: bool) {
        let fn_: i32 = self.form_number();
        if mode {
            if fn_ == 0 || fn_ == 1 {
                self.init_type_and_form(402, 14);
            } else if fn_ == 7 {
                self.init_type_and_form(402, 15);
            }
        } else if fn_ == 14 {
            self.init_type_and_form(402, 1);
        } else if fn_ == 15 {
            self.init_type_and_form(402, 7);
        }
    }
    /// Sets a Group to be, or not to be WithoutBackP
    pub fn set_without_back_p(&self, mode: bool) {
        let fn_: i32 = self.form_number();
        if mode {
            if fn_ == 0 || fn_ == 1 {
                self.init_type_and_form(402, 7);
            } else if fn_ == 14 {
                self.init_type_and_form(402, 15);
            }
        } else if fn_ == 7 {
            self.init_type_and_form(402, 1);
        } else if fn_ == 15 {
            self.init_type_and_form(402, 14);
        }
    }
    /// Returns True if `me` is Ordered
    pub fn is_ordered(&self) -> bool {
        let fn_: i32 = self.form_number();
        fn_ > 7
    }
    /// Returns True if `me` is WithoutBackP
    pub fn is_without_back_p(&self) -> bool {
        let fn_: i32 = self.form_number();
        fn_ == 7 || fn_ == 15
    }
    /// Enforce a new value for the type and form
    pub fn set_user(&self, type_: i32, form: i32) -> Result<(), Failure> {
        if type_ <= 5000 {
            return Err(exceptions::out_of_range("IgesBasicGroup::SetUser"));
        }
        self.init_type_and_form(type_, form);
        Ok(())
    }
    /// Changes the count of item
    /// If greater, new items are null
    /// If lower, old items are lost
    pub fn set_nb(&self, nb: i32) -> Result<(), Failure> {
        let the_entities = self.state.read().entities.clone();
        let mut oldnb: i32 = self.nb_entities();
        if nb == oldnb || nb <= 0 {
            return Ok(());
        }
        let newents: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        if oldnb > nb {
            oldnb = nb;
        }
        for i in 1..=oldnb {
            iges_array1_set(
                &newents,
                i,
                iges_array1_item(&the_entities, i, "IgesBasicGroup::set_nb")?,
                "IgesBasicGroup::set_nb",
            )?;
        }
        self.state.write().entities = newents.clone();
        Ok(())
    }
    /// returns the specific entity from the Group
    pub fn value(&self, index: i32) -> Result<Handle<StandardTransient>, Failure> {
        let the_entities = self.state.read().entities.clone();
        Ok(
            iges_array1_item(&the_entities, index, "IgesBasicGroup::value")?
                .cast::<StandardTransient>(),
        )
    }
    /// Sets a new value for item `Index`
    pub fn set_value(&self, index: i32, ent: &Handle<IgesDataIgesEntity>) -> Result<(), Failure> {
        let the_entities = self.state.read().entities.clone();
        iges_array1_set(
            &the_entities,
            index,
            ent.clone(),
            "IgesBasicGroup::set_value",
        )?;
        Ok(())
    }
}
iges_entity! {
/// defines GroupWithoutBackP, Type `402` Form `7`
/// this class defines a Group without back pointers
/// It inherits from Group
custom_default IgesBasicGroupWithoutBackP, IgesBasicGroupWithoutBackPState, "IgesBasicGroupWithoutBackP", parent IgesBasicGroup, {  }
}
impl Default for IgesBasicGroupWithoutBackP {
    fn default() -> Self {
        let this = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        this.init_type_and_form(402, 7);
        this
    }
}
impl IgesBasicGroupWithoutBackP {
    #[doc = "`IgesBasicGroupWithoutBackP()`."]
    pub fn new() -> Self {
        Self::default()
    }
}
iges_entity! {
/// defines Hierarchy, Type `406` Form `10`
/// Provides ability to control the hierarchy of each
/// directory entry attribute.
IgesBasicHierarchy, IgesBasicHierarchyState, "IgesBasicHierarchy", parent IgesDataIgesEntity, { nb_property_values: i32, line_font: i32, view: i32, entity_level: i32, blank_status: i32, line_weight: i32, color_num: i32 }
}
impl IgesBasicHierarchy {
    iges_methods! {
    get nb_property_values = nb_property_values: i32;
    get new_line_font = line_font: i32;
    get new_view = view: i32;
    get new_entity_level = entity_level: i32;
    get new_blank_status = blank_status: i32;
    get new_line_weight = line_weight: i32;
    get new_color_num = color_num: i32;
    }
    /// This method is used to set the fields of the class
    /// Hierarchy
    /// - nbPropVal     : Number of Property values = 6
    /// - aLineFont     : indicates the line font
    /// - aView         : indicates the view
    /// - aEntityLevel  : indicates the entity level
    /// - aBlankStatus  : indicates the blank status
    /// - aLineWt       : indicates the line weight
    /// - aColorNum     : indicates the color num
    ///   aLineFont, aView, aEntityLevel, aBlankStatus, aLineWt and
    ///   aColorNum can take 0 or 1.
    ///   0 : The directory entry attribute will apply to entities
    ///   physically subordinate to this entity.
    ///   1 : The directory entry attribute of this entity will not
    ///   apply to physically subordinate entities.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_line_font: i32,
        a_view: i32,
        an_entity_level: i32,
        a_blank_status: i32,
        a_line_wt: i32,
        a_color_num: i32,
    ) {
        {
            let mut state = self.state.write();
            state.line_font = a_line_font;
            state.view = a_view;
            state.entity_level = an_entity_level;
            state.blank_status = a_blank_status;
            state.line_weight = a_line_wt;
            state.color_num = a_color_num;
            state.nb_property_values = nb_prop_val;
        }
        self.init_type_and_form(406, 10);
    }
}
iges_entity! {
/// defines Name, Type `406` Form `15`
/// Used to specify a user defined name
IgesBasicName, IgesBasicNameState, "IgesBasicName", kind dyn IgesDataNameEntity, { nb_property_values: i32, name: Handle<HAsciiString> }
}
impl IgesBasicName {
    iges_methods! {
    get nb_property_values = nb_property_values: i32;
    get value = name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class Name
    /// - nbPropVal  : Number of property values, always = 1
    /// - aName      : Stores the Name
    pub fn init(&self, nb_prop_val: i32, a_name: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.name = a_name.clone();
            state.nb_property_values = nb_prop_val;
        }
        self.init_type_and_form(406, 15);
    }
}
impl IgesDataNameEntity for IgesBasicName {
    fn value(&self) -> Handle<HAsciiString> {
        IgesBasicName::value(self)
    }
}
