#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::IgesBasicGroup;
use crate::iges_basic::entity_shape::iges_array1_lower;
use crate::iges_data::iges_array1_item;
use crate::iges_data::{IgesDataIgesEntity, IgesDataSingleParentEntity};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines OrderedGroup, Type `402` Form `14`
/// this class defines an Ordered Group with back pointers
/// Allows a collection of a set of entities to be
/// maintained as a single entity, but the group is
/// ordered.
/// It inherits from Group
custom_default IgesBasicOrderedGroup, IgesBasicOrderedGroupState, "IgesBasicOrderedGroup", parent IgesBasicGroup, {  }
}
impl Default for IgesBasicOrderedGroup {
    fn default() -> Self {
        let this = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        this.init_type_and_form(402, 14);
        this
    }
}
impl IgesBasicOrderedGroup {
    #[doc = "`IgesBasicOrderedGroup()`."]
    pub fn new() -> Self {
        Self::default()
    }
}
iges_entity! {
/// defines OrderedGroupWithoutBackP, Type `402` Form `15`
/// Allows a collection of a set of entities to be
/// maintained as a single entity, but the group is
/// ordered and there are no back pointers.
/// It inherits from Group
custom_default IgesBasicOrderedGroupWithoutBackP, IgesBasicOrderedGroupWithoutBackPState, "IgesBasicOrderedGroupWithoutBackP", parent IgesBasicGroup, {  }
}
impl Default for IgesBasicOrderedGroupWithoutBackP {
    fn default() -> Self {
        let this = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        this.init_type_and_form(402, 15);
        this
    }
}
impl IgesBasicOrderedGroupWithoutBackP {
    #[doc = "`IgesBasicOrderedGroupWithoutBackP()`."]
    pub fn new() -> Self {
        Self::default()
    }
}
iges_entity! {
/// defines SingleParent, Type `402` Form `9`
/// It defines a logical structure of one independent
/// (parent) entity and one or more subordinate (children)
/// entities
IgesBasicSingleParent, IgesBasicSingleParentState, "IgesBasicSingleParent", kind dyn IgesDataSingleParentEntity, { nb_parent_entities: i32, parent_entity: Handle<IgesDataIgesEntity>, children: Handle<HArray1<Handle<IgesDataIgesEntity>>> }
}
impl IgesBasicSingleParent {
    iges_methods! {
    get nb_parent_entities = nb_parent_entities: i32;
    get single_parent = parent_entity: Handle<IgesDataIgesEntity>;
    len nb_children = children;
    item child = children: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// SingleParent
    /// - nbParentEntities : Indicates number of Parents, always = 1
    /// - aParentEntity    : Used to hold the Parent Entity
    /// - allChildren      : Used to hold the children
    pub fn init(
        &self,
        nb_parent_entities: i32,
        a_parent_entity: &Handle<IgesDataIgesEntity>,
        all_children: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_children.is_null()
            && iges_array1_lower(all_children, "IgesBasicSingleParent::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesBasicSingleParent : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.parent_entity = a_parent_entity.clone();
            state.children = all_children.clone();
            state.nb_parent_entities = nb_parent_entities;
        }
        self.init_type_and_form(402, 9);
        Ok(())
    }
}
impl IgesDataSingleParentEntity for IgesBasicSingleParent {
    fn single_parent(&self) -> Handle<IgesDataIgesEntity> {
        IgesBasicSingleParent::single_parent(self)
    }
    fn nb_children(&self) -> Result<i32, Failure> {
        Ok(IgesBasicSingleParent::nb_children(self))
    }
    fn child(&self, num: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        IgesBasicSingleParent::child(self, num)
    }
}
iges_entity! {
/// defines SingularSubfigure, Type `408` Form `0`
/// Defines the occurrence of a single instance of the
/// defined Subfigure.
IgesBasicSingularSubfigure, IgesBasicSingularSubfigureState, "IgesBasicSingularSubfigure", parent IgesDataIgesEntity, { subfigure_def: Handle<IgesBasicSubfigureDef>, translation: Xyz, scale_factor: f64, has_scale_factor: bool }
}
impl IgesBasicSingularSubfigure {
    iges_methods! {
    get subfigure = subfigure_def: Handle<IgesBasicSubfigureDef>;
    get translation = translation: Xyz;
    get scale_factor = scale_factor: f64;
    }
    /// This method is used to set the fields of the class
    /// SingularSubfigure
    /// - aSubfigureDef : the Subfigure Definition entity
    /// - aTranslation  : used to store the X,Y,Z coord
    /// - hasScale      : Indicates the presence of scale factor
    /// - aScale        : Used to store the scale factor
    pub fn init(
        &self,
        a_subfigure_def: &Handle<IgesBasicSubfigureDef>,
        a_translation: Xyz,
        has_scale: bool,
        a_scale: f64,
    ) {
        {
            let mut state = self.state.write();
            state.subfigure_def = a_subfigure_def.clone();
            state.translation = a_translation;
            state.has_scale_factor = has_scale;
            state.scale_factor = a_scale;
        }
        self.init_type_and_form(408, 0);
    }
    /// returns a boolean indicating whether scale factor
    /// is present or not
    pub fn has_scale_factor(&self) -> bool {
        self.state.read().has_scale_factor
    }
    /// returns the Translation after transformation
    pub fn transformed_translation(&self) -> Result<Xyz, Failure> {
        let the_translation = self.state.read().translation;
        let mut tmp: Xyz = the_translation;
        if self.has_transf() {
            self.location()?.transforms(&mut tmp);
        }
        Ok(tmp)
    }
}
iges_entity! {
/// defines SubfigureDef, Type `308` Form `0`
/// This Entity permits a single definition of a detail to
/// be utilized in multiple instances in the creation of
/// the whole picture
IgesBasicSubfigureDef, IgesBasicSubfigureDefState, "IgesBasicSubfigureDef", parent IgesDataIgesEntity, { depth: i32, name: Handle<HAsciiString>, assoc_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> }
}
impl IgesBasicSubfigureDef {
    iges_methods! {
    get depth = depth: i32;
    get name = name: Handle<HAsciiString>;
    len nb_entities = assoc_entities;
    item associated_entity = assoc_entities: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// SubfigureDef
    /// - aDepth           : It indicates the amount of nesting
    /// - aName            : the subfigure name
    /// - allAssocEntities : the associated entities
    pub fn init(
        &self,
        a_depth: i32,
        a_name: &Handle<HAsciiString>,
        all_assoc_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_assoc_entities.is_null()
            && iges_array1_lower(all_assoc_entities, "IgesBasicSubfigureDef::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesBasicSubfigureDef : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.depth = a_depth;
            state.name = a_name.clone();
            state.assoc_entities = all_assoc_entities.clone();
        }
        self.init_type_and_form(308, 0);
        Ok(())
    }
    /// returns the specific entity as indicated by Index
    /// raises exception if Index `= 0 or Index ` NbEntities()
    pub fn value(&self, index: i32) -> Result<Handle<StandardTransient>, Failure> {
        let the_assoc_entities = self.state.read().assoc_entities.clone();
        Ok(
            iges_array1_item(&the_assoc_entities, index, "IgesBasicSubfigureDef::value")?
                .cast::<StandardTransient>(),
        )
    }
}
