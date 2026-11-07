//! IgesDrawNetworkSubfigureDef.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGESNetworkSubfigureDef,
///
/// This class differs from the ordinary subfigure definition
/// in that it defines a specialized subfigure, one whose
/// instances may participate in networks.
///
/// The Number of associated(child) Connect Point Entities
/// in the Network Subfigure Instance must match the number
/// in the Network Subfigure Definition, their order must
/// be identical, and any unused points of connection in
/// the instance must be indicated by a null(zero) pointer.
IgesDrawNetworkSubfigureDef, IgesDrawNetworkSubfigureDefState, "IgesDrawNetworkSubfigureDef", parent IgesDataIgesEntity, {
the_depth: i32,
the_name: Handle<HAsciiString>,
the_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_type_flag: i32,
the_designator: Handle<HAsciiString>,
the_designator_template: Handle<IgesGraphTextDisplayTemplate>,
the_point_entities: Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
}
}

impl IgesDrawNetworkSubfigureDef {
    iges_methods! {
    get depth = the_depth: i32;
    get name = the_name: Handle<HAsciiString>;
    get type_flag = the_type_flag: i32;
    get designator = the_designator: Handle<HAsciiString>;
    get designator_template = the_designator_template: Handle<IgesGraphTextDisplayTemplate>;
    }
    /// This method is used to set fields of the class
    /// NetworkSubfigureDef
    /// - aDepth           : Depth of Subfigure
    ///   (indicating the amount of nesting)
    /// - aName            : Subfigure Name
    /// - allEntities      : Associated subfigures Entities exclusive
    ///   of primary reference designator and
    ///   Control Points.
    /// - aTypeFlag        : Type flag determines which Entity
    ///   belongs in which design
    ///   (Logical design or Physical design)
    /// - aDesignator      : Designator HAsciiString and its Template
    /// - allPointEntities : Associated Connect Point Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_depth: i32,
        a_name: &Handle<HAsciiString>,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        a_type_flag: i32,
        a_designator: &Handle<HAsciiString>,
        a_template: &Handle<IgesGraphTextDisplayTemplate>,
        all_point_entities: &Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
    ) -> Result<(), Failure> {
        if !all_point_entities.is_null() {
            if (iges_array1_lower(all_point_entities, "IgesDrawNetworkSubfigureDef::init")? != 1)
                || (iges_array1_lower(all_entities, "IgesDrawNetworkSubfigureDef::init")? != 1)
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawNetworkSubfigureDef : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_depth = a_depth;
            state.the_name = a_name.clone();
            state.the_entities = all_entities.clone();
            state.the_type_flag = a_type_flag;
            state.the_designator = a_designator.clone();
            state.the_designator_template = a_template.clone();
            state.the_point_entities = all_point_entities.clone();
        }
        self.init_type_and_form(320, 0);
        Ok(())
    }
    /// returns Number of Associated(child) entries in subfigure exclusive
    /// of primary reference designator and Control Points
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_entities(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_entities,
            "IgesDrawNetworkSubfigureDef::nb_entities",
        )
    }
    /// returns the Index'th entity in subfigure exclusive of primary
    /// reference designator and Control Points
    /// raises exception if Index at most0 or Index &gt; NbEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_entities,
            index,
            "IgesDrawNetworkSubfigureDef::entity",
        )
    }
    /// returns True if Text Display Template is specified for
    /// primary designator else returns False
    pub fn has_designator_template(&self) -> bool {
        !(self.state.read().the_designator_template.is_null())
    }
    /// returns the Number Of Associated(child) Connect Point Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_point_entities(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_point_entities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_point_entities,
                "IgesDrawNetworkSubfigureDef::nb_point_entities",
            )?
        })
    }
    /// returns True is Index'th Associated Connect Point Entity is present
    /// else returns False
    /// raises exception if Index is out of bound
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn has_point_entity(&self, index: i32) -> Result<bool, Failure> {
        if self.state.read().the_point_entities.is_null() {
            return Ok(false);
        }
        return Ok(!(iges_array1_item(
            &self.state.read().the_point_entities,
            index,
            "IgesDrawNetworkSubfigureDef::has_point_entity",
        )?
        .is_null()));
    }
    /// returns the Index'th Associated Connect Point Entity
    /// raises exception if Index at most 0 or Index &gt; NbPointEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn point_entity(&self, index: i32) -> Result<Handle<IgesDrawConnectPoint>, Failure> {
        iges_array1_item(
            &self.state.read().the_point_entities,
            index,
            "IgesDrawNetworkSubfigureDef::point_entity",
        )
    }
}
